use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use reqwest::Client as HttpClient;
use rig_core::{
    client::{CompletionClient, ProviderClient},
    providers::gemini,
};
use serde::{Deserialize, Serialize};

use crate::{
    ai_models::{AiRepositoryAnalysis, RepositoryAiAnalysisResponse},
    models::{ErrorResponse, StoredRepositoryResponse},
    state::AppState,
};

type ApiError = (StatusCode, Json<ErrorResponse>);

#[derive(Serialize)]
struct GroqRequest<'a> {
    model: &'a str,
    messages: Vec<GroqMessage<'a>>,
    temperature: f32,
    response_format: GroqResponseFormat,
}

#[derive(Serialize)]
struct GroqMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct GroqResponseFormat {
    r#type: &'static str,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<GroqChoice>,
}

#[derive(Deserialize)]
struct GroqChoice {
    message: GroqResponseMessage,
}

#[derive(Deserialize)]
struct GroqResponseMessage {
    content: String,
}

pub async fn analyze_repository(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<RepositoryAiAnalysisResponse>, ApiError> {
    let repository = sqlx::query_as!(
        StoredRepositoryResponse,
        r#"
        SELECT
            id AS "database_id!", github_id, owner, name, full_name,
            description, stars, forks, open_issues, subscribers, language,
            license, archived, has_issues, url, default_branch, topics,
            github_created_at, github_updated_at, pushed_at, readme
        FROM repositories
        WHERE id = $1
        "#,
        repository_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|error| {
        eprintln!("Database error: {error}");
        internal_error("Failed to load repository")
    })?
    .ok_or_else(|| not_found("Repository not found"))?;

    let readme_excerpt: String = repository.readme.chars().take(18_000).collect();
    let prompt = format!(
        r#"Analyze this open-source GitHub repository.

Provide a concise technical summary, scores from 0 to 100 for documentation quality,
onboarding, and beginner friendliness, plus concrete strengths, weaknesses, and
actionable suggested improvements. Use only the supplied data and do not invent features.

Return valid JSON with exactly these fields:
{{"summary":"string","documentation_quality_score":0,"onboarding_score":0,"beginner_friendliness_score":0,"strengths":["string"],"weaknesses":["string"],"suggested_improvements":["string"]}}

Repository: {}
Description: {}
Language: {}
Stars: {}
Forks: {}
Open issues: {}

README:
{}"#,
        repository.full_name,
        repository.description.as_deref().unwrap_or("No description"),
        repository.language.as_deref().unwrap_or("Unknown"),
        repository.stars,
        repository.forks,
        repository.open_issues,
        readme_excerpt,
    );

    let (analysis, model) = match analyze_with_gemini(&state, &prompt).await {
        Ok(analysis) => (analysis, format!("gemini:{}", state.gemini_model)),
        Err(gemini_error) => {
            eprintln!("Gemini failed; trying Groq fallback: {gemini_error}");

            let api_key = state.groq_api_key.as_deref().ok_or_else(|| {
                internal_error("Gemini failed and GROQ_API_KEY is not configured")
            })?;

            let analysis = analyze_with_groq(
                api_key,
                &state.groq_model,
                &prompt,
            )
            .await
            .map_err(|error| {
                eprintln!("Groq fallback failed: {error}");
                internal_error("Both Gemini and Groq are temporarily unavailable")
            })?;

            (analysis, format!("groq:{}", state.groq_model))
        }
    };

    let stored_analysis = sqlx::query_as!(
        RepositoryAiAnalysisResponse,
        r#"
        INSERT INTO repository_ai_analyses (
            repository_id, summary, documentation_quality_score,
            onboarding_score, beginner_friendliness_score, strengths,
            weaknesses, suggested_improvements, model
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (repository_id) DO UPDATE SET
            summary = EXCLUDED.summary,
            documentation_quality_score = EXCLUDED.documentation_quality_score,
            onboarding_score = EXCLUDED.onboarding_score,
            beginner_friendliness_score = EXCLUDED.beginner_friendliness_score,
            strengths = EXCLUDED.strengths,
            weaknesses = EXCLUDED.weaknesses,
            suggested_improvements = EXCLUDED.suggested_improvements,
            model = EXCLUDED.model,
            analyzed_at = NOW()
        RETURNING repository_id, summary, documentation_quality_score,
            onboarding_score, beginner_friendliness_score, strengths,
            weaknesses, suggested_improvements, model, analyzed_at
        "#,
        repository_id,
        analysis.summary.as_str(),
        analysis.documentation_quality_score,
        analysis.onboarding_score,
        analysis.beginner_friendliness_score,
        analysis.strengths.as_slice(),
        analysis.weaknesses.as_slice(),
        analysis.suggested_improvements.as_slice(),
        model,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|error| {
        eprintln!("Database error: {error}");
        internal_error("Failed to save AI analysis")
    })?;

    Ok(Json(stored_analysis))
}

async fn analyze_with_gemini(
    state: &AppState,
    prompt: &str,
) -> Result<AiRepositoryAnalysis, String> {
    let client = gemini::Client::from_env()
        .map_err(|error| error.to_string())?;
    let extractor = client
        .extractor::<AiRepositoryAnalysis>(state.gemini_model.as_str())
        .build();
    extractor.extract(prompt.to_string()).await.map_err(|error| error.to_string())
}

async fn analyze_with_groq(
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<AiRepositoryAnalysis, String> {
    let request = GroqRequest {
        model,
        messages: vec![GroqMessage {
            role: "user",
            content: prompt,
        }],
        temperature: 0.2,
        response_format: GroqResponseFormat { r#type: "json_object" },
    };

    let response = HttpClient::new()
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    let status = response.status();
    let body = response.text().await.map_err(|error| error.to_string())?;
    if !status.is_success() {
        return Err(format!("Groq returned HTTP {status}: {body}"));
    }

    let response: GroqResponse = serde_json::from_str(&body)
        .map_err(|error| format!("invalid Groq response: {error}"))?;
    let content = response
        .choices
        .first()
        .ok_or_else(|| "Groq returned no choices".to_string())?
        .message
        .content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    serde_json::from_str(content)
        .map_err(|error| format!("invalid analysis JSON from Groq: {error}"))
}

pub async fn get_repository_ai_analysis(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<RepositoryAiAnalysisResponse>, ApiError> {
    let analysis = sqlx::query_as!(
        RepositoryAiAnalysisResponse,
        r#"SELECT repository_id, summary, documentation_quality_score,
            onboarding_score, beginner_friendliness_score, strengths,
            weaknesses, suggested_improvements, model, analyzed_at
           FROM repository_ai_analyses WHERE repository_id = $1"#,
        repository_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|error| {
        eprintln!("Database error: {error}");
        internal_error("Failed to load AI analysis")
    })?
    .ok_or_else(|| not_found("Repository AI analysis not found"))?;

    Ok(Json(analysis))
}

fn not_found(message: &str) -> ApiError {
    (StatusCode::NOT_FOUND, Json(ErrorResponse { error: message.to_string() }))
}

fn internal_error(message: &str) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: message.to_string() }))
}

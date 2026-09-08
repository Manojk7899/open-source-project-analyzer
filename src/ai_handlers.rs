use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rig_core::{
    client::{CompletionClient, ProviderClient},
    providers::gemini,
};

use crate::{
    ai_models::{AiRepositoryAnalysis, RepositoryAiAnalysisResponse},
    models::{ErrorResponse, StoredRepositoryResponse},
    state::AppState,
};

type ApiError = (StatusCode, Json<ErrorResponse>);

pub async fn analyze_repository(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<RepositoryAiAnalysisResponse>, ApiError> {
    let repository = sqlx::query_as!(
        StoredRepositoryResponse,
        r#"
        SELECT
            id AS "database_id!",
            github_id,
            owner,
            name,
            full_name,
            description,
            stars,
            forks,
            open_issues,
            subscribers,
            language,
            license,
            archived,
            has_issues,
            url,
            default_branch,
            topics,
            github_created_at,
            github_updated_at,
            pushed_at,
            readme
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

    let client = gemini::Client::from_env().map_err(|error| {
        eprintln!("Gemini client error: {error}");

        internal_error("Failed to configure Gemini")
    })?;

    let extractor = client
        .extractor::<AiRepositoryAnalysis>(state.gemini_model.as_str())
        .build();

    let readme_excerpt: String = repository.readme.chars().take(18_000).collect();

    let prompt = format!(
        r#"
Analyze this open-source GitHub repository.

Provide:
- a concise technical summary
- documentation quality score from 0 to 100
- onboarding score from 0 to 100
- beginner friendliness score from 0 to 100
- concrete strengths
- concrete weaknesses
- actionable suggested improvements

Base the analysis on the supplied repository data and README.
Do not invent features that are not present.

Repository: {}
Description: {}
Language: {}
Stars: {}
Forks: {}
Open issues: {}

README:

{}
        "#,
        repository.full_name,
        repository
            .description
            .as_deref()
            .unwrap_or("No description"),
        repository.language.as_deref().unwrap_or("Unknown"),
        repository.stars,
        repository.forks,
        repository.open_issues,
        readme_excerpt,
    );

    let analysis = extractor.extract(prompt).await.map_err(|error| {
        eprintln!("Gemini extraction error: {error}");

        internal_error("Failed to analyze repository with Gemini")
    })?;

    let stored_analysis = sqlx::query_as!(
        RepositoryAiAnalysisResponse,
        r#"
        INSERT INTO repository_ai_analyses (
            repository_id,
            summary,
            documentation_quality_score,
            onboarding_score,
            beginner_friendliness_score,
            strengths,
            weaknesses,
            suggested_improvements,
            model
        )
        VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9
        )
        ON CONFLICT (repository_id)
        DO UPDATE SET
            summary = EXCLUDED.summary,
            documentation_quality_score =
                EXCLUDED.documentation_quality_score,
            onboarding_score =
                EXCLUDED.onboarding_score,
            beginner_friendliness_score =
                EXCLUDED.beginner_friendliness_score,
            strengths = EXCLUDED.strengths,
            weaknesses = EXCLUDED.weaknesses,
            suggested_improvements =
                EXCLUDED.suggested_improvements,
            model = EXCLUDED.model,
            analyzed_at = NOW()
        RETURNING
            repository_id,
            summary,
            documentation_quality_score,
            onboarding_score,
            beginner_friendliness_score,
            strengths,
            weaknesses,
            suggested_improvements,
            model,
            analyzed_at
        "#,
        repository_id,
        analysis.summary.as_str(),
        analysis.documentation_quality_score,
        analysis.onboarding_score,
        analysis.beginner_friendliness_score,
        analysis.strengths.as_slice(),
        analysis.weaknesses.as_slice(),
        analysis.suggested_improvements.as_slice(),
        state.gemini_model.as_str(),
    )
    .fetch_one(&state.db)
    .await
    .map_err(|error| {
        eprintln!("Database error: {error}");

        internal_error("Failed to save Gemini analysis")
    })?;

    Ok(Json(stored_analysis))
}

pub async fn get_repository_ai_analysis(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<RepositoryAiAnalysisResponse>, ApiError> {
    let analysis = sqlx::query_as!(
        RepositoryAiAnalysisResponse,
        r#"
        SELECT
            repository_id,
            summary,
            documentation_quality_score,
            onboarding_score,
            beginner_friendliness_score,
            strengths,
            weaknesses,
            suggested_improvements,
            model,
            analyzed_at
        FROM repository_ai_analyses
        WHERE repository_id = $1
        "#,
        repository_id
    )
        .fetch_optional(&state.db)
        .await
        .map_err(|error| {
            eprintln!("Database error: {error}");

            internal_error(
                "Failed to load Gemini analysis",
            )
        })?
        .ok_or_else(|| {
            not_found("Repository AI analysis not found")
        })?;

    Ok(Json(analysis))
}

fn not_found(message: &str) -> ApiError {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}

fn internal_error(message: &str) -> ApiError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    ai_models::RepositoryAiAnalysisResponse,
    models::{ErrorResponse, StoredRepositoryResponse},
    report_models::RepositoryReportResponse,
    scoring::RepositoryScoreResponse,
    state::AppState,
};

type ApiError = (StatusCode, Json<ErrorResponse>);

pub async fn get_repository_report(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<RepositoryReportResponse>, ApiError> {
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
        internal_error("Failed to load repository report")
    })?
    .ok_or_else(|| not_found("Repository not found"))?;

    let deterministic_score = sqlx::query_as!(
        RepositoryScoreResponse,
        r#"
        SELECT
            repository_id,
            activity_score,
            documentation_score,
            community_score,
            maintenance_score,
            overall_score,
            reasons,
            calculated_at
        FROM repository_scores
        WHERE repository_id = $1
        "#,
        repository_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|error| {
        eprintln!("Database error: {error}");
        internal_error("Failed to load deterministic score")
    })?
    .ok_or_else(|| not_found("Repository score not found"))?;

    let ai_analysis = sqlx::query_as!(
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
        internal_error("Failed to load AI analysis")
    })?
    .ok_or_else(|| not_found("Repository AI analysis not found"))?;

    let ai_average = (ai_analysis.documentation_quality_score
        + ai_analysis.onboarding_score
        + ai_analysis.beginner_friendliness_score) as f64
        / 3.0;

    let final_score =
        ((deterministic_score.overall_score as f64 + ai_average) / 2.0).round() as i32;

    Ok(Json(RepositoryReportResponse {
        repository,
        deterministic_score,
        ai_analysis,
        final_score,
    }))
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
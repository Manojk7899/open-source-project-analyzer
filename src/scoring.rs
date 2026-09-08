use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{
    models::{
        ErrorResponse,
        StoredRepositoryResponse,
    },
    state::AppState,
};

type ApiError = (StatusCode, Json<ErrorResponse>);

#[derive(Debug, Serialize)]
pub struct RepositoryScoreResponse {
    pub repository_id: i64,
    pub activity_score: i32,
    pub documentation_score: i32,
    pub community_score: i32,
    pub maintenance_score: i32,
    pub overall_score: i32,
    pub reasons: Vec<String>,
    pub calculated_at: DateTime<Utc>,
}

struct CalculatedScore {
    activity_score: i32,
    documentation_score: i32,
    community_score: i32,
    maintenance_score: i32,
    overall_score: i32,
    reasons: Vec<String>,
}


pub async fn calculate_repository_score(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<RepositoryScoreResponse>, ApiError> {
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

    let score = score_repository(&repository);

    let stored_score = sqlx::query_as!(
        RepositoryScoreResponse,
        r#"
        INSERT INTO repository_scores (
            repository_id,
            activity_score,
            documentation_score,
            community_score,
            maintenance_score,
            overall_score,
            reasons
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (repository_id)
        DO UPDATE SET
            activity_score = EXCLUDED.activity_score,
            documentation_score =
                EXCLUDED.documentation_score,
            community_score = EXCLUDED.community_score,
            maintenance_score = EXCLUDED.maintenance_score,
            overall_score = EXCLUDED.overall_score,
            reasons = EXCLUDED.reasons,
            calculated_at = NOW()
        RETURNING
            repository_id,
            activity_score,
            documentation_score,
            community_score,
            maintenance_score,
            overall_score,
            reasons,
            calculated_at
        "#,
        repository_id,
        score.activity_score,
        score.documentation_score,
        score.community_score,
        score.maintenance_score,
        score.overall_score,
        score.reasons.as_slice(),
    )
        .fetch_one(&state.db)
        .await
        .map_err(|error| {
            eprintln!("Database error: {error}");
            internal_error("Failed to save repository score")
        })?;

    Ok(Json(stored_score))
}

pub async fn get_repository_score(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<RepositoryScoreResponse>, ApiError> {
    let score = sqlx::query_as!(
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
            internal_error("Failed to load repository score")
        })?
        .ok_or_else(|| not_found("Repository score not found"))?;

    Ok(Json(score))
}

fn score_repository(
    repository: &StoredRepositoryResponse,
) -> CalculatedScore {
    let mut reasons = Vec::new();

    let activity_score =
        calculate_activity_score(&repository.pushed_at);

    if activity_score >= 85 {
        reasons.push(
            "The repository has recent development activity."
                .to_string(),
        );
    }

    let documentation_score =
        calculate_documentation_score(&repository.readme);

    if documentation_score >= 85 {
        reasons.push(
            "The repository has substantial documentation."
                .to_string(),
        );
    }

    let community_score =
        calculate_community_score(repository.stars);

    if community_score >= 85 {
        reasons.push(
            "The repository has a large community."
                .to_string(),
        );
    }

    let maintenance_score = calculate_maintenance_score(
        repository.archived,
        repository.has_issues,
        repository.license.is_some(),
    );

    if maintenance_score >= 85 {
        reasons.push(
            "The repository appears actively maintained."
                .to_string(),
        );
    }

    let overall_score = (
        activity_score
            + documentation_score
            + community_score
            + maintenance_score
    ) / 4;

    CalculatedScore {
        activity_score,
        documentation_score,
        community_score,
        maintenance_score,
        overall_score,
        reasons,
    }
}

fn calculate_activity_score(pushed_at: &str) -> i32 {
    let pushed_at =
        DateTime::parse_from_rfc3339(pushed_at)
            .map(|date| date.with_timezone(&Utc));

    let Ok(pushed_at) = pushed_at else {
        return 40;
    };

    let days_since_push =
        Utc::now().signed_duration_since(pushed_at).num_days();

    match days_since_push {
        0..=30 => 100,
        31..=90 => 85,
        91..=180 => 70,
        181..=365 => 50,
        _ => 25,
    }
}

fn calculate_documentation_score(readme: &str) -> i32 {
    match readme.len() {
        5000.. => 100,
        2000..=4999 => 85,
        500..=1999 => 65,
        1..=499 => 40,
        _ => 10,
    }
}

fn calculate_community_score(stars: i64) -> i32 {
    match stars {
        10_000.. => 100,
        1_000..=9_999 => 90,
        100..=999 => 75,
        10..=99 => 60,
        _ => 35,
    }
}

fn calculate_maintenance_score(
    archived: bool,
    has_issues: bool,
    has_license: bool,
) -> i32 {
    if archived {
        return 20;
    }

    let mut score = 60;

    if has_issues {
        score += 20;
    }

    if has_license {
        score += 20;
    }

    score
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
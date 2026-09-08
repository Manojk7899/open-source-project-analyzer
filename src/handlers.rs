use axum::{
    extract::{Path,State},
http::StatusCode,Json, };
use url::Url;

use crate::models::{
        ErrorResponse,
        GitHubReadmeResponse,
        GitHubRepositoryResponse,
        RepositoryFetchResponse,
        RepositoryPreviewResponse,
        RepositoryUrlRequest,
        RepositoryListItemResponse,
        StoredRepositoryResponse
    };
use crate::state::AppState;

type ApiError = (StatusCode, Json<ErrorResponse>);
pub async fn preview_repository(
    Json(payload): Json<RepositoryUrlRequest>,
) -> Result<Json<RepositoryPreviewResponse>, ApiError> {
    let parsed_url = Url::parse(&payload.url)
        .map_err(|_| bad_request("Invalid repository URL"))?;

    if parsed_url.host_str() != Some("github.com") {
        return Err(bad_request(
            "Only GitHub repository URLs are supported",
        ));
    }

    let segments: Vec<&str> = parsed_url
        .path_segments()
        .map(|segments| {
            segments
                .filter(|segment| !segment.is_empty())
                .collect()
        })
        .unwrap_or_default();

    if segments.len() < 2 {
        return Err(bad_request(
            "The URL must contain an owner and repository name",
        ));
    }

    let owner = segments[0].to_string();
    let name = segments[1]
        .trim_end_matches(".git")
        .to_string();

    let normalized_url =
        format!("https://github.com/{owner}/{name}");

    Ok(Json(RepositoryPreviewResponse {
        owner,
        name,
        normalized_url,
    }))
}

fn bad_request(message: &str) -> ApiError {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}

pub async fn fetch_repository(
    State(state):State<AppState>,
    Json(payload): Json<RepositoryUrlRequest>,
) -> Result<Json<RepositoryFetchResponse>, ApiError> {
    let parsed_url = Url::parse(&payload.url)
        .map_err(|_| bad_request("Invalid repository URL"))?;

    if parsed_url.host_str() != Some("github.com") {
        return Err(bad_request(
            "Only GitHub repository URLs are supported",
        ));
    }

    let segments: Vec<&str> = parsed_url
        .path_segments()
        .map(|segments| {
            segments
                .filter(|segment| !segment.is_empty())
                .collect()
        })
        .unwrap_or_default();

    if segments.len() < 2 {
        return Err(bad_request(
            "The URL must contain an owner and repository name",
        ));
    }

    let owner = segments[0];
    let name = segments[1].trim_end_matches(".git");

    let client = reqwest::Client::new();

    let repository_url =
        format!("https://api.github.com/repos/{owner}/{name}");

    let repository_response = client
        .get(repository_url)
        .header(
            reqwest::header::USER_AGENT,
            "open-source-project-analyzer",
        )
        .send()
        .await
        .map_err(|_| {
            external_service_error(
                "Failed to connect to the GitHub API",
            )
        })?;

    if repository_response.status()
        == reqwest::StatusCode::NOT_FOUND
    {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "GitHub repository not found".to_string(),
            }),
        ));
    }

    if !repository_response.status().is_success() {
        return Err(external_service_error(
            "GitHub API returned an error",
        ));
    }

    let repository = repository_response
        .json::<GitHubRepositoryResponse>()
        .await
        .map_err(|_| {
            external_service_error(
                "Failed to parse the GitHub response",
            )
        })?;

    let readme_url = format!(
        "https://api.github.com/repos/{owner}/{name}/readme"
    );

    let readme_response = client
        .get(readme_url)
        .header(
            reqwest::header::USER_AGENT,
            "open-source-project-analyzer",
        )
        .send()
        .await
        .map_err(|_| {
            external_service_error(
                "Failed to fetch the repository README",
            )
        })?;

    let readme = if readme_response.status().is_success() {
        let readme_metadata = readme_response
            .json::<GitHubReadmeResponse>()
            .await
            .map_err(|_| {
                external_service_error(
                    "Failed to parse the README response",
                )
            })?;

        client
            .get(readme_metadata.download_url)
            .send()
            .await
            .map_err(|_| {
                external_service_error(
                    "Failed to download the README",
                )
            })?
            .text()
            .await
            .map_err(|_| {
                external_service_error(
                    "Failed to read the README",
                )
            })?
    } else {
        String::new()
    };
    let license=repository
            .license
            .map(|license| license.spdx_id);
         let database_id = sqlx::query_scalar!(
    r#"
    INSERT INTO repositories (
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
    )
    VALUES (
        $1, $2, $3, $4, $5,
        $6, $7, $8, $9, $10,
        $11, $12, $13, $14, $15,
        $16, $17, $18, $19, $20
    )
    ON CONFLICT (github_id)
    DO UPDATE SET
        owner = EXCLUDED.owner,
        name = EXCLUDED.name,
        full_name = EXCLUDED.full_name,
        description = EXCLUDED.description,
        stars = EXCLUDED.stars,
        forks = EXCLUDED.forks,
        open_issues = EXCLUDED.open_issues,
        subscribers = EXCLUDED.subscribers,
        language = EXCLUDED.language,
        license = EXCLUDED.license,
        archived = EXCLUDED.archived,
        has_issues = EXCLUDED.has_issues,
        url = EXCLUDED.url,
        default_branch = EXCLUDED.default_branch,
        topics = EXCLUDED.topics,
        github_created_at = EXCLUDED.github_created_at,
        github_updated_at = EXCLUDED.github_updated_at,
        pushed_at = EXCLUDED.pushed_at,
        readme = EXCLUDED.readme,
        updated_at = NOW()
    RETURNING id
    "#,
    repository.id,
    repository.owner.login.as_str(),
    repository.name.as_str(),
    repository.full_name.as_str(),
    repository.description.as_deref(),
    repository.stargazers_count,
    repository.forks_count,
    repository.open_issues_count,
    repository.subscribers_count,
    repository.language.as_deref(),
    license.as_deref(),
    repository.archived,
    repository.has_issues,
    repository.html_url.as_str(),
    repository.default_branch.as_str(),
    repository.topics.as_slice(),
    repository.created_at.as_str(),
    repository.updated_at.as_str(),
    repository.pushed_at.as_str(),
    readme.as_str(),
)
        .fetch_one(&state.db)
        .await
        .map_err(|error| {
            eprintln!("Database error: {error}");
            database_error("Failed to save repository")
        })?;

    

    Ok(Json(RepositoryFetchResponse {
        database_id,
        github_id: repository.id,
        owner: repository.owner.login,
        name: repository.name,
        full_name: repository.full_name, 
        description: repository.description,
        stars: repository.stargazers_count,
        forks: repository.forks_count,
        open_issues: repository.open_issues_count,
        subscribers: repository.subscribers_count,
        language: repository.language,
        license,
        archived: repository.archived,
        has_issues: repository.has_issues,
        url: repository.html_url,
        default_branch: repository.default_branch,
        topics: repository.topics,
        github_created_at: repository.created_at,
        github_updated_at: repository.updated_at,
        pushed_at: repository.pushed_at,
        readme,
    }))
}
pub async fn list_repositories(
    State(state): State<AppState>,
) -> Result<Json<Vec<RepositoryListItemResponse>>, ApiError> {
    let repositories = sqlx::query_as!(
        RepositoryListItemResponse,
        r#"
        SELECT
            id AS "database_id!",
            full_name,
            description,
            stars,
            forks,
            open_issues,
            language,
            license,
            archived,
            default_branch
        FROM repositories
        ORDER BY stars DESC
        "#
    )
        .fetch_all(&state.db)
        .await
        .map_err(|error| {
            eprintln!("Database error: {error}");
            database_error("Failed to load repositories")
        })?;

    Ok(Json(repositories))
}

pub async fn get_repository(
    State(state): State<AppState>,
    Path(repository_id): Path<i64>,
) -> Result<Json<StoredRepositoryResponse>, ApiError> {
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
            database_error("Failed to load repository")
        })?;

    match repository {
        Some(repository) => Ok(Json(repository)),
        None => Err(not_found("Repository not found")),
    }
}

fn database_error(message: &str) -> ApiError {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}
fn external_service_error(message: &str) -> ApiError {
    (
        StatusCode::BAD_GATEWAY,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}
fn not_found(message: &str) -> ApiError {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}
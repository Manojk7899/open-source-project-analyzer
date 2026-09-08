use axum::{
    routing::{get, post},
    Router,
};
use crate::scoring::{
    calculate_repository_score,
    get_repository_score,
};
use crate::handlers::{
    fetch_repository,
    get_repository,
    list_repositories,
    preview_repository,
};
use crate::ai_handlers::{
    analyze_repository,
    get_repository_ai_analysis
};
use crate::report_handlers::{
    get_repository_report
};
use crate::state::AppState;

pub fn repository_routes() -> Router<AppState> {
    Router::new()
        .route("/preview", post(preview_repository))
        .route("/fetch", post(fetch_repository))
        .route("/", get(list_repositories))
        .route("/{id}", get(get_repository))
        .route(
            "/{id}/score",
            post(calculate_repository_score).get(get_repository_score),
        )
        .route("/{id}/ai-analysis", 
        post(analyze_repository)
        .get(get_repository_ai_analysis))
        .route("/{id}/report", get(get_repository_report))
}

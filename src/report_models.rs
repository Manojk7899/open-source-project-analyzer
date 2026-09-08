use serde::Serialize;

use crate::{
    ai_models::RepositoryAiAnalysisResponse,
    models::StoredRepositoryResponse,
    scoring::RepositoryScoreResponse,
};

#[derive(Debug, Serialize)]
pub struct RepositoryReportResponse {
    pub repository: StoredRepositoryResponse,
    pub deterministic_score: RepositoryScoreResponse,
    pub ai_analysis: RepositoryAiAnalysisResponse,
    pub final_score: i32,
}
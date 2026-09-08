use chrono::{DateTime,Utc};
use schemars::JsonSchema;
use serde::{Serialize,Deserialize};
#[derive(Debug,Serialize,Deserialize,JsonSchema)]
pub struct AiRepositoryAnalysis{
    pub summary:String,
    pub documentation_quality_score:i32,
    pub onboarding_score:i32,
    pub beginner_friendliness_score:i32,
    pub strengths:Vec<String>,
    pub weaknesses:Vec<String>,
    pub suggested_improvements:Vec<String>,
}
#[derive(Debug,Serialize)]
pub struct RepositoryAiAnalysisResponse {
    pub repository_id: i64,
    pub summary: String,
    pub documentation_quality_score: i32,
    pub onboarding_score: i32,
    pub beginner_friendliness_score: i32,
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub suggested_improvements: Vec<String>,
    pub model: String,
    pub analyzed_at: DateTime<Utc>,
}
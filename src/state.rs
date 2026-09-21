use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub gemini_model: String,
    pub groq_api_key: Option<String>,
    pub groq_model: String,
}

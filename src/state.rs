use sqlx::PgPool;
#[derive(Clone)]
pub struct AppState{
    pub db:PgPool,  
    pub gemini_model:String,
}
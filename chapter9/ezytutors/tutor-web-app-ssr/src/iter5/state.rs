use sqlx::postgres::PgPool;

pub struct AppState {
    pub db: PgPool,
}

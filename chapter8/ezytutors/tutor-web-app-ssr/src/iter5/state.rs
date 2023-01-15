
// Listing 8.x - start

use sqlx::postgres::PgPool;

pub struct AppState {
    pub db: PgPool,
}


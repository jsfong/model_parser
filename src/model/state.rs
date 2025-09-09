#[derive(Clone, Debug)]
pub struct AppState {
    pub pg_pool: sqlx::Pool<sqlx::Postgres>,
}
use sqlx::{Pool, Postgres};
pub struct AppState {
    pub db: Pool<Postgres>
}
impl AppState {
    pub fn new(db: Pool<Postgres>) -> Self {
        AppState {db}
    }
}
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TachesModel {
    pub id: i32,
    pub titre: String,
    pub description: Text,
    pub id_projet: i32,
    pub terminer: Boolean,
}

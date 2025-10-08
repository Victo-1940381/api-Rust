use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SoustachesModel {
    pub id: i32,
    pub titre: String,
    pub description: Text,
    pub responsable: i32,
    pub id_tache: i32,
    pub terminer: Boolean,
}

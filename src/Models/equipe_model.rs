use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct EquipeModel {
    pub id: i32,
    pub nom: String,
}
impl EquipeModel {
    pub fn new(id: i32, nom:String) -> Self {
        EquipeModel { id,nom}
    }
}


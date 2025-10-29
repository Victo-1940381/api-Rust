use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TachesModel {
    pub id: i32,
    pub titre: String,
    pub description: String,
    pub id_projet: i32,
    pub terminer: bool,
}
impl TachesModel {
    pub fn new(id: i32, titre:String,description:String,id_projet:i32,terminer:bool) -> Self {
        TachesModel { id, titre, description, id_projet, terminer }
    }
}

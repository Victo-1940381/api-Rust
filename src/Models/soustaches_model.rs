use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SoustachesModel {
    pub id: i32,
    pub titre: String,
    pub description: String,
    pub responsable: i32,
    pub id_tache: i32,
    pub terminer: bool,
}
impl SoustachesModel {
    pub fn new(id:i32,titre:String,description:String,responsable:i32,id_tache: i32,terminer:bool)-> Self{
        SoustachesModel { id, titre, description, responsable, id_tache, terminer }
    }
    
}


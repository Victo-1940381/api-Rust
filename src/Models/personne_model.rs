use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PersonnesModel {
    pub id: i32,
    pub nom: String,
    pub prenom: String,
    pub age: i32,
    pub equipe: i32,
    pub is_chef: bool,
}
impl PersonnesModel {
    pub fn new(id:i32,nom:String,prenom:String,age:i32,equipe:i32,is_chef:bool) -> Self {
        PersonnesModel{ id,nom,prenom,age,equipe,is_chef}
    }
}

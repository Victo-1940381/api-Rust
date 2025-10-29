use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PersonnesModel {
    pub id: i32,
    pub nom: String,
    pub prenom: String,
    pub age: Int,
    pub equipe: i32,
    pub is_chef: Boolean,
}

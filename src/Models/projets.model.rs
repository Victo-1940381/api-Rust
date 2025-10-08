use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ProjetsModel {
    pub id: i32,
    pub nom: String,
    pub date_debut: Date,
    pub date_fin: Date,
    pub fini: Boolean,
    pub equiperesponsables: i32,
}

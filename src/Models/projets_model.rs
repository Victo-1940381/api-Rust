use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDate};

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ProjetsModel {
    pub id: i32,
    pub nom: String,
    pub date_debut: NaiveDate,
    pub date_de_fin: Option<NaiveDate>,
    pub fini: bool,
    pub equiperesponsables: i32,
}
impl ProjetsModel {
    pub fn new(id:i32,nom:String,date_debut:NaiveDate,date_de_fin: Option<NaiveDate>,fini:bool,equiperesponsables:i32) -> Self{
        ProjetsModel{id,nom,date_debut,date_de_fin,fini,equiperesponsables}
    }
}
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate};
#[derive(Deserialize,Debug)]
pub struct ParamOptions{
    pub id: i32,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct CreateProjetSchema{
    pub nom: String,
    pub date_debut:NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_fin: Option<NaiveDate>,
    pub fini:bool,
    pub equiperesponsables:i32,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateProjetSchema{
    pub nom: Option<String>,
    pub date_debut:Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
    pub fini:Option<bool>,
    pub equiperesponsables:Option<i32>,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateProjetEtatSchema{
    pub fini:Option<bool>,
}

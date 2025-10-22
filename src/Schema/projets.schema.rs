use serde::{Deserialize, Serialize};

#[derive(Deserialize,Debug)]
pub struct ParamOptions{
    pub id: i32,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct CreateProjetSchema{
    pub nom: String,
    pub date_debut:Date,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_fin: Option<Date>,
    pub fini:Boolean,
    pub equiperesponsables:i32,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateProjetSchema{
    pub nom: String,
    pub date_debut:Date,
    pub date_fin: Option<Date>,
    pub fini:Boolean,
    pub equiperesponsables:i32,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateProjetEtatSchema{
    pub id: i32,
    pub fini:Boolean,
}

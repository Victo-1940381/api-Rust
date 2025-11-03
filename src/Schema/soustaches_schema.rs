use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub id :i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct CreatesoustacheSchema{
    pub titre: String,
    pub description: String,
    pub responsable: i32,
    pub id_tache: i32,
    pub terminer: bool,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct updatesoustacheSchema{
    pub titre: Option<String>,
    pub description: Option<String>,
    pub responsable: Option<i32>,
    pub id_tache: Option<i32>,
    pub terminer: Option<bool>,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct updatesoustacheEtatSchema{
    pub terminer:Option<bool>
}
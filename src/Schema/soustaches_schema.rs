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
    pub titre: String,
    pub description: String,
    pub responsable: i32,
    pub id_tache: i32,
    pub terminer: bool,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct updatesoustacheEtatSchema{
    pub id: i32,
    pub terminer:bool,
}
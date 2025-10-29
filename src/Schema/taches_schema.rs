use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub id :i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct CreatetacheSchema{
    pub titre: String,
    pub description: String,
    pub id_projet: i32,
    pub terminer: bool,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct updatetacheSchema{
    pub titre: String,
    pub description: String,
    pub id_projet: i32,
    pub terminer: bool,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct updatetacheEtatSchema{
    pub id: i32,
    pub terminer:bool,
}
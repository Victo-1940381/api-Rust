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
    pub titre: Option<String>,
    pub description: Option<String>,
    pub id_projet: Option<i32>,
    pub terminer: Option<bool>,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct updatetacheEtatSchema{
    pub terminer:Option<bool>,
}
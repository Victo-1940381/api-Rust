use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEquipeSchema{
    pub nom: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEquipeSchema{
    pub nom: String,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub id: int,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEquipeSchema{
    pub nom: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEquipeSchema{
    pub nom: Option<String>,
}

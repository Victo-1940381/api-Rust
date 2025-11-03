use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub id: i32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePersonneSchema{
    pub nom: String,
    pub prenom: String,
    pub age:i32,
    pub equipe: i32,
    pub is_chef: bool,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct UpdatePersonneSchema{
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub age:Option<i32>,
    pub equipe: Option<i32>,
    pub is_chef: Option<bool>,
}

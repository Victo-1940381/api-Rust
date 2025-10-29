use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub id: i32
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePersonneSchema{
    pub nom: String,
    pub prenom: String,
    pub age:Int,
    pub equipe: i32,
    pub is_chef: Boolean,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct UpdatePersonneSchema{
    pub nom: String,
    pub prenom: String,
    pub age:Int,
    pub equipe: i32,
    pub is_chef: Boolean,
}

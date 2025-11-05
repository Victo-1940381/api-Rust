use crate::{
    models::personne_model::PersonnesModel,
    schema::personne_schema::{CreatePersonneSchema,UpdatePersonneSchema},
    handler::appState::AppState,
};
use actix_web::{delete,get,patch,post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/personne")]
pub async fn personne_list_handler(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        PersonnesModel,
        "SELECT * FROM personnes Order by id"
    )
    .fetch_all(&data.db)
    .await;
    if query_result.is_err(){
        let message = "quelque chose est arrive lors du fetch des donner";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));

    }
    let personnes = query_result.unwrap();
    let json_response = serde_json::json!({
        "status":"success",
        "results": personnes.len(),
        "personnes": personnes
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/personne/{id}")]
async fn get_personne_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let personne_id = path.into_inner();
    let query_result = sqlx::query_as!(PersonnesModel, "SELECT * FROM personnes WHERE id = $1", personne_id)
    .fetch_one(&data.db)
    .await;
match query_result{
    Ok(personne) =>{
        let personne_response = serde_json::json!({"status":"success","data":serde_json::json!({
            "personne":personne
        })});

        return HttpResponse::Ok().json(personne_response);
    }
    Err(_) =>{
        let message = format!("personne avec id: {} pas trouver",personne_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status":"echec","message": message}));
    }

}
}
#[post("/personne/ajout")]
async fn create_personnne_handler(
    body: web::Json<CreatePersonneSchema>,
    data: web::Data<AppState>,
)
 -> impl Responder {
    let query_result = sqlx::query_as!(
    PersonnesModel,
    "INSERT INTO personnes(nom,prenom,age,equipe,is_chef) values ($1,$2,$3,$4,$5) RETURNING *",
    body.nom.to_string(),
    body.prenom.to_string(),
    body.age,
    body.equipe,
    body.is_chef,
)
.fetch_one(&data.db)
.await;
match query_result {
    Ok(personne) => {
        let personne_response = serde_json::json!({"status": "reussi", "data": serde_json::json!({"personne":personne})});

        return HttpResponse::Ok().json(personne_response);
    }
    Err(e) =>{
        if e.to_string().contains("duplicate key value violates unique constraint"){
            return HttpResponse::BadRequest()
            .json(serde_json::json!({"status":"echec", "message":"cet personne existe deja"})) 
        }
      

        return HttpResponse::InternalServerError()
        .json(serde_json::json!({"status":"erreur","message": format!("{:?}", e)}));
    }
}
}
 
#[patch("/personne/modif/{id}")]
async fn edit_personne_handler(
    path: web::Path<i32>,
    body: web::Json<UpdatePersonneSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let personne_id = path.into_inner();
    let query_result = sqlx::query_as!(PersonnesModel, "select * from personnes where id = $1", personne_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("la personne avec l'id : {}  n'existe pas", personne_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "echec", "message": message}));  
    }
    let personne = query_result.unwrap();

    let query_result = sqlx::query_as!(
        PersonnesModel,
        "update personnes set nom = $1, prenom =$2, age = $3, equipe = $4, is_chef = $5  where id = $6 returning *",
        body.nom.to_owned().unwrap_or(personne.nom),
        body.prenom.to_owned().unwrap_or(personne.prenom),
        body.age.to_owned().unwrap_or(personne.age),
        body.equipe.to_owned().unwrap_or(personne.equipe),
        body.is_chef.to_owned().unwrap_or(personne.is_chef),
        personne_id
    )
    .fetch_one(&data.db)
    .await;

match query_result{
    Ok(personne) =>{
        let personne_response = serde_json::json!({"status":"reussi", "data": serde_json::json!({
            "personne":personne
        })});
        return HttpResponse::Ok().json(personne_response);
    }
    Err(err) =>{
        let message = format!("Erreur: {:?}", err);
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status":"erreur","message":message}));
    }
}
}

#[delete("/personne/delete/{id}")]
async fn delete_personne_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let personne_id = path.into_inner();
    let rows_affected = sqlx::query!("delete from personnes where id = $1", personne_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

if rows_affected ==0 {
    let message = format!("la personne avec l'id: {}  n'a pas été trouvé", personne_id);
    return HttpResponse::NotFound().json(json!({"status":"echec","message":message}));  
}
else{
    let message = format!("la personne avec l'id: {}  a été supprimer", personne_id);
    return HttpResponse::Ok().json(json!({"status":"réussi","message":message}));  
}
}



use crate::{
    models::projets_model::ProjetsModel,
    schema::projets_schema::{CreateProjetSchema,UpdateProjetEtatSchema,UpdateProjetSchema},
    handler::appState::AppState,
};
use actix_web::{delete,get,patch,post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/projets")]
pub async fn projets_list_handler(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        ProjetsModel,
        "SELECT * FROM projets Order by id"
    )
    .fetch_all(&data.db)
    .await;
    if query_result.is_err(){
        let message = "quelque chose est arrive lors du fetch des donner";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));

    }
    let projets = query_result.unwrap();
    let json_response = serde_json::json!({
        "status":"success",
        "results": projets.len(),
        "projets": projets
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/projets/{id}")]
async fn get_projets_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let projet_id = path.into_inner();
    let query_result = sqlx::query_as!(ProjetsModel, "SELECT * FROM projets WHERE id = $1", projet_id)
    .fetch_one(&data.db)
    .await;
match query_result{
    Ok(projet) =>{
        let projet_response = serde_json::json!({"status":"success","data":serde_json::json!({
            "projet":projet
        })});

        return HttpResponse::Ok().json(projet_response);
    }
    Err(_) =>{
        let message = format!("projet avec id: {} pas trouver",projet_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status":"echec","message": message}));
    }

}
}
#[post("/projets/ajout")]
async fn create_projet_handler(
    body: web::Json<CreateProjetSchema>,
    data: web::Data<AppState>,
)
 -> impl Responder {
    let query_result = sqlx::query_as!(
    ProjetsModel,
    "INSERT INTO projets(nom,date_debut,date_de_fin,fini,equiperesponsables) values ($1,$2,$3,$4,$5) RETURNING *",
    body.nom.to_string(),
    body.date_debut,
    body.date_de_fin,
    body.fini,
    body.equiperesponsables,
)
.fetch_one(&data.db)
.await;
match query_result {
    Ok(projet) => {
        let projet_response = serde_json::json!({"status": "reussi", "data": serde_json::json!({"projet": projet})});

        return HttpResponse::Ok().json(projet_response);
    }
    Err(e) =>{
        if e.to_string().contains("duplicate key value violates unique constraint"){
            return HttpResponse::BadRequest()
            .json(serde_json::json!({"status":"echec", "message":"ce projet existe deja"})) 
        }
      

        return HttpResponse::InternalServerError()
        .json(serde_json::json!({"status":"erreur","message": format!("{:?}", e)}));
    }
}
}
 
#[patch("/projets/modif/{id}")]
async fn edit_projet_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateProjetSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let projet_id = path.into_inner();
    let query_result = sqlx::query_as!(ProjetsModel, "select * from projets where id = $1", projet_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("le projet avec l'id : {}  n'existe pas", projet_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "echec", "message": message}));  
    }
    let projet = query_result.unwrap();

    let query_result = sqlx::query_as!(
        ProjetsModel,
        "update projets set nom = $1, date_debut= $2, date_de_fin = $3, fini = $4, equiperesponsables = $5  where id = $6 returning *",
        body.nom.to_owned().unwrap_or(projet.nom),
        body.date_debut.to_owned().unwrap_or(projet.date_debut),
        body.date_de_fin,
        body.fini.to_owned().unwrap_or(projet.fini),
        body.equiperesponsables.to_owned().unwrap_or(projet.equiperesponsables),
        projet_id
    )
    .fetch_one(&data.db)
    .await;

match query_result{
    Ok(projet) =>{
        let projet_response = serde_json::json!({"status":"reussi", "data": serde_json::json!({
            "projet":projet
        })});
        return HttpResponse::Ok().json(projet_response);
    }
    Err(err) =>{
        let message = format!("Erreur: {:?}", err);
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status":"erreur","message":message}));
    }
}
}

#[delete("/projets/delete/{id}")]
async fn delete_projet_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let projet_id = path.into_inner();
    let rows_affected = sqlx::query!("delete from projets where id = $1", projet_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

if rows_affected ==0 {
    let message = format!("le projet avec l'id: {}  n'a pas été trouvé", projet_id);
    return HttpResponse::NotFound().json(json!({"status":"echec","message":message}));  
}
else{
    let message = format!("le projet avec l'id: {}  a été supprimer", projet_id);
    return HttpResponse::Ok().json(json!({"status":"réussi","message":message}));  
}
}



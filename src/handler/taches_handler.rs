use crate::{
    models::taches_model::TachesModel,
    schema::taches_schema::{CreatetacheSchema,updatetacheEtatSchema,updatetacheSchema},
    handler::appState::AppState,
};
use actix_web::{delete,get,patch,post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/tache")]
pub async fn tache_list_handler(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        TachesModel,
        "SELECT * FROM taches Order by id"
    )
    .fetch_all(&data.db)
    .await;
    if query_result.is_err(){
        let message = "quelque chose est arrive lors du fetch des donner";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));

    }
    let tache = query_result.unwrap();
    let json_response = serde_json::json!({
        "status":"success",
        "results": tache.len(),
        "tache": tache
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/tache/{id}")]
async fn get_tache_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let tache_id = path.into_inner();
    let query_result = sqlx::query_as!(TachesModel, "SELECT * FROM taches WHERE id = $1", tache_id)
    .fetch_one(&data.db)
    .await;
match query_result{
    Ok(tache) =>{
        let tache_response = serde_json::json!({"status":"success","data":serde_json::json!({
            "tache":tache
        })});

        return HttpResponse::Ok().json(tache_response);
    }
    Err(_) =>{
        let message = format!("tache avec id: {} pas trouver",tache_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status":"echec","message": message}));
    }

}
}
#[post("/tache/ajout")]
async fn create_tache_handler(
    body: web::Json<CreatetacheSchema>,
    data: web::Data<AppState>,
)
 -> impl Responder {
    let query_result = sqlx::query_as!(
    TachesModel,
    "INSERT INTO taches(titre,description,id_projet,terminer) values ($1,$2,$3,$4) RETURNING *",
    body.titre.to_string(),
    body.description.to_string(),
    body.id_projet,
    body.terminer,
)
.fetch_one(&data.db)
.await;
match query_result {
    Ok(tache) => {
        let tache_response = serde_json::json!({"status": "reussi", "data": serde_json::json!({"tache": tache})});

        return HttpResponse::Ok().json(tache_response);
    }
    Err(e) =>{
        if e.to_string().contains("duplicate key value violates unique constraint"){
            return HttpResponse::BadRequest()
            .json(serde_json::json!({"status":"echec", "message":"cet tache existe deja"})) 
        }
      

        return HttpResponse::InternalServerError()
        .json(serde_json::json!({"status":"erreur","message": format!("{:?}", e)}));
    }
}
}
 
#[patch("/tache/modif/{id}")]
async fn edit_tache_handler(
    path: web::Path<i32>,
    body: web::Json<updatetacheSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let tache_id = path.into_inner();
    let query_result = sqlx::query_as!(TachesModel, "select * from taches where id = $1", tache_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("la tache avec l'id : {}  n'existe pas", tache_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "echec", "message": message}));  
    }
    let tache = query_result.unwrap();

    let query_result = sqlx::query_as!(
        TachesModel,
        "update taches set titre = $1, description= $2, id_projet = $3, terminer = $4  where id = $5 returning *",
        body.titre.to_owned().unwrap_or(tache.titre),
        body.description.to_owned().unwrap_or(tache.description),
        body.id_projet.to_owned().unwrap_or(tache.id_projet),
        body.terminer.to_owned().unwrap_or(tache.terminer),
        tache_id
    )
    .fetch_one(&data.db)
    .await;

match query_result{
    Ok(tache) =>{
        let tache_response = serde_json::json!({"status":"reussi", "data": serde_json::json!({
            "tache":tache
        })});
        return HttpResponse::Ok().json(tache_response);
    }
    Err(err) =>{
        let message = format!("Erreur: {:?}", err);
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status":"erreur","message":message}));
    }
}
}

#[delete("/tache/delete/{id}")]
async fn delete_tache_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let tache_id = path.into_inner();
    let rows_affected = sqlx::query!("delete from taches where id = $1", tache_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

if rows_affected ==0 {
    let message = format!("le tache avec l'id: {}  n'a pas été trouvé", tache_id);
    return HttpResponse::NotFound().json(json!({"status":"echec","message":message}));  
}
else{
    let message = format!("le tache avec l'id: {}  a été supprimer", tache_id);
    return HttpResponse::Ok().json(json!({"status":"réussi","message":message}));  
}
}



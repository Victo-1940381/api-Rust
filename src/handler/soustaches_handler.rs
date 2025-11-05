use crate::{
    models::soustaches_model::SoustachesModel,
    schema::soustaches_schema::{CreatesoustacheSchema,updatesoustacheEtatSchema,updatesoustacheSchema},
    handler::appState::AppState,
};
use actix_web::{delete,get,patch,post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/soustache")]
pub async fn soustache_list_handler(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        SoustachesModel,
        "SELECT * FROM soustaches Order by id"
    )
    .fetch_all(&data.db)
    .await;
    if query_result.is_err(){
        let message = "quelque chose est arrive lors du fetch des donner";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));

    }
    let soustache = query_result.unwrap();
    let json_response = serde_json::json!({
        "status":"success",
        "results": soustache.len(),
        "soustache": soustache
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/soustache/{id}")]
async fn get_soustache_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let soustache_id = path.into_inner();
    let query_result = sqlx::query_as!(SoustachesModel, "SELECT * FROM soustaches WHERE id = $1", soustache_id)
    .fetch_one(&data.db)
    .await;
match query_result{
    Ok(soustache) =>{
        let soustache_response = serde_json::json!({"status":"success","data":serde_json::json!({
            "soustache":soustache
        })});

        return HttpResponse::Ok().json(soustache_response);
    }
    Err(_) =>{
        let message = format!("soustache avec id: {} pas trouver",soustache_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status":"echec","message": message}));
    }

}
}
#[post("/soustache/ajout")]
async fn create_soustache_handler(
    body: web::Json<CreatesoustacheSchema>,
    data: web::Data<AppState>,
)
 -> impl Responder {
    let query_result = sqlx::query_as!(
    SoustachesModel,
    "INSERT INTO soustaches(titre,description,responsable,id_tache,terminer) values ($1,$2,$3,$4,$5) RETURNING *",
    body.titre.to_string(),
    body.description.to_string(),
    body.responsable,
    body.id_tache,
    body.terminer,
)
.fetch_one(&data.db)
.await;
match query_result {
    Ok(soustache) => {
        let soustache_response = serde_json::json!({"status": "reussi", "data": serde_json::json!({"soustache": soustache})});

        return HttpResponse::Ok().json(soustache_response);
    }
    Err(e) =>{
        if e.to_string().contains("duplicate key value violates unique constraint"){
            return HttpResponse::BadRequest()
            .json(serde_json::json!({"status":"echec", "message":"cet soustache existe deja"})) 
        }
      

        return HttpResponse::InternalServerError()
        .json(serde_json::json!({"status":"erreur","message": format!("{:?}", e)}));
    }
}
}
 
#[patch("/soustache/modif/{id}")]
async fn edit_soustache_handler(
    path: web::Path<i32>,
    body: web::Json<updatesoustacheSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let soustache_id = path.into_inner();
    let query_result = sqlx::query_as!(SoustachesModel, "select * from soustaches where id = $1", soustache_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("la soustache avec l'id : {}  n'existe pas", soustache_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "echec", "message": message}));  
    }
    let soustache = query_result.unwrap();

    let query_result = sqlx::query_as!(
        SoustachesModel,
        "update soustaches set titre = $1, description= $2, responsable= $3, id_tache = $4, terminer = $5  where id = $6 returning *",
        body.titre.to_owned().unwrap_or(soustache.titre),
        body.description.to_owned().unwrap_or(soustache.description),
        body.responsable.to_owned().unwrap_or(soustache.responsable),
        body.id_tache.to_owned().unwrap_or(soustache.id_tache),
        body.terminer.to_owned().unwrap_or(soustache.terminer),
        soustache_id
    )
    .fetch_one(&data.db)
    .await;

match query_result{
    Ok(soustache) =>{
        let soustache_response = serde_json::json!({"status":"reussi", "data": serde_json::json!({
            "soustache":soustache
        })});
        return HttpResponse::Ok().json(soustache_response);
    }
    Err(err) =>{
        let message = format!("Erreur: {:?}", err);
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status":"erreur","message":message}));
    }
}
}

#[delete("/soustache/delete/{id}")]
async fn delete_soustache_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let soustache_id = path.into_inner();
    let rows_affected = sqlx::query!("delete from soustaches where id = $1", soustache_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

if rows_affected ==0 {
    let message = format!("le soustache avec l'id: {}  n'a pas été trouvé", soustache_id);
    return HttpResponse::NotFound().json(json!({"status":"echec","message":message}));  
}
else{
    let message = format!("le soustache avec l'id: {}  a été supprimer", soustache_id);
    return HttpResponse::Ok().json(json!({"status":"réussi","message":message}));  
}
}



use crate::{
    models::equipe_model::EquipeModel,
    schema::equipe_schema::{CreateEquipeSchema,UpdateEquipeSchema},
    handler::appState::AppState,
};
use actix_web::{delete,get,patch,post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;


#[get("/equipe")]
pub async fn equipe_list_handler(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        EquipeModel,
        "SELECT * FROM equipe Order by id"
    )
    .fetch_all(&data.db)
    .await;
    if query_result.is_err(){
        let message = "quelque chose est arrive lors du fetch des donner";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));

    }
    let equipes = query_result.unwrap();
    let json_response = serde_json::json!({
        "status":"success",
        "results": equipes.len(),
        "equipes": equipes
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/equipe/{id}")]
async fn get_equipe_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let equipe_id = path.into_inner();
    let query_result = sqlx::query_as!(EquipeModel, "SELECT * FROM equipe WHERE id = $1", equipe_id)
    .fetch_one(&data.db)
    .await;
match query_result{
    Ok(equipe) =>{
        let equipe_response = serde_json::json!({"status":"success","data":serde_json::json!({
            "equipe":equipe
        })});

        return HttpResponse::Ok().json(equipe_response);
    }
    Err(_) =>{
        let message = format!("equipe avec id: {} pas trouver",equipe_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status":"fail","message": message}));
    }

}
}
#[post("/equipe/ajout")]
async fn create_equipe_handler(
    body: web::Json<CreateEquipeSchema>,
    data: web::Data<AppState>,
)
 -> impl Responder {
    let query_result = sqlx::query_as!(
    EquipeModel,
    "INSERT INTO equipe (nom) values ($1) RETURNING *",
    body.nom.to_string()
)
.fetch_one(&data.db)
.await;
match query_result {
    Ok(equipe) => {
        let equipe_response = serde_json::json!({"status": "reussi", "data": serde_json::json!({"equipe":equipe})});

        return HttpResponse::Ok().json(equipe_response);
    }
    Err(e) =>{
        if e.to_string().contains("duplicate key value violates unique constraint"){
            return HttpResponse::BadRequest()
            .json(serde_json::json!({"status":"echec", "message":"un equipe avec ce nom existe deja"})) 
        }
      

        return HttpResponse::InternalServerError()
        .json(serde_json::json!({"status":"erreur","message": format!("{:?}", e)}));
    }
}
}
#[patch("/equipe/modif/{id}")]
async fn edit_equipe_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateEquipeSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let equipe_id = path.into_inner();
    let query_result = sqlx::query_as!(EquipeModel, "select * from equipe where id = $1", equipe_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("equipe avec l'id : {} pas trouvé", equipe_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "echec", "message": message}));  
    }
    let equipe = query_result.unwrap();

    let query_result = sqlx::query_as!(
        EquipeModel,
        "update equipe set nom = $1 where id = $2 returning *",
        body.nom.to_owned().unwrap_or(equipe.nom),
        equipe_id
    )
    .fetch_one(&data.db)
    .await;

match query_result{
    Ok(equipe) =>{
        let equipe_response = serde_json::json!({"status":"reussi", "data": serde_json::json!({
            "equipe":equipe
        })});
        return HttpResponse::Ok().json(equipe_response);
    }
    Err(err) =>{
        let message = format!("Erreur: {:?}", err);
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status":"erreur","message":message}));
    }
}
}

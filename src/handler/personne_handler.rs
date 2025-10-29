use crate::{
    models::equipe_model::EquipeModel,
    schema::equipe_schema::{CreateEquipeSchema,UpdateEquipeSchema},
    handler::appState::AppState,
};
use actix_web::{delete,get,patch,post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;





use crate::handler::{equipe_handler,personne_handler};
use actix_web::{ web, HttpResponse, Responder};
pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
        .service(equipe_handler::equipe_list_handler)
        .service(personne_handler::equies_list_handler);
    conf.service(scope);
}
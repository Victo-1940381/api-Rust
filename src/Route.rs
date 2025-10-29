use crate::handler::{equipe_handler,personne_handler,projets_handler,soustaches_handler,taches_handler};
use actix_web::{ web, HttpResponse, Responder};
pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
        .service(equipe_handler::equipe_list_handler)
        .service(equipe_handler::get_equipe_handler);
    conf.service(scope);
}
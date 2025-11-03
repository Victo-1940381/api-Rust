use crate::handler::{equipe_handler,personne_handler,projets_handler,soustaches_handler,taches_handler};
use actix_web::{ web, HttpResponse, Responder};
pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
        .service(equipe_handler::equipe_list_handler)
        .service(equipe_handler::get_equipe_handler)
        .service(equipe_handler::create_equipe_handler)
        .service(equipe_handler::edit_equipe_handler)
        .service(equipe_handler::delete_equipe_handler)
        .service(personne_handler::personne_list_handler)
        .service(personne_handler::get_personne_handler)
        .service(personne_handler::create_personnne_handler)
        .service(personne_handler::edit_personne_handler)
        .service(personne_handler::delete_personne_handler);
    conf.service(scope);
}
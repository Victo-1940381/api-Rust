use crate::{handler::{equipe_handler,personne_handler,projets_handler,soustaches_handler,taches_handler::{self, tache_list_handler}}, models::projets_model::ProjetsModel};
use actix_web::{ web, HttpResponse, Responder};
pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
        //equipe route
        .service(equipe_handler::equipe_list_handler)
        .service(equipe_handler::get_equipe_handler)
        .service(equipe_handler::create_equipe_handler)
        .service(equipe_handler::edit_equipe_handler)
        .service(equipe_handler::delete_equipe_handler)
        //personne route
        .service(personne_handler::personne_list_handler)
        .service(personne_handler::get_personne_handler)
        .service(personne_handler::create_personnne_handler)
        .service(personne_handler::edit_personne_handler)
        .service(personne_handler::delete_personne_handler)
        //projet route
        .service(projets_handler::projets_list_handler)
        .service(projets_handler::get_projets_handler)
        .service(projets_handler::create_projet_handler)
        .service(projets_handler::edit_projet_handler)
        .service(projets_handler::delete_projet_handler)
        //soustache route
        .service(soustaches_handler::soustache_list_handler)
        .service(soustaches_handler::get_soustache_handler)
        .service(soustaches_handler::create_soustache_handler)
        .service(soustaches_handler::edit_soustache_handler)
        .service(soustaches_handler::delete_soustache_handler)
        //tache route
        .service(taches_handler::tache_list_handler)
        .service(taches_handler::get_tache_handler)
        .service(taches_handler::create_tache_handler)
        .service(taches_handler::edit_tache_handler)
        .service(taches_handler::delete_tache_handler);
    conf.service(scope);
}
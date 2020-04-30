use interactive_paper::{resolve_intent, State, StateData, upload};
use std::sync::{RwLock, Arc};
use env_logger::Env;
use actix_web::{web, HttpRequest, HttpServer, get, post, HttpResponse, App, Responder, Error};
use actix_multipart::Multipart;
use actix_web::middleware::Logger;


#[post("/upload")]
async fn upload_document(state: web::Data<StateData>, mut payload: Multipart) -> HttpResponse {
    match upload(state, payload).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) =>  HttpResponse::InternalServerError().json(e)
    }
}

#[get("{text}")]
async fn resolve_input(state: web::Data<StateData>, req: HttpRequest) -> impl Responder {
    let text: String = req.match_info().get("text").unwrap().parse().unwrap();

    match resolve_intent(state, &text).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().json(err)    
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = State::new();

    let wrapped_state = Arc::new(RwLock::new(state));

     // logging
     env_logger::from_env(Env::default().default_filter_or("info")).init();

     // server
    let ip = "127.0.0.1:8088";

    HttpServer::new(move|| {
        App::new()
            .data(wrapped_state.clone())
            .service(upload_document)
            .service(resolve_input)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    }).bind(ip)?
    .run()
    .await
}

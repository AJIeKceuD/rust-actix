use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::time::Duration;

#[get("/")]
pub(crate) async fn hello() -> impl Responder {
    tokio::time::sleep(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here

    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub(crate) async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// this function could be located in a different module
pub(crate) fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

// this function could be located in a different module
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

use std::io::{Error};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, http::StatusCode, web};

use crate::coal;
struct AppState {
    source: String,
}

async fn render(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
    .status(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(
        coal::find_page(
            &data.source, 
        req.match_info().get("page").unwrap_or("index")
        ).unwrap()
    )
}

#[actix_web::main]
pub async fn start(source: String, port: String) -> Result<(), Error> {
    println!("🚂 Starting LoCoal Dev Server on port {} with source [{}]", port, source);

    let server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                source: source.clone(),
            })
            .route("/", web::get().to(render))
            .route("/{page}", web::get().to(render))
    });
    server.bind(format!("127.0.0.1:{}", port))?.run().await?;

    Ok(())
}
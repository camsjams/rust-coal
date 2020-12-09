use std::io::{Error};
use actix_files::Files;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

use crate::coal;
struct AppState {
    source: String,
    version: String,
}

async fn render(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    match coal::find_page(&data.source,req.match_info().get("page").unwrap_or("index"), &data.version) {
        Ok(response) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(response),
        _ => HttpResponse::NotFound()
            .content_type("text/html; charset=utf-8")
            .body("Not Found")
    }
}

#[actix_web::main]
pub async fn start(source: String, port: String) -> Result<(), Error> {
    println!("🚂 Starting LoCoal Dev Server on port {} with source [{}]", port, source);

    let server = HttpServer::new(move || {
        App::new()
            .data(AppState {
                source: source.clone(),
                version: env!("CARGO_PKG_VERSION").to_string()
            })
            .route("/", web::get().to(render))
            .route("/{page}/", web::get().to(render))
            .service(
                Files::new(
                    "/", 
                    format!("{}", source.clone())
                )
            )
    });
    server.bind(format!("127.0.0.1:{}", port))?.run().await?;

    Ok(())
}
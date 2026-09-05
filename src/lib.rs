use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health", web::get().to(health_check)))
        .listen(listener)?
        .run();
    Ok(server)
}

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

use be_email_news_letter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}

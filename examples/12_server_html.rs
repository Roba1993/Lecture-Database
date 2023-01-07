use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(askama_actix::Template, Default, Deserialize, Debug)]
#[template(path = "12_server.html")]
struct HelloTemplate {
    name: String,
}

#[get("/")]
async fn greet() -> impl Responder {
    HelloTemplate {
        name: "Stranger".into(),
    }
}

#[post("/")]
async fn home(item: web::Form<HelloTemplate>) -> impl Responder {
    HelloTemplate {
        name: item.name.clone(),
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home).service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

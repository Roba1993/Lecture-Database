use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:pwd@localhost")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn home(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let res = sqlx::query("SELECT * FROM users;")
        .fetch_all(&**pool)
        .await
        .unwrap();

    let mut output = String::from("User\n\n");

    for row in res {
        let id: i64 = row.try_get("id").unwrap();
        let firstname: String = row.try_get("firstname").unwrap();
        let lastname: String = row.try_get("lastname").unwrap();
        let age: i32 = row.try_get("age").unwrap();
        let student: bool = row.try_get("student").unwrap();

        output.push_str(&format!("{id} - {firstname} - {lastname} - {age} - {student}\n"));
    }

    output
}

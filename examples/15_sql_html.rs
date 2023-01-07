use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Pool, Postgres, Row,
};

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
            .service(filter)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/")]
async fn filter(filter: web::Form<Filter>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let res = sqlx::query("SELECT * FROM users WHERE firstname like $1;")
        .bind(&filter.filter)
        .fetch_all(&**pool)
        .await
        .unwrap();

    let users: Vec<User> = res.iter().map(|row| User::from_row(&row)).collect();

    IndexTemplate {
        name: filter.filter.clone(),
        users,
    }
}

#[get("/")]
async fn home(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let res = sqlx::query("SELECT * FROM users;")
        .fetch_all(&**pool)
        .await
        .unwrap();

    let users: Vec<User> = res.iter().map(|row| User::from_row(&row)).collect();

    IndexTemplate {
        name: "Superuser".into(),
        users,
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Filter {
    filter: String,
}

#[derive(askama_actix::Template)]
#[template(path = "15_index.html")]
struct IndexTemplate {
    name: String,
    users: Vec<User>,
}

#[derive(Debug, Clone)]
struct User {
    id: i64,
    firstname: String,
    lastname: String,
    age: u8,
    student: bool,
}

impl User {
    fn from_row(row: &PgRow) -> User {
        let age: i32 = row.try_get("age").unwrap();
        User {
            id: row.try_get("id").unwrap(),
            firstname: row.try_get("firstname").unwrap(),
            lastname: row.try_get("lastname").unwrap(),
            age: age as u8,
            student: row.try_get("student").unwrap(),
        }
    }
}

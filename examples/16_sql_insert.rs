use actix_web::{get, post, web, App, HttpServer, Responder};
use fake::{
    faker::{
        boolean::en::Boolean,
        name::en::{FirstName, LastName},
    },
    Fake,
};
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
            .service(add)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

#[post("/")]
async fn filter(filter: web::Form<Filter>, pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let res = sqlx::query("SELECT * FROM users WHERE firstname like $1 OR lastname like $1 LIMIT 100;")
        .bind(&filter.filter)
        .fetch_all(&**pool)
        .await
        .unwrap();

    let users: Vec<User> = res.iter().map(|row| User::from_row(&row)).collect();

    IndexTemplate {
        filter: filter.filter.clone(),
        users,
    }
}

#[get("/add/{amount}")]
async fn add(pool: web::Data<Pool<Postgres>>, amount: web::Path<usize>) -> impl Responder {
    // create the amount of users
    for _ in 0..*amount {
        let user = User::new_fake();
        sqlx::query(
            "INSERT INTO users (firstname, lastname, age, student) VALUES ($1, $2, $3, $4);",
        )
        .bind(user.firstname)
        .bind(user.lastname)
        .bind(user.age as i32)
        .bind(user.student)
        .execute(&**pool)
        .await
        .unwrap();
    }

    let res = sqlx::query("SELECT * FROM users LIMIT 100;")
        .fetch_all(&**pool)
        .await
        .unwrap();

    let users: Vec<User> = res.iter().map(|row| User::from_row(&row)).collect();

    IndexTemplate {
        filter: "".into(),
        users,
    }
}

#[get("/")]
async fn home(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let res = sqlx::query("SELECT * FROM users LIMIT 100;")
        .fetch_all(&**pool)
        .await
        .unwrap();

    let users: Vec<User> = res.iter().map(|row| User::from_row(&row)).collect();

    IndexTemplate {
        filter: "".into(),
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
    filter: String,
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

    fn new_fake() -> User {
        User {
            id: 0,
            firstname: FirstName().fake(),
            lastname: LastName().fake(),
            age: (1..120).fake(),
            student: Boolean(50).fake(),
        }
    }
}

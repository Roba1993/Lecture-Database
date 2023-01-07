use actix_web::{get, web, App, HttpServer, Responder};
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
        let user = User::from_row(&row);
        output.push_str(&user.format());
        output.push_str("\n");
    }

    output
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

    pub fn format(&self) -> String {
        let student = if self.student { "is student, " } else { "" };

        format!(
            "Person: {} {} {}has age: {} and id: {}",
            self.firstname, self.lastname, student, self.age, self.id
        )
    }
}

use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:pwd@localhost")
        .await
        .unwrap();

    let mut transaction = pool.begin().await.unwrap();
    let user = User {
        id: 111_111_111,
        firstname: "Bernd".into(),
        lastname: "Brot".into(),
        age: 14,
        student: false,
    };

    sqlx::query(
        "INSERT INTO users (id, firstname, lastname, age, student) VALUES ($1, $2, $3, $4, $5);",
    )
    .bind(user.id)
    .bind(&user.firstname)
    .bind(&user.lastname)
    .bind(user.age as i32)
    .bind(user.student)
    .execute(&mut transaction)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO users (id, firstname, lastname, age, student) VALUES ($1, $2, $3, $4, $5);",
    )
    .bind(user.id)
    .bind(&user.firstname)
    .bind(&user.lastname)
    .bind(user.age as i32)
    .bind(user.student)
    .execute(&mut transaction)
    .await
    .unwrap();

    transaction.commit().await.unwrap();

    println!("Done...");
    Ok(())
}

#[derive(Debug, Clone)]
struct User {
    id: i64,
    firstname: String,
    lastname: String,
    age: u8,
    student: bool,
}

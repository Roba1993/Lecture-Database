use postgres::{Client, NoTls};

fn main() {
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    client
        .execute(
            "UPDATE users SET firstname=$1 where id=$2;",
            &[
                &"2; DROP TABLE users;",
                &(2 as i64)
            ],
        )
        .unwrap();
}
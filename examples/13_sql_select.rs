use postgres::{Client, NoTls};

fn main() {
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    for row in client.query("SELECT * FROM users;", &[]).unwrap() {
        let id: i64 = row.get("id");
        let firstname: String = row.get("firstname");
        println!("{id}: {firstname}");
    }

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
    pub fn new(id: i64, firstname: &str, lastname: &str, age: u8) -> Self {
        User {
            id,
            firstname: firstname.into(),
            lastname: lastname.into(),
            age,
            student: false,
        }
    }

    pub fn print(&self) {
        println!(
            "Person: {} {} has age: {} and id: {}",
            self.firstname, self.lastname, self.age, self.id
        );
    }
}

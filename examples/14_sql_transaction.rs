use postgres::{Client, NoTls};

fn main() {
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();
    
    let mut persons = Vec::new();
    persons.push(User::new(10215, "Robert", "Schütte", 30));
    persons.push(User::new(10216, "Flo", "Hamer", 28));
    persons.push(User::new(10217, "Bernd", "Fröhlich", 24));

    let person = User::new(10215, "Robert", "Schütte", 30);

    let mut tx = client.transaction().unwrap();
    for person in persons {
        tx.execute(
            "INSERT INTO users (firstname, lastname, age, student) VALUES ($1, $2, $3, $4)",
            &[&person.firstname, &person.lastname, &(person.age as i32), &person.student],
        ).unwrap();
    }
    tx.commit().unwrap();
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

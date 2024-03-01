use rand::Rng;

fn main() {
    let mut persons = Vec::new();
    persons.push(User::new(10215, "Robert", "Schütte", 30));
    persons.push(User::new(10216, "Flo", "Hamer", 28));
    persons.push(User::new(10217, "Bernd", "Fröhlich", 24));

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..persons.len());
    persons[index].print();
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

fn main() {
    let person1 = User::new(10215, "Robert", "Schütte", 30);
    let mut person2 = User::new(person1.id + 1, "Flo", "Hamer", person1.age);
    person2.student = true;

    person1.print();
    person2.print();
    println!("Total age: {:?}", person1.age + person2.age);
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

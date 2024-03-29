fn main() {
    let person1 = User {
        id: 10215,
        firstname: String::from("Robert"),
        lastname: "Schütte".into(),
        age: 30,
        student: false,
    };

    println!("Person 1: {:?}", person1);
    println!("Person: {} {}", person1.firstname, person1.lastname);
}

#[derive(Debug)]
struct User {
    id: i64,
    firstname: String,
    lastname: String,
    age: u8,
    student: bool,
}
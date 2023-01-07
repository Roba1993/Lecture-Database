fn main() {
    let person1 = User {
        id: 10215,
        firstname: String::from("Robert"),
        lastname: "Schütte".into(),
        age: 30,
        student: false,
    };
    let mut person2 = person1.clone();
    person2.firstname = "Flo".into();
    person2.lastname = "Hamer".into();
    person2.id = person2.id + 1;
    person2.student = true;

    println!("Person 1: {:?}", person1);
    println!("Person 2: {:?}", person2);
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

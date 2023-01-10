#![allow(unused)]

fn main() {
    let person1 = User {
        id: 10215,
        firstname: String::from("Robert"),
        lastname: "Schütte".into(),
        age: 30,
        student: false,
    };

    let addr = Address {
        user: person1,
        street: "Grünaustrasse".into(),
        number: "22b".into(),
        city: "Dietikon".into(),
        postcode: "8953".into(),
    };

    println!("Adress: {:?}", addr);
}

#[derive(Debug, Clone)]
struct User {
    id: i64,
    firstname: String,
    lastname: String,
    age: u8,
    student: bool,
}

#[derive(Debug, Clone)]
struct Address {
    user: User,
    street: String,
    number: String,
    city: String,
    postcode: String,
}

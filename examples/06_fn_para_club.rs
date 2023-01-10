fn main() {
    check("Robert", "Zürich", 30);
    check("Lena", "Koblenz", 26);
    check("Judith", "Zürich", 30);
    check("Maxi", "Hamburg", 30);
    check("Ben", "Stttgart", 22);
    check("Leni", "Heilbronn", 9);
    check("Roxi", "Heilbronn", 14);
    check("Franz", "Frankfurth", 17);
    check("Lukas", "Stuttgart", 20);
    check("Anna", "Berlin", 23);
    check("Anika", "Hohenburg", 24);
}

fn check(name: &str, addr: &str, age: u8) {
    println!("{name} of age {age} from {addr} wants to enter the club");

    if age < 10 {
        println!("No entry: {age} Really??? Oo");
    } else if age < 16 {
        println!("No entry: Get you a fake id...");
    } else if age < 18 {
        println!("Come In: At 24:00 you're out!");
    } else if age > 40 {
        println!("No entry: No grandparents allowed");
    } else if age < 21
        && [
            "Jan", "Niklas", "David", "Paul", "Lukas", "Leon", "Lea", "Maria", "Marie", "Sophie",
            "Anna", "Julia",
        ]
        .contains(&name)
    {
        println!("No entry: Find a better fake id...");
    } else if [
        "Zürich",
        "Hamburg",
        "Berlin",
        "Bremen",
        "München",
        "Köln",
        "Frankfruth",
    ]
    .contains(&addr)
    {
        println!("Come In: You know the rules");
    } else {
        println!("Come In: Be aware, this is real club, not like in {addr}");
    }
    println!("");
}

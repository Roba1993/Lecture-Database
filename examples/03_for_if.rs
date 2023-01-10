fn main() {
    println!("Age Printer");
    for x in 18..40 {
        if x < 30 {
            println!("Robert is only {x} years old");
        } else {
            println!("Robert is already {x} years old");
        }
    }
}

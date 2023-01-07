fn main() {
    let var = 30;
    let name = "Robert";

    if var >= 25 && name == "Robert" {
        println!("Robert ist alt");
    } else if name == "Robert" {
        println!("Robert ist jung");
    } else if var >= 25 {
        println!("Du bist alt");
    } else {
        println!("Du bist jung");
    }
}

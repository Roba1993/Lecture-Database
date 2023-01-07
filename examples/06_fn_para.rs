fn main() {
    let res = calculate(1, 1);
    println!("Result: {}", res);

    let res = calculate(150, 150);
    println!("Result: {}", res);
}

fn calculate(a: u8, b: u8) -> u8 {
    if a > 255/2 || b > 255/2 {
        return 0;
    }

    a + b
}


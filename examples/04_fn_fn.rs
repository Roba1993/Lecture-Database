fn main() {
    loops();
}

fn loops() {
    for _ in 0..10 {
        write_text();
    }
}

fn write_text() {
    println!("Written from a function");
}

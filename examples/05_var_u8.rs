fn main() {
    let variable: u8 = 66;
    println!("Decimal: {}", variable);
    println!("Hex:   {:#2x}", variable);
    println!("Binär: {:#010b}", variable);
    println!("Char:    {}", variable as char);
}
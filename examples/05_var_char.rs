fn main() {
    let variable: char = 'B';
    println!("Decimal: {}", variable as u8);
    println!("Hex:   {:#2x}", variable as u8);
    println!("Binär: {:#010b}", variable as u8);
    println!("Char:    {}", variable);
}

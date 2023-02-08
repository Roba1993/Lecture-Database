use std::ops::Add;

fn main() {
    let mut value: u128 = 1;
    for i in 0..20 {
        calculate(&i, &mut value);
    }
}

fn calculate(i: &i32, v: &mut u128) {
    *v = v.add(*v);
    println!("Loop: {i} - Value: {v}");
}

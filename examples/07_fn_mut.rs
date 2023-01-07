use std::ops::Mul;

fn main() {
    let mut value: u64 = 0;
    for i in 0..20 {
        calculate(&i, &mut value);
    }
}

fn calculate(i: &i32, v: &mut u64) {
    *v = v.mul(*v);
    println!("Loop: {i} - Value: {v}");
}


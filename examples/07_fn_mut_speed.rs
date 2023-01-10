fn main() {
    let mut speed = 0;
    accelerate(&mut speed);
    accelerate(&mut speed);
    accelerate(&mut speed);
    breaking(&mut speed);
    accelerate(&mut speed);
    accelerate(&mut speed);
    breaking(&mut speed);
    accelerate(&mut speed);
    println!("End Speed: {speed}");
}

fn accelerate(speed: &mut isize) {
    if *speed <= 0 {
        *speed = 10;
    } else if *speed < 50 {
        *speed += *speed;
    } else if *speed < 150 {
        *speed += *speed / 2;
    } else if *speed < 300 {
        *speed += 10;
    }

    println!("Speed: {speed}");
}

fn breaking(speed: &mut isize) {
    if *speed < 0 {
        *speed = 0;
    } else {
        *speed = *speed / 2;
    }

    println!("Speed: {speed}");
}

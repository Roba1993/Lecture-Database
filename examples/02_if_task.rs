fn main() {
    // Alter
    let age = 20;
    // Ampel licht
    let light = "on";
    // Auto Status
    let status = "on";

    if age >= 18 && age < 70 && light == "on" && status == "on" {
        println!("Accelerate");
    } else if age >= 18 && age < 70 && light == "on" && status != "on" {
        println!("Start the car");
    } else {
        println!("No right to accelerate");
    }
}

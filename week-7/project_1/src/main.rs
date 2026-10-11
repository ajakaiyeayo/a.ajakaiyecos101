use std::f64::consts::PI;
use std::io;

// Prompts the user and keeps asking until a valid positive number is entered
fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(n) if n > 0.0 => return n,
            _ => println!("Invalid input. Enter a positive number."),
        }
    }
}

fn trapezium_area() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");
    height / 2.0 * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter diagonal 1:");
    let diagonal2 = read_number("Enter diagonal 2:");
    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    base * altitude
}

fn cube_surface_area() -> f64 {
    let side = read_number("Enter the side length:");
    6.0 * side * side
}

fn cylinder_volume() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    PI * radius * radius * height
}

fn main() {
    println!("===== SHAPE CALCULATOR =====");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("Enter your choice (1-5):");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => println!("Area of trapezium = {:.2}", trapezium_area()),
        "2" => println!("Area of rhombus = {:.2}", rhombus_area()),
        "3" => println!("Area of parallelogram = {:.2}", parallelogram_area()),
        "4" => println!("Surface area of cube = {:.2}", cube_surface_area()),
        "5" => println!("Volume of cylinder = {:.2}", cylinder_volume()),
        _ => println!("Invalid choice. Please run again and pick 1 to 5."),
    }
}
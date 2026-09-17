use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a, b, and c (separated by spaces):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let values: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid numbers"))
        .collect();

    if values.len() != 3 {
        println!("Please enter exactly three values: a, b, c");
        return;
    }

    let (a, b, c) = (values[0], values[1], values[2]);

    if a == 0.0 {
        println!("a cannot be zero (not a quadratic equation).");
        return;
    }

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let sqrt_d = d.sqrt();
        let root1 = (-b + sqrt_d) / (2.0 * a);
        let root2 = (-b - sqrt_d) / (2.0 * a);
        println!("Two distinct real roots:");
        println!("x1 = {:.4}", root1);
        println!("x2 = {:.4}", root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Exactly one real root:");
        println!("x = {:.4}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imag_part = (-d).sqrt() / (2.0 * a);
        println!("No real roots. Complex roots:");
        println!("x1 = {:.4} + {:.4}i", real_part, imag_part);
        println!("x2 = {:.4} - {:.4}i", real_part, imag_part);
    }
}
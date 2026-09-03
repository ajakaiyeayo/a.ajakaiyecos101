fn main() {
    let sales = [450000.0, 1500000.0, 750000.0, 2850000.0, 250000.0];

    let sum: f64 = sales.iter().sum();
    let average = sum / sales.len() as f64;

    println!("Sum of sales: N{:.2}", sum);
    println!("Average sales: N{:.2}", average);
}    
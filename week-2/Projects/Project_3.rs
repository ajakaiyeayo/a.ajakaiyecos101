fn main() {
    let p: f64 = 210000.0;
    let k: f64 = 5.0;
    let n: i32 = 3;

    let a = p * (1.0 - k / 100.0).powi(n);

    println! ("The value of the TV after 3 years is N{:.2}", a);
  }  
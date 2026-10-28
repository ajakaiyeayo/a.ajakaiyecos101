struct Laptop {
    brand: String,
    quantity: u32,
    price: u64,
}
impl Laptop {
    fn cost(&self, n: u32) -> u64 {
        self.price * n as u64
    }
}
fn main() {
    let laptops = vec![
        Laptop { brand:String::from("HP"),  quantity: 10, price: 650_000},
        Laptop {brand:String::from("IBM"),   quantity: 6, price: 755_000},
        Laptop {brand:String::from("Toshiba"),  quantity: 10, price: 550_000},
        Laptop {brand:String::from("Dell"), quantity: 4, price: 850_000},
        ];

        let mut total: u64 = 0;
        for laptop in &laptops {
            let c = laptop.cost(3);
            println!("{}: N{}", laptop.brand, c);
            total += c;
        }

        println!("Total cost: N{}", total);

    }
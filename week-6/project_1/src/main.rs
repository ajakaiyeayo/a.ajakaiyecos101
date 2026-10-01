use std::io;

fn main() {
    let menu = [
        ("P", "Poundo Yam / Edinkaiko Soup", 3200),
        ("F", "Fried Rice & Chicken", 3000),
        ("A", "Amala & Ewedu Soup", 2500),
        ("E", "Eba & Egusi Soup", 2000),
        ("W", "White Rice & Stew", 2500),
    ];

    println!("--- THE RESTAURANT MENU ---");
    for (code, name, price) in menu {
        println!("{}  {:<28} N{}", code, name, price);
    }

    let mut food = String::new();
    println!("\nFood type:");
    io::stdin().read_line(&mut food).unwrap();
    let food = food.trim().to_uppercase();

    let mut qty = String::new();
    println!("Quantity:");
    io::stdin().read_line(&mut qty).unwrap();
    let qty: u32 = qty.trim().parse().unwrap_or(0);

    match menu.iter().find(|m| m.0 == food) {
        Some((_, name, price)) if qty > 0 => {
            let total = price * qty;
            let discount = if total > 10000 { total / 20 } else { 0 };
            println!("\n{} x{} = N{}", name, qty, total);
            println!("Discount: N{}", discount);
            println!("Pay: N{}", total - discount);
        }
        _ => println!("Invalid food type or quantity."),
    }
}
use std::io;

fn main() {
    let mut exp_input = String::new();
    println!("Is the employee experienced? (yes/no):");
    io::stdin()
        .read_line(&mut exp_input)
        .expect("Failed to read line");
    let experienced = exp_input.trim().eq_ignore_ascii_case("yes");

    let mut age_input = String::new();
    println!("Enter the employee's age:");
    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    let incentive: u32 = if !experienced {
        100_000
    } else if age >= 40 {
        1_560_000
    } else if age >= 30 {
        1_480_000
    } else {
        1_300_000
    };

    println!("Annual incentive: N{}", format_with_commas(incentive));
}

fn format_with_commas(n: u32) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().rev().collect();

    for (i, c) in chars.iter().enumerate() {
        if i != 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }

    result.chars().rev().collect()
}
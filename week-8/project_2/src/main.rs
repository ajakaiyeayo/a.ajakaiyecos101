fn highest_years(list: &[(&str, u32)]) -> u32 {
    let mut highest = 0;
    for (_, years) in list.iter() {
        if *years > highest {
            highest = *years;
        }
    }
    highest
}

fn main() {
    let candidates: Vec<(&str, u32)> = vec![
        ("Adaeze", 4),
        ("Tunde", 9),
        ("Chidi", 6),
        ("Fatima", 12),
        ("Emeka", 3),
        ("Obinna", 12),
    ];

    if candidates.is_empty() {
        println!("No candidates to interview.");
        return;
    }

    let top = highest_years(&candidates);
    println!("Highest years of experience: {}", top);

    for (name, years) in candidates.iter() {
        if *years == top {
            println!("{} ({} years)", name, years);
        }
    }
}
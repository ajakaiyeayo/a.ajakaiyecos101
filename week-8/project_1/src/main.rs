fn find_level(title: &str, table: &Vec<(&str, [&str; 4])>) -> String {
    for (level, titles) in table.iter() {
        for t in titles.iter() {
            if *t == "-" {
                continue; // skip the empty cell in the table
            }
            if t.to_lowercase() == title.to_lowercase() {
                return level.to_string();
            }
        }
    }
    String::from("Not found")
}

fn main() {
    let table: Vec<(&str, [&str; 4])> = vec![
        ("APS 1-2",   ["Intern", "-", "Paralegal", "Placement"]),
        ("APS 3-5",   ["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]),
        ("APS 5-8",   ["Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"]),
        ("EL1 8-10",  ["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"]),
        ("EL2 10-13", ["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]),
        ("SES",       ["CEO", "Dean", "Partner", "Principal"]),
    ];

    let positions = ["Associate", "Director", "Paralegal", "dean", "Janitor"];

    for p in positions.iter() {
        println!("{} -> {}", p, find_level(p, &table));
    }
}
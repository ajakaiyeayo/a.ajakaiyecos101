use std::io::Write;

fn main() {

    let lager = "Lager\n33 Export\nDesperados\nGolberg\nGulder\nHeineken\nStar\n\n";
    let stout = "Stout\nLegend\nTurbo King\nWilliams\n\n";
    let non_alcoholic = "Non-Alcoholic\nMaltina\nAmstel Malta\nMalta Gold\nFayrouz\n";

    let mut file = std::fs::File::create("breweries.txt").expect("create failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");

    println!("Data written to file");
}

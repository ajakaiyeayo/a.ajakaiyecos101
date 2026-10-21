use std::io::Write;

fn main() {

    let header = "\nLager, Stout, Non-Alcoholic";
    let row1 = "\n33 Export, Legend, Maltina";
    let row2 = "\n Desperados, Turbo King, Amstel Malta";
    let row3 = "\n Goldberg, Williams, Malta Gold";
    let row4 = "\n Gulder, , Fayrouz";
    let row5 = "\n Heineken, , ";
    let row6 = "\n Star, , ";


  let mut file = std::fs::File::create("breweries.csv").expect("write failed");
  file.write_all(header.as_bytes()).expect("write failed");
  file.write_all(row1.as_bytes()).expect("write failed");
  file.write_all(row2.as_bytes()).expect("write failed");
  file.write_all(row3.as_bytes()).expect("write failed");
  file.write_all(row4.as_bytes()).expect("write failed");
  file.write_all(row5.as_bytes()).expect("write failed");
  file.write_all(row6.as_bytes()).expect("write failed");

println!("breweries.csv created.");
}
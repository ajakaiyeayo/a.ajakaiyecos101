use std::io::Write;

fn main() {
    let vheader:Vec<&str> = vec!["Student Name", "Matric Number", "Department", "Level"];
    let vrow1:Vec<&str> = vec!["Oluchi Mordi", "ACC1021111", "Accounting", "300"];
    let vrow2:Vec<&str> = vec!["Adams Aliyu", "ECO10110101", "Economics", "100"];
    let vrow3:Vec<&str> = vec!["Shania Bolade", "CSC10328828", "Computer", "200"];
    let vrow4:Vec<&str> = vec!["Adekunle Gold", "EEE11020202", "Electrical", "200"];
    let vrow5:Vec<&str> = vec!["Bianca Edemoh", "MEE10202001", "Mechanical", "100"];

    let mut file = std::fs::File::create("PAU STMIS.csv").expect("Create Failed");
   file.write_all(vheader[0].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vheader[1].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vheader[2].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vheader[3].as_bytes()).expect("Write failed");
   file.write_all("\n".as_bytes()).expect("Write failed");

    file.write_all(vrow1[0].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow1[1].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow1[2].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow1[3].as_bytes()).expect("Write failed");
   file.write_all("\n".as_bytes()).expect("Write failed");

    file.write_all(vrow2[0].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow2[1].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow2[2].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow2[3].as_bytes()).expect("Write failed");
   file.write_all("\n".as_bytes()).expect("Write failed");

    file.write_all(vrow3[0].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow3[1].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow3[2].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow3[3].as_bytes()).expect("Write failed");
   file.write_all("\n".as_bytes()).expect("Write failed");

   file.write_all(vrow4[0].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow4[1].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow4[2].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow4[3].as_bytes()).expect("Write failed");
   file.write_all("\n".as_bytes()).expect("Write failed");

    file.write_all(vrow5[0].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow5[1].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow5[2].as_bytes()).expect("Write failed");
   file.write_all(",".as_bytes()).expect("Write failed");
   file.write_all(vrow5[3].as_bytes()).expect("Write failed");
   file.write_all("\n".as_bytes()).expect("Write failed");

   
   println!("PAU STMIS FILE CREATED");

}

use std::io::Write;

fn main() {
    let vheader_alldata = vec!["Commissioner", "Ministry", "Zone"];
    let commissioner:Vec<&str> = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry:Vec<&str> = vec!["Internal affairs", "Justice","Defense", "Power & Steel", "Petroleum"];
    let zone:Vec<&str> = vec!["South West", "North East", "South South", "South West", "South East"];

     let mut file = std::fs::File::create("EFCC Merge File.csv").expect("Create Failed");
   
for i in 0..commissioner.len() {

   file.write_all(commissioner[i].as_bytes())
    .expect("Write failed");

   file.write_all(",".as_bytes())
    .expect("Write failed");

    file.write_all(ministry[i].as_bytes())
    .expect("Write failed");

   file.write_all(",".as_bytes())
    .expect("Write failed");

     file.write_all(zone[i].as_bytes())
    .expect("Write failed");

   file.write_all("\n".as_bytes())
    .expect("Write failed");

}

   println!("The File has been successfully created");

}

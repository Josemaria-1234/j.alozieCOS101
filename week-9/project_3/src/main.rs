use std::io::Read;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Convicted_Ministers.txt").expect("CREATE FAILED");
    file.write_all("Name of Commisioner               Ministry                  Geopolitical Zone\n".as_bytes()).expect("WRITE FAILED");

    let name = vec!["Aigbogun Alamba Daudu   ", "Murtala Afeez Bendu     ", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi    ", "Osazuwa Faith Etieye    ",];
    let ministry = vec!["Internal Affairs", "Justice         ", "Defense         ", "Power & Steel   ", "Petroleum       "];
    let gp = vec!["South West ", "North East ", "South South", "South West ", "South East ",];

    for i in 0..5 {
        file.write_all(format!("{}          {}          {}\n", name[i], ministry[i], gp[i]).as_bytes()).expect("WRITE FAILED");
    }
    println!("\nData written to file successfully.");
    println!();

    let mut file = std::fs::File::open("Convicted_Ministers.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
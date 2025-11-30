use std::io::Read;
use std::io::Write;

fn main() {

    let mut file = std::fs::File::create("Nigeria_Brewery_Ltd.txt").expect("create failed");
    file.write_all("Lager         Stout          Non-Alcoholic
33 Export      Legend         Maltina
Goldberg       Turbo king     Amstel Maltina
Gulder         Williams       Malta Goldberg
Heineken                      Fayrouz
Star"
        .as_bytes()).expect("write failed");

    println!("\nData written to file." );
}

fn main() {
    let mut city : Vec<String> = Vec::new();
    println!("The city vector has element: {}", city.len());
    
    let mut city_num = String::new();
    println!("How many cities do you want to enter?");
    std::io::stdin().read_line(&mut city_num).expect("Failed to read input");
    let city_num: u32 = city_num.trim().parse().expect("Invalid Input");
    
    for count in 0..city_num {
        let mut new_city = String::new();
        println!("Enter the name of city {}:", count + 1);
        std::io::stdin().read_line(&mut new_city).expect("Failed to read input");
        let new_city:String = new_city.trim().parse().expect("Invalid Input");
        city.push(new_city);
    }

    println!("Your preferred cities are:");
    let mut count =1;
    for i in city
    {
        println!("{}. {}", count, i);
        count +=1;
    }
}
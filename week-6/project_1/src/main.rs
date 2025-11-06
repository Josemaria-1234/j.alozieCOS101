use std::io;

fn main() {
    println!("Welcome to the Food Ordering System!");
    println!("Here is our menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");

    println!("\nPlease enter the letter of the food item you'd like to order:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
        
    let choice = choice.trim().to_uppercase();

    match choice.as_str() {
        "P" => println!("You ordered Poundo Yam/Edinkaiko Soup. Total: N3,200"),
        "F" => println!("You ordered Fried Rice & Chicken. Total: N3,000"),
        "A" => println!("You ordered Amala & Ewedu Soup. Total: N2,500"),
        "E" => println!("You ordered Eba & Egusi Soup. Total: N2,000"),
        _ => println!("Invalid choice. Please select a valid menu item."),
    }
}
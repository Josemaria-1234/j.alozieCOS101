use std::io;

fn main() {
    println!("Welcome to Jose's Palace where you are served the most exquisite meals! Heaven on Earth!");
    println!("Our Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,500");
    println!("W = White Rice & Stew - N2,500");

    println!("\nEnter the letter of the meal you want, as shown on the menu:");
    let mut item = String::new();
    io::stdin().read_line(&mut item).expect("Failed to read input");
    let item = item.trim().to_uppercase();

    println!("How many Portions would you like to have ?");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let qty: u32 = qty.trim().parse().unwrap_or(0);

    let price = match item.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" | "E" | "W" => 2500,
        _ => {
            println!("Invalid letter typed.");
            return;
        }
    };

    let total = price * qty;

    if total > 10_000 {
        let discount = total / 20; // 5% of total
        let final_total = total - discount;
        println!("You get a 5% discount of N{}!", discount);
        println!("Your discounted price to pay: N{}", final_total);
    } else {
        println!("Your total pay is: N{}", total);
    }

    println!("Thanks for Eating at Jose's palace");
}

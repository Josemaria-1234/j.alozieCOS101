use std::io;

fn main() {
    // Ask if the user is experienced

    println!("Are you experienced in this field ? (yes/no):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Convert input to lowercase and trim lowercase
    let input = input.trim().to_lowercase();

    // Convert to boolean
    let experience: bool = input == "yes";

    if experience {
        // Ask for age
        println!("Enter your age:");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let mut age: i32 = age.trim().parse().expect("Please enter a valid number");

        if age >= 40 {
            println!("Your incentive is #1,560,000.00");
        }

        else if age >= 30 && age < 40 {
            println!("Your incentive is #1,480,000.00");
        }

        else if age <28 {
            println!("Your incentive is #1,300,000.00");
        }

    }

    
    else {
        println!("Your incentive is #100,000");
    }
    


    
}

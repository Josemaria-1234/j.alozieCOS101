use std::io;

fn main() {
    println!("Choose a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_input("Enter your choice (1-5):").trim().parse::<u32>().unwrap_or(0);

    match choice {
        1 => {
            println!("Formula for Area of Trapezium is (a x b)h 
                                __________
                                    2");
            let height = read_input("Enter height:").trim().parse::<f64>().unwrap();
            let a = read_input("Enter 1st base:").trim().parse::<f64>().unwrap();
            let b = read_input("Enter 2nd base:").trim().parse::<f64>().unwrap();
            let area = (height / 2.0) * (a + b);
            println!("Area of Trapezium = {:.2}", area);
        }
        2 => {
            println!("Formula for Area of Rhombus is (d1 x d2)
                               __________
                                   2");
            let d1 = read_input("Enter diagonal1:").trim().parse::<f64>().unwrap();
            let d2 = read_input("Enter diagonal2:").trim().parse::<f64>().unwrap();
            let area = 0.5 * d1 * d2;
            println!("Area of Rhombus = {:.2}", area);
        }
        3 => {
            println!("Formula for Area of Trapezium is b x h");
            let base = read_input("Enter base:").trim().parse::<f64>().unwrap();
            let height = read_input("Enter height:").trim().parse::<f64>().unwrap();
            let area = base * height;
            println!("Area of Parallelogram = {:.2}", area);
        }
        4 => {
            println!("Formula for Area of Cube is L³");
            let length = read_input("Enter length:").trim().parse::<f64>().unwrap();
            let area = 6.0 * length.powi(2);
            println!("Area of Cube = {:.2}", area);
        }
        5 => {
            println!("Formula for Volume of Cylinder is πr²h");
            let radius = read_input("Enter radius:").trim().parse::<f64>().unwrap();
            let height = read_input("Enter height:").trim().parse::<f64>().unwrap();
            let volume = std::f64::consts::PI * radius.powi(2) * height;
            println!("Volume of Cylinder = {:.2}", volume);
        }
        _ => println!("Invalid choice. Please run the program again."),
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

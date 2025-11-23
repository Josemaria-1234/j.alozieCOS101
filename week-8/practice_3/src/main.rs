//method to print the get value
fn value(n:Option<&char>) {
    println!("Element of vector: {:?}", n);
}

fn main() {
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'A', 'N'];

    let mut index = String::new();
    println!("\n Enter an index value between 0 and 8: ");
    std::io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Invalid input");

    let ch: Option<&char> = v.get(index);
    value(ch);
}
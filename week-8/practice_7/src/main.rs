fn main() {
    let dt_tuple: (&str, f32,  u8) = ("Rust", 3.14, 100);
    //dt = data type
    println!("Tuple contents: {:?}", dt_tuple);

    let ndt_tuple = ("Rust", "lol", 100);
    //ndt = no data type
    println!("Tuple contents: {:?}", ndt_tuple);

    println!("Value at Index 0 = {}", dt_tuple.0);
    println!("Value at Index 1 = {}", dt_tuple.1);
    println!("Value at Index 2 = {}", dt_tuple.2);
}
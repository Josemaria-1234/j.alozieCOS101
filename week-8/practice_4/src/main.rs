fn main() {
    //Name vector
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];
    let age = vec![34,27,19,24,32,19,17,18];

    println!("Age allocation: ");

    for i in 0..age.len(){
        println!("{} is {} years old", name[i], age[i]);
    }
}
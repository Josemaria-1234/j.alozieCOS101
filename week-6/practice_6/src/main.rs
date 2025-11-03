fn main() {

    let n1 = "Eletrical".to_string();
    let n2 = "Eletronic".to_string();
    let n3 = "Engineering".to_string();
    let n4 = n1 + &n2 + &n3;

    //About Eletrical/Electronic
    println!("\nThe {} is formed by the apiration to 
     train Eletrical/Eletronic Engineering professionals
     in the areas of design, building and maintenance of
     eletrical control systems,", n4);

    let w1 = "Computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2;
    println!();
    println!("{} is aimed at developing competent, creative,
     innovative, entrepreneurial and ethically-minded persons,
     capable of creating value in diverse fields of
     Computer Science.",w3 );
}

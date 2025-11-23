
use std::io;
fn main() {
    let mut candidate: Vec<(String, u32)> = Vec::new();
    let mut n = String::new();
    println!("Enter number of employee candidates: ");
    io::stdin().read_line(&mut n).expect("FAILED TO READ LINE");
    let n: usize = n.trim().parse().expect("INVALID INPUT");
    
    for i in 0..n {
        let mut name = String::new();
        println!("Enter candidate {}'s name: ",i + 1);
        io::stdin().read_line(&mut name).expect("FAILED TO READ INPUT");
        let name:String = name.trim().to_string();

        let mut xp = String::new();
        println!("Enter candidate {}'s years of experience: ", i + 1);
        io::stdin().read_line(&mut xp).expect("FAILED TO READ INPUT");
        let xp: u32 = xp.trim().parse().expect("INVALID INPUT");
        candidate.push((name, xp));
    }

    let mut max = &candidate[0];
    for c in &candidate {
        if c.1 > max.1 {
            max = c;
        }
    }
    println!("Candidate with the most experience: {} with {} years", max.0, max.1);
}
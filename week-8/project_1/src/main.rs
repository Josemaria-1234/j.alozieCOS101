use std::io;
fn main() {
    let job:Vec<&str> = vec!["Office Administrator", "Academic", "Lawyer", "Teacher"]; 
    println!("Jobs: {:?}", job);

    let office_admin: Vec<&str> = vec!["Intern", "Administrator", "Senior administrator", "Office Manager", "Director", "CEO"];
    let academic: Vec<&str> = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer: Vec<&str> = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2","Senior Associate 3-4", "Partner"];
    let teacher: Vec<&str> = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("Enter job: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("FAILED TO READ LINE");
    let choice = choice.trim().to_lowercase();
    println!();

    if choice == job[0].to_lowercase().trim() {
        println!("Job titles: {:?}", office_admin);
        aps(office_admin);
    } else if choice == job[1].to_lowercase().trim() {
        println!("Job titles: {:?}", academic);
        aps(academic);
    } else if choice == job[2].to_lowercase().trim() {
        println!("Job titles: {:?}", lawyer);
        aps(lawyer);
    } else if choice == job[3].to_lowercase().trim() {
        println!("Job titles: {:?}", teacher);
        aps(teacher);
    } else {println!("Invalid Job entered.")} 
}

fn aps(x:Vec<&str>) {
    let mut lvl = String::new();
    println!("Enter job title: ");
    io::stdin().read_line(&mut lvl).expect("FAILED TO READ LINE");
    let lvl:String = lvl.trim().to_lowercase();
    println!();

    println!("Enter years of experience: ");
    let mut y = String::new();
    io::stdin().read_line(&mut y).expect("FAILED TO READ LINE");
    let y: u32 = y.trim().parse().expect("INVALID INPUT");
    println!();

    if lvl == x[0].to_lowercase() && y >= 1 && y <= 2 {
        println!("Staff Level: APS 1-2");
    } else if lvl == x[1].to_lowercase() && y >= 3 && y <= 5 {
        println!("Staff Level: APS 3-5");
    } else if lvl == x[2].to_lowercase() && y >= 6 && y <= 8 {
        println!("Staff Level: APS 6-8");
        //using 6-8 instead of 5-8 to avoid ovwerlap errors
    } else if lvl == x[3].to_lowercase() && y >= 9 && y <= 10 {
        println!("Staff Level: EL1 9-10");
        //9-10 instead of 8-10 to avoid overlap errors
    } else if lvl == x[4].to_lowercase() && y >= 11 && y <= 13 {
        println!("Staff Level: EL2 11-13");
        //11-13 instead of 10-13 to avoid overlap errors
    } else if lvl == x[5].to_lowercase() && y > 13 {
        println!("Staff Level: SES");
    } else {println!("Invalid job title entered, or years of experience do not match APS levels");}
}
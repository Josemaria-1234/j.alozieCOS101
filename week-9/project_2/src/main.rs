use std::io::Write;


fn main() {
   let mut file = std::fs::File::create("PAU-SMIS.txt").expect("create failed");
    file.write_all("Student name   |    Matric. number   |    Department   |   Level\n"
        .as_bytes())
        .expect("write failed");

    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold", "Blanca Edemoh"];
    let matric_no = vec!["ACC10211111","ECO101110101","CSC10328828","EEE11020202","MEE1020222001"];
    let dept = vec!["Accounting", "Economics", "Computer","Eletrical","Mechanical"];
    let level = vec!["300", "100", "200", "200", "100"];

    for i in 0..5{
        let line = format!("   {}   |   {}   |   {}   |   {}\n",student_name[i], matric_no[i], dept[i], level[i]);
        file.write_all(line.as_bytes()).expect("write failed");
    }
    println!("\nData has been written to file successfully." );

}

fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The sum of num is:{}",num);
}

fn mutate_num_to_zero(param_num:mut i32){
    *param_num = *param_num*0;// de reference
    println!("The param_num value is :{}",param_num);
}
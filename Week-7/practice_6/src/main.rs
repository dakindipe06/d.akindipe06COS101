fn main() {
    let mut num:i64 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of no is:{}",num);
}

fn mutate_num_to_zero(param_num:&mut i64) {
    *param_num = *param_num*0; //de reference
    println!("param_num value is :{}",param_num);
}

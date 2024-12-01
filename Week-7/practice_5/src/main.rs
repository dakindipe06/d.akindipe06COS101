fn main() {
    let num:i64 = 5;
    mutate_num_to_zero(num);
    println!("The value of no is:{}",num);
}

fn mutate_num_to_zero(mut param_num: i64) {
    param_num = param_num*0;
    println!("param_num value is :{}",param_num);
}

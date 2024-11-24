fn main() {
    let a:i64 = 10;
    let b:i64 = 20;

    println!("Value of a:{} ",a);
    println!("Value of b : {} ",b);

    let mut res = a>b ;
    println!("a greater than b: {} ",res);

    res = a<b ;
    println!("a lesser than b: {} ",res) ;


    res = a>=b ;
    println!("a greater than of equal to b: {} ",res);

    res = a<=b;
    println!("a is lesser or equal to b: {} ",res);

    res = a==b ;
    println!("a is equal to b: {}",res) ;

    res = a!=b ;
    println!("a is not equal to b: {} ",res);
}

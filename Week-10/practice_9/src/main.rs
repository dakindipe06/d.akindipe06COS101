//define dimensions of a rectangele
struct Rectangle {
    width:u64, height:u64
}

//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self)->u64{
        //use the . operator to fetch the value via the self keyword
        self.width * self.height
    }
}

fn main() {
    // instantiate the structure
    let small = Rectangle {
        width:10,
        height:20
    };
    //print the rectangle's area
    println!("width is {} \n height is {} \n area of Rectangle
    is {}",small.width,small.height,small.area());
}
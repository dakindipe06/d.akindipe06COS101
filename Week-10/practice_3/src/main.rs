fn main() {
   
   let v = vec![20, 40, 60, 80];
   // vector v owns the object in heap

   let v2 = v;
   let v2_return = display(v2);
   println!("In main {:?}",v);
}

fn display(v:Vec<i64>)->Vec<i64> {
    // returning same vector
    println!("inside display {:?}",v);
    return v;
}
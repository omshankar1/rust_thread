#![allow(dead_code, unused_imports, unused_variables)]
fn main() {
    // POD types: COPY
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);


    // Simple extension to POD types
    type TripletInt = (i32, i32, i32);
    let t: TripletInt = (3,3,3);
    let s = t;
    println!("{:?}, {:?}", s, t);

    type TripletStrSlice<'a> = (i32, i32, &'a str);
    let t: TripletStrSlice = (3,3, "Hello");
    let s = t;
    println!("{:?}, {:?}", s, t);
    
    type TripletStr = (i32, i32, String);
    // let t: TripletStr = (3,3, "Hello".to_string());
    // let s = t;
    // println!("{:?}, {:?}", s, t);

    #[derive(Debug, Copy, Clone)]
    struct StructTuple(i32, i32);
    
    let t = StructTuple(3,3);
    println!("{:?}", t);
    let s = t;
    println!("{:?}, {:?}", s, t);
}

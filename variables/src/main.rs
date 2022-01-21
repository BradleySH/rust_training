use std::io;

fn main() {
   //let x = 2.0; // f64
   //let y: f32 = 3.0; //f32

   // Tuple 
   //let tup: (i32, f64, u8) = (500, 6.4, 1);

//    let tup = (500, 6.4, 1);

//    let (x, y, z) = tup;

//    println!("The value of y is: {}", y);

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    //  ARRAY TYPE you know the fixed number need to be in the array

    //let a = [1, 2, 3, 4, 5];
      
    // let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);



}

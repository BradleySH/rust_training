#![allow(unused)]
use std::fs::File;
fn main() {
    //  Panics
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    // Recovrable errors with results
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    // T represent the type of the error that will be returned in a success case withing the Ok
    //  E represents the type of error that will be returned in a failure case within Err

    //let f = File::open("hello.txt");
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem oprning the file: {:?}", error),
    };
}

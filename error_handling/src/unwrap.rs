use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
unwrap
let f = File::open("hello.txt").unwrap();

expect
let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

//  Propogating Errors

// fn read_username_from_file() -> Result<String, io::Error> {
//   let f = File::open("hello.txt");

//   let mut f = match f {
//     Ok(file) => file,
//     Err(e) => return Err(e),
//   };

//   let mut s = String::new();

//   match f.read_to_string(&mut s) {
//     Ok(_) => Ok(s),
//     Err(e) => Err(e),
//   }
// }

//  -- Shortcut for propogating errors: the ? Operator
fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}
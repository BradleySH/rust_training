// matching on differernt errors
use std::fs::File;
use std::io:;ErrorKind;

// fn main() {
//   let f = File::open("hello.txt");

//   let f = match f {
//     Ok(file) => file,
//     Err(error) +> match error.kind() {
//       ErrorKind::NotFound => match File::create("hello.txt") {
//         Ok(fc) => fc,
//         Err(e) => panic!("Problem creating the file: {:?}", e),
//       },
//       other_error => {
//         panic!("Problem opening the file: {:?}", other_error)
//       }
//     },
//   };
// }

//  More seasoned coder would use This
fn main() {
  let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
});
}

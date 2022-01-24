// fn main() {
//     let v: Vec<i32> = Vec::new();
//      defaults to i32
//     let v = vec![1, 2, 3];

//       MODIFY VECTOR
//     let mut v = Vec::new();

//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
// }
// --------------------------------------------------
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);

//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element"),
//     }
// }
// -----------------------------------------------------
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let does_not_exist = &v[100];
//     let does_not_exist = v.get(100);
// }
// --------------------------------------------------
// fn main() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//     }
// }
// fn main1() {
//     let v1 = vec![100, 32, 57];
//     for i in &v1 {
//         println!("{}", i);
//     }
// }
// --------------------------------------------------
// ENUMS with Vectors
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

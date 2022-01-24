// hashmaps
// stores a mapping of keys of type to values of type
// fn main() {
//   use std::collections::HashMap;

//   let mut scores = Hashmap::new();

//   scores.insert(String::from("Blue"), 10);
//   scores.insert(String::from("Yellow"), 50);
// }

// -- Using collect()
// fn main() {
//   use std::collections::HashMap;

//   let teams = vec![String::from("Blue"), String::from("Yellow")];
//   let initial_scores = vec![10, 50];

//   let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
// }

// Ownership of hashmaps
// fn main() {
//   use std::collections::HashMap;

//   let field_name = String::from("Favorite color");
//   let field_value = String::from("Blue");

//   let mut map = HashMap::new();
//   map.insert(field_name, field_value);
//   // field_name and field_value are invalid at this point, try using them and
//   // see what compiler error you get!
// }

// Getting the value of the hashmap by providing the key to the get method

// fn main() {
//   use std::collections::HashMap;

//   let mut scores = Hashmap::new();

//   scores.insert(String::from("Blue"), 10);
//   scores.insert(String::from("Yellow"), 50);

//   let team_name = String::from("Blue");
//   let score = scores.get(&team_name);
// }

// fn main() {
//   use std::collections::HashMap;

//   let mut scores = HashMap::new();

//   scores.insert(String::from("Blue"), 10);
//   scores.insert(String::from("Yellow"), 50);

//   for (key, value) in &scores {
//     println!("{} {}", key, value);
//   }
// }

//   ---- OVERWITING a VALUE
// fn main() {
//   use std::collections::HashMap;

//   let mut scores = HashMap::new();

//   scores.insert(String::from("Blue"), 10);
//   scores.insert(String::from("Blue"), 25);

//   println!("{:?}", scores);

// }

// fn main() {
//   use std::collections::HashMap;

//   let mut scores = HashMap::new();
//   scores.insert(String::from("Blue"), 10);

//   scores.entry(String::from("Yellow")).or_insert(50);
//   scores.entry(String::from("Blue")).or_insert(50);

//   println!("{:?}", scores);
// }

//  -------------------UPDATING a value based on the old value

fn main() {
  use std::collections::HashMap;

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}

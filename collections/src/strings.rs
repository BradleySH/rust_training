fn main() {
  //let mut s = String::new();

  // concat strings - appending to a string with push_str and push
  let mut s = String::from("foo");
  s.push_str("bar");

  // use string slice as we dont want to take ownership
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {}", s2);

  // push methods
  let mut s = String::from("lo");
  s.push("l");
}

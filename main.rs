fn main() {
  let mut name = "Pascal";
  println!("{}", name);
  
  name = "Alice";
  println!("{}", name);
}

fn main() {
  let name = "Pascal";
  let another_name = "Alice";
  println!("{} and {}", name, another_name);
}
fn main() {
  let first = "Pascal".to_string();
  let last = "Precht".to_string();
  
  say_name(first, last);
}

fn say_name(first: String, last: String) {
  println!("{} {}", first, last);
}
fn main() {
  let first = "Pascal".to_string();
  
  say_first_name(&first);
  say_first_name(&first);
}

fn say_first_name(first: &String) {
  println!("{}", first);
}






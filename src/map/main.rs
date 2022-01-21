use std::collections::HashMap;
fn main() {
  let mut map = HashMap::new();
  map.insert("foo", "bar");
  map.insert("color", "red");
  println!("{}", map.get("color").unwrap());
  println!(
    "{}",
    match map.get("foo1") {
      Some(x) => x.to_string(),
      None => "Error".to_string(),
    }
  );
  map.insert("color", "blue");
  println!("{:?}", map);
  map.entry("color").or_insert("yellow");
  for p in map.iter() {
    println!("{:?}", p);
  }
}

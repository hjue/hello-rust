fn main() {
  let v = vec![1, 2, 3, 4];
  println!(
    "{}",
    match v.get(5) {
      Some(x) => x.to_string(),
      None => 0.to_string(),
    }
  );
  println!("{}", v[3]);
}

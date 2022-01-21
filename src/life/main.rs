fn longer(s1: &str, s2: &str) -> &str {
  if s1.len() > s2.len() {
    s1
  } else {
    s2
  }
}
fn main() {
  let r;

  let s1 = "hello1";
  let s2 = "world";
  r = longer(s1, s2);

  println!("r: {}", r);
}

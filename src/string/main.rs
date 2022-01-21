fn main() {
  let mut s1 = String::new();
  let s2 = String::from("world");
  s1 += "hello";
  s1.push_str(", ");
  let s3 = s1 + "," + &s2;
  println!("{}", s3);
  for i in s2.chars() {
    println!("{}", i);
  }
  println!("{}", "你好".chars().count()); // 2
  println!("{}", "你好".len()); // 6
  println!("{}", "你好".chars().nth(1).unwrap());
}

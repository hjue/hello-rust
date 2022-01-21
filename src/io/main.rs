fn read_args() {
  let args = std::env::args();
  for arg in args {
    println!("{}", arg);
  }
}

fn read_input() {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input);
  println!("You typed: {}", input);
}
fn main() {
  read_input();
}

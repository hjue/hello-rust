mod sub_module;
use std::fs::File;

fn main() {
  println!("Hello!");
  File::open("a.txt").unwrap(); //.expect("Couldn't open");
  sub_module::message();
}

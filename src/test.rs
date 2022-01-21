mod second_module;
use std::fs::File;

use crate::second_module::message;
fn main() {
  enum Book {
    Papery { index: u32 },
    Electronic { url: String },
  }

  let book = Book::Papery { index: 1001 };
  let ebook = Book::Electronic {
    url: String::from("url..."),
  };
  match book {
    Book::Papery { index } => {
      println!("Papery book {}", index);
    }
    Book::Electronic { url } => {
      println!("E-book {}", url);
    }
  }
  // panic!("error occured");
  let f2 = File::open("hello.txt").expect("Failed to open.");
  println!("{}", message());
}

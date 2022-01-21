fn main() {
  struct Article {
    title: String,
    words: i32,
  }

  let a: Article = Article {
    title: String::from("hello"),
    words: 10,
  };

  println!("{}", a.title);
}

mod calculator;
mod visibility;
pub use visibility::Book;
fn main() {
    println!("{}", calculator::add(2, 3));
    println!("{}", calculator::advanced::square(10));

    let book = Book::new("Rust", "Anoop");
    println!("name: {}, author:{}", book.title, book.get_author());
}

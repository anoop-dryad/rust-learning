// ============================================================================
// Layer 4, Check 2a — Methods with impl (&self / &mut self / self)
//
// Run: cargo run -p structs --bin methods
//
// The struct (data) and impl (behavior) are SEPARATE blocks.
// The first parameter chooses how the method uses the instance — the SAME
// ownership choice as Layer 3 function params, spelled for `self`:
//   &self      -> borrow (read)      | most common
//   &mut self  -> mutable borrow     | method modifies the instance
//   self       -> takes ownership    | method CONSUMES the instance (rare; name it into_*)
// ============================================================================

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32, // a count can't be negative -> unsigned is the idiomatic choice
    available: bool,
}

impl Book {
    // &self: only reads -> shared borrow. Callable many times; book stays valid.
    fn is_long(&self) -> bool {
        self.pages > 300
    }

    // &self: reads, returns an OWNED String built with format!
    fn description(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }

    // &mut self: mutates -> mutable borrow. Requires the instance be `mut`.
    fn mark_as_unavailable(&mut self) {
        self.available = false;
    }

    // self (by value): CONSUMES the whole Book. After calling this, the book is
    // MOVED into the method and gone. Convention: methods taking self that convert
    // into something are named `into_*`.
    fn into_title(self) -> String {
        self.title
    }
}

fn main() {
    let mut book = Book {
        title: String::from("Rust Learning"),
        author: String::from("Anoop KS"),
        pages: 400,
        available: true,
    };

    // --- &self methods: borrow, book stays usable ---
    println!("is long?    : {}", book.is_long());
    println!("description : {}", book.description());

    // --- &mut self method: mutate (book is `mut`) ---
    book.mark_as_unavailable();
    println!("available?  : {}", book.available);

    // --- self method: CONSUMES book. Must be LAST — nothing can use book after. ---
    println!("into_title  : {}", book.into_title()); // book is MOVED here

    // The WHOLE book moved (Book is not Copy), so NOTHING of it survives:
    // println!("{}", book.pages); // ERROR[E0382]: borrow of moved value: `book`
    //
    // NOTE the distinction from a PARTIAL move:
    //   let x = book.author;      // moves ONE field -> book.pages would still work
    //   book.into_title();        // moves the WHOLE book -> nothing of book survives
}

// ============================================================================
// 04-structs / structs.rs
// Layer 4, Check 1 — Structs: named types with typed fields
//
// Run: cargo run -p structs --bin structs
//
// A struct bundles related data into named, typed fields. Shape is fixed and
// checked at compile time (unlike a Python dict you build key-by-key).
// ============================================================================

// #[derive(Debug)] auto-generates debug printing so {:?} and {:#?} work.
// You'll put this on almost every struct.
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

fn main() {
    basics();
    mutability();
    shorthand();
    update_and_partial_move();
}

// ----------------------------------------------------------------------------
// 1. DEFINE (above) vs CREATE (here). Every field must be given a value.
//    Access with dot syntax. Print whole struct with {:?} / {:#?}.
// ----------------------------------------------------------------------------
fn basics() {
    println!("--- basics ---");
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Anoop KS"),
        pages: 100,
        available: true,
    };

    // individual fields with {}
    println!("{} by {}", book.title, book.author);
    println!("pages: {}, available: {}", book.pages, book.available);

    // whole struct (needs #[derive(Debug)])
    println!("{book:?}"); // compact
    println!("{book:#?}"); // pretty, multi-line
    println!();
}

// ----------------------------------------------------------------------------
// 2. MUTABILITY is all-or-nothing: the whole binding is `mut` or nothing is.
//    There is NO per-field `mut`.
// ----------------------------------------------------------------------------
fn mutability() {
    println!("--- mutability ---");
    let mut book = Book {
        title: String::from("Draft"),
        author: String::from("Anoop KS"),
        pages: 50,
        available: true,
    };
    book.pages = 200; // OK: `book` is mut
    book.available = false;
    println!(
        "updated: {} pages, available {}",
        book.pages, book.available
    );
    println!();
}

// ----------------------------------------------------------------------------
// 3. FIELD INIT SHORTHAND: when a variable name matches a field name, skip
//    the `field: field` repetition.
// ----------------------------------------------------------------------------
fn shorthand() {
    println!("--- field shorthand ---");
    let b = make_book(String::from("Go in Action"), String::from("Anoop KS"));
    println!("{b:?}");
    println!();
}

fn make_book(title: String, author: String) -> Book {
    Book {
        title,  // shorthand for title: title
        author, // shorthand for author: author
        pages: 0,
        available: true,
    }
}

// ----------------------------------------------------------------------------
// 4. STRUCT UPDATE `..other` + the PARTIAL MOVE (Layer 3 move rule inside structs).
//    `..book` COPIES Copy fields and MOVES non-Copy fields out of `book`.
//    After it, `book` is PARTIALLY MOVED (not dropped): moved fields are gone,
//    Copy fields survive, non-Copy fields you overrode were never moved.
// ----------------------------------------------------------------------------
fn update_and_partial_move() {
    println!("--- struct update + partial move ---");
    let book = Book {
        title: String::from("Original"),
        author: String::from("Anoop KS"),
        pages: 100,
        available: true,
    };

    // We override `title`, so book.title is NOT moved. `..book` takes author
    // (moved) and pages/available (copied).
    let book2 = Book {
        title: String::from("Second Edition"),
        ..book
    };
    println!("book2: {book2:#?}");

    // --- What survives on the original `book` afterward? ---
    println!("book.pages (Copy -> copied, OK)      : {}", book.pages); // works
    println!("book.available (Copy -> copied, OK)  : {}", book.available); // works
    println!("book.title (overridden, not moved)   : {}", book.title); // works

    // These do NOT compile — `author` was MOVED into book2:
    // println!("{}", book.author); // ERROR[E0382]: borrow of moved value: `book.author`

    // And `book` as a WHOLE is partially moved, so this fails too:
    // println!("{book:?}");        // ERROR[E0382]: borrow of partially moved value: `book`
    println!();
}

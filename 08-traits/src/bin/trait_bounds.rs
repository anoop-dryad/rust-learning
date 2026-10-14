// ============================================================================
// Layer 8, Check 3 — Trait bounds: generics + traits combined
//
// Run: cargo run -p traits --bin trait_bounds
//
// A trait bound restricts a generic T to "types that implement trait X".
// Generics give you T (any type); bounds narrow it to "T that can do X".
// This is the mechanism behind PLUGGABLE designs: a generic fn works on EVERY
// type that implements the trait — including future ones — so you extend the
// system by ADDING types, never by editing the machinery.
// ============================================================================

use std::fmt;

trait Analyzer {
    fn analyze(&self) -> String;
}

struct Book {
    name: String,
    author: String,
}

struct Tweet {
    user: String,
    content: String,
}

// Book implements BOTH Analyzer and Display.
impl Analyzer for Book {
    fn analyze(&self) -> String {
        format!("{} by {}", self.name, self.author)
    }
}
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} by {}", self.name, self.author)
    }
}

// Tweet implements ONLY Analyzer (no Display).
impl Analyzer for Tweet {
    fn analyze(&self) -> String {
        format!("@{}: {}", self.user, self.content)
    }
}

// --- Three equivalent ways to write a bound ---

// 1. inline <T: Analyzer>
fn announce<T: Analyzer>(item: &T) -> String {
    format!("announce: {}", item.analyze()) // can call analyze() BECAUSE T: Analyzer
}

// 2. impl Trait shorthand (no named T) — same thing
fn announce_impl(item: &impl Analyzer) -> String {
    format!("announce: {}", item.analyze())
}

// 3. multiple bounds with `+` — require BOTH traits, and USE both in the body.
fn announce_full<T: Analyzer + fmt::Display>(item: &T) -> String {
    format!("{} | display: {}", item.analyze(), item)
}

fn main() {
    let book = Book {
        name: String::from("Rust"),
        author: String::from("Anoop KS"),
    };
    let tweet = Tweet {
        user: String::from("anoop"),
        content: String::from("hello rust"),
    };

    // One function, MANY types — Book and Tweet both satisfy Analyzer.
    println!("{}", announce(&book));
    println!("{}", announce(&tweet));
    println!("{}", announce_impl(&tweet)); // impl Trait form, same result

    // announce_full needs Analyzer + Display. Book has both:
    println!("{}", announce_full(&book));

    // --- FAILURE CASES (uncomment ONE to see the contract enforced) ---
    //
    // i32 doesn't implement Analyzer:
    // println!("{}", announce(&17));
    //   error[E0277]: the trait bound `{integer}: Analyzer` is not satisfied
    //
    // Tweet has Analyzer but NOT Display, so it fails the `+ Display` bound:
    // println!("{}", announce_full(&tweet));
    //   error[E0277]: `Tweet` doesn't implement `std::fmt::Display`
    //   -> ALL bounds must be satisfied, not just some.
}

// ----------------------------------------------------------------------------
// Returning impl Trait — "some type that implements Analyzer", unnamed:
//   fn make() -> impl Analyzer { Book { ... } }
// Caller can call .analyze() on the result without knowing it's a Book.
// (Caveat: all return paths must be the SAME concrete type.)
//
// WHEN to create a custom trait (recap): when 2+ types genuinely share behavior,
// or you need "any type that can do X", or you're satisfying a std trait. The
// "open to extension" benefit is a REASON traits are powerful, NOT a reason to
// create one early — design it from 2-3 real cases; refactoring to a trait later
// is cheap and safe, so there's no penalty for waiting.
// ----------------------------------------------------------------------------

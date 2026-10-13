// ============================================================================
// Layer 8, Check 2 — Traits: shared behavior (Rust's interfaces)
//
// Run: cargo run -p traits --bin traits
//
// A trait = a set of behavior a type can implement (like an interface).
// Define the CONTRACT once; each type provides its own IMPLEMENTATION.
//
// When to CREATE a custom trait (don't do it speculatively):
//   1. Multiple types need to be treated the SAME way (uniform behavior).
//   2. You want a function generic over "any type that can do X" (trait bounds).
//   3. You're satisfying a STANDARD/ecosystem trait (Display, Iterator, From...).
// Default: write concrete code first; EXTRACT a trait when a pattern repeats.
// A trait with only ONE implementor is usually pointless (just use an impl block).
// ============================================================================

use std::fmt;

// --- Define the trait: one required method + one DEFAULT method ---
trait Describe {
    fn describe(&self) -> String; // required: each type must provide this

    // default method: implementors get it free (can override). Built on describe().
    // Uses .chars().take(n) NOT byte slicing: never panics on short input, and
    // respects UTF-8 char boundaries (a char can be 1-4 bytes in a String).
    fn short(&self) -> String {
        self.describe().chars().take(10).collect()
    }
}

struct Book {
    name: String,
    author: String,
}

#[derive(Debug)] // auto-implements the Debug trait -> {:?}
struct Car {
    model: String,
    manufacturer: String,
}

// --- Implement the trait FOR each type: same method name, type-specific body ---
impl Describe for Book {
    fn describe(&self) -> String {
        format!("{} by {}", self.name, self.author)
    }
}

impl Describe for Car {
    fn describe(&self) -> String {
        format!(
            "model: {} from manufacturer: {}",
            self.model, self.manufacturer
        )
    }
}

// --- Display is NOT derivable (it's a human-judgment call) -> implement by hand.
//     This is what makes `println!("{car}")` work. Same mechanism #[derive(Debug)]
//     generates for Debug, just written out.
impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} by {}", self.model, self.manufacturer)
    }
}

fn main() {
    let book = Book {
        name: String::from("Rust"),
        author: String::from("Anoop"),
    };
    let car = Car {
        model: String::from("Swift"),
        manufacturer: String::from("Maruthi"),
    };

    // trait methods (use {} — these return human-readable Strings)
    println!("{}", book.describe());
    println!("{}", car.describe());
    println!("{}", car.short()); // default method, no impl needed on Car

    // Display ({}) you wrote by hand; Debug ({:?}) was derived — same mechanism.
    println!("{car}"); // Display
    println!("{car:?}"); // Debug (derived)
}

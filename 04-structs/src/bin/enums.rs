// ============================================================================
// Layer 4, Check 2b — Enums, Option, and match
//
// Run: cargo run -p structs --bin enums
//
// An enum value is EXACTLY ONE of its variants. Each variant can carry its own
// data of its own type. This makes illegal states unrepresentable.
// ============================================================================

// Simple enum — variants with no data (like named constants, but a real type).
#[derive(Debug)]
enum Status {
    Available,
    CheckedOut,
    Lost,
}

// Data-carrying enum — each variant holds different data:
#[derive(Debug)]
enum BookLocation {
    OnShelf(u32),                          // tuple-style: a shelf number
    CheckedOutBy(String),                  // tuple-style: a borrower name
    InRepair { since: String, cost: f64 }, // struct-style: named fields
    Lost,                                  // unit: no data
}

fn main() {
    simple_enum();
    data_enum();
    option_basics();
    matching();
}

fn simple_enum() {
    println!("--- simple enum ---");
    let s = Status::CheckedOut; // `::` accesses a variant
    println!("{s:?}");
    // A Status is exactly one of the three — invalid states are impossible.
    let _ = Status::Available;
    let _ = Status::Lost;
    println!();
}

fn data_enum() {
    println!("--- data-carrying enum ---");
    let shelf = BookLocation::OnShelf(16);
    let out = BookLocation::CheckedOutBy(String::from("Anoop"));
    let repair = BookLocation::InRepair {
        since: String::from("Jan"),
        cost: 50.00,
    };
    let lost = BookLocation::Lost;
    println!("{shelf:?}\n{out:?}\n{repair:?}\n{lost:?}");
    println!();
}

// ----------------------------------------------------------------------------
// Option<T> is just a std-library enum:
//     enum Option<T> { Some(T), None }
// Rust has NO null. "Might be absent" is encoded in the TYPE: Option<T> means
// maybe-a-value; plain T means definitely-a-value. The compiler forces you to
// handle None before reaching the value -> null crashes are impossible.
// ----------------------------------------------------------------------------
fn option_basics() {
    println!("--- Option ---");
    let some_n: Option<i32> = Some(5);
    let no_n: Option<i32> = None;
    println!("{some_n:?}  {no_n:?}"); // Some(5)  None

    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    let found: Option<&i32> = numbers.get(7); // out of range -> None (no panic)
    println!("numbers.get(7) = {found:?}"); // None
    println!();
}

// ----------------------------------------------------------------------------
// match — DESTRUCTURES (pulls the inner data out) and is EXHAUSTIVE (you must
// handle every variant, or it won't compile). Exhaustiveness is a feature: add a
// variant later and the compiler points at every match that needs updating.
// ----------------------------------------------------------------------------
fn matching() {
    println!("--- match ---");

    let places = [
        BookLocation::OnShelf(16),
        BookLocation::CheckedOutBy(String::from("Anoop")),
        BookLocation::InRepair {
            since: String::from("Jan"),
            cost: 50.0,
        },
        BookLocation::Lost,
    ];
    for loc in &places {
        println!("{}", describe(loc));
    }

    // Option via match — the safe way to get a value out (forces the None arm):
    let numbers = [10, 20, 30];
    match numbers.get(1) {
        Some(value) => println!("got {value}"),
        None => println!("nothing there"),
    }
    match numbers.get(9) {
        Some(value) => println!("got {value}"),
        None => println!("nothing there"), // this arm runs
    }
    println!();
}

fn describe(loc: &BookLocation) -> String {
    match loc {
        BookLocation::OnShelf(n) => format!("on shelf {n}"), // binds the u32 to n
        BookLocation::CheckedOutBy(who) => format!("checked out by {who}"), // binds the String
        BookLocation::InRepair { since, cost } => {
            format!("in repair since {since}, cost ${cost}") // binds both fields
        }
        BookLocation::Lost => String::from("lost"),
        // Remove any arm above -> error[E0004]: non-exhaustive patterns.
        // A catch-all `_ => String::from("unknown")` would match "everything else".
    }
}

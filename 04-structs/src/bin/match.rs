// ============================================================================
// Layer 4, Check 2b (match) — pattern matching on enums
//
// Run: cargo run -p structs --bin match
//
// match does TWO things an if-chain can't:
//   1. DESTRUCTURES  — pulls the data out of a variant and binds it to a name.
//   2. EXHAUSTIVE     — the compiler forces you to handle every case, or it won't
//                       compile (error[E0004]). This is a feature: add a variant
//                       later and the compiler points at every match to fix.
// ============================================================================

#[derive(Debug)]
enum BookLocation {
    OnShelf(u32),                          // tuple variant: a number
    CheckedOutBy(String),                  // tuple variant: a String
    InRepair { since: String, cost: f64 }, // struct variant: named fields
    Lost,                                  // unit variant: no data
}

fn main() {
    // Construct ONE OF EACH. (Writing a match arm for a variant does NOT count as
    // constructing it — that's why a describe-only program still warns "variant
    // never constructed". Making the values here clears that warning.)
    let places = [
        BookLocation::OnShelf(10),
        BookLocation::CheckedOutBy(String::from("Anoop KS")),
        BookLocation::InRepair {
            since: String::from("Jan"),
            cost: 50.0,
        },
        BookLocation::Lost,
    ];

    for loc in &places {
        // `loc` is already &BookLocation (iterating &places), so pass it directly.
        // Use {} (display) not {:?} — describe returns a clean human-readable string.
        println!("location: {}", describe(loc));
    }
}

// Takes &BookLocation (borrow — describe only reads; it doesn't consume the value).
fn describe(loc: &BookLocation) -> String {
    match loc {
        // Each arm DESTRUCTURES: binds the inner data to a name you can use.
        BookLocation::OnShelf(n) => format!("on shelf {n}"), // n = the u32
        BookLocation::CheckedOutBy(who) => format!("checked out by {who}"), // who = the String
        BookLocation::InRepair { since, cost } => {
            format!("in repair since {since}, expected cost {cost}") // both fields
        }
        BookLocation::Lost => String::from("lost"),
        // --- EXHAUSTIVENESS ---
        // Delete ANY arm above (and no `_`) -> HARD ERROR, not a warning:
        //   error[E0004]: non-exhaustive patterns: `BookLocation::Lost` not covered
        //
        // A catch-all makes it exhaustive again and compiles:
        //   _ => String::from("something else"),
        // TRADEOFF: `_` silences E0004, so if you ADD a new variant later, it
        // silently routes to `_` instead of the compiler making you handle it.
        // On enums YOU own, prefer listing every variant and SKIP `_`, so the
        // compiler keeps nagging you when the enum grows.
    }
}

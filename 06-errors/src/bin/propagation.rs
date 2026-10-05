// ============================================================================
// Layer 6, Check 1 — panic vs Result, the ? operator, Box<dyn Error>
//
// Run: cargo run -p errors --bin propagation -- <file>
//   (or hardcode a path in main)
//
// Rust splits errors into TWO categories:
//   - UNRECOVERABLE -> panic!   (a bug / broken invariant; program stops)
//   - RECOVERABLE   -> Result<T,E> (expected failure the caller handles; a VALUE)
// Recoverable errors are VALUES, not control flow — a fallible fn says so in its
// return type. No invisible propagation (unlike Python exceptions).
// ============================================================================

use std::error::Error;
use std::fs;

fn main() {
    // read_number returns Result<i32, Box<dyn Error>>. Box<dyn Error> is the
    // "any error" catch-all: ? converts EACH different error type into it.
    match read_number("06-errors/src/files/number.txt") {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => eprintln!("error: {e}"), // {} = clean Display message (not {:?})
    }

    // unwrap_expect_demo(); // opt-in: shows the panic smells
}

// ----------------------------------------------------------------------------
// ? on a Result:  Ok(v) -> unwrap to v and continue;  Err(e) -> immediately
// `return Err(e)` from THIS function (converting e via the From trait into the
// function's declared error type). That's why ? only works in a fn returning
// Result (or Option) — it needs somewhere to return the Err to.
//
// Here read_to_string fails with io::Error and parse fails with ParseIntError —
// TWO different types — yet both propagate, because both convert into Box<dyn Error>.
// ----------------------------------------------------------------------------
fn read_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?; // io::Error -> Box<dyn Error>
    let n: i32 = contents.trim().parse()?; // ParseIntError -> Box<dyn Error>
    //        ^^^^ the : i32 annotation tells parse WHAT to parse into
    Ok(n)
}

// ----------------------------------------------------------------------------
// panic! and its cousins — the UNRECOVERABLE path.
// unwrap()  -> gives the inner value, but PANICS on None/Err.
// expect(m) -> same, but panics with YOUR message (strictly better than unwrap).
// Both are a SMELL in production: each is a potential crash. Prefer ? / handling.
// ----------------------------------------------------------------------------
#[allow(dead_code)]
fn unwrap_expect_demo() {
    let ok: Result<i32, _> = "5".parse::<i32>();
    println!("unwrap ok: {}", ok.unwrap()); // 5

    let bad: Result<i32, _> = "nope".parse::<i32>();
    // let n = bad.unwrap();                 // PANICS: "invalid digit found in string"
    let n = bad.expect("expected a valid integer"); // PANICS with YOUR message
    println!("{n}");
}

// ----------------------------------------------------------------------------
// ? only works where there's a Result/Option to return to. This does NOT compile:
//
// fn broken() {
//     let _ = fs::read_to_string("x")?;  // ERROR[E0277]: the `?` operator can only
//                                        // be used in a function that returns Result
// }
// ----------------------------------------------------------------------------

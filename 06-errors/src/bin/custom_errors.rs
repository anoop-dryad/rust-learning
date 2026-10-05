// ============================================================================
// Layer 6, Check 2 — custom error types (enum + From + ?)
//
// Run: cargo run -p errors --bin custom_errors
//
// Box<dyn Error> is great for quick app code, but the caller only gets "some
// error". A CUSTOM error enum lets callers match on WHICH failure occurred and
// react differently. This is the production/library pattern.
// ============================================================================

use std::fs;
use std::io;
use std::num::ParseIntError;

// 1. The error type is just an enum (Layer 4). Each variant = a kind of failure.
#[derive(Debug)]
enum AppError {
    Io(String), // carries a message
    Parse(String),
    Empty, // no data
}

// 2. Implement From so `?` can CONVERT other errors into AppError.
//    `?` calls the From trait — this is what makes `?` target YOUR type.
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err.to_string())
    }
}

// 3. The function body is IDENTICAL to the Box<dyn Error> version — the From
//    impls are what make `?` produce AppError instead.
//    TWO ways a variant gets created:
//      - automatically via ?+From  (Io, Parse)
//      - manually via return Err(..) (Empty) — for YOUR OWN validation failures.
//    (From is for converting OTHER people's errors; return Err is for yours.)
fn read_number(path: &str) -> Result<i32, AppError> {
    let contents = fs::read_to_string(path)?; // io::Error -> AppError (From)
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return Err(AppError::Empty); // manual: our own validation failure
    }
    let n: i32 = trimmed.parse()?; // ParseIntError -> AppError (From)
    Ok(n)
}

fn main() {
    // Point at different files to exercise each variant.
    for path in [
        "06-errors/src/files/number.txt",
        "06-errors/src/files/missing.txt",
        "06-errors/src/files/empty.txt",
        "06-errors/src/files/bad.txt",
    ] {
        match read_number(path) {
            Ok(n) => println!("{path}: got {n}"),
            // Each variant handled DISTINCTLY — the payoff of a custom error type.
            Err(AppError::Io(s)) => eprintln!("{path}: file error - {s}"),
            Err(AppError::Parse(s)) => eprintln!("{path}: invalid number - {s}"),
            Err(AppError::Empty) => eprintln!("{path}: file was empty"),
        }
    }
}

// ----------------------------------------------------------------------------
// REAL-WORLD SHORTCUT: the `thiserror` crate generates From + Display from
// annotations, so you don't hand-write the boilerplate above:
//
//   #[derive(Debug, thiserror::Error)]
//   enum AppError {
//       #[error("file error: {0}")]
//       Io(#[from] std::io::Error),         // #[from] GENERATES the From impl
//       #[error("invalid number: {0}")]
//       Parse(#[from] std::num::ParseIntError),
//       #[error("file was empty")]
//       Empty,
//   }
//
// thiserror -> for LIBRARY error types (precise). anyhow / Box<dyn Error> -> for
// APP code (quick, less precise). Both are external crates added to Cargo.toml.
// We hand-wrote From here first so you know what thiserror generates.
// ----------------------------------------------------------------------------

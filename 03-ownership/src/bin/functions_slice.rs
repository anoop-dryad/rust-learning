// ============================================================================
// Layer 3, Check 3 — Ownership through functions, the &str idiom, slices
//
// Run: cargo run -p ownership --bin functions_slices
//
// Parameter decision rule (the practical payoff of Layer 3):
//   String       -> function TAKES ownership (consumes/keeps it). Rarely the default.
//   &str         -> function READS string data. Idiomatic default (accepts more input).
//   &mut String  -> function MUTATES the caller's string in place.
//   &[T]         -> function READS a slice of an array/Vec (like &str, but for lists).
// ============================================================================

fn main() {
    ownership_out();
    str_idiom();
    slices();
    expression_vs_statement();
}

// ----------------------------------------------------------------------------
// 1. OWNERSHIP FLOWS OUT via return values.
//    A value created in a function normally drops at the function's end (rule 3)
//    -- but RETURNING it moves ownership OUT to the caller, so it escapes the drop.
// ----------------------------------------------------------------------------
fn ownership_out() {
    println!("--- ownership out via return ---");
    let s = gives_ownership(); // s now OWNS the returned String
    println!("got: {s}");

    let s2 = takes_and_gives_back(s); // ownership moves in, then back out to s2
    println!("round-tripped: {s2}");
    println!();
}

fn gives_ownership() -> String {
    String::from("Anoop") // created here; ownership MOVES out (not dropped)
}

fn takes_and_gives_back(text: String) -> String {
    text // hand ownership back to the caller
}

// ----------------------------------------------------------------------------
// 2. THE &str IDIOM — prefer &str over &String for READ-only string params.
//    A &str parameter accepts BOTH a borrowed String (via deref coercion) AND a
//    string literal. A &String would accept only the former. More general, zero cost.
//    (Clippy lints &String params, suggesting &str.)
// ----------------------------------------------------------------------------
fn str_idiom() {
    println!("--- &str idiom ---");
    let owned = String::from("Anoop");
    let literal = "Kuttikattu"; // this is already a &str

    println!("len of owned   : {}", calculate_length(&owned)); // &String -> &str
    println!("len of literal : {}", calculate_length(literal)); // &str directly
    println!("owned still valid: {owned}"); // borrowed, not moved
    println!();
}

fn calculate_length(text: &str) -> usize {
    text.len() // tail expression: no `return`, no `;` -> this IS the value
}

// ----------------------------------------------------------------------------
// 3. SLICES — a borrowed VIEW into a contiguous run of data. Stores PTR + LEN.
//    Owns nothing (a borrow), so all borrow rules apply.
//      &str    = a slice of string bytes
//      &[T]    = a slice of an array or Vec
//    Prefer &[T] over &Vec<T> as a param, same reasoning as &str over &String.
// ----------------------------------------------------------------------------
fn slices() {
    println!("--- slices ---");

    let s = String::from("hello world");
    let hello = &s[0..5]; // &str slice -> "hello"  (byte range)
    let world = &s[6..11]; // "world"
    println!("string slices: {hello} / {world}");

    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let middle = &nums[1..4]; // &[i32] slice -> [2, 3, 4]
    println!("array slice  : {middle:?}");
    println!("sum of slice : {}", sum(&nums)); // whole array as a slice
    println!("sum of middle: {}", sum(middle)); // sub-range, same function
    println!();

    // CAVEAT: range indexing panics if out of bounds, and string slicing panics
    // if it splits a multi-byte UTF-8 char (a char can be 1-4 bytes inside a String).
}

fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for v in values {
        total += v;
    }
    total // tail expression
}

// ----------------------------------------------------------------------------
// 4. EXPRESSION vs STATEMENT (why we skip the `;` on a return).
//    - No semicolon = it's an EXPRESSION -> its value is the block's value.
//    - Semicolon    = it's a STATEMENT  -> the value is DISCARDED (block yields ()).
//    `if`, `match`, and `{}` blocks are all expressions in Rust.
//    Use `return` only for EARLY exits; use the bare tail expression for the final value.
// ----------------------------------------------------------------------------
fn expression_vs_statement() {
    println!("--- expression vs statement ---");

    // a block is an expression; `a + 1` has no `;`, so it's the block's value
    let x = {
        let a = 2;
        a + 1 // no semicolon -> block evaluates to 3
    };
    println!("x = {x}"); // 3

    // `if` is an expression too
    let n = 7;
    let label = if n > 0 { "positive" } else { "non-positive" };
    println!("label = {label}");

    // early return demo
    println!("describe(-1) = {}", describe(-1));
    println!("describe(0)  = {}", describe(0));
    println!("describe(9)  = {}", describe(9));
    println!();
}

fn describe(n: i32) -> &'static str {
    if n < 0 {
        return "negative"; // early exit -> `return` + `;`
    }
    if n == 0 {
        return "zero";
    }
    "positive" // final value -> tail expression, no `return`, no `;`
}

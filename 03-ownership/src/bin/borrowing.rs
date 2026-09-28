// ============================================================================
// Layer 3, Check 2 — Borrowing: &T (shared) and &mut T (mutable)
//
// Run: cargo run -p ownership --bin borrowing
//
// BORROW = use a value without taking ownership (no move, no clone).
//
// THE CENTRAL LAW (readers XOR one writer):
//   For one value, at one moment, you may have EITHER
//     - any number of SHARED borrows   &T     (many readers), OR
//     - exactly ONE mutable borrow     &mut T (one writer),
//   never both at once, and never two writers.
//   (Same shape as XOR: "not both true". Forbidden case = 1 XOR 1 = 0.)
//
// WHY: reading while writing -> the writer could change/reallocate the data
//   out from under the reader (use-after-free). Two writers -> data race.
//   Rust proves their absence at COMPILE time. No GC, no locks needed.
// ============================================================================

fn main() {
    shared_borrow();
    mutable_borrow();
    nll_fix();
    // The three error demos below live in commented blocks — uncomment ONE,
    // run, read the error, then re-comment. They will NOT compile as-is.
}

// ----------------------------------------------------------------------------
// 1. SHARED borrow (&T) — lend for READING. Owner keeps ownership; stays valid.
// ----------------------------------------------------------------------------
fn shared_borrow() {
    println!("--- shared borrow (&T) ---");
    let s = String::from("Anoop");
    let len = calculate_length(&s); // lend s by reference
    println!("length of {s}: {len}"); // s STILL VALID — only borrowed
    println!();
}

fn calculate_length(text: &String) -> usize {
    text.len() // idiomatic: bare expression, no `return`, no `;`
} // `text` (a reference) drops here — the String is NOT freed (borrower owns nothing)

// ----------------------------------------------------------------------------
// 2. MUTABLE borrow (&mut T) — lend for WRITING. Three things must line up:
//    (a) the value is `mut`, (b) you pass `&mut s`, (c) the param is `&mut String`.
//
//    LESSON: in-place mutating methods return `()` (unit), NOT the new value.
//    push_str changes `text` in place; it does not hand back a string.
//      let x = text.push_str("!");  // x is (), not the string!
// ----------------------------------------------------------------------------
fn mutable_borrow() {
    println!("--- mutable borrow (&mut T) ---");
    let mut s = String::from("Anoop"); // must be `mut` to lend mutably
    update(&mut s); // lend a MUTABLE reference
    println!("after update: {s}"); // "Anoop!" — original changed
    println!();
}

fn update(text: &mut String) {
    text.push_str("!"); // mutates in place; returns ()
    println!("inside update: {text:?}"); // "Anoop!"
}

// ----------------------------------------------------------------------------
// 3. NON-LEXICAL LIFETIMES (NLL) — a borrow ends at its LAST USE, not the `}`.
//    So "reader, finish reading, THEN writer" is fine — they don't overlap.
// ----------------------------------------------------------------------------
fn nll_fix() {
    println!("--- NLL: sequence a reader then a writer ---");
    let mut s = String::from("Anoop");

    let r = &s; // shared borrow
    println!("read first: {r}"); // r's LAST USE — its borrow ENDS here

    let w = &mut s; // now OK: no shared borrow is alive
    w.push_str(" KS");
    println!("then wrote: {w}"); // "Anoop KS"
    println!();
}

// ============================================================================
// ERROR DEMOS — uncomment ONE block at a time, run, read the error, re-comment.
// ============================================================================

// --- E0596: cannot mutate through a SHARED borrow --------------------------
// fn e0596() {
//     let mut s = String::from("Anoop");
//     bad_update(&s);            // passing a shared &, but the fn tries to mutate
// }
// fn bad_update(text: &String) {
//     text.push_str("!");        // ERROR[E0596]: cannot borrow `*text` as mutable
// }

// --- E0499: two MUTABLE borrows at once (two writers) ----------------------
// fn e0499() {
//     let mut s = String::from("Anoop");
//     let s1 = &mut s;
//     let s2 = &mut s;           // ERROR[E0499]: cannot borrow `s` as mutable
//     println!("{s1}, {s2}");    //   more than once at a time
// }

// --- E0502: SHARED + MUTABLE overlapping (reader while writer) -------------
// fn e0502() {
//     let mut s = String::from("Anoop");
//     let s1 = &s;               // shared borrow...
//     let s2 = &mut s;           // ERROR[E0502]: cannot borrow `s` as mutable
//     println!("{s1}, {s2}");    //   because it is also borrowed as immutable
// }                              // (s1 is used below, so its borrow OVERLAPS s2)

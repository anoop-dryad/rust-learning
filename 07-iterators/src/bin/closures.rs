// ============================================================================
// 07-iterators / closures.rs
// Layer 7, Check 1 — Closures: anonymous functions that capture their environment
//
// Run: cargo run -p iterators --bin closures
//
// A closure is an anonymous function you can store/pass around (like a Python
// lambda) that can CAPTURE variables from the surrounding scope. Closures are
// the fuel for iterators (map/filter/... all take closures).
// ============================================================================

fn main() {
    basics();
    capturing();
    capture_by_borrow();
    capture_by_move();
}

// ----------------------------------------------------------------------------
// 1. SYNTAX: |params| body. Types usually inferred. Multi-line body uses { }.
// ----------------------------------------------------------------------------
fn basics() {
    println!("--- basics ---");
    let add_one = |x| x + 1; // param/return types inferred
    println!("add_one(10) = {}", add_one(10)); // 11

    let add = |a: i32, b: i32| a + b;
    println!("add(5, 10)  = {}", add(5, 10)); // 15

    let describe = |n: i32| {
        let label = if n > 0 { "positive" } else { "non-positive" };
        format!("{n} is {label}") // returns a String
    };
    println!("{}", describe(-9)); // {} not {:?} — it's a clean String
    println!();
}

// ----------------------------------------------------------------------------
// 2. CAPTURING — the superpower a plain fn can't do: use a variable from the
//    surrounding scope that isn't a parameter.
// ----------------------------------------------------------------------------
fn capturing() {
    println!("--- capturing ---");
    let threshold = 100; // not a parameter...
    let is_big = |n: i32| n > threshold; // ...but the closure captures it
    println!("is_big(99)  = {}", is_big(99)); // false
    println!("is_big(150) = {}", is_big(150)); // true
    println!();
}

// ----------------------------------------------------------------------------
// 3. CAPTURE BY BORROW (default) — a closure that only READS captures by shared
//    borrow, so the original is still usable afterward.
// ----------------------------------------------------------------------------
fn capture_by_borrow() {
    println!("--- capture by borrow ---");
    let name = String::from("Anoop");
    let printer = || println!("captured: {name}"); // reads -> shared borrow
    printer();
    println!("name still usable: {name}"); // OK — only borrowed
    println!();
}

// ----------------------------------------------------------------------------
// 4. CAPTURE BY MOVE — `move` forces the closure to take OWNERSHIP of captures.
//    For a non-Copy type (String), the original is then invalidated (the move
//    rule, in a closure). Needed later for threads (closure must own its data).
// ----------------------------------------------------------------------------
fn capture_by_move() {
    println!("--- capture by move ---");
    let name = String::from("Anoop");
    let owner = move || println!("owned: {name}"); // move: closure OWNS name
    owner();

    // println!("{name}"); // ERROR[E0382]: borrow of moved value: `name`
    //   move occurs because `name` is a String (not Copy); it was moved into the
    //   closure, so it can't be used here. (An i32 would be COPIED and still work.)
    println!("(name was moved into the closure)");
    println!();
}

// ============================================================================
// Layer 3, Check 1 — The move rule
//
// Run: cargo run -p ownership --bin move_rule
//
// THE THREE RULES OF OWNERSHIP:
//   1. Each value has an owner.
//   2. There can be only ONE owner at a time.
//   3. When the owner goes out of scope, the value is DROPPED (freed).
//
// Move = give the value away (source invalidated).
// Copy = duplicate a cheap stack value (both stay valid).  [scalars]
// Clone = make an expensive independent heap copy (two owners, two buffers).
// ============================================================================

fn main() {
    move_on_assignment();
    copy_types();
    clone_demo();
    move_into_function();
    clumsy_return_fix();
}

// ----------------------------------------------------------------------------
// 1. MOVE ON ASSIGNMENT — `let b = a` transfers ownership; `a` is invalidated.
//    Heap-owning types (String, Vec) MOVE. Only one owner at a time (rule 2).
//
//    WHY: two owners of one heap buffer -> both try to free it at scope end ->
//    DOUBLE FREE (memory corruption / security bug). The move rule prevents it
//    at compile time, with no garbage collector.
// ----------------------------------------------------------------------------
fn move_on_assignment() {
    println!("--- move on assignment ---");
    let s1 = String::from("hi");
    let s2 = s1; // ownership MOVES from s1 to s2; s1 is now invalid

    // println!("{s1}"); // ERROR[E0382]: borrow of moved value: `s1`
    //                   //   value moved here ------^ ... borrowed here after move
    println!("s2 (new owner) = {s2}");
    println!();
}

// ----------------------------------------------------------------------------
// 2. COPY TYPES don't move — they duplicate.
//    Scalars (i32, f64, bool, char) and tuples of only-Copy types live on the
//    STACK, are cheap to duplicate, and implement Copy. So `let y = x` COPIES:
//    two independent values, both valid, no heap -> nothing to double-free.
// ----------------------------------------------------------------------------
fn copy_types() {
    println!("--- Copy types (no move) ---");
    let x = 5;
    let y = x; // COPY, not move
    println!("x = {x}, y = {y}"); // both valid: 5, 5

    let a = true;
    let b = a;
    println!("a = {a}, b = {b}");
    println!();
}

// ----------------------------------------------------------------------------
// 3. CLONE — when you genuinely want two independent heap copies.
//    .clone() allocates NEW heap memory and copies the bytes. Two owners, two
//    separate buffers -> no double free, both valid. It's EXPLICIT because it
//    costs an allocation + copy; Rust makes you see where you pay it.
// ----------------------------------------------------------------------------
fn clone_demo() {
    println!("--- clone (deep copy) ---");
    let s1 = String::from("hello");
    let s2 = s1.clone(); // new buffer, copied bytes
    println!("s1 = {s1}, s2 = {s2}"); // both valid, separate buffers
    println!();
}

// ----------------------------------------------------------------------------
// 4. MOVE INTO A FUNCTION — passing an owned value by value MOVES it.
//    After the call the caller's variable is invalidated; the function's
//    parameter becomes the owner and drops the value when it ends.
// ----------------------------------------------------------------------------
fn move_into_function() {
    println!("--- move into function ---");
    let s = String::from("owned");
    takes_ownership(s); // s is MOVED into the function

    // println!("{s}"); // ERROR[E0382]: borrow of moved value: `s`
    //                  // s no longer owns anything after the call.
    println!("(s was moved into the function and is gone here)");
    println!();
}

fn takes_ownership(text: String) {
    println!("function received: {text}");
} // text goes out of scope -> the String is DROPPED (freed) here

// ----------------------------------------------------------------------------
// 5. THE CLUMSY FIX (motivates borrowing in Check 2).
//    To keep using the value, the function can RETURN it back, and you rebind.
//    This works but is awkward — you're shuttling ownership out and back just
//    to let a function look at the data. Check 2 (borrowing) fixes this cleanly.
// ----------------------------------------------------------------------------
fn clumsy_return_fix() {
    println!("--- clumsy return-it-back fix ---");
    let s = String::from("round trip");
    let s = takes_and_returns(s); // move in... and rebind what comes back out
    println!("still usable via rebind: {s}");
    println!();
}

fn takes_and_returns(text: String) -> String {
    println!("borrowed-by-moving: {text}");
    text // give ownership back to the caller
}

// ============================================================================
// Layer 2, Check 2 — Stack vs Heap, Owner vs Borrow
//
// Run: cargo run -p types --bin stack_heap
//
// Core principle:
//   known size at compile time  -> STACK (fast, automatic)
//   unknown / growable size     -> HEAP  (with a fixed-size handle on the stack)
// ============================================================================

fn main() {
    borrow_vs_owner();
    conversions();
    fields_and_sizes();
    len_vs_capacity();
    growth_steps();
}

// ----------------------------------------------------------------------------
// 1. &str (BORROW) vs String (OWNER)
//
//   &str   = a REFERENCE that borrows text baked into the binary (read-only).
//            Two fields on the stack: ptr + len. Owns nothing, frees nothing.
//   String = an OWNER of heap-allocated bytes.
//            Three fields on the stack: ptr + len + capacity. Frees at scope end.
//
// Both contain a pointer INTERNALLY, but their meaning is opposite:
//   the &str borrows; the String owns and cleans up.
// ----------------------------------------------------------------------------
fn borrow_vs_owner() {
    println!("--- borrow vs owner ---");

    let borrowed: &str = "Anoop"; // points at text in the binary
    let owned: String = String::from("Anoop"); // owns a heap copy

    println!("borrowed (&str)  = {borrowed}");
    println!("owned    (String)= {owned}");

    // A &str literal CANNOT grow — it borrows read-only data and has no capacity.
    // borrowed.push_str("ks"); // ERROR: no method `push_str` on &str

    // Only the OWNER can grow its buffer:
    let mut owned = owned; // shadow to make it mutable
    owned.push_str(" K.S. Nair");
    println!("owned grew to    = {owned}");
    println!();
}

// ----------------------------------------------------------------------------
// 2. CONVERSIONS: &str -> String must be EXPLICIT (it allocates + copies).
//
//   let x: String = "Anoop"; // ERROR[E0308]: expected `String`, found `&str`
//
// Rust won't silently convert borrow -> owned because it costs an allocation.
// You must ask for it:
// ----------------------------------------------------------------------------
fn conversions() {
    println!("--- &str -> String conversions ---");
    let a: String = String::from("Anoop"); // most explicit
    let b: String = "Anoop".to_string(); // idiomatic, very common
    let c: String = "Anoop".to_owned(); // "own a copy of this borrow"
    let d: String = "Anoop".into(); // infers target from the annotation
    println!("{a} {b} {c} {d}");
    println!();
}

// ----------------------------------------------------------------------------
// 3. THE FIELDS ARE VISIBLE IN THE SIZES.
//   &str   = ptr + len            -> 2 * 8 = 16 bytes (on 64-bit)
//   String = ptr + len + capacity -> 3 * 8 = 24 bytes (on 64-bit)
//   &i32   = just a pointer       -> 8 bytes
// This is the "&str has 2 fields, String has 3 fields" claim, proven.
// ----------------------------------------------------------------------------
fn fields_and_sizes() {
    println!("--- sizes of the handles (bytes) ---");
    println!("&str   = {}", std::mem::size_of::<&str>()); // 16  (ptr+len)
    println!("String = {}", std::mem::size_of::<String>()); // 24  (ptr+len+cap)
    println!("&i32   = {}", std::mem::size_of::<&i32>()); // 8   (just a pointer)
    println!();
}

// ----------------------------------------------------------------------------
// 4. len vs capacity — different things, capacity >= len ALWAYS.
//   len      = bytes actually used right now
//   capacity = bytes allocated on the heap (room before it must reallocate)
//
// One BIG push allocates exactly what's needed -> capacity ends up == len.
// (Doubling only shows with many SMALL pushes — see function 5.)
// ----------------------------------------------------------------------------
fn len_vs_capacity() {
    println!("--- len vs capacity ---");
    let mut name = String::from("Anoop");
    println!("first: len {} cap {}", name.len(), name.capacity()); // 5 / 5

    name.push_str("Kuttikattu Sukumaran Nair"); // one big push
    println!("full:  len {} cap {}", name.len(), name.capacity()); // cap == len here
    println!();
}

// ----------------------------------------------------------------------------
// 5. GROWTH STEPS — watch capacity jump AHEAD of len when you push small.
// This is the over-allocation optimization: grabbing spare room now so the
// NEXT few pushes don't each trigger a reallocation.
//
// The exact step values are a std-library implementation detail — this prints
// the REAL ones your toolchain produces, so you read data, not my guess.
// ----------------------------------------------------------------------------
fn growth_steps() {
    println!("--- capacity growth steps (push one char at a time) ---");
    let mut s = String::new(); // empty: capacity 0
    let mut last_cap = s.capacity();
    println!("start: len {} cap {}", s.len(), s.capacity());

    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        s.push(c);
        if s.capacity() != last_cap {
            println!("after {:>2} chars -> cap {}", s.len(), s.capacity());
            last_cap = s.capacity();
        }
    }
    println!();
}

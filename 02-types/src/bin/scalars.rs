// ============================================================================
// Layer 2, Check 1 — Scalar types: integers, floats, bool, char
//
// Run (debug):   cargo run -p types --bin scalars
// Run (release): cargo run -p types --bin scalars --release
//
// The file runs fully by default. The overflow PANIC demo lives in its own
// function that main() does NOT call — uncomment the call to see it.
// ============================================================================

fn main() {
    integer_types();
    sizes();
    overflow_safe();
    floats_bool_char();
    division();

    // --- OPT-IN: uncomment ONE line at a time and observe ---
    //overflow_panic(); // debug: panics. release (--release): silently wraps to 0.
}

// ----------------------------------------------------------------------------
// 1. INTEGER TYPES
// Naming: i = signed, u = unsigned. The NUMBER is BITS. 1 byte = 8 bits.
//   i8 i16 i32 i64 i128     u8 u16 u32 u64 u128     isize/usize (pointer-sized)
// Default integer type = i32.
// ----------------------------------------------------------------------------
fn integer_types() {
    println!("--- integer types ---");

    let a = 5; // no annotation -> inferred i32 (the default)
    let b: u8 = 255; // unsigned 8-bit -> range 0..=255
    let c: i64 = 9_000_000_000; // underscores are just visual separators (ignored)

    // usize is what you index/measure lengths with; matches machine pointer size.
    let len: usize = 3;

    println!("a (i32) = {a}");
    println!("b (u8)  = {b}   // max value for u8");
    println!("c (i64) = {c}   // too big for i32, needs i64");
    println!("len (usize) = {len}");
    println!();
}

// ----------------------------------------------------------------------------
// 2. SIZES — prove them yourself instead of guessing.
// std::mem::size_of::<T>() returns the size IN BYTES.
// (usize/isize are platform-dependent: 8 bytes on 64-bit machines like an M-series Mac.)
// ----------------------------------------------------------------------------
fn sizes() {
    println!("--- sizes (bytes) ---");
    println!("u8    = {}", std::mem::size_of::<u8>()); // 1  (8 bits)
    println!("u16   = {}", std::mem::size_of::<u16>()); // 2  (16 bits)
    println!("i32   = {}", std::mem::size_of::<i32>()); // 4  (32 bits)
    println!("i64   = {}", std::mem::size_of::<i64>()); // 8  (64 bits)
    println!("bool  = {}", std::mem::size_of::<bool>()); // 1  (only 2 states, but CPUs address bytes)
    println!("char  = {}", std::mem::size_of::<char>()); // 4  (any of ~1.1M Unicode scalars)
    println!("f64   = {}", std::mem::size_of::<f64>()); // 8
    println!("usize = {}", std::mem::size_of::<usize>()); // 8 on 64-bit
    println!();
}

// ----------------------------------------------------------------------------
// 3. OVERFLOW — the SAFE, idiomatic ways.
// Bare `+` says "I'm certain this can't overflow." When it might, name your intent:
//   checked_*    -> Option (None if it would overflow)   [safe]
//   wrapping_*   -> deliberately wraps around             [modular arithmetic]
//   saturating_* -> clamps at the type's min/max          [no wrap, no panic]
//   overflowing_* -> deliberately wraps around and provide a boolean to denote overflow  [modular arithmetic, bool]
// (Option is covered properly in the error-handling layer; for now None = "would overflow".)
// ----------------------------------------------------------------------------
fn overflow_safe() {
    println!("--- overflow: the safe methods ---");
    let x: u8 = 255;
    println!("checked_add(1)    = {:?}", x.checked_add(1)); // None
    println!("wrapping_add(1)   = {}", x.wrapping_add(1)); // 0
    println!("saturating_add(1) = {}", x.saturating_add(1)); // 255
    println!("overflowing_add(1) = {:?}", x.overflowing_add(1)); // (0, true)
    println!();
}

// ----------------------------------------------------------------------------
// 4. OVERFLOW PANIC DEMO (opt-in — main() does not call this).
//
// Note: `let x: u8 = 255; x + 1` with a constant-known value may be REJECTED
// AT COMPILE TIME by the `arithmetic_overflow` lint (Rust catches provable
// overflow before running). To see genuine RUNTIME behavior, we hide the value
// from the compiler with std::hint::black_box (stable since Rust 1.66) so it
// can't const-fold it away.
//
//   debug build   -> panics: "attempt to add with overflow"  (a safety check)
//   release build -> the check is compiled out; value silently wraps to 0
//
// LESSON: never rely on overflow to panic in production. Handle it explicitly (see #3).
// ----------------------------------------------------------------------------
#[allow(dead_code)] // tells the compiler "yes, I know this is unused for now"
fn overflow_panic() {
    let x: u8 = std::hint::black_box(255); // black_box: "pretend you don't know this is 255"
    let y = x + 1; // debug: PANIC here.  release: y == 0
    println!("y = {y}");
}

// ----------------------------------------------------------------------------
// 5. FLOATS, BOOL, CHAR
// Python contrasts:
//   - char is SINGLE quotes and its own type; strings are DOUBLE quotes.
//     'z' (char) is NOT "z" (string). Python has no separate char type.
//   - bool is strictly its own type: `true + 1` does NOT compile
//     (unlike Python where True + 1 == 2).
//   - char is a 4-byte Unicode scalar, so it holds any symbol, not just ASCII.
// ----------------------------------------------------------------------------
fn floats_bool_char() {
    println!("--- floats, bool, char ---");

    let pi = 3.14; // inferred f64 (the default float); f32 also exists
    let flag: bool = true;

    let ascii: char = 'a';
    let accented: char = 'ñ';
    let japanese: char = 'あ';
    let emoji: char = '😻';

    println!("pi (f64)   = {pi}");
    println!("flag (bool)= {flag}");
    println!("chars      = {ascii} {accented} {japanese} {emoji}  // all char, all 4 bytes");

    // This would NOT compile — bool is not a number:
    // let n = flag + 1; // error[E0369]: cannot add {integer} to bool
    println!();
}

// ----------------------------------------------------------------------------
// 6. DIVISION — the classic Python gotcha.
// Integer / integer  -> TRUNCATES toward zero, stays an integer.
// Float   / float     -> real division.
// (Python 3's `/` always gives a float; Rust keeps integer division integer.)
// ----------------------------------------------------------------------------
fn division() {
    println!("--- division ---");
    let int_div = 7 / 2; // 3   (truncated, NOT 3.5)
    let float_div = 7.0 / 2.0; // 3.5
    let also_int = 5 / 3; // 1

    //let wrong = 5.0 / 3; // error, different types division not possible

    println!("7 / 2     = {int_div}   // integer division truncates");
    println!("7.0 / 2.0 = {float_div}");
    println!("5 / 3     = {also_int}");
    println!();
}

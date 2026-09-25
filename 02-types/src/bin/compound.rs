// ============================================================================
// Layer 2, Check 3 — Compound types: tuple & array (+ the Vec rhyme)
//
// Run: cargo run -p types --bin compound
//
// Both tuple and array are FIXED-SIZE -> they live on the STACK.
// That's the contrast with the growable heap types (String, Vec).
// ============================================================================

fn main() {
    tuples();
    arrays();
    array_vs_vec();

    // --- OPT-IN: uncomment to witness the RUNTIME out-of-bounds panic ---
    // out_of_bounds_runtime();
}

// ----------------------------------------------------------------------------
// 1. TUPLE — a fixed group of MIXED types.
//   - Length is fixed; a 3-tuple is a DIFFERENT type from a 2-tuple.
//   - Each position's type is baked in: (String, i32, bool).
//   Access by .index (zero-based) or by destructuring.
// ----------------------------------------------------------------------------
fn tuples() {
    println!("--- tuple ---");
    let person: (String, i32, bool) = (String::from("Anoop"), 38, true);

    // access by position
    println!("name (.0)  : {}", person.0);
    println!("age  (.1)  : {}", person.1);

    // destructure. NOTE the `&`: we borrow the tuple so we don't MOVE the
    // String out of it (ownership — Layer 3). Without `&`, `person.0` would
    // be moved into `name` and `person` partially invalidated.
    let (name, age, active) = &person;
    println!("destructured: {name}, {age}, {active}");
    println!();
}

// ----------------------------------------------------------------------------
// 2. ARRAY — a fixed number of the SAME type. Type is [T; N].
//   - The LENGTH is part of the type: [i32; 5] != [i32; 4].
//   - Fixed size known at compile time -> stack.
//   - Safe access with .get(i) -> Option<&T>  (Some(&v) or None). Never panics.
// ----------------------------------------------------------------------------
fn arrays() {
    println!("--- array ---");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3]; // shorthand: [0, 0, 0]

    println!("arr           : {:?}", arr); // {:?} = debug format (whole structure)
    println!("zeros         : {:?}", zeros);
    println!("len           : {}", arr.len()); // 5
    println!("arr[2]        : {}", arr[2]); // 3  (valid index, direct)

    // A CONSTANT bad index does NOT compile — the compiler proves it will panic:
    //   let bad = arr[10];
    //   error: this operation will panic at runtime
    //   note: `#[deny(unconditional_panic)]` on by default
    // (Same idea as the arithmetic_overflow lint: provable failure -> hard error.)

    // Safe access instead — returns Option, never panics:
    println!("arr.get(10)   : {:?}", arr.get(10)); // None
    println!("arr.get(2)    : {:?}", arr.get(2)); // Some(3)  (a Some wrapping a &ref)
    println!();
}

// ----------------------------------------------------------------------------
// 3. THE RHYME: [T; N] is to Vec<T> as str is to String.
//   fixed / stack           growable / heap
//   ---------------------   -----------------------
//   [i32; 5]   (array)  ->  Vec<i32>  (vector)
//   str        (slice)  ->  String
//   Both pairs: "fixed size, stack" -> "growable, heap, owned".
// ----------------------------------------------------------------------------
fn array_vs_vec() {
    println!("--- array vs Vec (fixed vs growable) ---");

    let fixed: [i32; 3] = [1, 2, 3]; // cannot grow — size is in the type
    println!("fixed array   : {:?} (len {})", fixed, fixed.len());

    let mut v: Vec<i32> = vec![1, 2, 3]; // `vec!` macro builds a Vec
    v.push(4); // grows — heap reallocation, like String::push_str
    v.push(5);
    println!(
        "grown vec      : {:?} (len {}, cap {})",
        v,
        v.len(),
        v.capacity()
    );

    // arrays convert into Vec (target type known from annotation):
    let from_arr: Vec<i32> = [10, 20, 30].into();
    println!("array -> Vec   : {:?}", from_arr);
    println!();
}

// ----------------------------------------------------------------------------
// 4. RUNTIME out-of-bounds panic (opt-in; main() does not call this).
// To reach the RUNTIME check we must hide the index from the compiler so the
// unconditional_panic lint can't prove it in advance (same trick as overflow).
//   -> thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10'
// ----------------------------------------------------------------------------
#[allow(dead_code)]
fn out_of_bounds_runtime() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let i = std::hint::black_box(10); // "pretend you don't know this is 10"
    let val = arr[i]; // PANICS here at runtime
    println!("{val}");
}

// ============================================================================
// Layer 5, Check 1 — Vec<T>: a growable list (heap)
//
// Run: cargo run -p collections --bin vectors
//
// Vec<T> is the growable version of [T; N] (fixed array). Like a Python list,
// but HOMOGENEOUS — one element type only.
// ============================================================================

fn main() {
    basics();
    access();
    iteration();
    // mutate_while_iterating();  // uncomment to see the E0502 compile error
}

// ----------------------------------------------------------------------------
// 1. CREATE + grow. push/pop are Python's append/pop. pop returns Option
//    (None if empty -> no panic on an empty vec).
// ----------------------------------------------------------------------------
fn basics() {
    println!("--- basics ---");
    let mut nums: Vec<i32> = Vec::new(); // empty, annotated
    let primes = vec![2, 3, 5, 7]; // vec! macro, inferred Vec<i32>

    nums.push(10);
    nums.push(20);
    nums.push(30);
    let last = nums.pop(); // -> Option<i32>: Some(30)

    println!("nums   : {nums:?}"); // [10, 20]
    println!("popped : {last:?}"); // Some(30)
    println!("primes : {primes:?}");
    println!();
}

// ----------------------------------------------------------------------------
// 2. ACCESS — same lesson as arrays (Layer 2):
//    v[i]    -> direct, PANICS if out of bounds
//    v.get(i)-> Option<&T>, safe (None if out of bounds)
// ----------------------------------------------------------------------------
fn access() {
    println!("--- access ---");
    let v = vec![10, 20, 30];
    println!("v[0]      : {}", v[0]); // 10
    // let _ = v[99];                 // would PANIC: index out of bounds
    println!("v.get(1)  : {:?}", v.get(1)); // Some(20)
    println!("v.get(99) : {:?}", v.get(99)); // None
    println!();
}

// ----------------------------------------------------------------------------
// 3. ITERATE — the choice is OWNERSHIP (Layer 3) again:
//    &v      shared borrow  -> read; v still usable after   (90% case)
//    &mut v  mutable borrow -> modify in place (needs *x to write through the ref)
//    v       MOVE           -> consumes v; v is GONE after the loop
// ----------------------------------------------------------------------------
fn iteration() {
    println!("--- iteration ---");
    let mut nums = vec![1, 2, 3];

    for n in &nums {
        print!("{n} "); // read via shared borrow
    }
    println!("(read via &nums; nums still usable)");

    for n in &mut nums {
        *n += 100; // mutate in place; *n writes THROUGH the &mut reference
    }
    println!("after &mut : {nums:?}"); // [101, 102, 103]

    for n in nums {
        print!("{n} "); // MOVES nums into the loop
    }
    println!("(nums was MOVED here — unusable after this loop)");
    // println!("{nums:?}"); // would ERROR[E0382]: use of moved value
    println!();
}

// ----------------------------------------------------------------------------
// 4. COMMON MISTAKE — mutate a Vec while iterating a &borrow of it.
//    The for-loop holds a SHARED borrow of v for its whole body; v.push needs a
//    MUTABLE borrow -> can't have both. This is "readers XOR one writer" (Layer 3).
// ----------------------------------------------------------------------------
// #[allow(dead_code)]
// fn mutate_while_iterating() {
//     let mut v = vec![1, 2, 3];
//     for x in &v {
//         v.push(*x); // ERROR[E0502]: cannot borrow `v` as mutable because it is
//         //               also borrowed as immutable
//     }
// }

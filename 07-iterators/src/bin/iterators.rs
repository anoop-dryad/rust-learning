// ============================================================================
// Layer 7, Check 2 — Iterators: lazy, chainable, zero-cost sequence processing
//
// Run: cargo run -p iterators --bin iterators
//
// ADAPTORS (map, filter, take, enumerate, rev) are LAZY — they describe a
// transformation, do nothing on their own.
// CONSUMERS (collect, sum, count, max, find, for_each) DRIVE the iterator and
// produce a result. Nothing runs until a consumer pulls values through.
// Bonus: iterator chains are ZERO-COST — compiled down to ~the same code as a
// hand-written loop. Elegance AND speed.
// ============================================================================

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    // --- filter + map + collect: a declarative pipeline ---
    let transformed: Vec<i32> = nums
        .iter() // yields &i32 (borrow)
        .filter(|&n| n % 2 == 0) // keep evens      (closure)
        .map(|&v| v * 10) // multiply by 10  (closure)
        .collect(); // CONSUME into a Vec
    println!("transformed: {transformed:?}"); // [20, 40, 60]

    // --- consumers: sum / count / max ---
    let sum: i32 = nums.iter().sum();
    println!("sum: {sum}"); // 21

    let div3: usize = nums.iter().filter(|&a| a % 3 == 0).count();
    println!("divisible by 3: {div3}"); // 2 (3, 6)

    match nums.iter().max() {
        Some(v) => println!("max: {v}"), // 6
        None => println!("empty"),
    }

    // --- LAZINESS: an adaptor with NO consumer does nothing ---
    let lazy = nums.iter().map(|n| n * 2); // nothing happens here — no consumer
    // (clippy would warn: iterators are lazy and do nothing unless consumed)
    let forced: Vec<i32> = lazy.collect(); // NOW it runs
    println!("forced: {forced:?}"); // [2, 4, 6, 8, 10, 12]

    // --- enumerate: pairs each item with its index ---
    for (i, val) in nums.iter().enumerate() {
        println!("  [{i}] = {val}");
    }

    // --- for-loop vs iterator (same result; iterator is idiomatic) ---
    let mut loop_count = 0;
    for n in &nums {
        if n % 2 == 0 {
            loop_count += 1;
        }
    }
    let iter_count = nums.iter().filter(|&x| x % 2 == 0).count();
    println!("for: {loop_count}  iter: {iter_count}"); // both 3
}

// ----------------------------------------------------------------------------
// iter() vs iter_mut() vs into_iter() — the SAME borrow/move choice as a for loop:
//   v.iter()      -> &T      shared borrow   (v still usable)      [most common]
//   v.iter_mut()  -> &mut T  mutable borrow  (v must be mut)
//   v.into_iter() -> T       MOVE / consumes (v gone afterward)
// The `into_` prefix = "consumes/takes ownership" (same convention as into_title).
// `for x in &v` desugars to v.iter().
// ----------------------------------------------------------------------------

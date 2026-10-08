// ============================================================================
// Layer 7, Check 3 — closures + iterators, the real idiomatic patterns
//
// Run: cargo run -p iterators --bin combining
// ============================================================================

use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // --- collect into a HashMap: each mapped item is a (key, value) tuple ---
    let squares: HashMap<i32, i32> = nums
        .iter()
        .filter(|&n| n % 2 == 0) // evens only
        .map(|&n| (n, n * n)) // (key, value) pairs
        .collect();
    println!("even squares: {squares:?}"); // {2:4, 4:16, 6:36, 8:64}

    // --- find: returns Option (the element might not exist) ---
    let first_match = nums.iter().find(|&&n| n / 3 == 2); // first n in 6..=8 -> 6
    match first_match {
        Some(n) => println!("found: {n}"),
        None => println!("not found"),
    }

    // --- any / all: return bool ---
    let any_gt_6 = nums.iter().any(|&n| n > 6);
    let all_positive = nums.iter().all(|&n| n > 0);
    println!("any>6: {any_gt_6}, all positive: {all_positive}");

    // --- top N: sort_by (descending) then take(n). This IS textstat's top-words. ---
    let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    values.sort_by(|a, b| b.cmp(a)); // b before a = descending; clean form (no extra &)
    let top3: Vec<_> = values.iter().take(3).collect();
    println!("top 3: {top3:?}"); // [9, 8, 7]

    // --- collect into Result<Vec<_>>: SHORT-CIRCUITS on the first error ---
    let strs = vec!["1", "2", "x", "4"];
    let parsed: Result<Vec<i32>, _> = strs.iter().map(|s| s.parse::<i32>()).collect();
    println!("parsed: {parsed:?}"); // Err(..) — the "x" fails the WHOLE collect
    // (Collecting Vec<Result> into Result<Vec> stops at the first Err and returns it.
    //  Ties iterators to Layer 6 error handling.)
}

// ============================================================================
// The `&` in closures — a quick rule (you converge with the compiler, but FYI):
//
//   The closure receives whatever the iterator yields. nums.iter() yields &i32.
//   - CONSUMING adaptors (map) pass the item DIRECTLY   -> closure gets &i32.
//       |&n| peels one layer -> n is i32.
//   - PEEKING adaptors (filter, find) pass a REFERENCE to the item -> &&i32.
//       |&&n| peels both -> n is i32.  |&n| peels one -> n is &i32 (auto-deref
//       makes arithmetic still work). Both compile.
//
//   `&` in a closure PATTERN (|&n|) = DESTRUCTURING (peeling reference layers).
//   `&` in cmp(&a.1)                = CREATING a reference to pass as an argument.
//   sort_by's closure gets &T, &T already, so `b.cmp(a)` needs no extra &.
//     (But for tuples: b.1 is a VALUE, so cmp needs &b.1 to get a reference.)
//
//   Practical rule: write it; if the compiler says "expected &i32, found i32"
//   (or vice versa), its help line tells you to add/remove a `&`. Converge in one.
//
// iter() -> &T (borrow) | iter_mut() -> &mut T (mut borrow) | into_iter() -> T (move)
// ============================================================================

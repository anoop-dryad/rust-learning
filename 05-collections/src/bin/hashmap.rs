// ============================================================================
// Layer 5, Check 2 — HashMap<K, V>: key-value collection (Rust's dict)
//
// Run: cargo run -p collections --bin hashmaps
// ============================================================================

use std::collections::HashMap; // NOT in the prelude — must import

fn main() {
    basics();
    word_frequency();
}

// ----------------------------------------------------------------------------
// 1. BASICS. insert adds/overwrites; get returns Option<&V> (missing key = None,
//    not a crash). Iteration order is NOT guaranteed.
// ----------------------------------------------------------------------------
fn basics() {
    println!("--- basics ---");
    let mut ages: HashMap<String, i32> = HashMap::new();
    ages.insert(String::from("Anoop"), 38);
    ages.insert(String::from("Manasa"), 35);
    ages.insert(String::from("Anoop"), 39); // same key -> OVERWRITES (now 39)

    match ages.get("Anoop") {
        Some(age) => println!("Anoop: {age}"), // 39
        None => println!("no Anoop"),
    }
    match ages.get("Akhil") {
        Some(age) => println!("Akhil: {age}"),
        None => println!("Akhil not present"), // this runs
    }

    for (name, age) in &ages {
        println!("{name}: {age}"); // (name, age) destructures each pair; order arbitrary
    }
    println!();
}

// ----------------------------------------------------------------------------
// 2. WORD FREQUENCY — the canonical counting idiom (this IS textstat Checkpoint 3).
//
//    entry(k).or_insert(0) returns a &mut V to the value for key k, inserting 0
//    first if the key was absent. Then *count += 1 increments THROUGH that &mut.
//    Handles "first sight" (insert 0 -> +1 = 1) and "seen before" (+1) in one line.
//
//    Note: split_whitespace() yields &str; the map key is String, so word.into()
//    converts &str -> owned String (an allocation per word). An alternative is
//    HashMap<&str, i32> (keys borrow the text: no allocation, but tied to the
//    text's lifetime). String keys = simpler; &str keys = faster. A real tradeoff.
// ----------------------------------------------------------------------------
fn word_frequency() {
    println!("--- word frequency ---");
    let text = String::from("the cat the dog the cat");
    let mut counts: HashMap<String, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word.into()).or_insert(0); // &mut i32
        *count += 1; // increment through the mutable reference
    }

    for (word, n) in &counts {
        println!("{word} : {n}"); // {the: 3, cat: 2, dog: 1} (order arbitrary)
    }
    println!();
}

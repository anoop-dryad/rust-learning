# 05 — Collections (Vec & HashMap)

> Layer 5. Status: **DONE** (Vec, HashMap). Opened with a Result recap.
> Feeds textstat Checkpoint 3 (word frequency = HashMap entry().or_insert()).

---

## TL;DR

- **Vec<T>** = growable list (heap). Homogeneous (one type). push/pop; pop -> Option.
- Access: `v[i]` panics out of bounds; `v.get(i)` -> Option<&T> (safe).
- Iterate: `&v` (read) / `&mut v` (write, needs `*x`) / `v` (move). 90% is `&v`.
- Can't mutate a Vec while iterating a `&v` of it -> E0502 (readers XOR one writer).
- **HashMap<K, V>** = key-value (dict). `use std::collections::HashMap`. Unordered.
- `.get(k)` -> Option<&V> (missing key = None, no crash).
- **`entry(k).or_insert(0)`** returns a `&mut V` (inserting default if absent) ->
  the counting idiom: `*count += 1`.

---

## Result recap (used in textstat Ch.2, formalized in Layer 6)

```rust
enum Result<T, E> { Ok(T), Err(E) }   // like Option, but failure carries an ERROR
enum Option<T>    { Some(T), None }   // failure is just "nothing", no reason
```
- **Option** = "a value might be absent" (empty vec, no first word). None = nothing.
- **Result** = "an operation might fail and you'd want to know WHY" (file read, parse).
  Err(e) carries the reason. That's why read_to_string returns Result, not Option.

=====================================================================
# CHECK 1 — Vec<T>
=====================================================================

```rust
let mut nums: Vec<i32> = Vec::new();   // empty, annotated
let primes = vec![2, 3, 5, 7];          // vec! macro, inferred

nums.push(10);                          // grow (append)
let last = nums.pop();                  // -> Option<i32> (None if empty)

nums[0];                                // direct index, PANICS out of bounds
nums.get(99);                           // Option<&T>, safe -> None
```

**Homogeneous:** `Vec<i32>` holds only i32 (Python lists mix types; Vec doesn't).
For mixed types -> a Vec of an enum.

### Iteration = ownership choice (Layer 3)
```rust
for x in &nums    { /* read */ }        // shared borrow; nums survives  (usual)
for x in &mut nums{ *x += 1; }          // mutable borrow; *x writes through the ref
for x in nums     { /* ... */ }         // MOVES nums; unusable afterward (E0382 if reused)
```

### Mutate-while-iterating = E0502
```rust
for x in &v { v.push(*x); }  // ERROR[E0502]: cannot borrow `v` as mutable
                             // because it is also borrowed as immutable
```
The for-loop holds a SHARED borrow of v for its whole body; push needs a MUTABLE
borrow. "Readers XOR one writer" (Layer 3) — caught at COMPILE time (Python raises
RuntimeError for this at runtime).

=====================================================================
# CHECK 2 — HashMap<K, V>
=====================================================================

```rust
use std::collections::HashMap;          // NOT in the prelude

let mut m: HashMap<String, i32> = HashMap::new();
m.insert(String::from("a"), 10);
m.insert(String::from("a"), 15);        // same key -> OVERWRITES (now 15)

m.get("a");                             // Option<&V>: Some(&15)
m.get("zzz");                           // None (missing key, no crash)

for (k, v) in &m { /* ... */ }          // destructure each pair; order NOT guaranteed
```

Python contrasts: must `use` the import; `.get()` -> Option (vs Python dict[k] KeyError);
no guaranteed iteration order.

### The counting idiom: entry().or_insert()
```rust
let mut counts: HashMap<String, i32> = HashMap::new();
for word in text.split_whitespace() {
    let count = counts.entry(word.into()).or_insert(0); // &mut i32
    *count += 1;                                         // increment THROUGH the &mut
}
// {the: 3, cat: 2, dog: 1}
```
- `entry(k).or_insert(0)` RETURNS a `&mut V` to the value, inserting 0 first if the
  key was absent. One line handles "first sight" and "seen before".
- `*count` because or_insert gives a mutable REFERENCE; `*` writes through it.
- Python equivalent: `counts[w] = counts.get(w, 0) + 1` or defaultdict.

### Key type tradeoff (revisit at textstat Ch.3)
- `HashMap<String, i32>` — keys are OWNED copies (word.into() allocates per word).
  Simpler, always works.
- `HashMap<&str, i32>`   — keys BORROW the source text (no allocation) but the map
  can't outlive the text (a lifetime constraint). Faster.
Both legitimate; a real engineering decision.

---

## Sticking points (my doubts -> corrections)

- **or_insert "sets 0 if absent"** — incomplete. It RETURNS a `&mut V` to the value
  (existing, or a freshly-inserted default). The returned &mut is what you increment.
- **word.into()** — split_whitespace yields &str; a String-keyed map needs owned
  keys, so .into() converts &str -> String (allocates). (Layer 2 conversion at work.)
- **bare `for x in nums`** — MOVES the vec; can't use nums afterward. Use `&nums`
  unless you deliberately want to consume it.
- **{:?} vs {}** — a plain i32 prints with {}; {:?} (debug) is for structures (the
  whole Vec/HashMap). Don't default to {:?} on scalars.

---

## Tooling learned
```
cargo run -p collections --bin vectors
cargo run -p collections --bin hashmaps
```
- `cargo clippy` is great on Vec code: suggests iterators over index loops, flags
  needless .clone(), recommends .iter() patterns.

---

## Reference (unverified this session - no live search)
- Book 8.1 Vectors: https://doc.rust-lang.org/book/ch08-01-vectors.html
- Book 8.3 Hash Maps: https://doc.rust-lang.org/book/ch08-03-hash-maps.html
- Book 8.2 Strings (depth, for later): https://doc.rust-lang.org/book/ch08-02-strings.html
If any 404: start at https://doc.rust-lang.org/book/ and navigate to Ch. 8.

---

## 2-minute self-quiz
1. Vec vs Python list — two differences?
2. What does .pop() return and why?
3. &v / &mut v / v in a for loop — the difference?
4. Why can't you push to a vec while iterating &v of it? Which law / error code?
5. What does HashMap.get() return?
6. What does entry(k).or_insert(0) RETURN (not just set)?
7. Why the `*` in `*count += 1`?
8. Result vs Option — what does the failure case carry in each?

# 07 — Iterators & Closures

> Layer 7. Status: **DONE** (closures, iterators, combining patterns).
> The heart of idiomatic Rust — loops become declarative expressions.
> Feeds textstat Ch.3+ (word frequency, top-N words).

---

## TL;DR

- **Closure** = anonymous fn `|params| body` that can CAPTURE surrounding
  variables. Captures by borrow by default; `move` forces ownership capture.
- **Iterator** = lazy, chainable sequence processing.
  - ADAPTORS (map, filter, take, enumerate, rev) are LAZY — describe only.
  - CONSUMERS (collect, sum, count, max, find, any, all) DRIVE it.
  - Nothing runs until a consumer pulls values through. ZERO-COST (compiles to
    ~the same code as a hand loop).
- `iter()` -> &T (borrow) | `iter_mut()` -> &mut T | `into_iter()` -> T (move).
- `collect` can build Vec, HashMap, String, Result<Vec<_>> (short-circuits), ...

=====================================================================
# CHECK 1 — Closures
=====================================================================

```rust
let add_one = |x| x + 1;                 // types usually inferred
let add = |a: i32, b: i32| a + b;
let describe = |n: i32| {                // multi-line body in { }
    let l = if n > 0 { "pos" } else { "neg" };
    format!("{n} is {l}")
};
```

**Capturing** — the thing a plain fn can't do:
```rust
let threshold = 100;
let is_big = |n: i32| n > threshold;     // captures `threshold` from scope
```

**Capture mode = ownership (Layer 3):**
- default: captures by BORROW (reads -> shared borrow; original still usable).
- `move`: captures by OWNERSHIP.
```rust
let name = String::from("x");
let c = move || println!("{name}");      // move: closure OWNS name
// println!("{name}");                    // ERROR[E0382]: borrow of moved value
```
(Non-Copy -> moved; a Copy type like i32 would be copied and still usable.
 The compiler even says "...does not implement the Copy trait".)
Needed later for threads (a thread's closure must own its data).

=====================================================================
# CHECK 2 — Iterators
=====================================================================

```rust
let out: Vec<i32> = nums.iter()
    .filter(|&n| n % 2 == 0)   // keep evens  (adaptor, lazy)
    .map(|&v| v * 10)          // transform   (adaptor, lazy)
    .collect();                // CONSUME -> Vec
```
Read as a pipeline. Declarative (what, not how) and zero-cost. Clippy suggests
this form over manual for-loops.

**Lazy vs consuming:**
```rust
let lazy = nums.iter().map(|n| n * 2);   // does NOTHING (no consumer)
let out: Vec<_> = lazy.collect();         // NOW it runs
```
If a chain "does nothing", you forgot the consumer. (Clippy: "iterators are lazy
and do nothing unless consumed".)

Consumers: collect, sum, product, count, max, min, find, any, all, for_each.
Adaptors: map, filter, take(n), skip(n), enumerate, rev.

**iter / iter_mut / into_iter — the borrow/move choice again:**
```rust
v.iter()       // &T     shared borrow   (v survives)        [most common]
v.iter_mut()   // &mut T mutable borrow  (v must be mut)
v.into_iter()  // T      MOVE / consumes (v gone)
```
`into_` = consumes/takes ownership (same convention as into_title).
`for x in &v` desugars to `v.iter()`.

=====================================================================
# CHECK 3 — Combining (the real idiom)
=====================================================================

**collect builds different types (the left annotation decides):**
```rust
let v:  Vec<i32>          = it.collect();
let m:  HashMap<i32,i32>  = nums.iter().map(|&n| (n, n*n)).collect(); // (k,v) tuples
let r:  Result<Vec<i32>,_>= strs.iter().map(|s| s.parse()).collect(); // SHORT-CIRCUITS
//   r is Err on the first failing parse — ties iterators to Layer 6.
```

**find / any / all:**
```rust
nums.iter().find(|&&n| n > 3)   // Option<&i32> (might not exist)
nums.iter().any(|&n| n > 3)     // bool
nums.iter().all(|&n| n > 0)     // bool
```

**Top N (= textstat top-words):**
```rust
values.sort_by(|a, b| b.cmp(a));           // descending (b before a)
let top3: Vec<_> = values.iter().take(3).collect();
// for (word,count) tuples: sort_by(|a,b| b.1.cmp(&a.1))  — b.1 is a VALUE so cmp needs &
```

---

## Sticking points (my doubts -> corrections)

- **move + the wrong closure call** — calling clo() instead of mclo() left mclo
  unused (warning) and masked the move test. Call the closure you mean.
- **{:?} creeping in** — use {} for strings/bools/numbers; {:?} for structures
  (Vec, HashMap, enums, tuples).
- **needless `;` after a block** — `if {..};` / `for {..};` / `match {..};` — drop
  the trailing semicolon (clippy: needless_semicolon).
- **unused iterator / unused Result** — a lazy chain with no consumer, or a
  computed Result you never read, both warn. Consume / print it.
- **sort_by(|a,b| b.cmp(&a))** — the extra & is non-idiomatic; `b.cmp(a)` for
  plain refs. (Tuples need `&a.1` because `.1` is a value, not a ref.)
- **the `&` in closures** — pattern `&` = DESTRUCTURE (peel ref layers off what
  the iterator yields); arg `&` (cmp(&x)) = CREATE a reference. map (consuming)
  gets the item directly (&i32); filter/find (peeking) get a ref to it (&&i32).
  Practical: write it, let the compiler's "expected &i32 found i32" tell you to
  add/remove a `&`. Converge in one compile.

---

## Tooling learned
```
cargo run -p iterators --bin closures
cargo run -p iterators --bin iterators
cargo run -p iterators --bin combining
```
- clippy is especially active here: suggests iterator chains over loops, flags
  lazy-unconsumed iterators, needless semicolons, non-idiomatic sort closures.

---

## Reference (unverified this session - no live search)
- Book 13.1 Closures: https://doc.rust-lang.org/book/ch13-01-closures.html
- Book 13.2 Iterators: https://doc.rust-lang.org/book/ch13-02-iterators.html
- Iterator trait (full method list — bookmark it):
  https://doc.rust-lang.org/std/iter/trait.Iterator.html
If any 404: start at https://doc.rust-lang.org/book/ and navigate to Ch. 13.

---

## 2-minute self-quiz
1. What can a closure do that a plain fn can't?
2. Borrow vs move capture — what does `move` change, and when do you need it?
3. Lazy adaptor vs consumer — name two of each; why does a lone map() do nothing?
4. iter / iter_mut / into_iter — what does each yield?
5. Name three types collect can build.
6. What does collecting into Result<Vec<_>> do on the first error?
7. The shape of a top-N pipeline (which methods)?
8. In a closure pattern, what does the `&` in `|&n|` do?

# 08 — Traits & Generics

> Layer 8. Status: **DONE** (generics, traits, trait bounds).
> The last big conceptual layer. Ties together From/Copy/Debug/Display/Ord —
> all of which are traits you'd already met. Feeds textstat (Display output,
> and a pluggable Analyzer trait WHEN there are 2+ analyzers).

---

## TL;DR

- **Generics** `<T>` = write logic once for any type; checked at COMPILE time;
  ZERO-COST (monomorphization: compiler stamps out a specialized copy per type).
- You can only DO to T what you PROMISE via **trait bounds**. Use the MINIMAL
  trait that covers what you actually do (PartialOrd for `>`, not Ord).
- **Trait** = shared behavior (an interface). Define the contract; each type
  provides its own impl. `#[derive(Debug)]` AUTO-implements the Debug trait.
- **Trait bound** `<T: Trait>` = "any type that implements Trait" — the basis of
  pluggable designs (new types plug in by implementing the trait; machinery
  never changes).
- Create a custom trait REACTIVELY (2+ types share behavior), not speculatively.

=====================================================================
# CHECK 1 — Generics
=====================================================================

```rust
fn first<T>(list: &[T]) -> &T { &list[0] }          // no bound: doesn't touch the value
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... } // `>` needs PartialOrd
struct Pair<T> { first: T, second: T }              // generic struct
```
- `<T>` declares a TYPE PARAMETER. You've used generics all along: Option<T>,
  Vec<T>, HashMap<K,V>. Now you're on the DEFINING side.
- **Monomorphization:** at compile time Rust generates a specialized copy of the
  generic per concrete type used -> no runtime dispatch -> zero-cost. (Python's
  duck typing checks types at RUNTIME, which costs.)

**Bounds = promises about T.** You can only use capabilities you've declared:
```rust
// `+` via Add needs THREE things pinned:
fn sum<T: std::ops::Add<Output = T> + Copy>(a: &T, b: &T) -> T { *a + *b }
//          Add<Output=T> : addable AND result is T
//          + Copy        : so *a/*b COPY (can't MOVE out of a &ref — Layer 3)
//          multiple bounds joined with +
```

**PartialOrd vs Ord (a real gotcha):**
- `PartialOrd` = `<`,`>` comparison; SOME values may be incomparable. f64 has this.
- `Ord` = TOTAL order (every pair comparable). f64 does NOT (NaN). Needed by
  .sort(), .max(), BTreeMap keys.
- So `largest<T: Ord>` silently REJECTS Vec<f64>. Use `T: PartialOrd` (minimal,
  what `>` needs) to accept floats. Over-constraining excludes valid types.

=====================================================================
# CHECK 2 — Traits
=====================================================================

```rust
trait Describe {
    fn describe(&self) -> String;              // required method (contract)
    fn short(&self) -> String {                // DEFAULT method (free, overridable)
        self.describe().chars().take(10).collect()
    }
}
impl Describe for Book { fn describe(&self) -> String { ... } }
impl Describe for Car  { fn describe(&self) -> String { ... } }
```
- Define the CONTRACT once; each type provides its own body. Same method name,
  type-specific impl. (Like an interface / Protocol, checked at compile time.)
- You can impl a trait for a type you didn't define (even i32), as long as you
  defined the trait.
- **Default methods**: implementors get them free (only must provide the required
  ones). short() uses .chars().take() NOT byte slicing — byte split_at(n) PANICS
  on short input and on mid-UTF-8-char boundaries (a char is 1-4 bytes).

**#[derive(Debug)] = auto-implementing a trait.** Debug is a trait; derive
generates `impl Debug for YourType` for you. Same for Clone, Copy, PartialEq...
**Display ({}) is NOT derivable** — "how it looks to a human" is your judgment,
so you write `impl fmt::Display` by hand:
```rust
impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} by {}", self.model, self.manufacturer)
    }
}
```
(This is how textstat gets `println!("{stats}")` output.)

## WHEN to create a custom trait (the design skill)
Create one when:
  1. 2+ types need to be treated the SAME way (uniform behavior), OR
  2. a fn must be generic over "any type that can do X", OR
  3. you're satisfying a STANDARD trait (Display, Iterator, From, Default...).
DON'T create one when: only one type implements it (just use an impl block);
"might need it later" (speculative); OOP-inheritance reflexes.
Rule: write concrete code first; EXTRACT a trait when a pattern repeats across
2-3 real types. Refactoring to a trait later is cheap/safe -> no penalty for
waiting, and you need 2-3 examples to design the RIGHT abstraction.

=====================================================================
# CHECK 3 — Trait bounds
=====================================================================

```rust
fn announce<T: Analyzer>(item: &T)  { ... item.analyze() ... } // inline
fn announce(item: &impl Analyzer)   { ... }                     // impl Trait shorthand
fn announce<T>(item: &T) where T: Analyzer { ... }              // where clause
fn full<T: Analyzer + fmt::Display>(item: &T) { ... }           // multiple bounds (+)
fn make() -> impl Analyzer { Book { ... } }                     // return impl Trait
```
- `<T: Analyzer>` lets you CALL analyze() on T inside the fn — the bound promises
  the method exists. Plain `<T>` would reject the call.
- One function works on EVERY type implementing the trait — Book, Tweet, and any
  future type. ALL bounds (joined with +) must be satisfied.

**Failure cases (the lesson):**
- announce(&17) -> E0277: `{integer}: Analyzer` not satisfied (i32 isn't Analyzer).
- full(&tweet) where Tweet has Analyzer but not Display -> E0277: `Tweet` doesn't
  implement Display. ALL bounds required, not just some.

**Pluggable design (textstat's future analyzers):**
```rust
trait Analyzer { fn analyze(&self, text: &str) -> String; }
fn run<T: Analyzer>(a: &T, text: &str) { println!("{}", a.analyze(text)); }
```
Add WordCounter, ReadabilityScorer (each `impl Analyzer`) -> run works on them
with ZERO changes. Open to EXTENSION (add types), closed to modification (never
edit run). THIS is why trait bounds power plugin-style systems.
(Related: Box<dyn Trait> trait objects hold DIFFERENT types in one Vec at
runtime — like Box<dyn Error> — covered at smart pointers, Layer 12.)

---

## Sticking points (my doubts -> corrections)

- **Ord vs PartialOrd** — Ord is stricter (total order); it excludes f64 (NaN).
  For `>` use PartialOrd (minimal). Over-constraining silently rejects valid types.
- **generic `sum` needed Add<Output=T> + Copy** — "T can be added" wasn't enough:
  must pin the output type AND promise Copy so `*a` doesn't MOVE out of a &ref.
  Generics force you to declare EVERY capability you use on T.
- **short() byte-slicing bug** — split_at(11) panics on short strings and on
  mid-UTF-8-char. Fixed with .chars().take(10).collect() (safe + char-correct;
  reuses Layer 7 iterators).
- **"traits are open to extension, so create them early?"** — NO. Extensibility
  is why traits are POWERFUL, not WHEN to create one. Trigger = 2+ types sharing
  behavior. You need real examples to design the right trait; deferring is free.
- **{:?} vs {}** — {} for human-readable Strings (describe/short); {:?} for whole
  structures.

---

## Tooling learned
```
cargo run -p traits --bin generics
cargo run -p traits --bin traits
cargo run -p traits --bin trait_bounds
```
- error[E0277] = a trait bound is not satisfied (the type doesn't implement the
  required trait). The most common trait-related error.
- #[derive(Debug, Clone, Copy, PartialEq, ...)] auto-generates those trait impls.

---

## Reference (unverified this session - no live search)
- Book 10.1 Generic Data Types: https://doc.rust-lang.org/book/ch10-01-syntax.html
- Book 10.2 Traits: https://doc.rust-lang.org/book/ch10-02-traits.html
- std fmt (Display/Debug): https://doc.rust-lang.org/std/fmt/
- std cmp (Ord/PartialOrd): https://doc.rust-lang.org/std/cmp/
If any 404: start at https://doc.rust-lang.org/book/ or /std/ and navigate.

---

## 2-minute self-quiz
1. What problem do generics solve? What is monomorphization?
2. Why does largest<T> need a bound but first<T> doesn't?
3. PartialOrd vs Ord — why does Ord exclude f64?
4. What is a trait, and what does #[derive(Debug)] actually do?
5. Why is Display not derivable when Debug is?
6. What does <T: Trait> let you do inside the fn that plain <T> doesn't?
7. Three ways to write a bound? How do you require two traits?
8. Why is <T: Trait> the mechanism behind pluggable designs?
9. When should you CREATE a custom trait (and when not)?

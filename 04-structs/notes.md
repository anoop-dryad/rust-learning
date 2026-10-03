# 04 — Structs & Enums

> Layer 4. Concepts: **DONE** (structs, methods, enums/Option/match).
> Applied: textstat Checkpoint 1 (weekend hands-on).
> This is where Rust stops being about rules and starts being EXPRESSIVE:
> model data (struct), give it behavior (impl), make illegal states impossible
> (enum), handle absence without null (Option), branch safely (match).

---

## TL;DR

- **struct** = named type with typed fields. Define the shape, then instantiate;
  every field must get a value. Mutability is all-or-nothing (the binding is `mut`).
- **impl** = methods. First param `&self` (read) / `&mut self` (write) / `self`
  (consume) — the Layer 3 ownership choice, for methods.
- **`self` (by value) moves the WHOLE struct** (if non-Copy) -> nothing survives.
  Different from a PARTIAL move of one field (Copy fields survive).
- **enum** = a value is EXACTLY ONE variant; variants can carry different data.
- **Option<T>** = std enum `{ Some(T), None }`. Rust has NO null; absence is in
  the type and the compiler forces you to handle None. No null crashes.
- **match** = destructures (pulls inner data out) + exhaustive (handle every case
  or E0004). `_` is the catch-all but trades away per-variant checking.

=====================================================================
# CHECK 1 — Structs
=====================================================================

```rust
#[derive(Debug)]           // enables {:?} and {:#?}
struct Book {
    title: String,
    author: String,
    pages: u32,            // a count can't be negative -> unsigned is idiomatic
    available: bool,
}
let book = Book { title: String::from("x"), author: String::from("y"),
                  pages: 100, available: true };
book.title;                // dot access
```

- **Define vs create:** `struct Book {...}` is the shape (no data); `Book {...}`
  makes an instance. ALL fields must be given values (no uninitialized/missing).
- **Mutability all-or-nothing:** no per-field `mut`. `let mut book` makes the whole
  instance mutable.
- **Field shorthand:** `Book { title, author, .. }` when var names match field names.
- **Struct update `..other`:** fills remaining fields from another instance. It
  COPIES Copy fields and MOVES non-Copy fields out of `other` -> `other` becomes
  PARTIALLY MOVED. (If you override a non-Copy field, that one isn't moved.)
- **`#[derive(Debug)]`** -> `{:?}` (compact) and `{:#?}` (pretty). Put it on ~every struct.

=====================================================================
# CHECK 2a — Methods (impl)
=====================================================================

Data (struct) and behavior (impl) are SEPARATE blocks.

```rust
impl Book {
    fn is_long(&self) -> bool { self.pages > 300 }         // &self: read
    fn description(&self) -> String {                       // &self: read, returns owned
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
    fn mark_unavailable(&mut self) { self.available = false; } // &mut self: write
    fn into_title(self) -> String { self.title }            // self: CONSUMES the book
}
```

| First param | Meaning | Use when |
|---|---|---|
| `&self` | borrow (read) | most methods |
| `&mut self` | mutable borrow | method modifies the instance (instance must be `mut`) |
| `self` | takes ownership | method CONSUMES it (rare; name it `into_*`) |

Same `&T / &mut T / T` choice as Layer 3 function params, spelled for `self`.

**Associated functions (constructors):** a fn in `impl` with NO self, returning
`Self`, called with `::` (like `String::from`):
```rust
impl Book { fn new(title: String) -> Self { Book { title, author: String::new(), pages: 0, available: true } } }
let b = Book::new(String::from("x"));   // Self = the type this impl is for
```

=====================================================================
# CHECK 2b — Enums, Option, match
=====================================================================

## Enums — a value is EXACTLY ONE variant
```rust
#[derive(Debug)]
enum BookLocation {
    OnShelf(u32),                          // tuple variant: a number
    CheckedOutBy(String),                  // tuple variant: a String
    InRepair { since: String, cost: f64 }, // struct variant: named fields
    Lost,                                  // unit variant: no data
}
let loc = BookLocation::OnShelf(16);       // :: to construct
```
Each variant can carry different data. Invalid states are UNREPRESENTABLE (vs a
String "status" you could typo). Make illegal states impossible.

## Option<T> — the no-null design
```rust
enum Option<T> { Some(T), None }           // it's just an enum in std
```
Rust has NO null. "Might be absent" is in the TYPE: `Option<T>` = maybe a value;
plain `T` = definitely a value. The compiler forces you to handle `None` before
touching the inner value -> null crashes are impossible.
```rust
let found: Option<&i32> = numbers.get(7);  // out of range -> None, no panic
// can't do `found + 1`: it's an Option, not a number. Must unwrap the possibility.
```

## match — destructure + exhaustive
```rust
fn describe(loc: &BookLocation) -> String {
    match loc {
        BookLocation::OnShelf(n) => format!("on shelf {n}"),          // binds n
        BookLocation::CheckedOutBy(who) => format!("out, {who}"),     // binds who
        BookLocation::InRepair { since, cost } => format!("{since} {cost}"), // both
        BookLocation::Lost => String::from("lost"),
    }
}
```
Two things `match` does that an `if`-chain can't:
1. **Destructures** — pulls inner data out and binds it to a name (OnShelf(n) -> n).
2. **Exhaustive** — must handle every variant or it won't compile:
   `error[E0004]: non-exhaustive patterns: ... not covered`.
   (Feature: add a variant later -> compiler flags every match to update.)
`_ => ...` is the catch-all. It satisfies exhaustiveness BUT trades away
per-variant checking: a new variant silently routes to `_`. On enums YOU own,
list every variant and SKIP `_`.

---

## Sticking points (my doubts -> corrections)

- **"After consuming self, scalar fields should survive (like partial move)?"**
  NO. `self`-by-value / `let b = book` moves the WHOLE struct as one unit. Since
  Book is not Copy, the ENTIRE `book` is invalid afterward — Copy fields included.
  Copy-fields-survive applies only to a PARTIAL move (moving ONE field):
    let x = book.author;   // partial -> book.pages still works
    book.into_title();     // whole move -> nothing of book survives
- **into_title vs consume_title naming** — method taking `self` to convert ->
  convention is `into_*`. (And the call name must match the definition.)
- **`{:?}` put quotes around my output** — that's DEBUG format. For a clean
  human string use `{}` (display). {:?} = for debugging, {} = for people.
- **"variant never constructed" warning** — writing a match ARM for a variant
  does NOT construct it. The dead_code warning means you never MADE one. Fix by
  constructing each (or `#[allow(dead_code)]` for deliberate cases). `#![warn(..)]`
  turns warnings ON; `#![allow(..)]` turns them OFF.
- **i32 vs u32 for a count** — prefer unsigned for "can't be negative"; makes the
  bad state unrepresentable (type-as-design).

---

## Tooling learned
```
cargo run -p structs --bin structs
cargo run -p structs --bin methods
cargo run -p structs --bin enums
cargo run -p structs --bin match
```
- `#[derive(Debug)]` for {:?}/{:#?}; `{}` display vs `{:?}` debug.
- `#[allow(dead_code)]` (item) / `#![allow(dead_code)]` (whole file) to silence,
  but prefer fixing the cause — warnings are free code review.
- error[E0004] non-exhaustive match is an ERROR (mandatory), not a warning.

---

## Reference (unverified this session - no live search)
- Book 5.1 Defining Structs: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
- Book 5.3 Method Syntax: https://doc.rust-lang.org/book/ch05-03-method-syntax.html
- Book 6.1 Defining an Enum (incl Option): https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
- Book 6.2 match: https://doc.rust-lang.org/book/ch06-02-match.html
If any 404: start at https://doc.rust-lang.org/book/ and navigate to Ch. 5-6.

---

## 2-minute self-quiz
1. Define vs instantiate a struct — what's required at instantiation?
2. Can you make a single field `mut`? How do you get a mutable field?
3. The three `self` forms and when to use each?
4. Why does `book.into_title()` make ALL of book unusable, but `let x = book.author` doesn't?
5. How is a Rust enum more powerful than a Python enum?
6. What IS Option<T>, and how does it prevent null crashes?
7. Two things match does that an if-chain can't?
8. Why is `_` on an enum you own sometimes a bad idea?
9. What's an associated function (e.g. `new`) and how is it called?

# 03 — Ownership

> Layer 3. Status: **DONE** (all 3 checks).
> (1) move rule  (2) borrowing (&T, &mut T)  (3) functions, the &str idiom, slices.
> This is the conceptual spine of Rust — everything later builds on it.

---

## TL;DR

- **3 rules of ownership:** (1) each value has an owner; (2) only ONE owner at a
  time; (3) when the owner leaves scope, the value is DROPPED (freed).
- **Move:** `let b = a` on a heap type (String, Vec) transfers ownership; `a` is
  invalidated. Prevents DOUBLE FREE. No GC.
- **Copy:** scalars (i32, bool, char...) are stack-only and cheap -> duplicated,
  both stay valid.
- **Clone:** explicit deep copy (new heap buffer). Costly; don't clone just to
  dodge the borrow checker.
- **Borrow:** use without owning. `&T` = shared/read (many). `&mut T` =
  mutable/write (exactly one). Never both at once.
- **Return moves ownership OUT** to the caller (value escapes the drop).
- **Param rule:** `String` (take), `&str` (read, default), `&mut String` (mutate),
  `&[T]` (read a list).
- **No `;` = the value; `;` discards it.** `return` only for early exits.

=====================================================================
# CHECK 1 — The move rule
=====================================================================

## The three rules
1. Each value has an owner.
2. Only ONE owner at a time.
3. Owner leaves scope -> value dropped (freed).

## Move on assignment
```rust
let s1 = String::from("hi");
let s2 = s1;          // ownership MOVES s1 -> s2; s1 now invalid
// println!("{s1}");  // ERROR[E0382]: borrow of moved value: `s1`
```
**Why:** two owners -> both free the same heap buffer at scope end -> DOUBLE FREE
(memory corruption / security bug). Move makes exactly one owner free it once,
proven at COMPILE time, no garbage collector.

## Copy vs move
- Heap-owning types (String, Vec) -> **MOVE** (source invalidated).
- Stack-only scalars (i32, f64, bool, char, tuples of Copy types) -> **COPY**
  (both valid; nothing on the heap to double-free).
```rust
let x = 5; let y = x; // COPY -> x and y both valid
```

## Clone = explicit deep copy
```rust
let s2 = s1.clone(); // new heap buffer + copied bytes; two owners, two buffers
```
Explicit because it ALLOCATES + COPIES (costly). SMELL: cloning just to silence a
move error -> you probably want to BORROW instead (Check 2).

## Move into a function
Passing an owned value by value MOVES it; caller's variable is invalidated.
```rust
takes_ownership(s); // s moved in
// println!("{s}"); // ERROR[E0382]
```
Clumsy fix: function RETURNS it and you rebind (`let s = f(s);`). Motivates borrowing.

=====================================================================
# CHECK 2 — Borrowing (&T and &mut T)
=====================================================================

## Borrow = use without owning (no move, no clone)
```rust
let len = calculate_length(&s); // lend s; s stays valid
fn calculate_length(text: &str) -> usize { text.len() }
```
The reference drops at the function's end, but the String is NOT freed (a borrower
owns nothing -> no cleanup duty).

## THE CENTRAL LAW — readers XOR one writer
For one value, at one moment:
- any number of SHARED borrows `&T` (readers), **OR**
- exactly ONE mutable borrow `&mut T` (writer),
- **never both, never two writers.**

Same shape as XOR: "not both true." Forbidden case = `1 XOR 1 = 0`
(readers AND a writer at once).

| readers? | writer? | allowed |
|---|---|---|
| yes | no  | yes |
| no  | yes | yes |
| yes | yes | **NO** (the XOR-forbidden case) |
| no  | no  | yes |

**Why:** reader + writer at once -> writer could mutate/REALLOCATE (push_str moving
the heap buffer) out from under the reader -> USE-AFTER-FREE. Two writers -> DATA
RACE. Rust forbids both at COMPILE time (basis of "fearless concurrency").

## Mutating through a borrow — 3 things must line up
(a) value is `mut`, (b) pass `&mut s`, (c) param is `&mut String`.
```rust
let mut s = String::from("hi");
update(&mut s);
fn update(text: &mut String) { text.push_str("!"); }
```
LESSON: in-place mutating methods return `()` (unit), NOT the new value.
`let x = text.push_str("!");` -> x is (), not the string.

## The three borrow errors (met on purpose)
- **E0596** cannot mutate through a SHARED borrow (`&T`). Fix: `&mut`.
- **E0499** two MUTABLE borrows at once (two writers).
- **E0502** SHARED + MUTABLE overlapping (reader while writer).

## NLL (non-lexical lifetimes) — borrows end at LAST USE, not the `}`
```rust
let r = &s; println!("{r}");  // r's borrow ENDS at its last use
let w = &mut s;               // OK now — no reader alive
```
Sequence a reader then a writer = fine; the ban is only on OVERLAP in time.

=====================================================================
# CHECK 3 — Functions, the &str idiom, slices
=====================================================================

## Ownership flows OUT via return
Returning a value moves ownership to the caller, so it ESCAPES the end-of-scope
drop. That's how a String made inside a function survives the call.
Decision: function KEEPS/consumes -> take by value (String). Only READS -> borrow.

## The &str idiom (compiles-but-not-idiomatic fix)
Prefer `&str` over `&String` for read-only string params.
```rust
fn calculate_length(text: &str) -> usize { text.len() } // idiomatic
```
A `&str` param accepts BOTH a borrowed String (via deref coercion) AND a literal;
`&String` accepts only the former. More general, zero cost. (Clippy flags &String.)
Same idea for lists: prefer `&[T]` over `&Vec<T>`.

## Slices — a borrowed VIEW; stores PTR + LEN
```rust
let hello = &s[0..5];    // &str slice of string bytes
let mid   = &nums[1..4]; // &[i32] slice of an array/Vec
```
Owns nothing (a borrow) -> borrow rules apply. `&str` IS a slice (ptr+len — the two
fields from Layer 2). CAVEAT: range indexing panics out of bounds; string slicing
panics if it splits a multi-byte UTF-8 char (char = 1-4 bytes inside a String).

## Parameter decision rule (everyday payoff)
| Param type | Use when |
|---|---|
| `String` | function takes/keeps ownership. Rarely the default. |
| `&str` | function only READS string data. **Idiomatic default.** |
| `&mut String` | function MUTATES the caller's string. |
| `&[T]` | function READS a slice of an array/Vec. |

## Expressions vs statements (why we skip `;` on a return)
- **No `;` = EXPRESSION** -> its value is the block's/function's value (tail expr).
- **`;` = STATEMENT** -> value DISCARDED; block yields `()`.
```rust
fn f() -> usize { text.len() }   // returns the len
fn g() -> usize { text.len(); }  // ERROR: returns (), not usize
```
`if`, `match`, `{}` blocks are all expressions:
```rust
let x = { let a = 2; a + 1 };            // x == 3
let label = if n > 0 { "pos" } else { "neg" };
```
Use `return` only for EARLY exits (`return` always takes a `;`); use the bare tail
expression for the final value.

---

## Sticking points (my doubts -> corrections)

- **"the String is the borrower"** -> No. The String is the VALUE being borrowed;
  the REFERENCE (`text: &str`) is the borrower. Keep value / owner / borrower distinct.
- **cloned twice to avoid move errors** -> that's the SMELL. Borrow (`&`) instead of
  clone when the callee only needs to look.
- **`push_str` return value** -> it returns `()`, mutates in place. Don't bind its result.
- **"&str is a generic string type"** -> not "generic" (that's `<T>` type params).
  It's MORE GENERAL as an input: accepts &String and literals.
- **"a slice is a view"** -> yes, but specifically PTR + LEN (no capacity, no ownership).
- **XOR = binary insight** -> the borrow rule is mutual exclusion, same shape as
  `1 XOR 1 = 0`: readers and a writer can't both be true at once.

---

## Tooling learned
```
cargo run -p ownership --bin move_rule
cargo run -p ownership --bin borrowing
cargo run -p ownership --bin functions_slices
```
- Read the compiler error CODES (E0382 move, E0596/E0499/E0502 borrows) — they map
  1:1 to a concept.
- Compiler help lines are literal fixes ("remove this semicolon to return this value").
- `cargo clippy` will suggest `&str` over `&String`, bare returns over `return x;`.

---

## Reference (unverified this session - no live search)
- Book 4.1 What Is Ownership? (move, Copy, clone, drop):
  https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- Book 4.2 References and Borrowing (&, &mut, the law):
  https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
- Book 4.3 The Slice Type: https://doc.rust-lang.org/book/ch04-03-slices.html
- Book 3.3 Functions (statements vs expressions):
  https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
If any 404: start at https://doc.rust-lang.org/book/ and navigate to Ch. 3-4.

---

## 2-minute self-quiz
1. State the three rules of ownership.
2. Why does `let b = a` invalidate `a` for a String but not an i32?
3. What bug does the move rule prevent, and how?
4. State the borrow checker's central law in one sentence.
5. Why can't a shared and a mutable borrow of one value overlap in time?
6. What 3 things must line up to mutate a value through a function?
7. Why prefer `&str` over `&String` as a parameter?
8. What does a slice store? (two things)
9. What does adding `;` to a function's last expression do to the return value?
10. Param rule: when String vs &str vs &mut String?

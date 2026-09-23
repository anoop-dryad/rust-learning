# 02 — Types & the Stack

> Layer 2. Status: **In progress** (Check 1 of 3 done).
> Covered: scalar types, sizes, overflow, floats/bool/char, division.
> Next: **Check 2 — stack vs heap** (the concept this layer is really about),
> then Check 3 — compound types (tuple, array).

---

## TL;DR (read this first)

- Rust makes you care about **how big a value is** and **where it lives** —
  Python/Java hide both.
- The number in `u8`, `i32`, `u64` is **BITS**, not bytes. **1 byte = 8 bits.**
- Four scalar types: **integers, floats, bool, char.** Defaults: integer = `i32`,
  float = `f64`.
- Integer overflow has **two** guards: a **compile-time lint** (provable cases)
  and a **runtime check** (debug panics, release silently wraps).
- `char` = a 4-byte **Unicode scalar** (any of ~1.1M symbols). `bool` = 1 byte.
- Integer `/` **truncates** (`7/2 == 3`). Float `/` is real (`7.0/2.0 == 3.5`).
- **known size -> STACK. growable size -> HEAP** (with a fixed-size handle on the stack).
- `&str` = **borrow** (2 fields: ptr+len, points at read-only binary text, owns nothing).
- `String` = **owner** (3 fields: ptr+len+cap, heap data, frees it at end of scope).
- No garbage collector: **one owner** per heap value, freed automatically at scope end.
  That rule IS ownership (Layer 3).

---

=====================================================================
# CHECK 1 — Scalar types
=====================================================================

## Sizes — the bits vs bytes rule

`std::mem::size_of::<T>()` returns size **in bytes**. Prove, don't guess.

| Type | Bits | Bytes | Notes |
|---|---|---|---|
| `u8` / `i8` | 8 | 1 | `u8` range 0..=255 |
| `u16` / `i16` | 16 | 2 | |
| `i32` (default) | 32 | 4 | |
| `i64` / `u64` | 64 | 8 | |
| `f64` (default float) | 64 | 8 | |
| `bool` | — | 1 | only 2 states, but CPUs address bytes not bits |
| `char` | 32 | 4 | any Unicode scalar |
| `usize` / `isize` | — | 8* | *pointer-sized: 8 on 64-bit (my Mac) |

Integer naming: `i` = signed, `u` = unsigned; the number = bits.
`i8 i16 i32 i64 i128` / `u8 u16 u32 u64 u128` / `isize usize`.
`usize` is the type for **indexing and lengths**.

---

## Overflow — TWO separate mechanisms

This is the subtle part. There are two different guards, and they fire at
different times.

| Mechanism | When | Fires in `--release`? |
|---|---|---|
| `arithmetic_overflow` **lint** | **compile time**, when overflow is provable from constants | **Yes** — it's a hard compile error (deny-by-default) |
| runtime overflow **check** | **runtime**, when the compiler couldn't prove it | **No** — debug panics, release silently **wraps** |

Example of the lint (constants -> caught before running):
```rust
let a: u8 = 255;
println!("{}", a + 1);
// error: this arithmetic operation will overflow
// note: `#[deny(arithmetic_overflow)]` on by default
```
Errors identically in debug AND release, because compilation is the same for both.

To see the **runtime** behavior, hide the value from the compiler so it can't
const-fold and the lint can't fire:
```rust
let x: u8 = std::hint::black_box(255); // "pretend you don't know this is 255"
let y = x + 1;
// debug build   -> PANIC: "attempt to add with overflow"
// release build -> y == 0 (silently wraps, two's complement)
```
`black_box` is stable since Rust 1.66.

**LESSON:** never rely on overflow to panic in production. When overflow is
possible, say what you mean:
```rust
let x: u8 = 255;
x.checked_add(1)    // -> None  (Option; safe)
x.wrapping_add(1)   // -> 0     (deliberate wrap / modular)
x.saturating_add(1) // -> 255   (clamp at max)
```
Bare `+` = "I'm certain this can't overflow." (Option covered fully in the
error-handling layer; for now `None` = "would have overflowed".)

---

## Floats, bool, char

```rust
let pi = 3.14;        // f64 by default (f32 also exists)
let flag: bool = true;
let letter: char = 'z';   // SINGLE quotes
let emoji: char = '😻';  // (an emoji) still one char, still 4 bytes
```

Python/Java contrasts:
- **`char` (single quotes) != string (double quotes)** — different types.
  `'z'` is not `"z"`. Python has no separate char type.
- **`bool` is strictly its own type.** `true + 1` does **not** compile
  (error E0369). Python treats `True + 1 == 2`; Rust does not.
- **`char` is 4 bytes** because it's a Unicode scalar, not an ASCII byte — it
  must be able to hold any of ~1.1M code points, so it reserves 4 bytes even for
  `'a'`. A type has one fixed size.

> Foreshadow: a `char` is always 4 bytes, but the SAME character inside a
> `String` is stored as UTF-8 (1–4 bytes: `'a'`=1, emoji=4). "What you see on
> screen" != "bytes in memory." Matters a lot at `String` vs `&str`.

---

## Division gotcha

```rust
7 / 2      // 3    integer division truncates toward zero (NOT 3.5)
5 / 3      // 1
7.0 / 2.0  // 3.5  float division
```
Python 3's `/` always gives a float; Rust keeps integer division an integer.
Want a float result? Make both operands floats.

---


=====================================================================
# CHECK 2 — Stack vs Heap, Owner vs Borrow
=====================================================================

## The principle

- **Known size at compile time -> STACK.** Fast, automatic, pushed/popped with scope.
  (All scalars: a u8 is ALWAYS 1 byte, so the compiler reserves exact room.)
- **Unknown / growable size -> HEAP.** The value's bytes live on the heap; a small
  fixed-size handle (pointer + bookkeeping) lives on the stack.

## &str (BORROW) vs String (OWNER) — the canonical example

```rust
let x: &str   = "Anoop";                 // borrow: points at text baked in the binary
let y: String = String::from("Anoop");   // owner: holds a heap copy
```

Memory picture:
```
x: &str  (BORROW)                 y: String  (OWNER)
[ ptr ] -> binary (read-only)     [ ptr ] -> heap: A n o o p
[ len ]                           [ len ]      (y frees this at scope end)
                                  [ cap ]
2 fields; owns nothing;           3 fields; OWNS the heap data;
grows nothing; frees nothing      can grow; frees it
```

**Owner vs reference (the model everything hinges on):**
- **Owner** = holds the data, is RESPONSIBLE for freeing it at end of scope.
- **Reference/borrow** = temporarily looks at data someone else owns; frees nothing.
- A `String` is NOT "a pointer to a string" - it IS the string (it contains a
  pointer internally, but it owns what that points to).

## Two kinds of borrow (preview of Layer 3's central rule)

```rust
let r1: &T     = &v;      // SHARED (immutable) borrow  - can look, not change. MANY allowed.
let r2: &mut T = &mut v;  // EXCLUSIVE (mutable) borrow - can change. ONLY ONE, no shared alongside.
```
The `mut` in `&mut` is not cosmetic: "one mutable XOR many shared" is the borrow
checker's core law.

5-type labeling result: i32=owner, &i32=shared borrow, String=owner,
&mut String=**mutable** borrow, &str=shared borrow.

## &str -> String must be EXPLICIT (it allocates + copies)

```rust
let x: String = "Anoop";        // ERROR[E0308]: expected String, found &str
```
Rust won't silently convert borrow -> owned (costs an allocation). Ask for it:
```rust
String::from("Anoop")   // most explicit; conversion written on the destination type
"Anoop".to_string()     // idiomatic, string-specific
"Anoop".to_owned()      // general "make the owned version of this borrow" (&str->String, &[T]->Vec<T>...)
"Anoop".into()          // converts into whatever type is EXPECTED; needs target known from context
```
- `String::from(x)` and `x.into()` are the SAME conversion, opposite ends
  (From written on destination, Into called on source).
- `.into()` with no annotation fails: "type annotations needed" - it can't guess the target.
- While learning: prefer `String::from` / `to_string`. Recognize `to_owned` and `into`.

## len vs capacity

- `len` = bytes actually used now. `capacity` = bytes allocated on heap. **capacity >= len always.**
- One BIG push allocates exactly what's needed -> capacity ends up == len.
- Many SMALL pushes -> capacity jumps AHEAD in steps (over-allocation, often doubling)
  so the next few pushes don't reallocate each time. (Exact steps are a std impl detail.)

## Sizes prove the field counts

```
size_of::<&str>()   == 16   // ptr + len            (2 * 8 on 64-bit)
size_of::<String>() == 24   // ptr + len + capacity (3 * 8)
size_of::<&i32>()   == 8    // just a pointer
```

## The doorway to ownership

Heap values raise a question stack values don't: **who frees this, and when?**
- Python/Java: a garbage collector, at a cost you don't control.
- Rust: NO GC. Exactly **one owner**; when the owner goes out of scope, Rust frees
  the heap memory - deterministically, at the closing `}`. That IS ownership (Layer 3).

---


## Sticking points (my doubts -> the correction)

**Q: "u8 uses 8 bytes? char 4 bytes, bool 1 byte — why does char have more?"**
First error: `u8` is 8 **bits** = **1 byte**, same size as `bool`. The number is
always bits; divide by 8 for bytes. As for char > bool: `char` must represent any
of ~1.1M Unicode symbols -> needs 4 bytes. `bool` only distinguishes 2 states -> 1
byte is plenty (can't go below 1 byte because CPUs address memory per-byte).
"One character" hides a lot: an ASCII char fits in 1 byte, but Rust's `char` is
built to hold *any* character in *any* script plus emoji, and one fixed size must
fit the biggest case.

**Q: (live compile error) `255 + 1` errored the same in debug and release — why
not a panic?**
Because that's the **compile-time lint**, not the runtime check. With constant
operands the compiler proves the overflow and refuses to build — in both
profiles, since compilation is identical. The debug-panic / release-wrap
difference only applies to the *runtime* check, which you only reach when the
compiler couldn't prove the value (hence `black_box` in the demo).

**"both x and y are pointers to a string?"** No. `x` (&str) is a REFERENCE (borrows).
`y` (String) is an OWNER (holds + frees). Both contain a pointer internally, but the
meaning is opposite: borrow vs own.

**"&mut String is just a borrower?"** True but incomplete - it's the MUTABLE/exclusive
kind. Only one at a time; no shared borrows alongside it.

**"to_owned vs into?"** to_owned = "own a copy of this borrow" (general). into =
"convert into the expected type" (needs target known). Both reach String from &str.

**println! inline `{name}` only works for a BARE variable** - not `{name.len()}` or
expressions. For those use `{}` + argument: `println!("{}", name.len())`.
`{:?}` = debug format (prints whole structures: arrays, tuples, later your own types).

---

## Tooling learned

```
cargo run -p types --bin scalars              # debug build
cargo run -p types --bin scalars --release    # release build (overflow checks off)
```
- `std::mem::size_of::<T>()` — size in bytes; verify sizes instead of guessing.
- `std::hint::black_box(v)` — stops const-folding so you can observe runtime behavior.
- `#[allow(dead_code)]` — an **attribute** (`#[...]` syntax) that silences the
  "never used" warning for an intentionally-uncalled item.
- `#[deny(arithmetic_overflow)]` is **on by default** — provable overflow is a
  hard error, not a warning.

---

## Reference

- The Rust Book, Ch. 3.2 — Data Types (scalar types, integer sizes, overflow,
  checked/wrapping/saturating): https://doc.rust-lang.org/book/ch03-02-data-types.html
- `std::mem::size_of`: https://doc.rust-lang.org/std/mem/fn.size_of.html
- `std::hint::black_box`: https://doc.rust-lang.org/std/hint/fn.black_box.html
- Book Ch 4.1 What Is Ownership? (stack/heap, String): https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
- Into / From traits: https://doc.rust-lang.org/std/convert/trait.Into.html , https://doc.rust-lang.org/std/convert/trait.From.html

---

## 2-minute self-quiz

1. How many bytes is a `u16`? An `i64`? (bits / 8)
2. Why is `char` 4 bytes but `bool` only 1?
3. Name the two overflow mechanisms and when each fires. Which one differs
   between debug and release?
4. What does `7 / 2` print? `7.0 / 2.0`?
5. Which method returns an `Option` when addition might overflow?
6. What decides stack vs heap for a value?
7. What 3 fields does a String hold on the stack? Where do the characters live?
8. &str vs String: which owns its bytes and frees them?
9. &T vs &mut T - how many of each can you have at once?
10. Why must "Anoop" -> String be written explicitly?
11. len vs capacity - which can be larger, and why?
12. Complete: `str` is to `String` as `[T; N]` is to ____ . (answer in Check 3)

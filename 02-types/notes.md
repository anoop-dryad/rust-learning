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

---

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

---

## 2-minute self-quiz

1. How many bytes is a `u16`? An `i64`? (bits / 8)
2. Why is `char` 4 bytes but `bool` only 1?
3. Name the two overflow mechanisms and when each fires. Which one differs
   between debug and release?
4. What does `7 / 2` print? `7.0 / 2.0`?
5. Which method returns an `Option` when addition might overflow?

# 01 — Variables, Mutability, Shadowing

> Layer 1. Status: **Done**.
> Quick-reference for everything covered, including the doubts that came up and
> how they were resolved (that's where the real clarity was).

---

## TL;DR (read this first)

- Rust is **compiled first, then run**. The whole program is type-checked before
  a binary exists. (Python is *interpreted* — errors surface line-by-line at
  runtime, possibly weeks later in prod.)
- Variables are **immutable by default**. Opt into mutation with `mut`.
- `mut` = change a value **in place**, but the **type is fixed for life**.
- Shadowing (`let` again with the same name) = a **brand-new variable** reusing
  the name. Can change the **type**. Does **not** overwrite the old one.
- **A name is not a variable.** The name is a label; `let` mints a new variable
  in memory; the label points at one variable at a time.
- A variable is **dropped when it goes out of scope** (the closing `}`). This is
  how Rust frees memory with no garbage collector.

---

## Core rules + examples

### 1. Immutable by default

```rust
let x = 5;
x = 6; // ERROR[E0384]: cannot assign twice to immutable variable `x`
```

Opt in:

```rust
let mut x = 5;
x = 6; // fine
```

Why: most bugs come from state changing unexpectedly. Making mutability explicit
means `let x` is a promise nothing will touch `x`. Default to immutable — add
`mut` only when the compiler/logic demands it. Clippy warns on unused `mut`.

### 2. `mut` vs shadowing

| | keyword | same variable? | can change type? | use when |
|---|---|---|---|---|
| **mut** | `x = ...` (no `let`) | yes, one binding | **no** | genuinely updating one thing (counter, accumulator) |
| **shadowing** | `let x = ...` | no — **new** variable | **yes** | transforming a value into a new form/type |

`mut` cannot change type (this errors):

```rust
let mut x = 5;
x = "hello"; // ERROR[E0308]: mismatched types — expected integer, found `&str`
```

Shadowing can (this is fine):

```rust
let x = 5;
let x = "hello"; // new variable, different type, same name — legal
```

**The tell:** `let` present = new variable (shadowing). `let` absent = touching
the existing variable (must keep its type).

### 3. Shadowing's real purpose — type transformation

The motivating use (not the toy `x + 1`): read raw text, turn it into the number
you need, keep one meaningful name.

```rust
let input = "42";                 // &str
let input: i32 = input            // i32 — same name, new type
    .trim()
    .parse()
    .expect("not a valid number");
println!("{}", input + 1);        // 43
```

> Caveat: `.parse()` returns a `Result` and `.expect()` crashes on bad input.
> That's a rough placeholder — proper handling comes in the **error handling
> (Result/Option)** layer. The `: i32` annotation tells `parse` what to produce
> (types/inference — Layer 2).

### 4. Scope + drop

```rust
let x = 5;          // A
let x = x + 1;      // B (6)
{
    let x = x * 2;  // C (12) — inner scope only
    println!("{x}"); // 12  → sees C
}                    // C dropped here
println!("{x}");     // 6   → sees B again (C never existed out here)
```

Prediction test result: **12 then 6**. The outer line can't print 12 because C
was dropped at `}` — the name falls back to B, which was never touched.
This drop-at-scope-end is the foundation of ownership (Layer 3).

---

## Sticking points (my doubts → the correction)

**Q: "Is shadowing a variable referencing two values?"**
No. No variable ever holds two values. Each `let` creates a *separate* variable
in memory. Shadowing re-points the *name* to a new variable. Two variables exist;
the name refers to one at a time.

**Q: "But `x` is the name of the variable, so both are the same — like Java?"**
This is the key thing to unlearn. In Java a name and its variable are welded:
one name = one slot = one type for the whole scope. **Rust separates them.** That
separation is *why* `let x = 5; let x = "hello";` is legal in Rust (new variable,
new type) but impossible in Java. The keyword `let` is the giveaway — every `let`
mints a new variable; the matching name is just label reuse. The old variable
still sits in memory, untouched.

Mental picture:
```
Name `x` ───► [var B: 6]        [var A: 5]   (A shadowed but still in memory)
```

**Q: earlier attempt used both `let mut count` AND `let count = count + 1`.**
That's the "common mistake": reflexive `mut`. The `mut` was dead because the next
`let` shadowed it away before any mutation. Compiler warns:
`warning: variable does not need to be mutable` (`unused_mut`). Lesson: read the
warnings — they're free coaching, no Python equivalent.

---

## Tooling learned

```
cargo run -p variables                 # run package (errors if >1 binary — ambiguous)
cargo run -p variables --bin shadowing # run a specific binary in src/bin/
cargo check                            # type-check whole workspace, no run (fast)
cargo clippy                           # idiomatic-Rust lints
```

- Files in `src/bin/*.rs` are each their own binary (own `fn main()`). Great for
  a learning repo: one experiment per file, runnable forever, named with `--bin`.
- A package needs **at least one target** (a binary or `src/lib.rs`) or it builds
  nothing. Don't delete `src/main.rs` unless a `src/bin/` file already exists.
- Optional: `default-run = "shadowing"` in `[package]` makes bare
  `cargo run -p variables` pick that binary.
- Function definition order in a file doesn't matter — the compiler reads the
  whole file first (compile-first model again).

---

## Reference

- The Rust Book, Ch. 3.1 — Variables and Mutability (incl. Shadowing):
  https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

*(Links unverified this session — no live search available. If it 404s, start at
https://doc.rust-lang.org/book/ and navigate to Ch. 3.1.)*

---

## 2-minute self-quiz

1. Why compile-before-run, and how does that differ from Python?
2. `mut` vs shadowing — which can change a variable's *type*, and why?
3. Name vs variable: why is `let x = 5; let x = "hello";` legal in Rust but not Java?
4. In the scope example, why does the outer `println!` print 6, not 12?
5. What's the idiomatic default — `let` or `let mut`?

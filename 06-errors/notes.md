# 06 — Error Handling

> Layer 6. Status: **DONE** (panic vs Result, ?, Box<dyn Error>, custom errors + From).
> Formalizes what textstat Ch.2 used early. Feeds textstat Ch.3 (custom AppError).

---

## TL;DR

- Two categories: **unrecoverable -> panic!** (bug), **recoverable -> Result<T,E>**
  (expected failure, a VALUE in the return type).
- **`?`**: on `Ok(v)` -> unwrap & continue; on `Err(e)` -> `return Err(e)` from the
  function, CONVERTING e via the **From trait** to the fn's error type.
- `?` only works in a fn returning **Result** (or Option) — it needs somewhere to
  return the Err to (error[E0277] otherwise).
- **Box<dyn Error>** = "any error" catch-all (quick app code).
- **Custom error enum + From** = precise, matchable errors (library/production).
- **unwrap()/expect()** panic on None/Err -> a smell in production.

=====================================================================
# CHECK 1 — panic vs Result, and ?
=====================================================================

## Two error categories
- **panic!** — unrecoverable; stops the program. You rarely write it; you've
  triggered it (overflow, out-of-bounds).
- **Result<T, E>** — recoverable; returned as a value. The fn signature SAYS it
  can fail. Errors are values, not control flow (unlike Python exceptions which
  propagate invisibly).

## unwrap / expect (the panic cousins)
```rust
x.unwrap()           // inner value, or PANICS on None/Err
x.expect("message")  // same, but panics with YOUR message (prefer over unwrap)
```
Both are a SMELL in real code — each is a potential crash. Fine for
examples/tests/prototypes; prefer `?` or explicit handling in production.

## The ? operator
```rust
let contents = fs::read_to_string(path)?;   // Ok -> unwrap; Err -> return it
// equivalent to:
let contents = match fs::read_to_string(path) {
    Ok(v) => v,
    Err(e) => return Err(e.into()),          // .into() = the From conversion
};
```
- On Ok: unwrap and continue. On Err: convert (via From) and return from the fn.
- Only usable in a fn returning Result/Option. In a `()` fn -> error[E0277]:
  "the `?` operator can only be used in a function that returns Result or Option".

## Box<dyn Error> — the catch-all
```rust
fn read_number(path: &str) -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?; // io::Error    -> Box<dyn Error>
    let n: i32 = contents.trim().parse()?;    // ParseIntError-> Box<dyn Error>
    Ok(n)
}
```
TWO different error types funnel through ONE Err arm because each converts into
Box<dyn Error>. The `: i32` annotation tells parse what to parse into.
Best app-level pattern: `-> Result<T, Box<dyn Error>>` + `?` everywhere + one
match in main.

=====================================================================
# CHECK 2 — Custom error types
=====================================================================

Box<dyn Error> loses specificity: caller gets "some error", can't react to WHICH.
A custom error enum fixes that.

## 1. Define the enum (just Layer 4 enums)
```rust
#[derive(Debug)]
enum AppError { Io(String), Parse(String), Empty }
```

## 2. Implement From so ? converts into it
```rust
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self { AppError::Io(e.to_string()) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e.to_string()) }
}
```
`?` calls From — once these exist, `?` on io::Error / ParseIntError produces AppError.

## 3. Same body; match each variant distinctly
```rust
fn read_number(path: &str) -> Result<i32, AppError> {
    let contents = fs::read_to_string(path)?;  // -> AppError (From)
    let t = contents.trim();
    if t.is_empty() { return Err(AppError::Empty); } // manual: OUR validation
    let n: i32 = t.parse()?;                    // -> AppError (From)
    Ok(n)
}

match read_number(path) {
    Ok(n) => ...,
    Err(AppError::Io(s))    => ...,  // react to file errors
    Err(AppError::Parse(s)) => ...,  // react to parse errors
    Err(AppError::Empty)    => ...,  // react to empty
}
```

## TWO ways a variant is created
- **Automatically** via `?` + `From`  (Io, Parse — converting OTHERS' errors).
- **Manually** via `return Err(..)`   (Empty — YOUR OWN validation failure).
Not every variant needs a From. (Until something constructs a variant, you'd get
a dead_code warning for it.)

## Real-world shortcut: thiserror / anyhow (external crates)
```rust
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("file error: {0}")]
    Io(#[from] std::io::Error),       // #[from] GENERATES the From impl
    #[error("invalid number: {0}")]
    Parse(#[from] std::num::ParseIntError),
}
```
- `thiserror` -> precise LIBRARY error types (generates From + Display).
- `anyhow` / Box<dyn Error> -> quick APP code (less precise, less boilerplate).
- These are DEPENDENCIES (added to Cargo.toml via `cargo add`). We hand-wrote
  From first so we know what thiserror generates — don't cargo-cult it.

---

## Sticking points (my doubts -> corrections)

- **"? handles different error types using Box<dyn Error>"** — incomplete. `?`
  uses the **From trait** to convert into whatever error type the fn returns.
  Box<dyn Error> is just one target that works for everything; a custom type
  works too if you impl From.
- **{e:?} vs {e} for errors** — use `{}` (Display) for clean user messages; `{:?}`
  (Debug) gives verbose structure (Os { code, kind, ... }). Errors are designed
  to have a nice Display. And send errors to stderr via `eprintln!`.
- **dead_code on an unused variant** — Empty needed `#[allow(dead_code)]` until
  something constructed it. The fix was to actually produce it (return Err(Empty)),
  not silence the warning.
- **parse needs a target type** — `let n: i32 = ...parse()?` (or `parse::<i32>()`),
  or parse can't know what to produce.

---

## Tooling learned
```
cargo run -p errors --bin propagation
cargo run -p errors --bin custom_errors
cargo add thiserror      # (later) adds a dependency to Cargo.toml
```
- error[E0277] = `?` in a non-Result/Option fn.
- `cargo add <crate>` is how you add dependencies (we'll use it for thiserror/anyhow).

---

## Reference (unverified this session - no live search)
- Book 9 Error Handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- Book 9.2 Recoverable Errors / ?: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
- std From: https://doc.rust-lang.org/std/convert/trait.From.html
- thiserror (crate, check current version on docs.rs): https://docs.rs/thiserror
- anyhow (crate): https://docs.rs/anyhow
If any 404: start at https://doc.rust-lang.org/book/ or /std/ and navigate.

---

## 2-minute self-quiz
1. Rust's two error categories, and when each applies?
2. What does `?` do on Ok vs Err (step by step)?
3. Why can `?` only be used in a Result/Option-returning fn?
4. Which TRAIT does `?` use to convert error types?
5. Box<dyn Error> vs a custom error enum — when each?
6. Two ways a custom error variant gets created?
7. Why is .unwrap() a production smell?
8. What does thiserror's #[from] generate?

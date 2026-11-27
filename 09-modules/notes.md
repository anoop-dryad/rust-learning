# 09 — Modules & Project Structure

> Layer 9. Status: **DONE** (modules in a file, splitting across files, visibility).
> Mechanical, not conceptually hard. Directly restructures textstat into lib.rs + modules.

---

## TL;DR

- `mod foo { ... }` = module inline; `mod foo;` (semicolon) = "load module from file foo.rs".
- Access items with `::` paths: `module::item`. `use path;` brings it into scope (like Python import).
- **Items are PRIVATE by default.** `pub` to expose. (Opposite of Python, which defaults public.)
- A `pub struct` does NOT make its fields pub — each field needs its own `pub`.
- `pub(crate)` = public within this crate, hidden from external crates. For internal helpers.
- `pub use path;` = bring into scope AND re-export — present a clean API hiding internal structure.
- File tree mirrors module tree. Submodules live in a folder named after the parent.

=====================================================================
# CHECK 1 — Modules within a file
=====================================================================

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }  // pub = visible outside
    fn helper() {}                                // private to this module
}

use math::add;        // bring into scope (like `from math import add`)
fn main() {
    let x = math::add(2, 3);  // full path
    let y = add(2, 3);        // or via `use`
}
```
- `mod name { }` creates a namespace. Access with `name::item`.
- **Private by default**; `pub` opts in. Anything not pub is invisible outside its module.
- Within a module, everything sees everything (a pub fn can call a private helper
  in the same module). Privacy is about crossing OUT of the module boundary.
- Nested: `mod outer { pub mod inner { pub fn hi() {} } }` -> `outer::inner::hi()`.
  Each level you cross must be reachable (inner & its items need pub).
- `pub` is about crossing boundaries, not a blanket switch — a sibling at the same
  level (e.g. main and a non-pub mod at crate root) can see each other already.

=====================================================================
# CHECK 2 — Splitting modules across files
=====================================================================

**`mod foo;` (semicolon, no body) = "the contents of module foo are in another file".**

```rust
// src/main.rs
mod calculator;                 // loads src/calculator.rs
fn main() { calculator::add(2, 3); }

// src/calculator.rs
pub mod advanced;               // loads src/calculator/advanced.rs (submodule -> folder!)
pub fn add(a: i32, b: i32) -> i32 { a + b }

// src/calculator/advanced.rs
pub fn square(a: i32) -> i32 { a * a }
```

File placement (the thing people get wrong):
```
src/
├── main.rs              # mod calculator;
├── calculator.rs        # pub mod advanced;
└── calculator/          # a submodule's file goes in a FOLDER named after the parent
    └── advanced.rs
```
- A top-level module `foo` -> `src/foo.rs`.
- A SUBMODULE of `foo` -> `src/foo/submodule.rs` (folder named after parent).
- Wrong placement -> error[E0583]: file not found for module `x`
  (the compiler tells you the exact path it wants).
- Module path mirrors the file tree: `calculator::advanced::square`.

**Library vs binary split (textstat's restructure):**
```
src/
├── main.rs     # thin: parse args, call into the library
├── lib.rs      # library root: declares modules, the public API
├── stats.rs    # logic (TextStats + analysis)
└── errors.rs   # custom error types
```
Logic in the library (testable, reusable); main.rs just wires it up. Production pattern.

=====================================================================
# CHECK 3 — Visibility & use nuances
=====================================================================

## pub has levels
```rust
pub fn a() {}          // anyone, incl external crates (your public API)
pub(crate) fn b() {}   // anywhere in THIS crate, NOT exported  <- the one you'll use most
pub(super) fn c() {}   // the parent module only
fn d() {}              // this module + descendants only (default)
```
Default to private; `pub` for real public API; `pub(crate)` for cross-module internal helpers.
Over-pub-ing leaks internals and makes refactoring harder (pub = a promise others may depend on).

## pub struct != pub fields
```rust
pub struct Book {
    pub title: String,   // this field is public
    author: String,      // PRIVATE even though the struct is pub
}
```
Expose the type, control which fields are directly accessible. Idiomatic: keep fields
private, add getters:
```rust
impl Book {
    pub fn author(&self) -> &str { &self.author }  // getter: no `get_` prefix (Rust convention)
}
```
(book.title works; book.author -> E0616 private; book.author() works.)

## use conveniences
```rust
use std::collections::{HashMap, HashSet};  // grouping
use std::fmt::Result as FmtResult;          // rename with `as`
use std::collections::*;                     // glob (use sparingly)
```

## pub use — re-export (the library API pattern)
```rust
mod stats;
pub use stats::TextStats;   // bring into scope AND re-expose at THIS level
// consumers write `textstat::TextStats`, NOT `textstat::stats::TextStats`
```
- `use` = bring into scope (stays private to this module).
- `pub use` = bring into scope AND re-export, so types appear at a cleaner path than
  where they're defined. Hides internal structure; presents a flat public API.

## The design question
Setting visibility = drawing the boundary between PUBLIC API (what consumers use) and
INTERNAL (what you can refactor freely). Tight boundary -> change internals without
breaking anyone. That's the whole point of modules.

---

## Sticking points (my doubts -> corrections)

- **#[warn(unused)] doesn't silence anything** — `warn` turns a lint ON (usually
  already the default). To silence use `#[allow(...)]`; to make it a hard error `#[deny(...)]`.
- **"unknown value could reach a match"** — No. A typed enum value is ALWAYS one of its
  variants; you can't create an "unknown" one. Putting a String where a BookLocation is
  expected is a COMPILE error (E0308), not a runtime surprise. The type system forbids it.
- **`_` arm unreachable** — if you listed every variant, there's no "everything else"
  left, so `_` is dead. On enums YOU own, omit `_` so adding a variant forces a compile
  error (safety net) instead of silently falling into `_`.
- **pub struct, private field** — book.author -> E0616. Each field needs its own pub;
  prefer a getter (named `author()`, not `get_author()`).
- **&i32 params** — for Copy types (i32) take by value, not &ref; reserve & for
  expensive-to-copy types (String, big structs).
- **&String::from("x")** — a literal is already &str; just pass "x" to a &str param.

---

## Tooling learned
```
cargo run -p modules
```
- error[E0603] = item is private (needs pub).
- error[E0616] = struct field is private.
- error[E0583] = file not found for module (wrong file placement for a submodule).
- Module path mirrors file tree; submodule files go in a folder named after the parent.

---

## Reference (unverified this session - no live search)
- Book 7.1 Packages & Crates: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
- Book 7.2 Defining Modules: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
- Book 7.3 Paths & pub: https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
- Book 7.4 use: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
- Book 7.5 Separating into files: https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
If any 404: start at https://doc.rust-lang.org/book/ and navigate to Ch. 7.

---

## 2-minute self-quiz
1. What does `mod foo;` (semicolon) tell the compiler?
2. Default visibility of a module's items? How to change it?
3. Does `pub struct` make its fields public?
4. What does `pub(crate)` mean, and when use it over `pub`?
5. What does `pub use` do that plain `use` doesn't?
6. Where does a submodule's file go (relative to its parent)?
7. How is Rust's module privacy different from Python's?

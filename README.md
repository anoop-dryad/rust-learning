# Rust-Learning

A cargo **workspace** for a focused Rust learning. Each concept we cover
lives in its own numbered package folder, so every snippet is runnable,
lintable, and testable on its own — while `cargo` treats the whole repo as one
project.

## Playground
Want to try code quickly without setting anything up? Use the
[online Rust Playground](https://play.rust-lang.org/) — no local
project needed.

## Prerequisites

- **Rust toolchain** via [rustup](https://rustup.rs) — gives you `rustc`,
  `cargo`, and `clippy`.
- Check it's there: `cargo --version` and `rustc --version`.

## Layout

```
rust-learning/
├── Cargo.toml          # workspace root — lists member packages (no code here)
├── README.md           # this file
├── 01-variables/       # one folder per concept
│   ├── Cargo.toml      #   its own package manifest
│   ├── notes.md        #   recap answers + what clicked, in my own words
│   └── src/    
│       └── bin /                   #  binary module
│            ├── shadowing.rs       #  runnable examples for this concept
│            └── next_main.rs       #  runnable examples for this concept
└── 02-.../                         #  added as we progress
```

The root `Cargo.toml` is just a container:

```toml
[workspace]
resolver = "2"
members = ["01-variables"]
```

## Conventions

- **Folder name is numbered** (`01-variables`, `02-...`) so concepts stay in
  learning order.
- **Package name is a plain identifier** (`variables`) — cargo requires crate
  names to be valid Rust identifiers, which can't start with a digit. So the
  folder can be `01-variables` while the package inside is `variables`. That's
  why `cargo new` below uses `--name`.
- Runnable code lives under `src/` (cargo only builds conventional locations —
  loose `.rs` files elsewhere are invisible to it).
- `notes.md` per folder holds recap answers and gotchas — the act of writing
  them is where it sticks.

## Adding a new concept package

From the repo root:

```
cargo new 02-ownership --name ownership
```

Then add it to the workspace members in the root `Cargo.toml`:

```toml
members = ["01-variables", "02-ownership"]
```

## Common commands

Run from the repo root:

```
cargo run -p variables      # run one package's main.rs
cargo check                 # type-check the whole workspace, no run (fast)
cargo clippy                # lint every package (idiomatic-Rust warnings)
cargo test                  # run tests across every package
cargo build                 # compile everything
```

`-p <package name>` selects a single member; omit it and most commands act on
the whole workspace.

`--bin <binary name>` selects a single member; omit it and most commands act on
the whole workspace.


## How To Run

```sh
cargo run -p <package-name> --bin <binary-name>
```

Example:

```sh
cargo run -p variables --bin shadowing
```

## Progress index

| Folder          | Concept                        | Status      |
|-----------------|--------------------------------|-------------|
| `01-variables`  | Variables, mutability, shadowing | In progress |



## Notes

- `target/` (build output) is generated and must stay out of git.
- The `edition` in each package's `Cargo.toml` (e.g. `2024`) is a
  language-evolution bucket, not a version — leave whatever `cargo new` writes.

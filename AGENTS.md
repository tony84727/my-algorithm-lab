# Repository Guidelines

## Project Structure & Module Organization

The main crate lives in `src/`. LeetCode solutions are organized under
`src/leetcode/algorithm_<id>/`, usually with `mod.rs`, optional alternative
implementations such as `brute.rs` or `dp.rs`, and a problem `README.md`.
Reusable textbook algorithms belong in `src/classic/`, while matrix and
machine-learning experiments live in `src/ml/`. Criterion benchmarks are in
`benches/`.

The `c/` tree is a separate GNU Autotools project with implementations and
`*_test.c` files. Python experiments live in `python/`; grouped solutions keep
tests beside their implementations, for example `python/leetcode_3459/`.

## Build, Test, and Development Commands

- `cargo build` compiles the Rust crate and the `create` binary.
- `cargo test` runs all Rust unit and parameterized tests.
- `cargo fmt --all -- --check` verifies standard Rust formatting.
- `cargo clippy --all-targets --all-features -- -D warnings` matches the CI lint
  gate.
- `cargo bench` runs the Criterion suites in `benches/`.
- `cd c && autoreconf -i && ./configure && make check` configures, builds, and
  tests the C project.
- `cd python && uv sync` creates the locked Python 3.13 environment.
- `cd python && uv run pytest` runs all Python tests. From `python/`, run a
  specific test with `uv run pytest leetcode_3459/test_brute.py`.

## Coding Style & Naming Conventions

Use rustfmt defaults (four-space indentation) and keep Clippy warning-free.
Name Rust modules and functions in `snake_case`, types in `UpperCamelCase`, and
LeetCode directories `algorithm_<number>`. Put distinct approaches in
descriptively named modules such as `brute`, `memoized`, or `sorting`. Follow
the existing C convention of lowercase filenames and `*_test.c` test files.

## Testing Guidelines

Keep Rust tests close to the implementation in `#[cfg(test)]` modules. Use
`#[test]` for focused cases and `test_case` for input tables; label cases
`"example 1"` or `"case 1"` consistently. Add regression cases for edge
conditions and run the focused test before the full suite. No coverage
percentage is mandated, but every new algorithm or bug fix should include
representative tests.

## Commit & Pull Request Guidelines

Recent commits use concise subjects such as `leetcode/1260: rotate_right` and
`fix clippy suggestions`. Prefer `leetcode/<id>: <approach>` for solution work
and an imperative summary for maintenance. Pull requests should explain the
algorithm and complexity, list validation commands, and link the relevant
problem or issue. Include screenshots only for changes with visual output, and
keep unrelated solutions in separate pull requests.

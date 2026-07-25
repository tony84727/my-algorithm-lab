---
name: generate-leetcode-example-tests
description: Transcribe user-provided LeetCode examples into repository-style Rust tests for an existing WIP solution.
---

# Generate LeetCode Example Tests

Turn the user's examples into a red-or-green harness around the repository's
WIP solution. Transcribe the examples; do not solve the algorithm.

## 1. Verify the target

Extract the problem number and any explicit target path from the request. Inspect
the target directory, its `README.md`, `mod.rs`, and its declaration in
`src/leetcode/mod.rs`.

The problem number, explicit `src/leetcode/algorithm_<id>/` path, and README must
identify the same problem. If they disagree, stop before editing and ask the
user to correct the mismatch. The target is verified only when every available
identifier agrees and the WIP directory exists.

## 2. Establish the Rust interface

Read the existing `Solution` implementation and use its public method signature
as the source of truth.

If `mod.rs` is empty, require the user to provide the exact Rust method name,
parameter types, and return type. Once provided, create:

```rust
pub struct Solution;

impl Solution {
    pub fn method(arguments: Types) -> ReturnType {
        todo!()
    }
}
```

Preserve existing implementation code and existing tests. Ensure the target has
a `pub mod algorithm_<id>;` declaration in `src/leetcode/mod.rs`, adding it only
when absent. The interface is established when the tests can call one exact,
known `Solution` method without guessing its public API.

## 3. Transcribe every example

Convert each user-provided `Input` and `Output` into exactly one test case,
preserving order and naming cases `"example 1"`, `"example 2"`, and so on.
Generate no inferred edge cases.

Match the nearest analogous tests in `src/leetcode/`:

- For a pure return value, prefer one parameterized function with
  `test_case::test_case` and `#[test_case(inputs => output; "example N")]`.
- Accept string literals as `&str` in the test wrapper when that keeps cases
  concise, then convert them to the method's required `String` type at the call.
- Use `crate::vecvec` for nested vectors when nearby matrix tests use it and it
  improves readability.
- For an in-place method, pass the expected value separately, call the method,
  and compare the mutated input with `assert_eq!`.
- For a stateful API or an example containing a sequence of calls, use one
  focused `#[test]` function per example.

Keep tests in `#[cfg(test)] mod tests`, import `super::*`, and add only helper
imports used by the chosen form. This step is complete only when every pasted
example appears exactly once and its Rust values preserve the provided input
and output.

## 4. Validate the harness

Keep edits scoped to the target algorithm module, a missing module declaration,
and formatting required for those edits. Run:

```bash
rustfmt --edition 2021 --check src/leetcode/algorithm_<id>/mod.rs
cargo test leetcode::algorithm_<id>::tests --no-run
cargo test leetcode::algorithm_<id>::tests
git diff --check
```

If formatting fails for the edited module, format that file and rerun the check.
For an existing implementation, report whether the example tests pass or expose
solution failures without changing the algorithm. For a generated `todo!()`
stub, the focused tests must compile and then fail at the stub rather than from
test syntax, types, or fixture conversion.

Finish by reporting the target file, number of examples transcribed, the exact
validation commands, and whether the harness is green or red. The skill is
complete only when compilation succeeds and every runtime result is accounted
for.

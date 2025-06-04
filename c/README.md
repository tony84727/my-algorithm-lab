# C Code

This directory contains C implementations of various algorithms and data structures. The project is built using the GNU Autotools suite.

## Project layout
- `configure.ac` and `Makefile.am` – top level build configuration.
- `test_helper/` – small library used by the tests.
- `leetcode/` – algorithm implementations and their unit tests.
- `binary_tree/` – binary tree implementation with accompanying tests.

## Building
1. Ensure `autoconf`, `automake` and a C compiler (e.g. `gcc`) are installed.
2. From this folder run:
   ```sh
   autoreconf -i
   ./configure
   make
   ```
   The commands generate the `configure` script, create Makefiles and then compile all sources.

## Running tests
Run `make check` to build and execute all test binaries listed in `Makefile.am`.  A summary of the test results is printed at the end of the run.

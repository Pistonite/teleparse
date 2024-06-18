# Inferring the regex pattern from terminals

If a token type should only produce terminals with
known literal values (for example, a set of keywords),
`regex` can be omitted since it can be inferred from
the terminals.

```rust
TXTPP#include ../../../tests/expand/pizza.rs
```
This is the Don't-Repeat-Yourself (DRY) principle.
In fact, `derive_lexicon` enforces it:
```rust,compile_fail
TXTPP#include ../../../tests/ui/lex_redundant_regex.rs
```
```console
TXTPP#include ../../../tests/ui/lex_redundant_regex.stderr
```

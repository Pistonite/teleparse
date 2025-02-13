# Lexer Validation

There are additional restrictions that are enforced at 
compile-time apart from the ones [covered previously](./regex_terminals_g.md)

## Duplicated Terminals

Having duplicated terminals is likely a mistake, because
those terminals are interchangeable in the syntax tree.
Likewise, you cannot have 2 terminals with no literals.
```rust,compile_fail
TXTPP#include ../../../teleparse/tests/ui/lex_no_dupe_literal.rs
```
```console
TXTPP#include ../../../teleparse/tests/ui/lex_no_dupe_literal.stderr
```

## Regex Features
Teleparse uses the [`logos`](https://docs.rc/logos) crate
for the lexer, which combines all the rules into a single
state machine for performance. Logos also imposes additional
restrictions on regex features that requires backtracking.
Please refer to their documentation for more information.

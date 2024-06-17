# Terminals

*Terminal*s are leaf nodes in the syntax tree. For example, in the "textbook" grammar:
```text
E  => T E'
E' => + T E' | ε
T  => F T'
T' => * F T' | ε
F  => ( E ) | id
```
The syntax `... => ...` is called a *production*. The left-hand side is the symbol to produce,
and the right-hand side is what can produce it. The left-hand symbol is also called a non-terminal,
and every symbol on the right-hand side that does not appear on the left-hand side is a terminal.

In this example, `+`, `*`, `(`, `)`, and `id` are terminals. The other symbols are non-terminals.

The terminal structs derived by `#[derive_lexicon]` are the building blocks
to define the syntax tree. For example:
```rust
TXTPP#include ../../../tests/book_terminal.rs
```

Here:
- We are generating a `Id` terminal to parse a token of type `Ident`, matching the regex `r"\w+"`.
- We are creating a non-terminal `ThreeIdents` with the production `ThreeIdents => Id Id Id`.
  - More about `#[derive_syntax]` in later sections
- We are also using the `terminal_parse` attribute to derive a `parse` method for the terminals
  for testing purposes.

# Lexical Analysis
From _Compilers: Principles, Techniques, and Tools_ by Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman:
> The lexical analyzer is the first phase of a compiler. Its main task is to read
> the input characters and produce as output a sequence of tokens that the parser
> uses for syntax analysis.

The `[derive_lexicon]` macro is used to declare token types and lexical analyzer rules (the lexer rules)
using an enum. It was already showcased in the beginning of the book with the full example. Let's take a closer look
here.

```rust
TXTPP#include ../../../tests/expand/lexicon_example.rs
```
Attributes on the enum:
- `#[derive_lexicon]` is the entry point, and processes the other `teleparse` attributes.
- `#[teleparse(ignore(...))]` defines the patterns that the lexer should skip between tokens.

Attributes on the variants:
- `#[teleparse(terminal(...))]` generates structs that can be used to put in the syntax tree.
  - The example generates `Integer`, `OpAdd`, `OpMul`, `ParenOpen` and `ParenClose` structs.
  - Some have a specific literal value to match. For example, `OpAdd` will only match a token of type `Op` that is the `+` character.
- `#[teleparse(regex(...))]` defines the pattern to match for the token type.


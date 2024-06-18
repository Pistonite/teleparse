# Teleparse
Teleparse is a powerful library designed for creating parsers for custom languages. This book serves as a comprehensive tutorial for using the library and provides a brief introduction to the essential concepts of lexical analysis and syntax analysis, which play crucial roles in compiler design. For more technical details, please refer to the [documentation on `docs.rs`](https://docs.rs/teleparse).

## Features

- Utilizes Rust's powerful proc-macro system to simplify language declaration, ensuring excellent synergy between data structures and the language being parsed. There's no need to learn a DSL for parser declaration.
- Includes an LL(1) top-down, non-recursive descent parser. The grammar can be verified as LL(1) through generated tests.
- Offers utilities for parsing common language constructs, such as optional symbols and symbol-delimited lists, with built-in error detection and recovery.
- No separate build tool needed to generate parser code.

## Credits
- The "Dragon Book" _Compilers: Principles, Techniques, and Tools_ by Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman, used as reference for implementation.
- The lexer implementation is backed by the *ridiculously fast* [logos](https://github.com/maciejhirsz/logos) library

## Install
Add `teleparse` as a dependency in your project:
```console
$ cargo add teleparse
```

It is recommended to import the `teleparse::prelude` module in
your module that interfaces with `teleparse` to bring all required traits, macros
and utility types into scope. You will see almost all the examples do this.

```rust
use teleparse::prelude::*;
```

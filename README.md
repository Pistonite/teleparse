# teleparse

working in progress - Proc-macro powered LL(1) parsing library

This library is comparable to `serde` for parsing - All you need is define the syntax
as data types and call `parse()` on the root type.

Features:
- Syntax tree defined by macro attributes on structs and enums - no separate grammar file
- Proc-macro powered - no separate build step to generate parser code
- Provide a `#[test]` to ensure the grammar is LL(1), or fail at runtime
- Utils for parsing components into primitives like tuples, options, and delimited lists

Credits:
- The lexer implementation is backed by the *ridiculously fast* [logos](https://github.com/maciejhirsz/logos) library
- The "Dragon Book" _Compilers: Principles, Techniques, and Tools_ by Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman:

Progress:
- [x] Lexer/Tokens
  - [ ] Macro for terminals
- [x] Parser
  - [x] LL(1) stuff
  - [x] Macros
  - [x] Semantic Tokens (token type applied later by the parser)
    - [ ] Tests
    - [ ] Documentation
  - [ ] Tests
  - [ ] Documentation
  - [ ] Hooks
- [ ] Utils `tp`
  - [x] Tuples
  - [x] boxed
  - [x] option-based `tp::Option<T>` and `tp::Exists<T>`
  - [x] string-based `tp::Quote<X: From<String>, T>` `tp::Parse<X: FromStr>` (alias `tp::String`)
    - [ ] Documentation
  - [ ] iter-based `tp::Star<V: FromIterator<T>, T>` `tp::Plus<V: FromIterator<T>, T>` (alias `tp::Vec`, `tp::Nev`, `tp::VecDeque`, `tp::NevDeque`)
  - [ ] delimited `tp::Split<T, P>`, `tp::Pun<T, P>` (pun has optional trailing and split forbids trailing)
  - [ ] conversion `tp::Into<X, T: Into<X>>`
  - [ ] Documentation
- [ ] mdBook
  - [ ] derive_lexicon
  - [ ] derive_syntax
    - [ ] semantic tokens
  - [ ] using `tp`
  - [ ] hooks
  - [ ] using parser data

Here is the "textbook grammar" implemented in a few minutes (full version at `tests/first_follow.rs`)
```
E -> T E'
E' -> + T E' | ε
T -> F T'
T' -> * F T' | ε
F -> ( E ) | id
```
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"\s+"#))]
pub enum TokenType {
    #[teleparse(regex(r#"\w+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(
        OpAdd = "+",
        OpMul = "*",
    ))]
    Op,
    /// Parentheses
    #[teleparse(terminal(
        ParenOpen = "(",
        ParenClose = ")"
    ))]
    Paren,
}

#[derive_syntax]
#[teleparse(root)]
struct E { 
    first: T,
    rest: Eprime,
}

// Eplus has to be a separate struct because it contains Eprime.
// Eprime(tp::Option<(OpAdd, T, Box<Eprime>)>) 
// will cause a loop in Eprime -> tp::Option -> Eprime when trying
// to determine if traits are satisfied
#[derive_syntax]
struct Eprime(tp::Option<Eplus>);

#[derive_syntax]
struct Eplus {
    op: OpAdd,
    t: T,
    rest: Box<Eprime>,
}

#[derive_syntax]
struct T {
    first: F,
    rest: Tprime,
}

#[derive_syntax]
struct Tprime(tp::Option<Tstar>);

#[derive_syntax]
struct Tstar {
    op: OpMul,
    f: F,
    rest: Box<Tprime>,
}

#[derive_syntax]
enum F {
    Paren((ParenOpen, Box<E>, ParenClose)),
    Ident
}

let source = "(a+b)*(c+d)";
let t = E::parse(source).unwrap().unwrap();
```

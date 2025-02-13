# Textbook Example

In many tutorials that talk about compiler design, the example used is parsing a math expressions consist of variables (identifiers), `+`, `*`, and parenthese (`(` and `)`). For example, the following are valid expressions:
```text
a + b * c
(a + b) * c
(a + b) * (c + d)
```

The grammar can be described verbally as follows:
- An expression (`E`) is a list of terms (`T`) separated by `+`.
- A term (`T`) is a list of factors (`F`) separated by `*`.
- A factor (`F`) is either an identifier or an expression enclosed in parentheses.

You may also seen the following used to describe the above.
(It's called Backus-Naur Form or **BNF**. It's ok if you don't understand this for now. 
Some places might have `::=` instead of `=>`.)
```text
E  => T E'
E' => + T E' | ε
T  => F T'
T' => * F T' | ε
F  => ( E ) | id
```

This can *almost* be translated directly into Rust code with `teleparse`.
Some helper structs are needed to avoid loops when calculating trait requirements,
which are annotated with comments.
```rust
TXTPP#include ../../teleparse/tests/book_ex_recur.rs
```


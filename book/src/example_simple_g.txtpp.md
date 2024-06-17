# Textbook Example - Simplified

Let's revisit the textbook example and the verbal description:
- An expression (`E`) is a list of terms (`T`) separated by `+`.
- A term (`T`) is a list of factors (`F`) separated by `*`.
- A factor (`F`) is either an identifier or an expression enclosed in parentheses.
```text
E  => T E'
E' => + T E' | ε
T  => F T'
T' => * F T' | ε
F  => ( E ) | id
```
Notice that the verbal description is much easier to understand than the formal grammar.
This is because in the formal grammar, concepts like "list of" and "separated by"
need to be broken down to primitives and be defined using helper *productions* (rules).
`E'` and `T'` in the example are helpers to define the "list of" concept.

Teleparse, on the other hand, encapsulates these concepts into helper data structures
that you can use directly in your language definition. This makes the
data structures map closer to the verbal description.

In this example, the `tp::Split<T, P>` type is used to parse "list of `T` separated by `P` with no trailing separator".
```rust
TXTPP#include ../../tests/book_ex_simple.rs
```


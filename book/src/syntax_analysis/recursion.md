# Note on Recursion

Consider again the example shown in the beginning of the book:
```text
E  => T E'
E' => + T E' | ε
T  => F T'
T' => * F T' | ε
F  => ( E ) | id
```

Note that this grammar involves recursions. For example, the productions of `E'` and `T'`
involve themselves; and production of `E` involves `F` indirectly, which in turn involves `E`.

There are 2 things to note when implement a grammar that has recursion:
1. The data structure must have a finite size. This can be easily achieved by using a `Box`.
2. There must NOT be recursion when computing if trait requirements are satisfied.

We will go through an example to illustrate both points. Suppose we are implementing `E'` as:
```rust
// E' => + T E' | ε
#[derive_syntax]
struct Eprime(tp::Option<(OpAdd, T, Eprime)>);
```

Note we are using `tp::Option` to represent the `ε` case. This is a utility type used
to represent optional values. Utility types will be covered in the next chapter.

The first error you will get is:
```text
recursive type `Eprime` has infinite size
```
This is because `Eprime` contains `Eprime` itself, creating an infinite recursion when calculating the size.
To fix this, we can box the recursive part:
```rust
#[derive_syntax]
struct Eprime(tp::Option<(OpAdd, T, Box<Eprime>)>);
```

Now we will see the second error: 
```text
Overflow evaluating the requirement `Eprime: Produce`
```
This error message is a little cryptic. One might think it's related to
having `Eprime` referenced in itself. However, the root cause is that
`derive_syntax` infers the lexicon type by looking at the first fields of structs
and first variants of enums. We can walk through how Rust is evaluating this:
1. The lexicon type of `Eprime` is the same as the lexicon type of `tp::Option<(OpAdd, T, Box<Eprime>)>`.
2. The lexicon type of `tp::Option<(OpAdd, T, Box<Eprime>)>` is the same as that of `(OpAdd, T, Box<Eprime>)`.
3. The lexicon type of `(OpAdd, T, Box<Eprime>)` is the same as that of `OpAdd`, `T`, and `Box<Eprime>`, and they must be the same (according to the trait implementation for tuples)
4. The lexicon type of `OpAdd` is `TokenType`.
5. The lexicon type of `T` is `TokenType`, which is the same as that of `OpAdd`.
6. The lexicon type of `Box<Eprime>` is the same as that of `Eprime`.
7. We are already calculating the lexicon type of `Eprime`, resulting in overflow

To fix this, the example in the beginning of the book uses a separate struct `Eplus` to avoid the loop:
```rust
#[derive_syntax]
struct Eprime(tp::Option<Eplus>);

#[derive_syntax]
struct Eplus {
    op: OpAdd,
    _t: T,
    rest: Box<Eprime>,
}
```

In this case, the lexicon type of `Eplus` does not depend on `Eprime`, because
`derive_syntax` generates an implementation for `Eplus` instead of relying on
blanket trait implementation as in the tuple case.

Another place where this error can appear is in enums.
In the following example, `Paren` must not be the first variant of the enum.

```rust
#[derive_syntax]
enum F {
    Ident(Ident),
    Paren((ParenOpen, Box<E>, ParenClose)),
}
```

## Remove Recursion with Utility Types
In the first example above, the grammar requires recursion in `E'` to implement
a list-like structure (terms separated by `+`). Teleparse provides utility types
like `tp::Split` to simplify the implementation and provide out-of-the-box support
for panic recovery. Utility types are covered in Chapter 4.

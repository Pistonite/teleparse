# Option

`tp::Option<T>` and `tp::Exists<T>` are types used to represent optional syntax trees.
`tp::Option<T>` stores the syntax tree itself, and `tp::Exists<T>` stores a boolean value to test if the syntax tree is present.

## Production

```text
Option<T> => ε | T
```

## Example

`Ident` is a terminal struct not shown here

```rust
use teleparse::prelude::*;

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq)]
struct OptIdent(tp::Option<Ident>);

#[test]
fn test_none() -> Result<(), GrammarError> {
    let t = OptIdent::parse("+")?.unwrap();
    assert_eq!(t, OptIdent(Node::new(0..0, None).into()));

    Ok(())
}

#[test]
fn test_some() -> Result<(), GrammarError> {
    let t = OptIdent::parse("a")?.unwrap();
    assert_eq!(t, OptIdent(Node::new(0..1, Some(Ident::from_span(0..1)).into()));

    Ok(())
}
```

## Deref
`tp::Option<T>` / `tp::Exists<T>` implements `Deref` and `DerefMut`. So you can
use them as `&std::option::Option<T>` / `&bool`.
```rust
let t = OptIdent::parse("a")?.unwrap();
assert!(t.0.is_some());
```

## Parsing
There are 3 possible outcomes when parsing an `Option<T>`:
1. The next token is not in FIRST(T). The parser will return `None`.
2. The next token is in FIRST(T):
    1. If the parser can parse `T`, it will return `Some(T)`.
    2. If the parser cannot parse `T`, it will record an error, return `None`, and continue.

In any case, the parser will not panic.

## Common LL(1) Violations

1. `Option<Option<T>>` - since `Some(None)` and `None` are indistinguishable.
2. `(Option<T>, T)` - since (None, T) and (Some(T), ...) are indistinguishable.

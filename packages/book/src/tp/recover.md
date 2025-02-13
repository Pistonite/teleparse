# Recover

The `tp::Recover<T, R>` type adds recover logic.
It's equivalent production-wise to `(T, R)`, but if `T` or `R` fails to parse,
the parser will skip tokens until an `R` can be parsed.

The `head` and `tail` fields are used to access `T` and `R` respectively.

## Production
```text
Recover<T, R> => T R
```

## Example

`Ident` and `Semi` are terminal structs not shown here
```rust
use teleparse::prelude::*;

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq)]
struct Statement(tp::Recover<Ident, Semi>);

#[test]
fn parse_ok() -> Result<(), GrammarError> {
    let t = Statement::parse("a;")?;
    assert_eq!(
        t,
        Some(Statement(tp::Recover {
            head: Node::new(0..1, Some(Ident::from_span(0..1))),
            tail: Semi::from_span(1..2),
        }))
    );

    Ok(())
}

#[test]
fn parse_recover() -> Result<(), GrammarError> {
    let t = Statement::parse("1;")?;
    assert_eq!(
        t,
        Some(Statement(tp::Recover {
            head: Node::new(0..1, None),
            tail: Semi::from_span(1..2),
        }))
    );

    Ok(())
}
```

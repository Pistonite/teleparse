# Delimited List

These types are used to parse a list of items separated by a delimiter.
Error recovery is possible for these types with the help of the delimiter. See
[Error Recovery](#error-recovery) below.

- `tp::Split<T, P>` is used to parse a list of `T` separated by a delimiter `P`.
- `tp::Punct<T, P>` is used to parse a list of `T` separated by a delimiter `P`, with optional trailing delimiter.

Neither list types are allowed to be empty. If you need to parse an empty list, wrap it with `tp::Option`.

## Production
```text
Split<T, P> => T Star<(P, T)>
Punct<T, P> => T Option<(P, Option<Punct<T, P>)>
```

## Examples
`Ident` and `OpAdd` are terminal structs not shown here
```rust
use teleparse::prelude::*;
use teleparse::GrammarError;

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq)]
struct Terms(tp::Split<Ident, OpAdd>);

#[test]
fn parse_one() -> Result<(), GrammarError> {
    let mut parser = Parser::new("a + b + c")?;
    let terms = parser.parse::<Terms>()?.unwrap();

    let mut iter = terms.iter();
    assert_eq!(iter.next(), Some(Ident::from_span(0..1)));
    assert_eq!(iter.next(), Some(Ident::from_span(4..5)));
    assert_eq!(iter.next(), Some(Ident::from_span(8..9)));
    assert_eq!(iter.next(), None);

    Ok(())
}
```

## Deref
`Split<T, P>` and `Punct<T, P>` can be dereferenced as a vector of the items `T` (`&Vec<T>`).
The delimiters can be accessed using the `puncts` field.

## Error Recovery

When the parser fails to parse an item `T`:
- `Split` will record an error, and stop if the next token is in FOLLOW(Split\<T\>)
- `Punct` will only record an error if the next token is not in FIRST(T) or FOLLOW(Punct\<T\>),
  because it's possible to end on a delimiter.

Otherwise, it will try to recover:
- If the next token is in FIRST(P), it will skip this item and continue with parsing the delimiter.
- Otherwise, it will skip tokens until a token in FIRST(P), FIRST(T) or FOLLOW(self) is found.

Also note that there can be multiple delimiters between 2 elements because of recovery.
You cannot assume any relationship between the number and positions of the delimiters and the elements.

# Iteration

These types are used to parse a list of items directly after one another (i.e.
without a delimiter), and stops if the item cannot start with the next token.

- `tp::Plus<V: FromIterator<T>, T>` is one or more of `T`
- `tp::Star<V: FromIterator<T> + Default, T>` is zero or more of `T`
- `tp::Vec<T>` and `tp::VecDeque<T>` are aliases for `tp::Star<Vec<T>, T>` and
  `tp::Star<VecDeque<T>, T>` respectively
- `tp::Nev<T>` and `tp::NevDeque<T>` are aliases for `tp::Plus<Vec<T>, T>` and
    `tp::Plus<VecDeque<T>, T>` respectively. Nev stands for "non-empty vector".

The name "Plus" and "Star" are borrowed from regular expression notation "+" and
"*" to indicate "one or more" and "zero or more" respectively.

## Production
```text
Plus<T> => T Star<T>
Star<T> => Option<Plus<T>>
```

## Examples
`Ident` is a terminal struct not shown here
```rust
use teleparse::prelude::*;
use teleparse::GrammarError;

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq, Clone)]
struct IdentList(tp::Nev<Ident>);

#[test]
fn parse() -> Result<(), GrammarError> {
    let t = IdentList::parse("a b c  d e").unwrap();
    assert_eq!(
        t,
        IdentList(
            Node::new(
                0..10,
                vec![
                    Ident::from_span(0..1),
                    Ident::from_span(2..3),
                    Ident::from_span(4..5),
                    Ident::from_span(7..8),
                    Ident::from_span(9..10),
                ]
            )
            .into()
        )
    );

    Ok(())
}
```

## Deref
`Plus` and `Star` can be dereferenced to the inner vector type.
For example, you can use `tp::Vec<T>` as a `&std::vec::Vec<T>`.

## Parsing
`Plus` and `Star` share similar parsing logic. The only difference is that
if the next token is not in FIRST(T), the parser will return `Default::default()`
for `Star` but will panic for `Plus`.

The parser keeps trying to parse `T` while the next token is in FIRST(T). If the parser
panics while parsing `T`, it will record the error and recover with the items already parsed.
The only exception is the first item in a `Plus`, in which case the parser panics instead.

## Common LL(1) Violations

1. `Vec<Option<T>>` - since there can be an infinite number of `None`s.

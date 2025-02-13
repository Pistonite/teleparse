# String

These types are used to access the source of the syntax tree. For example, getting
the name of a variable.

- `tp::Parse<S: FromStr, T>` is used for types that implement `FromStr`, such as numeric types.
It stores the parse result. 
- `tp::Quote<S: From<&str>, T>` on the other hand, stores the string value directly.
- `tp::String` is an alias for `tp::Quote<String, T>`.

## Example
`Ident` and `Integer` are terminal structs not shown here.
```rust
use teleparse::prelude::*;

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq, Clone)]
struct Parsed {
    ident: tp::Parse<u32, Ident>,
    num: tp::Parse<u32, Integer>,
    float: tp::Parse<f32, Integer>,
}

#[test]
fn test_parse() {
    let t = Parsed::parse("abc 456 314").unwrap().unwrap();
    assert!(t.ident.is_err());
    assert_eq!(t.num, Node::new(4..7, Ok(456)).into());
    assert_eq!(t.float, Node::new(8..11, Ok(314.0)).into());

    assert_eq!(*t.num, Ok(456));
    assert_eq!(*t.float, Ok(314.0));
}
```

## Deref
`tp::Parse` can be dereferenced to the parse `Result`, and `tp::Quote` can be dereferenced to the inner string value.
```rust
#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq, Clone)]
struct Stringified(tp::String<Ident>);

let t = Stringified::parse("a").unwrap().unwrap();
assert_eq!(&*t.0, "a");
```

## Parsing
The string types parse exactly the same as the inner syntax tree type `T`. After parsing is successful,
the content is parsed/copied to the corresponding type.

If you want to avoid copying, you can use the inner type directly (which usually only stores a span), and use the span to access the source string when needed.

# Semantic

The same token type can have different semantic meaning in different contexts.
For example, the Rust code below:
```rust
fn main() {
    let x = 1;
}
```
`main` and `x` are both identifiers, but `main` is a function name and `x` is a 
variable name. One use case for this is syntax highlighting where different colors
can be displayed for the two words, for example.

To add a semantic type, add an variant to the enum with `derive_lexicon` without any `teleparse` attribute:

```rust
use teleparse::prelude::*;

#[derive_lexicon]
pub enum TokenType {
    #[teleparse(regex(r#"[a-zA-Z]+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(OpEq = "="))]
    Op,
    VariableName, // <- semantic type
}
```

Then add a `#[teleparse(semantic(...))]` attribute in the syntax tree.
Multiple semantic types can be added by separating them with `,`

```rust
#[derive_syntax]
pub struct Assignment {
    #[teleparse(semantic(VariableName))]
    pub variable: Ident,
    pub op: OpEq,
    pub expression: Ident,
}
```

The `VariableName` semantic type will now be applied to `variable`.

The semantic info is stored in `Parser`. You can access it after parsing by using `info().tokens`
```rust
use teleparse::prelude::*;
use teleparse::{Parser, GrammarError};
fn test() -> Result<(), GrammarError> {
    let source = "a = b";
    let mut parser = Parser::<TokenType>::new(source);
    let assignment = parser.parse::<Assignment>()?.unwrap();

    // Get the token info at `variable`
    let token = parser.info().tokens.at_span(assignment.variable.span()).unwrap();
    assert!(token.semantic.contains(TokenType::VariableName));

    Ok(())
}
```


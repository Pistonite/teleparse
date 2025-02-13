# Syntax Analysis

The `#[derive_syntax]` macro turns a struct or enum into a syntax tree node
as part of the [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
, or AST. These become the non-terminal symbols of your language.

## Structs
Structs are used to define sequences of symbols. For example, this production:
```text
X => A B C
```
Means to produce `X`, you need to produce `A`, `B`, and `C` in that order.

For a more real-world example, say we want to parse an assignment statement
like `x = 1`. For simplicity, we will assume the right hand side is always a number.
```text
AssignmentStatement => Ident OpAssign Number
```

Suppose `Ident`, `OpAssign` and `Number` are all terminals created with [`#[derive_lexicon]`](../lexical_analysis/derive_lexicon_g.md),
we can create `AssignmentStatement` like this:
```rust
use teleparse::prelude::*;

#[derive_syntax]
pub struct AssignmentStatement(pub Ident, pub OpAssign, pub Number);
```

Named fields work as well:
```rust
use teleparse::prelude::*;

#[derive_syntax]
pub struct AssignmentStatement {
    pub ident: Ident,
    pub op_assign: OpAssign,
    pub number: Number,
}
```

When the parser is expecting an `AssignmentStatement`, it will try to parse
`Ident`, then `OpAssign`, then `Number`, and put them in the struct.

## Enums
Enums are used to define choices (unions) of productions.
Continuing our example with `AssignmentStatement`, suppose we want to
create a `Statement` that can either be an assignment or a function call.

This can be denoted with
```text
Statement => AssignmentStatement | FunctionCallStatement
```

We can create `Statement` like this:
```rust
use teleparse::prelude::*;

#[derive_syntax]
pub enum Statement {
    Assignment(AssignmentStatement),
    FunctionCall(FunctionCallStatement),
}
```

When the parser is expecting a `Statement`, it will try to parse either
an `AssignmentStatement` or a `FunctionCallStatement`, and create a `Statement`
with the corresponding variant. We will cover how the parser decides which
path to take in the next section.

## Root

With the terminals and non-terminals, we can build the data structures for the entire language.
On the outermost level, there will be one symbol that is the "target" to parse.
We will refer to it as the root. For example, for a programming language,
the root might be the syntax for a file.

To indicate the symbol is root, use the `#[teleparse(root)]` attribute.
```rust
use teleparse::prelude::*;

#[derive_syntax]
#[teleparse(root)]
pub struct File {
    ...
}
```

This will derive the `Root` trait, which has a `parse()` function that can be called
to parse an input string to the root symbol. For more complex usage, you can use the `Parser` object.



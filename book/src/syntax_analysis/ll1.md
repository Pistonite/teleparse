# LL(1)

Teleparse forces the grammar created with `#[derive_syntax]` to be LL(1), which stands for:

- The parser scans the input **L**eft to right
- The parser derives the **L**eftmost tree first
- The parser only ever looks **1** token ahead

These rules help the parser to be more efficient. The parser will only need to look at the next token
to decide what to parse next, and it does not require backtracking.

To validate the grammar is LL(1), the `#[teleparse(root)]` attribute generates a test that 
can be ran with `cargo test` that fails if the grammar is not LL(1). For example, the following grammar is not LL(1):

```text
Decl      => PubEnum | PubStruct
PubEnum   => pub enum
PubStruct => pub struct
```

When parsing `Decl`, if the next token is `pub`, the parser will not know whether to parse `PubEnum` or `PubStruct`, since both start with `pub`.
If we implement this grammar with Teleparse, the generated test will fail with a message
to help you debug the issue.

```rust
use teleparse::prelude::*;

#[derive_lexicon]
pub enum TokenType {
    #[teleparse(terminal(KwPub = "pub", KwEnum = "enum", KwStruct = "struct"))]
    Keyword,
}

#[derive_syntax]
pub struct PubEnum {
    pub kw_pub: KwPub,
    pub kw_enum: KwEnum,
}

#[derive_syntax]
pub struct PubStruct {
    pub kw_pub: KwPub,
    pub kw_struct: KwStruct,
}

#[derive_syntax]
#[teleparse(root)]
pub enum Decl {
    PubEnum(PubEnum),
    PubStruct(PubStruct),
}
```
```text
thread 'Decl_root_test::is_ll1' panicked at <source location>
Decl is not LL(1): The non-terminal `Decl` has a FIRST/FIRST conflict producing either `PubEnum` or `PubStruct`. The conflict
ing terminals are: "pub"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

We can rewrite the grammar to resolve the conflict:
```text
Decl         => pub EnumOrStruct
EnumOrStruct => enum | struct
```

```rust
use teleparse::prelude::*;

#[derive_lexicon]
pub enum TokenType {
    #[teleparse(terminal(KwPub = "pub", KwEnum = "enum", KwStruct = "struct"))]
    Keyword,
}

#[derive_syntax]
pub enum EnumOrStruct {
    Enum(KwEnum),
    Struct(KwStruct),
}

#[derive_syntax]
#[teleparse(root)]
pub struct Decl {
    pub kw_pub: KwPub,
    pub data: EnumOrStruct,
}
```
Note that there are other conditions that make a grammar not LL(1). If you are unsure how to resolve them, please utilize search engines
to learn more about LL(1) grammars and how to resolve conflicts.

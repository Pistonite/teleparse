# Lexical Analysis
From _Compilers: Principles, Techniques, and Tools_ by Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman:
> The lexical analyzer is the first phase of a compiler. Its main task is to read
> the input characters and produce as output a sequence of tokens that the parser
> uses for syntax analysis.

This library implements the lexical analysis phase with the [`Lexicon`] trait,
which can be derived from an enum with the [`#[derive_lexicon]`](crate::derive_lexicon)
macro. This macro will:
- Derive the token type (`Keyword`, `Identifier`, etc) from the enum variants and assign them ordinal and bit positions.
- Derive the lexical analyzer (the [`Lexer`]) using attributes, to parse the input text into [`Token`]s.
- Derive the AST [`Terminal`]s for [syntax analysis](crate::syntax), which is the next phase of parsing.

```rust
TXTPP#include ../../tests/expand/lexicon_example.rs
```

The example above derives:
- The [`Lexicon`] trait **and super traits** for the enum `TokenType`.
- Lexer rules based on pattern matching.
- `Integer`, `OpAdd`, `OpMul`, `ParamOpen` and `ParamClose` structs in the current scope
  and the [`Terminal`] trait for them.

Usually you will not work with the lexer directly, but instead use the derived terminals
in the syntax tree. See [syntax analysis](crate::syntax) for more details.

<br>
<br>

## Enum Attribute - `ignore`
The `#[teleparse(ignore(...))]` attribute defines patterns to be ignored
by the lexer with regular expressions. A common example is whitespaces (`\s+`).
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"^\s+"#))]
pub enum MyToken {
    // ...
# #[teleparse(regex(r#"^\d+"#))]
# Integer,
}
 ```

<div class="warning">

Note that the regular expression must start with `^` and not match the empty string.
This instructs the lexer to only match from the beginning of the remaining input.
This is enforced by the macro.

</div>

You can also have multiple patterns separated by `,`
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"^\s+"#, r#"^\w+"#))]
pub enum MyToken {
    // ...
# #[teleparse(regex(r#"^\d+"#))]
# Integer,
}
 ```

<br>
<br>

## Enum Attribute - `terminal_parse`

This attribute derives the necessary traits to call `parse()` on the terminals,
as you will see in the examples below. In general, you don't need this attribute for applications.
<br>
<br>

## Variant Attributes - `terminal`, `regex`

A variant can have these combinations of attributes:
### 1. `terminal` and no `regex`

Each `terminal` then must have a literal string value to be matched. The rule to match the token type
is automatically inferred to be a pattern that matches any of the literal strings.
```rust
TXTPP#include ./expand/pizza.rs

# fn main() {
assert_eq!(
    Pizza::parse("pizza"),
    Ok(Some(Pizza(Token::new((0, 5), MyToken::Food))))
);
assert_eq!(
    Pizza::parse("pasta"),
    Ok(None)
);
assert_eq!(
    Pasta::parse("pasta"),
    Ok(Some(Pasta(Token::new((0, 5), MyToken::Food))))
);
# }
```
In fact, the macro will error if you try to define a regex when all the terminals have a literal string (Don't-Repeat-Yourself)
```compile_fail
# use teleparse::prelude::*;
# #[derive_lexicon]
# pub enum TokenType {
#[teleparse(terminal(
    OpAdd = "+", 
    OpSub = "-", 
    OpMul = "*", 
    OpDiv = "/",
), regex(r#"^[\+\-\*/]"#))] // error! the rule can already be inferred
Operator,
# }
```

<br>

### 2. both `terminal` and `regex`

This is the case where the regex is used to match the token type, but the terminals might need specializations:
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(terminal_parse)]
pub enum MyToken {
    #[teleparse(regex(r#"^\w+"#), terminal(Word, Secret = "rust"))]
    Word,
}

# fn main() {
let source = "rust";
// Word can be parsed
assert_eq!(
    Word::parse(source),
    Ok(Some(Word(Token::new((0, 4), MyToken::Word))))
);
// so is Secret
assert_eq!(
    Secret::parse(source),
    Ok(Some(Secret(Token::new((0, 4), MyToken::Word))))
);
// but not with others
let source = "javascript";
assert_eq!(
    Word::parse(source),
    Ok(Some(Word(Token::new((0, 10), MyToken::Word))))
);
assert_eq!(
    Secret::parse(source),
    Ok(None)
);
# }
```

<div class="warning">

Note that there's no "conflict" here! The lexer doesn't care about
if the terminal should be Word or Secret. That's the job of the parser.

</div>

Like `ignore`, the regular expressions are checked
```compile_fail
# use teleparse::prelude::*;
# #[derive_lexicon]
# pub enum MyToken {
// fail! invalid regex
#[teleparse(regex(r#"^\"#))]
Invalid, 
# }
```
```compile_fail
# use teleparse::prelude::*;
# #[derive_lexicon]
# pub enum MyToken {
// fail! the regex must start with ^
#[teleparse(regex(r#"\w+"#))]
MatchMiddle, 
# }
```
```compile_fail
# use teleparse::prelude::*;
# #[derive_lexicon]
# pub enum MyToken {
// fail! the regex must not match the empty string
#[teleparse(regex(r#"^"#))]
MatchEmpty, 
# }
```
There are checks against any literal present as well:
```compile_fail
# use teleparse::prelude::*;
# #[derive_lexicon]
# pub enum MyToken {
// fail! The regex matches a proper prefix of a literal. This is not allowed
// since the lexer will always match the prefix instead of the literal
#[teleparse(regex(r#"^key"#), terminal(Key, Keyboard = "keyboard"))]
DoesNotMatchTerminal, 
# }
```
```compile_fail
# use teleparse::prelude::*;
# #[derive_lexicon]
# pub enum MyToken {
// fail! of course it has to match the actual literal
#[teleparse(regex(r#"^foo"#), terminal(Key, Bar = "bar"))]
DoesNotMatchTerminal, 
# }
```
You also cannot have 2 terminals with the same literal, or 2 terminals that have
the no literal. In this case, the terminals are interchangeable in the AST, so you don't
need a second copy.
```compile_fail
# use teleparse::prelude::*;
# #[derive_lexicon]
# pub enum MyToken {
// fail! Two terminals matches "0"
#[teleparse(terminal(Zero = "0", Another = "0"))]
Integer,
# }
```
```compile_fail
# use teleparse::prelude::*;
#[derive_lexicon]
# pub enum MyToken {
// fail! these two terminals are interchangable. DRY!
#[teleparse(regex(r"^\d+"), terminal(Integer, FancyInteger))]
Integer,
# }
```

<br>

### 3. only `regex`

By having no terminals, you won't be able to put this token type in the AST.
However, the lexer will still produce those tokens and store them (unlike `ignore`).
You can use this for things like comments.

```rust
TXTPP#include ./expand/comment.rs
```

<br>
<br>

## Implementation notes

1. The bit flag representation is chosen automatically based on the number of variants.
   It can be one of `u8`, `u16`, `u32`, `u64`, or `u128`.
2. Currently, the lexer is implemented based on an array of rules. This means the order
   of the rules matter. The first rule that matches the input is chosen. This is for efficiency
   reasons. If there is a string that could match multiple rules, the longer one should be placed before
   the shorter one in the enum. Note that this might change in the future if the lexer switches to use a DFA.


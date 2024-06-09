# Lexical Analysis
From _Compilers: Principles, Techniques, and Tools_ by Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman:
> The lexical analyzer is the first phase of a compiler. Its main task is to read
> the input characters and produce as output a sequence of tokens that the parser
> uses for syntax analysis.

This library implements the lexical analysis phase with the [`Lexicon`] trait,
which can be derived from an enum with the [`#[derive_lexicon]`](crate::derive_lexicon)
macro. This macro will:
- Derive the token type (such as `Keyword` or `Ident`) from the enum variants and assign them ordinal and bit positions.
- Derive the lexical analyzer (the [`Lexer`]) using attributes, to parse the input text into [`Token`]s.
- Derive the AST Terminals for [syntax analysis](crate::syntax), which is the next phase of parsing.

```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r"\s"))] // ignore whitespaces
pub enum TokenType {
    /// Numbers in the expression
    #[teleparse(regex(r"\d+"), terminal(Integer))]
    Integer,
    /// The 4 basic operators
    #[teleparse(terminal(
        OpAdd = "+", 
        OpMul = "*", 
    ))]
    Operator,
    /// Parentheses
    #[teleparse(terminal(ParamOpen = "(", ParamClose = ")"))]
    Param,
}
```

The example above derives:
- The [`Lexicon`] trait **and super traits** for the enum `TokenType`.
- Lexer implementation backed by the *ridiculously fast* [`logos`] crate, which features static compilation of the rules to a DFA.
- `Integer`, `OpAdd`, `OpMul`, `ParamOpen` and `ParamClose` structs in the current scope
  and the Terminal trait for them.

Usually, you will not work with the lexer directly, but instead use the derived terminals
in the syntax tree. See [syntax analysis](crate::syntax) for more details.

## Attributes
[`derive_lexicon`](crate::derive_lexicon) supports the following attributes inside `#[teleparse(...)]`
- On the enum:
  - [`ignore`](#enum-attribute---ignore) skip patterns matching the regular expression between tokens
  - `terminal_parse` (for testing purposes), derive the necessary traits to call `parse()` on the terminals
- On the variants:
  - [`regex`](#variant-attributes---terminal-regex) Define the pattern to match for the token type
  - [`terminal`](#variant-attributes---terminal-regex) Define the AST terminals to derive for the token type

<div class="warning">

Note that multiple attributes should be put in the same `teleparse` attribute, i.e. `#[teleparse(ignore(...), terminal_parse)]`.

</div>

## Enum Attribute - `ignore`
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"\s+"#))]
pub enum MyToken {
    // ...
# #[teleparse(regex(r#"\d+"#))]
# Integer,
}
 ```

Multiple patterns should be separated by `,`.
Below is just a dummy example since it's easy to combine those two patterns into one manually.
However sometimes it's not that easy, and you might want to keep them separate.
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"\s+"#, r#"[a-zA-Z]+"#))]
pub enum MyToken {
    // ...
# #[teleparse(regex(r#"\d+"#))]
# Integer,
}
 ```

## Variant Attributes - `terminal`, `regex`

A variant can have these combinations of attributes:
### 1. `terminal` and no `regex`

Each `terminal` then must have a literal string value to be matched. The rule to match the token type
is automatically inferred to be a pattern that matches any of the literal strings.
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(terminal_parse)]
pub enum MyToken {
    #[teleparse(terminal(Pizza = "pizza", Pasta = "pasta"))]
    Food,
}

fn main() {}
```
In fact, the macro will error if you try to define a regex when all the terminals have a literal string (Don't-Repeat-Yourself)
```compile_fail
use teleparse::prelude::*;

#[derive_lexicon]
pub enum TokenType {
    #[teleparse(terminal(
        OpAdd = "+", 
        OpSub = "-", 
        OpMul = "*", 
        OpDiv = "/",
    ), regex(r#"[\+\-\*/]"#))]
    Operator,
}

fn main() {}
```
```console
error: Defining `regex` here is redundant because all terminals have a literal match pattern, so the rule can already be inferred.
  --> tests/ui/lex_redundant_regex.rs:11:5
   |
11 |     Operator,
   |     ^^^^^^^^
```

<br>

### 2. both `terminal` and `regex`

This is the case where the regex is used to match the token type,
but there are AST terminals that want to match against specific literals.
A real-world example is when a keyword can also be a legal identifier (i.e. in JavaScript)
```rust
use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(terminal_parse)]
pub enum MyToken {
    #[teleparse(regex(r#"\w+"#), terminal(Ident, KwClass = "class"))]
    Word,
}

# fn main() {
let source = "class";
// can be parsed as Ident and KwClass
assert_eq!(
    Ident::parse(source),
    Ok(Some(Ident(Token::new(0..5, MyToken::Word))))
);
assert_eq!(
    KwClass::parse(source),
    Ok(Some(KwClass(Token::new(0..5, MyToken::Word))))
);
// other words can only be parsed as Ident
let source = "javascript";
assert_eq!(
    Ident::parse(source),
    Ok(Some(Ident(Token::new(0..10, MyToken::Word))))
);
assert_eq!(
    KwClass::parse(source),
    Ok(None)
);
# }
```

<div class="warning">

Note that there's no "conflict" here! The `regex` is for the lexer,
and the literals are for the parser.

</div>

The regexes in `ignore` and `regex` are checked at compile time.
This library performs some basic checks which you can find examples in
the [test cases](https://github.com/Pistonite/teleparse/tree/main/tests/ui).
[`logos`] define additional restrictions on the regexes for performance.

One notable case is that, if there are also literals present for any terminal,
the regex must:
- has a match in the literal that starts at position 0
- the match must not be a proper prefix of the literal

For the first condition, suppose the regex is `board` and the literal is `keyboard`.
The lexer will never be able to emit `keyboard` when the rest of the input
starts with `board`.
```compile_fail
use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r#"board"#), terminal(Board, Keyboard = "keyboard"))]
    DoesNotMatchTerminal, 
}

fn main() {}
```
```console
error: This regex does not match the beginning of `keyboard`. This is likely a mistake, because the terminal will never be matched
 --> tests/ui/lex_regex_not_match_start.rs:5:23
  |
5 |     #[teleparse(regex(r#"board"#), terminal(Board, Keyboard = "keyboard"))]
  |                       ^^^^^^^^^^
```

For the second condition, suppose the regex is `key` and the literal is `keyboard`.
The lexer will again never be able to emit `keyboard`:
- If it were to emit `keyboard` of this token type, the rest of the input must start with `keyboard`
- However, if so, the lexer would emit `key` instead

```compile_fail
use teleparse::prelude::*;
#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r#"key"#), terminal(Key, Keyboard = "keyboard"))]
    DoesNotMatchTerminal, 
}

fn main() {}
```
```console
error: This regex matches a proper prefix of `keyboard`. This is likely a mistake, because the terminal will never be matched (the prefix will instead)
 --> tests/ui/lex_regex_not_match_is_prefix.rs:4:23
  |
4 |     #[teleparse(regex(r#"key"#), terminal(Key, Keyboard = "keyboard"))]
  |                       ^^^^^^^^
```

### 3. only `regex`

By having no terminals, you won't be able to put this token type in the AST.
However, the lexer will still produce those tokens and store them (unlike `ignore`).
You can use this for things like comments.

```rust
use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r"/\*([^\*]|(\*[^/]))*\*/"))]
    Comment,
}

fn main() {
    let input = "/* This is a comment */";
    let mut lexer = MyToken::lexer(input).unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new(0..23, MyToken::Comment))));

    // this will tell the parser to put this token aside 
    assert!(MyToken::Comment.should_extract());
}
```


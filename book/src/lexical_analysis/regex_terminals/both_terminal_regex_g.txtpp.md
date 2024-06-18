## Both `terminal` and `regex`

With `regex`, you can create a terminal type that matches
the token type and any literal content (that matches the regex)

```rust
use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r"\w+"), terminal(Ident))]
    Ident,
}
```

You can also add literal match to the terminal. A real-world usage
may be when a keyword can also be a legal identifier,
if an identifier is expected (i.e. in JavaScript)

```rust
TXTPP#include ../../../../tests/doc_lex_term_regex.rs
```

<div class="warning">

Note that there's no "conflict" here! The `regex` is for the lexer,
and the literals are for the parser. When seeing "class" in the source,
the lexer will produce a `Word` token with the content `"class"`.
It is up to the parsing context if a `Ident` or `KwClass` is expected.

</div>

## Static Checks
The macro does some sanity checks on the terminal literals when a `regex` is specified.
For each literal, the `regex` must:
- has a match in the literal that starts at the beginning (position 0)
- the match must not be a proper prefix of the literal

For the first condition, suppose the regex is `board` and the literal is `keyboard`.
The lexer will never be able to emit `keyboard` when the rest of the input
starts with `board`.
```rust,compile_fail
TXTPP#include ../../../../tests/ui/lex_regex_not_match_start.rs
```
```console
TXTPP#include ../../../../tests/ui/lex_regex_not_match_start.stderr
```

For the second condition, suppose the regex is `key` and the literal is `keyboard`.
The lexer will again never be able to emit `keyboard`:
- If it were to emit `keyboard` of this token type, the rest of the input must start with `keyboard`
- However, if so, the lexer would emit `key` instead

```rust,compile_fail
TXTPP#include ../../../../tests/ui/lex_regex_not_match_is_prefix.rs
```
```console
TXTPP#include ../../../../tests/ui/lex_regex_not_match_is_prefix.stderr
```


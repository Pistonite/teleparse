# Using `regex` and `terminal` attributes

The `#[teleparse(regex(...))]` and `#[teleparse(terminal(...))]` attributes
are used to define the pattern to match for the token type and terminals in the
syntax tree.

The simpliest example is to define a single terminal struct,
along with a `regex` for the lexer to produce the token type when the remaining
source code matches the regex.

```rust
TXTPP#include ../../../tests/doc_lex_term_simple.rs
```

You can also add additional terminals that have to match a specific
literal value.

```rust
TXTPP#include ../../../tests/doc_lex_term_regex.rs
```

<div class="warning">

Note that there's no "conflict" here! The `regex` is for the lexer,
and the literals are for the parser. When seeing "class" in the source,
the lexer will produce a `Word` token with the content `"class"`.
It is up to the parsing context if a `Ident` or `KwClass` is expected.

</div>

When such literals are present specified for the terminals
along with the `regex` for the variant, `derive_lexicon`
will do some checks at compile-time to make sure the literals
make sense.

For each literal, the `regex` must:
- has a match in the literal that starts at the beginning (position 0)
- the match must not be a proper prefix of the literal

For the first condition, suppose the regex is `board` and the literal is `keyboard`.
The lexer will never be able to emit `keyboard` when the rest of the input
starts with `board`.
```rust,compile_fail
TXTPP#include ../../../tests/ui/lex_regex_not_match_start.rs
```
```console
TXTPP#include ../../../tests/ui/lex_regex_not_match_start.stderr
```

For the second condition, suppose the regex is `key` and the literal is `keyboard`.
The lexer will again never be able to emit `keyboard`:
- If it were to emit `keyboard` of this token type, the rest of the input must start with `keyboard`
- However, if so, the lexer would emit `key` instead

```rust,compile_fail
TXTPP#include ../../../tests/ui/lex_regex_not_match_is_prefix.rs
```
```console
TXTPP#include ../../../tests/ui/lex_regex_not_match_is_prefix.stderr
```


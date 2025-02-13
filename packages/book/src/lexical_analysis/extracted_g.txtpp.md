# Handling Comments (Extracted Tokens)

Comments are special tokens that don't have meaning
in the syntax and are there just for human readers.

You may recall that you can skip unwanted patterns between
tokens by using `#[teleparse(ignore(...))]`. This can be used
for comments, but sometimes you *do* want to parse the comments.
For example for a transpiler that keeps the comments in the output.

In this senario, you can define a token type that doesn't have any `terminal`s.
The lexer will still produce those tokens, but instead of passing them to the parser,
they will be kept aside. You can query them using a `Parser` object later.

```rust
TXTPP#include ../../../teleparse/tests/doc_lex_comment.rs
```

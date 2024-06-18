## Only `regex`

By having no terminals, you won't be able to put this token type in the syntax tree.
However, the lexer will still produce those tokens and store them (unlike `ignore`).

For example, suppose you are making a transpiler that requires keeping the comments
from the source code in the output. It will be way too complicated to have comments
be part of the syntax tree. Instead, you can use this to extract the comments
and be able to query them later using the `Parser` API.

```rust
TXTPP#include ../../../../tests/expand/comment.rs
```

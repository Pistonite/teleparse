## Only `terminal`

Without a `regex`, each `terminal` must have a literal string value to be matched. 
The rule to match the token type is automatically inferred to be a pattern 
that matches any of the literal strings.

```rust
TXTPP#include ../../../../tests/expand/pizza.rs
```
In fact, the macro will error if you try to define a regex when all the terminals have a literal string (Don't-Repeat-Yourself)
```rust,compile_fail
TXTPP#include ../../../../tests/ui/lex_redundant_regex.rs
```
```console
TXTPP#include ../../../../tests/ui/lex_redundant_regex.stderr
```

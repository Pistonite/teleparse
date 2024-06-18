# Using `regex` and `terminal` attributes

These attributes on enum variants are used to define the pattern to match for the token type.
They can be used in one of the following combinations:
1. Only `terminal`
2. Both `terminal` and `regex`
3. Only `regex`
4. Neither attributes

The first three will allow the lexer to produce the token type.
The last only defines the token type variant, but no rules to match that type.
This means the lexer won't be able to emit that token. Rather, it can be used
for semantic purposes like "variable" or "function name", which can be added
by the parser later.


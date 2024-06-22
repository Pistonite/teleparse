# LL(1)

Teleparse forces the grammar created with `#[derive_syntax]` to be LL(1), which stands for:

- The parser scans the input **L**eft to right
- The parser derives the **L**eftmost tree first
- The parser only ever looks **1** token ahead

These rules help the parser to be more efficient, since it doesn't
require backtracking.

To validate the grammar is LL(1), Teleparse needs to know every production
in that grammar.

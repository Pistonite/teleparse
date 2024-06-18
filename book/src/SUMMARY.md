# Summary

- [Introduction](./intro.md)
  - [Textbook Example](./example_g.md)
  - [Textbook Example - Simplified](./example_simple_g.md)
  - [Parser Workflow](./flow.md)

# Lexical Analysis

- [`derive_lexicon`](./lexical_analysis/derive_lexicon_g.md)
  - [Terminals](./lexical_analysis/terminals_g.md)
    - [Using `regex` and `terminal` attributes](./lexical_analysis/regex_terminals.md)
    - [Only `terminal`](./lexical_analysis/regex_terminals/terminal_only_g.md)
    - [Both `terminal` and `regex`](./lexical_analysis/regex_terminals/both_terminal_regex_g.md)
    - [Only `regex`](./lexical_analysis/regex_terminals/regex_only_g.md)
    - [Neither (Semantic Types)](./lexical_analysis/regex_terminals/semantic_g.md)
  - [Regex Validation](./lexical_analysis/regex_validation_g.md)

# Syntax Analysis

- [`derive_syntax`](./syntax_analysis/derive_syntax_g.md)
  - [Structs (Sequence)](./syntax_analysis/structs_g.md)
  - [Enums (Union)](./syntax_analysis/enums_g.md)
  - [Note on Recursion](./syntax_analysis/recursion_g.md)
  - [LL(1) Validation](./syntax_analysis/ll1.md)
  - [Static Metadata](./syntax_analysis/static_metadata_g.md)

# Built-in Syntax Types
- [Option](./tp/option_g.md)
- [Quote and Parse](./tp/string_g.md)
- [Iteration](./tp/iter_g.md)
- [Delimited](./tp/delimited_g.md)
- [Recover](./tp/recover_g.md)
- [Loop](./tp/loop_g.md)

# Extras

- [Semantic](./semantic_analysis/semantic_g.md)
- [Hooks](./semantic_analysis/hooks_g.md)
- [Process Tokens](./semantic_analysis/parser_data_g.md)

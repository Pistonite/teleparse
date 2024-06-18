# Summary

- [Introduction](./intro.md)
  - [Textbook Example](./example_g.md)
  - [Textbook Example - simplified](./example_simple_g.md)
  - [Parser Workflow](./flow.md)

# Lexical Analysis

- [`derive_lexicon`](./lexical_analysis/derive_lexicon_g.md)
  - [Understanding Terminals](./lexical_analysis/terminals_g.md)
  - [Using `regex` and `terminal` Attributes](./lexical_analysis/regex_terminals_g.md)
  - [Inferring the Pattern](./lexical_analysis/inferred_regex_g.md)
  - [Handling Comments](./lexical_analysis/extracted_g.md)
  - [Semantic Tokens]()
  - [Lexer Validation](./lexical_analysis/lexer_validation_g.md)

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

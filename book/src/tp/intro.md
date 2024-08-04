# Utility Types

The `teleparse::tp` module provides utility types for language constructs that are hard to or not 
directly representable in a grammar. For example, a list of items separated by a delimiter,

## Examples of Common Utility Types

| Type | Description |
| ---- | ----------- |
| `tp::Option<T>` | An optional syntax tree `T` |
| `tp::Exists<T>` | A boolean value to test if an optional syntax tree `T` is present |
| `tp::String<T>` | Source of the syntax tree `T` stored in a `String` |
| `tp::Vec<T>`    | A list of syntax trees `T` stored in a `Vec` |
| `tp::VecDeque<T>`    | A list of syntax trees `T` stored in a `VecDeque` |
| `tp::Nev<T>`    | A non-empty vector of syntax trees `T` stored in a `Vec` |
| `tp::NevDeque<T>`    | A non-empty vector of syntax trees `T` stored in a `VecDeque` |
| `tp::Loop<T>`   | A loop that tries to parse `T` until the end of input |
| `tp::Split<T, P>` | A list of syntax trees `T` separated by a delimiter `P` |
| `tp::Punct<T, P>` | A list of syntax trees `T` separated by a delimiter `P`, with optional trailing delimiter |
| `tp::Recover<T, R>` | A recover boundary. If `T` fails to be parsed, the parser will skip until `R` is found |

There are other utility types that are used less often.

## Blanket Types

Besides the utility types above, teleparse also provide syntax tree implementation
for some types in the Rust standard library:

- `Box<T>` for any syntax tree `T`
- Tuples up to 5 elements (e.g. `(T1, T2, T3, T4, T5)`)

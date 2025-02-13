# Loop

The `tp::Loop<T>` type is also a list of `T` (like `Plus` and `Star`), but
it will keep skipping tokens when parsing `T` fails until the end of input.

This is useful if the root should be a list of items, and you may want to skip
invalid tokens in between.

## Production
```text
Loop<T> => T Loop<T>
```

## Deref
Just like `Plus` and `Star`, `Loop<T>` can be dereferenced to `&Vec<T>`.

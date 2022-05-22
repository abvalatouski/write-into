# `write-into`

Defines a trait built on top of `io::Write` to write things _into_ it.

## Example

```rust
use write_into::{BigEndian, WriteInto, write_into};

let mut buffer = Vec::new();
write_into(&mut buffer, BigEndian(0xCAFEBABEu32)).unwrap();
assert_eq!(&buffer, &[0xCA, 0xFE, 0xBA, 0xBE]);
```

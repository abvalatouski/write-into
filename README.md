# `write-into`

Defines a trait built on top of `io::Write` to write things _into_ it.

```rust
use std::io;

trait WriteInto {
    type Output;
    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output>;
}
```

The crate also provides wrappers, such as `BigEndian` and `LittleEndian`, to write values
in particular formats.

## Example

```rust
use write_into::{BigEndian, write_into};

let mut buffer = Vec::new();
write_into(&mut buffer, BigEndian(0xCAFEBABEu32)).unwrap();
assert_eq!(&buffer, &[0xCA, 0xFE, 0xBA, 0xBE]);
```

## Wrappers

| Wrapper        | Used to write values...                |
| -------------- | -------------------------------------- |
| `BigEndian`    | ... in big endian byte order.          |
| `LittleEndian` | ... in little endian byte order.       |
| `Plain`        | ... as they are represented in memory. |
| `Sleb128`      | ... in LEB-128 format _(signed)_.      |
| `Uleb128`      | ... in LEB-128 format _(unsigned)_.    |

# `write_into`

Defines a trait built on top of `io::Write` to write things _into_ it.

## Example

```rust
use leb128;
use std::{convert, io};
use write_into::{WriteInto, write_into};

// https://en.wikipedia.org/wiki/LEB128
struct Leb128<T>(T);

impl<T> WriteInto for Leb128<T>
where
    // `leb128` crate uses `u64` and I'm too lazy to write multiple implementations (._.)
    T: convert::Into<u64>
{
    type Output = ();

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<()> {
        leb128::write::unsigned(sink, self.0.into())?;
        Ok(())
    }
}

let mut buffer = Vec::new();
write_into(&mut buffer, Leb128(1337u32)).unwrap();
```

//! Defines a trait built on top of [`io::Write`] to write things _into_ it.
//!
//! Instead of writing blanket implementations it is better to use wrappers
//! with [`write_into`] function because there might be implementation conflicts
//! (e.g. between [`WriteInto`] for [`u8`] and [`WriteInto`] for any
//! [`IntoIterator`]).
//! 
//! # Example
//!
//! ```
//! use write_into::{BigEndian, WriteInto, write_into};
//! 
//! let mut buffer = Vec::new();
//! write_into(&mut buffer, BigEndian(0xCAFEBABEu32)).unwrap();
//! assert_eq!(&buffer, &[0xCA, 0xFE, 0xBA, 0xBE]);
//! ```

mod endianness;

use std::io;

pub use endianness::BigEndian;
pub use endianness::LittleEndian;

/// Writes value into I/O sink.
pub trait WriteInto {
    /// Result of [`WriteInto::write_into`] function (e.g. `()` or [`usize`]).
    type Output;

    /// Writes value into I/O sink.
    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output>;
}

/// An alias for [`WriteInto::write_into`] for writing `write_into(sink, Wrapper(...))` instead of
/// `Wrapper(...).write_into(sink)`.
#[inline]
pub fn write_into<T: WriteInto>(sink: &mut impl io::Write, value: T) -> io::Result<T::Output> {
    value.write_into(sink)
}

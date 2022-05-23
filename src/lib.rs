//! Defines a trait built on top of [`io::Write`] to write things _into_ it.
//! 
//! ```no_run
//! use std::io;
//! 
//! trait WriteInto {
//!     type Output;
//!     fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output>;
//! }
//! ```
//! 
//! The crate also provides wrappers, such as [`BigEndian`] and [`LittleEndian`], to write values
//! in particular formats.
//!
//! # Example
//!
//! ```
//! use write_into::{BigEndian, write_into};
//!
//! let mut buffer = Vec::new();
//! write_into(&mut buffer, BigEndian(0xCAFEBABEu32)).unwrap();
//! assert_eq!(&buffer, &[0xCA, 0xFE, 0xBA, 0xBE]);
//! ```

mod endianness;
mod plain;

use std::io;

pub use endianness::BigEndian;
pub use endianness::LittleEndian;
pub use plain::Plain;

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

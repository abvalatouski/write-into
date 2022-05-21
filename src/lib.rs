//! Defines a trait built on top of [`io::Write`] to write things _into_ it.
//! 
//! Instead of writing blanket implementations it its better to use wrappers
//! with [`write_into`] function because there might be implementation conflicts
//! (e.g. between [`WriteInto`] for [`u8`] and [`WriteInto`] for any
//! [`std::iter::IntoIterator`]).
//!
//! # Example
//!
//! ```
//! use leb128;
//! use std::{convert, io};
//! use write_into::{WriteInto, write_into};
//!
//! // https://en.wikipedia.org/wiki/LEB128
//! struct Leb128<T>(T);
//!
//! impl<T> WriteInto for Leb128<T>
//! where
//!     // `leb128` crate uses `u64` and I'm too lazy to write multiple implementations (._.)
//!     T: convert::Into<u64>
//! {
//!     type Output = ();
//! 
//!     fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
//!         leb128::write::unsigned(sink, self.0.into())?;
//!         Ok(())
//!     }
//! }
//!
//! let mut buffer = Vec::new();
//! write_into(&mut buffer, Leb128(1337u32)).unwrap();
//! ```

use std::io;

/// Writes value into I/O sink.
pub trait WriteInto {
    /// Result of [`WriteInto::write_into`] function (e.g. `()` or [`usize`]).
    type Output;

    /// Writes value into I/O sink.
    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output>;
}

/// An alias for [`WriteInto::write_into`] for writing `write_into(sink, Wrapper(...))` instead of
/// `Wrapper(...).write_into(sink)`.
pub fn write_into<T: WriteInto>(sink: &mut impl io::Write, value: T) -> io::Result<T::Output> {
    value.write_into(sink)
}

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
mod leb128;
mod plain;
mod sequence;
mod sized;

use std::io;

pub use endianness::BigEndian;
pub use endianness::LittleEndian;
pub use leb128::Sleb128;
pub use leb128::Uleb128;
pub use plain::Plain;
pub use sequence::Sequence;
pub use sequence::SizedSequence;
pub use sized::Sized;

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

/// Aligns position in the I/O sink to the given boundary and returns a new position.
///
/// # Example
///
/// ```
/// use std::io;
/// use write_into::{BigEndian, align_position, write_into};
///
/// let mut buffer = io::Cursor::new(Vec::new());
/// write_into(&mut buffer, BigEndian(0xAABBu16)).unwrap();
/// let aligned_position = align_position(&mut buffer, 4).unwrap();
/// write_into(&mut buffer, BigEndian(0xCCDDu16)).unwrap();
/// assert_eq!(aligned_position, 4);
/// assert_eq!(buffer.get_ref(), &[0xAA, 0xBB, 0x00, 0x00, 0xCC, 0xDD]);
/// ```
pub fn align_position(sink: &mut impl io::Seek, boundary: u64) -> io::Result<u64> {
    let position = sink.stream_position()?;
    let alignment = boundary - (position + boundary) % boundary;
    sink.seek(io::SeekFrom::Current(alignment as i64))
}

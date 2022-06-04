use super::{write_into, Plain, WriteInto};
use std::io;

/// Used to write values prepended with size of their representation.
///
/// # Example
///
/// ```
/// use write_into::{Sized, Plain, Uleb128, write_into};
///
/// let mut buffer = Vec::new();
/// let written = write_into(&mut buffer, Sized(Uleb128, Plain("Hello, Sailor!"))).unwrap();
/// assert_eq!(written, 14);
/// assert_eq!(&buffer, b"\x0EHello, Sailor!");
/// ```
pub struct Sized<T, S, F>(pub F, pub T)
where
    T: WriteInto,
    S: WriteInto,
    F: FnOnce(usize) -> S;

/// Returns how many bytes was taken by the representation of `T`.
impl<T, S, F> WriteInto for Sized<T, S, F>
where
    T: WriteInto,
    S: WriteInto,
    F: FnOnce(usize) -> S,
{
    type Output = usize;

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
        let mut buffer = Vec::new();
        write_into(&mut buffer, self.1)?;
        let written = buffer.len();

        write_into(sink, (self.0)(written))?;
        write_into(sink, Plain(&buffer[..]))?;

        Ok(written)
    }
}

/// Returns how many bytes was taken by the representation of `T`.
impl<T, S, F> WriteInto for &Sized<T, S, F>
where
    T: Copy + WriteInto,
    S: WriteInto,
    F: Copy + FnOnce(usize) -> S,
{
    type Output = usize;

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
        write_into(sink, Sized(self.0, self.1))
    }
}

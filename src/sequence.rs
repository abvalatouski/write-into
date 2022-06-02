use super::{write_into, WriteInto};
use std::io;
use std::iter::IntoIterator;

/// Used to write values from [`IntoIterator`].
///
/// # Example
///
/// ```
/// use write_into::{BigEndian, Sequence, write_into};
///
/// let mut buffer = Vec::new();
/// let written = write_into(&mut buffer, Sequence(&[
///     BigEndian(0xAABBu16),
///     BigEndian(0xCCDDu16),
/// ])).unwrap();
/// assert_eq!(written, 2);
/// assert_eq!(&buffer, &[0xAA, 0xBB, 0xCC, 0xDD]);
/// ```
pub struct Sequence<T>(pub T)
where
    T: IntoIterator,
    T::Item: WriteInto;

/// Returns how many items was written.
impl<T> WriteInto for Sequence<T>
where
    T: IntoIterator,
    T::Item: WriteInto,
{
    type Output = usize;

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<usize> {
        let mut written = 0;
        for item in self.0 {
            item.write_into(sink)?;
            written += 1;
        }

        Ok(written)
    }
}

impl<T> WriteInto for &Sequence<T>
where
    T: Copy + IntoIterator,
    T::Item: WriteInto,
{
    type Output = usize;

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
        write_into(sink, Sequence(self.0))
    }
}

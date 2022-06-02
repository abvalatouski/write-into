use super::{write_into, WriteInto};
use std::io;
use std::iter::{ExactSizeIterator, IntoIterator};

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

/// Returns how many items was written.
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

/// Used to write values from [`IntoIterator`] with known size.
///
/// # Example
///
/// ```
/// use write_into::{BigEndian, SizedSequence, write_into};
///
/// let mut buffer = Vec::new();
/// let written = write_into(&mut buffer, SizedSequence(|size| BigEndian(size as u16), &[
///     BigEndian(0xAABBu16),
///     BigEndian(0xCCDDu16),
/// ])).unwrap();
/// assert_eq!(written, 2);
/// assert_eq!(&buffer, &[0x00, 0x02, 0xAA, 0xBB, 0xCC, 0xDD]);
/// ```
pub struct SizedSequence<T, S, F>(pub F, pub T)
where
    T: IntoIterator,
    T::Item: WriteInto,
    T::IntoIter: ExactSizeIterator,
    S: WriteInto,
    F: FnOnce(usize) -> S;

/// Returns how many items was written.
impl<T, S, F> WriteInto for SizedSequence<T, S, F>
where
    T: IntoIterator,
    T::Item: WriteInto,
    T::IntoIter: ExactSizeIterator,
    S: WriteInto,
    F: FnOnce(usize) -> S,
{
    type Output = usize;

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
        let iterator = self.1.into_iter();
        let size = iterator.len();

        write_into(sink, (self.0)(size))?;
        let mut written = 0;
        for item in iterator {
            item.write_into(sink)?;
            written += 1;
        }

        Ok(written)
    }
}

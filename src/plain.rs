use super::{WriteInto, write_into};
use std::io;
use std::mem::size_of;
use std::slice::from_raw_parts;

/// Used to write values as they are represented in memory.
/// 
/// # Examples
///
/// Writing struct into a sink.
/// 
/// ```
/// use write_into::{Plain, write_into};
/// 
/// struct Rgba {
///     r: u8,
///     g: u8,
///     b: u8,
///     a: u8,
/// }
/// 
/// let color = Rgba { r: 0x18, g: 0x18, b: 0x18, a: 0xFF };
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, Plain(&color)).unwrap();
/// assert_eq!(&buffer, &[0x18, 0x18, 0x18, 0xFF]);
/// ```
/// 
/// Writing array into a sink.
/// 
/// ```
/// use write_into::{Plain, write_into};
/// 
/// let bytes: &[u8; 4] = b"\0asm";
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, Plain(bytes)).unwrap();
/// assert_eq!(&buffer, b"\0asm");
/// ```
/// 
/// Writing slice into a sink (the crate also provide implementation for [`Plain<&str>`]).
/// 
/// ```
/// use write_into::{Plain, write_into};
/// 
/// let bytes: &[u8] = b"([java/lang/String;)V";
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, Plain(bytes)).unwrap();
/// assert_eq!(&buffer, b"([java/lang/String;)V");
/// ```
pub struct Plain<T>(pub T);

/// Transmutes arbitrary value into a byte slice.
impl<T> WriteInto for Plain<&T> {
    type Output = ();

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
        // SAFETY:
        // - The slice points to a memory occupied by the data.
        // - The data is immutably borrowed.
        let bytes = unsafe {
            let data = self.0 as *const T as *const u8;
            from_raw_parts(data, size_of::<T>())
        };

        sink.write_all(&bytes)?;
        Ok(())
    }
}

/// Transmutes arbitrary slice into a byte slice.
impl<T> WriteInto for Plain<&[T]> {
    type Output = ();

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
        // SAFETY:
        // - The slice points to a memory occupied by the data.
        // - The data is immutably borrowed.
        let bytes = unsafe {
            let data = self.0 as *const [T] as *const u8;
            from_raw_parts(data, self.0.len() * size_of::<T>())
        };

        sink.write_all(&bytes)?;
        Ok(())
    }
}

impl WriteInto for Plain<&str> {
    type Output = ();

    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
        sink.write_all(self.0.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn write_u8() {
        let mut buffer = Vec::new();
        write_into(&mut buffer, Plain(&0x7Fu8)).unwrap();
        assert_eq!(&buffer, &[0x7F]);
    }

    #[test]
    fn write_str() {
        let bytes = "([java/lang/String;)V";
        let mut buffer = Vec::new();
        write_into(&mut buffer, Plain(bytes)).unwrap();
        assert_eq!(&buffer, b"([java/lang/String;)V");
    }

    #[test]
    fn write_slice_of_arrays() {
        let bytes: &[[u8; 2]] = &[[0x01, 0x02], [0x03, 0x04]];
        let mut buffer = Vec::new();
        write_into(&mut buffer, Plain(bytes)).unwrap();
        assert_eq!(&buffer, &[0x01, 0x02, 0x03, 0x04]);
    }
}

macro_rules! impl_write_into {
    ($($primitive:ty)*) => {
        $(
            impl WriteInto for Plain<$primitive> {
                type Output = ();

                fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
                    write_into(sink, Plain(&self.0))
                }
            }
        )*
    };
}

impl_write_into! {
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    bool char f32 f64
}

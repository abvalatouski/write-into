use super::WriteInto;
use std::io;

/// Used to write values in big endian byte order.
///
/// # Example
///
/// ```
/// use write_into::{BigEndian, WriteInto, write_into};
///
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, BigEndian(0xCAFEBABEu32)).unwrap();
/// assert_eq!(&buffer, &[0xCA, 0xFE, 0xBA, 0xBE]);
/// ```
pub struct BigEndian<T>(pub T);

/// Used to write values in little endian byte order.
///
/// # Example
///
/// ```
/// use write_into::{LittleEndian, WriteInto, write_into};
///
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, LittleEndian(0xCAFEBABEu32)).unwrap();
/// assert_eq!(&buffer, &[0xBE, 0xBA, 0xFE, 0xCA]);
/// ```
pub struct LittleEndian<T>(pub T);

macro_rules! impl_write_into {
    ($($wrapper:ident => { $($primitive:ident)* } )*) => {
        $(
            $(
                impl WriteInto for $wrapper<$primitive> {
                    type Output = ();

                    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
                        let bytes = convertion!($wrapper, self.0);
                        sink.write_all(&bytes)?;
                        Ok(())
                    }
                }
            )*
        )*
    };
}

macro_rules! convertion {
    (BigEndian, $expr:expr) => {
        ($expr).to_be_bytes()
    };
    (LittleEndian, $expr:expr) => {
        ($expr).to_le_bytes()
    };
}

impl_write_into! {
    BigEndian => {
        i16 i32 i64 i128
        u16 u32 u64 u128
    }
    LittleEndian => {
        i16 i32 i64 i128
        u16 u32 u64 u128
    }
}

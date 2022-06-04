use super::{write_into, WriteInto};
use std::io;
use std::mem::size_of;

/// Used to write values in big endian byte order.
///
/// # Example
///
/// ```
/// use write_into::{BigEndian, write_into};
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
/// use write_into::{LittleEndian, write_into};
///
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, LittleEndian(0xCAFEBABEu32)).unwrap();
/// assert_eq!(&buffer, &[0xBE, 0xBA, 0xFE, 0xCA]);
/// ```
pub struct LittleEndian<T>(pub T);

macro_rules! impl_write_into {
    ($($wrapper:ident => { $($primitive:ident)* } ),*,) => {
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

                impl WriteInto for &$wrapper<$primitive> {
                    type Output = ();

                    fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
                        write_into(sink, $wrapper(self.0))
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
        i8 i16 i32 i64 i128 isize
        u8 u16 u32 u64 u128 usize
        bool char f32 f64
    },
    LittleEndian => {
        i8 i16 i32 i64 i128 isize
        u8 u16 u32 u64 u128 usize
        bool char f32 f64
    },
}

trait EndiannessExts {
    type Repr;
    fn to_be_bytes(self) -> Self::Repr;
    fn to_le_bytes(self) -> Self::Repr;
}

macro_rules! impl_endianness_exts {
    ($($primitive:ident => $repr:ident),*,) => {
        $(
            impl EndiannessExts for $primitive {
                type Repr = [u8; size_of::<Self>()];

                fn to_be_bytes(self) -> Self::Repr {
                    $repr::from(self).to_be_bytes()
                }

                fn to_le_bytes(self) -> Self::Repr {
                    $repr::from(self).to_le_bytes()
                }
            } 
        )*
    };
}

impl_endianness_exts! {
    char => u32,
    bool => u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_be() {
        assert_eq!('\x7F'.to_be_bytes(), 0x7Fu32.to_be_bytes());
    }
    
    #[test]
    fn char_le() {
        assert_eq!('\x7F'.to_le_bytes(), 0x7Fu32.to_le_bytes());
    }
}

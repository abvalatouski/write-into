use super::{write_into, WriteInto};
use std::io;
use std::mem::{size_of, MaybeUninit};

/// Used to write values in LEB-128 format _(unsigned)_.
///
/// # Example
///
/// ```
/// use write_into::{Uleb128, write_into};
///
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, Uleb128(69u32)).unwrap();
/// assert_eq!(&buffer, &[0x45]);
/// ```
pub struct Uleb128<T>(pub T);

/// Used to write values in LEB-128 format _(signed)_.
///
/// # Example
///
/// ```
/// use write_into::{Sleb128, write_into};
///
/// let mut buffer = Vec::new();
/// write_into(&mut buffer, Sleb128(-69i32)).unwrap();
/// assert_eq!(&buffer, &[0xBB, 0x7F]);
/// ```
pub struct Sleb128<T>(pub T);

macro_rules! impl_write_into {
    ($($wrapper:ident => { $($primitive:ident)* }),*,) => {
        $(
            $(
                impl_impl!($wrapper, $primitive);
            )*
        )*
    }
}

macro_rules! impl_impl {
    (Uleb128, $primitive:ident) => {
        impl WriteInto for Uleb128<$primitive> {
            type Output = usize;

            fn write_into(mut self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
                // SAFETY:
                // The uninitialized value is valid.
                let mut buffer = unsafe {
                    // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.uninit_array
                    MaybeUninit::<[MaybeUninit<u8>; max_leb128_size(size_of::<Self>())]>::uninit()
                        .assume_init()
                };

                let mut written = 0;
                for byte in buffer.iter_mut() {
                    let mut value = self.0 as u8 & 0x7F;
                    self.0 >>= 7;
                    if self.0 != 0 {
                        value |= 0x80;
                    }

                    *byte = MaybeUninit::new(value);
                    written += 1;

                    if self.0 == 0 {
                        break;
                    }
                }

                // SAFETY:
                // - The slice is initialized.
                let bytes = unsafe {
                    // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.slice_assume_init_ref
                    &*(&buffer[..written] as *const [MaybeUninit<u8>] as *const [u8])
                };

                sink.write_all(bytes)?;
                Ok(written)
            }
        }

        impl WriteInto for &Uleb128<$primitive> {
            type Output = usize;

            fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
                write_into(sink, Uleb128(self.0))
            }
        }
    };
    (Sleb128, $primitive:ident) => {
        impl WriteInto for Sleb128<$primitive> {
            type Output = usize;

            fn write_into(mut self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
                // SAFETY:
                // The uninitialized value is valid.
                let mut buffer = unsafe {
                    // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.uninit_array
                    MaybeUninit::<[MaybeUninit<u8>; max_leb128_size(size_of::<Self>())]>::uninit()
                        .assume_init()
                };

                let mut written = 0;
                for byte in buffer.iter_mut() {
                    let mut value = self.0 as u8;
                    self.0 >>= 6; // Keeping sign bit.
                    let done = self.0 == 0 || self.0 == -1;
                    if done {
                        value &= 0x7F;
                    } else {
                        self.0 >>= 1;
                        value |= 0x80;
                    }

                    *byte = MaybeUninit::new(value);
                    written += 1;

                    if done {
                        break;
                    }
                }

                // SAFETY:
                // - The slice is initialized.
                let bytes = unsafe {
                    // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.slice_assume_init_ref
                    &*(&buffer[..written] as *const [MaybeUninit<u8>] as *const [u8])
                };

                sink.write_all(bytes)?;
                Ok(written)
            }
        }

        impl WriteInto for &Sleb128<$primitive> {
            type Output = usize;

            fn write_into(self, sink: &mut impl io::Write) -> io::Result<Self::Output> {
                write_into(sink, Sleb128(self.0))
            }
        }
    };
}

impl_write_into! {
    Uleb128 => {
        u8 u16 u32 u64 u128 usize
    },
    Sleb128 => {
        i8 i16 i32 i64 i128 isize
    },
}

const fn max_leb128_size(bytes: usize) -> usize {
    let bits = bytes * 8;
    let septets = count_bits_in_chunks(bits, 7);
    let bits_for_septents = septets * 7;
    let bits_for_continutation_bits = septets * 1;
    count_bits_in_chunks(bits_for_septents + bits_for_continutation_bits, 8)
}

const fn count_bits_in_chunks(bits: usize, chunk_size: usize) -> usize {
    let chunks = bits / chunk_size;
    let remaining = bits % chunk_size;
    chunks + if remaining != 0 { 1 } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use test_case::test_case;
    use validators::vec;

    mod validators {
        pub fn vec(expected: &[u8]) -> impl FnOnce(Vec<u8>) {
            let expected = expected.to_vec();
            move |actual| assert_eq!(&expected, &actual)
        }
    }

    #[test_case(  1 =>  2; "when u8"   )]
    #[test_case(  2 =>  3; "when u16"  )]
    #[test_case(  4 =>  5; "when u32"  )]
    #[test_case(  8 => 10; "when u64"  )]
    #[test_case( 16 => 19; "when u128" )]
    fn max_leb128_size_for_primitive_types(bytes: usize) -> usize {
        max_leb128_size(bytes)
    }

    #[test_case(     0 => using vec(&[ 0x00             ]); "when     0" )]
    #[test_case(    69 => using vec(&[ 0x45             ]); "when    69" )]
    #[test_case(   123 => using vec(&[ 0x7B             ]); "when   123" )]
    #[test_case(   127 => using vec(&[ 0x7F             ]); "when   127" )]
    #[test_case(   128 => using vec(&[ 0x80, 0x01       ]); "when   128" )]
    #[test_case(   228 => using vec(&[ 0xE4, 0x01       ]); "when   228" )]
    #[test_case(   255 => using vec(&[ 0xFF, 0x01       ]); "when   255" )]
    #[test_case(  4200 => using vec(&[ 0xE8, 0x20       ]); "when  4200" )]
    #[test_case( 16383 => using vec(&[ 0xFF, 0x7F       ]); "when 16383" )]
    #[test_case( 32767 => using vec(&[ 0xFF, 0xFF, 0x01 ]); "when 32767" )]
    #[test_case( 42000 => using vec(&[ 0x90, 0xC8, 0x02 ]); "when 42000" )]
    #[test_case( 65535 => using vec(&[ 0xFF, 0xFF, 0x03 ]); "when 65535" )]
    fn write_u16(number: u16) -> Vec<u8> {
        let mut buffer = Vec::new();
        write_into(&mut buffer, Uleb128(number)).unwrap();
        buffer
    }

    #[test_case( -32768 => using vec(&[ 0x80, 0x80, 0x7E ]); "when  minus 32768" )]
    #[test_case(  -8192 => using vec(&[ 0x80, 0x40       ]); "when  minus  8192" )]
    #[test_case(  -4200 => using vec(&[ 0x98, 0x5F       ]); "when  minus  4200" )]
    #[test_case(   -128 => using vec(&[ 0x80, 0x7F       ]); "when  minus   128" )]
    #[test_case(   -123 => using vec(&[ 0x85, 0x7F       ]); "when  minus   123" )]
    #[test_case(    -69 => using vec(&[ 0xBB, 0x7F       ]); "when  minus    69" )]
    #[test_case(    -34 => using vec(&[ 0x5E             ]); "when  minus    34" )]
    #[test_case(      0 => using vec(&[ 0x00             ]); "when            0" )]
    #[test_case(     34 => using vec(&[ 0x22             ]); "when           34" )]
    #[test_case(     69 => using vec(&[ 0xC5, 0x00       ]); "when           69" )]
    #[test_case(    123 => using vec(&[ 0xFB, 0x00       ]); "when          123" )]
    #[test_case(    127 => using vec(&[ 0xFF, 0x00       ]); "when          127" )]
    #[test_case(   4200 => using vec(&[ 0xE8, 0x20       ]); "when         4200" )]
    #[test_case(   8191 => using vec(&[ 0xFF, 0x3F       ]); "when         8191" )]
    #[test_case(  32767 => using vec(&[ 0xFF, 0xFF, 0x01 ]); "when        32767" )]
    fn write_i16(number: i16) -> Vec<u8> {
        let mut buffer = Vec::new();
        write_into(&mut buffer, Sleb128(number)).unwrap();
        buffer
    }
}

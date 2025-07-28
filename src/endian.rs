use crate::converter::ByteConvertable;

#[derive(Debug, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub fn to_bytes<T, const N: usize>(&self, value: T) -> [u8; N]
    where
        T: ByteConvertable<N>,
    {
        match self {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        }
    }

    pub fn from_bytes<T, const N: usize>(&self, bytes: [u8; N]) -> T
    where
        T: ByteConvertable<N>,
    {
        match self {
            Endianness::Little => T::from_le_bytes(bytes),
            Endianness::Big => T::from_be_bytes(bytes),
        }
    }
}

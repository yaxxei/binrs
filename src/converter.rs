pub trait ByteConvertable<const N: usize>: Sized {
    fn to_le_bytes(self) -> [u8; N];
    fn from_le_bytes(bytes: [u8; N]) -> Self;

    fn to_be_bytes(self) -> [u8; N];
    fn from_be_bytes(bytes: [u8; N]) -> Self;
}

macro_rules! impl_convertable {
    ($($ty:ty => $len:literal), *) => {
        $(
            impl ByteConvertable<$len> for $ty {
                fn to_le_bytes(self) -> [u8; $len] {
                    self.to_le_bytes()
                }

                fn from_le_bytes(bytes: [u8; $len]) -> Self {
                    Self::from_le_bytes(bytes)
                }

                fn to_be_bytes(self) -> [u8; $len] {
                    self.to_be_bytes()
                }

                fn from_be_bytes(bytes: [u8; $len]) -> Self {
                    Self::from_be_bytes(bytes)
                }
            }
        )*
    };
}

impl_convertable!(
    i16 => 2,
    u16 => 2,
    i32 => 4,
    u32 => 4,
    i64 => 8,
    u64 => 8,
    f32 => 4,
    f64 => 8,
    i128 => 16,
    u128 => 16
);

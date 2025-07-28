use crate::endian::Endianness;

#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub endian: Endianness,
}

impl Context {
    pub fn new(endian: Endianness) -> Self {
        Self { endian }
    }
}

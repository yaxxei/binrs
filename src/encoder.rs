use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{context::Context, converter::ByteConvertable, endian::Endianness, error::Error};

pub trait Encoder {
    fn context(&self) -> Context;

    fn encode_bytes(&mut self, slice: &[u8]) -> Result<(), Error>;

    fn encode<T, const N: usize>(&mut self, value: T) -> Result<(), Error>
    where
        T: ByteConvertable<N>,
    {
        let bytes = self.context().endian.to_bytes(value);
        self.encode_bytes(bytes.as_ref())
    }

    fn encode_i8(&mut self, value: i8) -> Result<(), Error> {
        self.encode_bytes(&[value as u8])
    }

    fn encode_u8(&mut self, value: u8) -> Result<(), Error> {
        self.encode_bytes(&[value])
    }

    fn encode_i16(&mut self, value: i16) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_u16(&mut self, value: u16) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_i32(&mut self, value: i32) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_u32(&mut self, value: u32) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_i64(&mut self, value: i64) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_u64(&mut self, value: u64) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_i128(&mut self, value: i128) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_u128(&mut self, value: u128) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_f32(&mut self, value: f32) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_f64(&mut self, value: f64) -> Result<(), Error> {
        self.encode(value)
    }

    fn encode_bool(&mut self, value: bool) -> Result<(), Error> {
        self.encode_bytes(&[value as u8])
    }

    fn encode_string(&mut self, value: &str) -> Result<(), Error> {
        self.encode(value.len() as u32)?;
        self.encode_bytes(value.as_bytes())
    }
}

pub struct BufferEncoder {
    buffer: Vec<u8>,
    context: Context,
}

impl Encoder for BufferEncoder {
    fn context(&self) -> Context {
        self.context
    }

    fn encode_bytes(&mut self, slice: &[u8]) -> Result<(), Error> {
        Ok(self.buffer.extend_from_slice(slice))
    }
}

impl BufferEncoder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            context: Context::new(Endianness::Little),
        }
    }

    pub fn with_ctx(context: Context) -> Self {
        Self {
            buffer: Vec::new(),
            context,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }
}

pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error>;

    fn encode_to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut encoder = BufferEncoder::new();
        self.encode(&mut encoder)?;
        Ok(encoder.into_bytes())
    }

    fn encode_with_ctx(&self, ctx: Context) -> Result<Vec<u8>, Error> {
        let mut encoder = BufferEncoder::with_ctx(ctx);
        self.encode(&mut encoder)?;
        Ok(encoder.into_bytes())
    }
}

impl Encode for i8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_i8(*self)
    }
}

impl Encode for u8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u8(*self)
    }
}

impl Encode for i16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_i16(*self)
    }
}

impl Encode for u16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u16(*self)
    }
}

impl Encode for i32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_i32(*self)
    }
}

impl Encode for u32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u32(*self)
    }
}

impl Encode for i64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_i64(*self)
    }
}

impl Encode for u64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u64(*self)
    }
}

impl Encode for i128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_i128(*self)
    }
}

impl Encode for u128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u128(*self)
    }
}

impl Encode for usize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u64(*self as u64)
    }
}

impl Encode for f32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_f32(*self)
    }
}

impl Encode for f64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_f64(*self)
    }
}

impl Encode for bool {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_bool(*self)
    }
}

impl Encode for char {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u32(*self as u32)
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_string(self)
    }
}

impl Encode for str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_string(self)
    }
}

impl<T: Encode> Encode for (T, T) {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)
    }
}

impl<T: Encode> Encode for (T, T, T) {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.0.encode(encoder)?;
        self.1.encode(encoder)?;
        self.2.encode(encoder)
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            Some(value) => {
                encoder.encode_u8(1)?;
                value.encode(encoder)
            }
            None => encoder.encode_u8(0),
        }
    }
}

impl<T: Encode, Er: Encode> Encode for Result<T, Er> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            Ok(value) => {
                encoder.encode_u8(1)?;
                value.encode(encoder)
            }
            Err(err) => {
                encoder.encode_u8(0)?;
                err.encode(encoder)
            }
        }
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u32(self.len() as u32)?;
        for item in self {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for HashSet<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u32(self.len() as u32)?;
        for item in self {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: Encode> Encode for BTreeSet<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u32(self.len() as u32)?;
        for item in self {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<K: Encode, V: Encode> Encode for HashMap<K, V> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u32(self.len() as u32)?;
        for (key, value) in self {
            key.encode(encoder)?;
            value.encode(encoder)?;
        }
        Ok(())
    }
}

impl<K: Encode, V: Encode> Encode for BTreeMap<K, V> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.encode_u32(self.len() as u32)?;
        for (key, value) in self {
            key.encode(encoder)?;
            value.encode(encoder)?;
        }
        Ok(())
    }
}

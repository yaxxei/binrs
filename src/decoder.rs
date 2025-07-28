use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{context::Context, converter::ByteConvertable, endian::Endianness, error::Error};

pub trait Decoder {
    fn context(&self) -> Context;

    fn decode_bytes(&mut self, len: usize) -> Result<&[u8], Error>;

    fn decode<T, const N: usize>(&mut self) -> Result<T, Error>
    where
        T: ByteConvertable<N>,
    {
        let bytes = self.decode_bytes(std::mem::size_of::<T>())?;
        let bytes: [u8; N] = bytes.try_into().map_err(|_| "Invalid Length")?;
        Ok(self.context().endian.from_bytes(bytes))
    }

    fn decode_i8(&mut self) -> Result<i8, Error> {
        let bytes = self.decode_bytes(1)?;
        Ok(bytes[0] as i8)
    }

    fn decode_u8(&mut self) -> Result<u8, Error> {
        let bytes = self.decode_bytes(1)?;
        Ok(bytes[0])
    }

    fn decode_i16(&mut self) -> Result<i16, Error> {
        self.decode()
    }

    fn decode_u16(&mut self) -> Result<u16, Error> {
        self.decode()
    }

    fn decode_i32(&mut self) -> Result<i32, Error> {
        self.decode()
    }

    fn decode_u32(&mut self) -> Result<u32, Error> {
        self.decode()
    }

    fn decode_i64(&mut self) -> Result<i64, Error> {
        self.decode()
    }

    fn decode_u64(&mut self) -> Result<u64, Error> {
        self.decode()
    }

    fn decode_i128(&mut self) -> Result<i128, Error> {
        self.decode()
    }

    fn decode_u128(&mut self) -> Result<u128, Error> {
        self.decode()
    }

    fn decode_usize(&mut self) -> Result<u128, Error> {
        self.decode()
    }

    fn decode_f32(&mut self) -> Result<f32, Error> {
        self.decode()
    }

    fn decode_f64(&mut self) -> Result<f64, Error> {
        self.decode()
    }

    fn decode_bool(&mut self) -> Result<bool, Error> {
        let bytes = self.decode_bytes(1)?;
        Ok(bytes[0] != 0)
    }

    fn decode_string(&mut self) -> Result<String, Error> {
        let len = self.decode_u32()? as usize;
        let bytes = self.decode_bytes(len)?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }
}

pub struct BufferDecoder<'a> {
    buffer: &'a [u8],
    position: usize,
    context: Context,
}

impl<'a> Decoder for BufferDecoder<'a> {
    fn context(&self) -> Context {
        self.context
    }

    fn decode_bytes(&mut self, len: usize) -> Result<&[u8], Error> {
        if self.position + len > self.buffer.len() {
            return Err("Not enough bytes to decode".into());
        }

        let slice = &self.buffer[self.position..self.position + len];
        self.position += len;
        Ok(slice)
    }
}

impl<'a> BufferDecoder<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            position: 0,
            context: Context::new(Endianness::Little),
        }
    }

    pub fn with_ctx(buffer: &'a [u8], context: Context) -> Self {
        Self {
            buffer,
            position: 0,
            context,
        }
    }

    pub fn remaining(&self) -> usize {
        self.buffer.len().saturating_sub(self.position)
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, pos: usize) {
        self.position = pos.min(self.buffer.len());
    }
}

pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error>;

    fn decode_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut decoder = BufferDecoder::new(bytes);
        Self::decode(&mut decoder)
    }

    fn decode_with_ctx(bytes: &[u8], ctx: Context) -> Result<Self, Error> {
        let mut decoder = BufferDecoder::with_ctx(bytes, ctx);
        Self::decode(&mut decoder)
    }
}

impl Decode for u8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_u8()
    }
}

impl Decode for i8 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_i8()
    }
}

impl Decode for u16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_u16()
    }
}

impl Decode for i16 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_i16()
    }
}

impl Decode for u32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_u32()
    }
}

impl Decode for i32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_i32()
    }
}

impl Decode for u64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_u64()
    }
}

impl Decode for i64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_i64()
    }
}

impl Decode for u128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_u128()
    }
}

impl Decode for i128 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_i128()
    }
}

impl Decode for f32 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_f32()
    }
}

impl Decode for f64 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_f64()
    }
}

impl Decode for bool {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_bool()
    }
}

impl Decode for String {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        decoder.decode_string()
    }
}

impl<T: Decode> Decode for (T, T) {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let a = T::decode(decoder)?;
        let b = T::decode(decoder)?;
        Ok((a, b))
    }
}

impl<T: Decode> Decode for (T, T, T) {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let a = T::decode(decoder)?;
        let b = T::decode(decoder)?;
        let c = T::decode(decoder)?;
        Ok((a, b, c))
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let tag = u8::decode(decoder)?;
        match tag {
            0 => Ok(None),
            1 => Ok(Some(T::decode(decoder)?)),
            _ => Err("Invalid Option Tag".into()),
        }
    }
}

impl<T: Decode, E: Decode> Decode for Result<T, E> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let tag = u8::decode(decoder)?;
        match tag {
            1 => Ok(Ok(T::decode(decoder)?)),
            0 => Ok(Err(E::decode(decoder)?)),
            _ => Err("Invalid Result Tag".into()),
        }
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let len = u32::decode(decoder)? as usize;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(decoder)?);
        }
        Ok(vec)
    }
}

impl<T: Decode + Eq + std::hash::Hash> Decode for HashSet<T> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let len = u32::decode(decoder)? as usize;
        let mut set = HashSet::with_capacity(len);
        for _ in 0..len {
            set.insert(T::decode(decoder)?);
        }
        Ok(set)
    }
}

impl<T: Decode + Ord> Decode for BTreeSet<T> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let len = u32::decode(decoder)? as usize;
        let mut set = BTreeSet::new();
        for _ in 0..len {
            set.insert(T::decode(decoder)?);
        }
        Ok(set)
    }
}

impl<K: Decode + Eq + std::hash::Hash, V: Decode> Decode for HashMap<K, V> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let len = u32::decode(decoder)? as usize;
        let mut map = HashMap::with_capacity(len);
        for _ in 0..len {
            map.insert(K::decode(decoder)?, V::decode(decoder)?);
        }
        Ok(map)
    }
}

impl<K: Decode + Ord, V: Decode> Decode for BTreeMap<K, V> {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, Error> {
        let len = u32::decode(decoder)? as usize;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            map.insert(K::decode(decoder)?, V::decode(decoder)?);
        }
        Ok(map)
    }
}

use std::convert::TryInto;

use anyhow::Result;
pub use bincode::{Encode as BinEncode, Decode as BinDecode};
use primitive_types::{H160, H256};

pub trait ConsensusCodec: Sized{
    fn consensus_encode(self) -> Vec<u8>;
    fn consensus_decode(buf: &[u8]) -> Result<Self>;
}

pub trait Encodable: Sized{
    fn encode(&self) -> Result<Vec<u8>>;
}

pub trait Decodable: Sized{
    fn decode(buf: &[u8]) -> Result<Self>;
}

pub trait Codec: Encodable + Decodable {}

impl<T> Codec for T where T: Encodable + Decodable {}

type Hash = [u8; 32];

impl Encodable for Hash {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.to_vec())
    }
}

impl Decodable for Hash {
    fn decode(buf: &[u8]) -> Result<Self> {
        let mut buff = [0; 32];
        buff.copy_from_slice(buf);
        Ok(buff)
    }
}

macro_rules! impl_codec_primitives {
    ($name : ident) => {
        impl Encodable for $name {
            fn encode(&self) -> Result<Vec<u8>> {
                Ok(self.to_be_bytes().to_vec())
            }
        }

        impl Decodable for $name {
            fn decode(buf: &[u8]) -> Result<$name> {
                Ok($name::from_be_bytes(buf.try_into()?))
            }
        }
    };
}

impl_codec_primitives!(u8);
impl_codec_primitives!(u16);
impl_codec_primitives!(u32);
impl_codec_primitives!(u64);
impl_codec_primitives!(u128);

impl Encodable for String {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}

impl Decodable for String {
    fn decode(buf: &[u8]) -> Result<Self> {
        Ok(String::from_utf8_lossy(buf).to_string())
    }
}

impl Encodable for H160 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}

impl Decodable for H160 {
    fn decode(buf: &[u8]) -> Result<Self> {
        Ok(H160::from_slice(buf))
    }
}

impl Encodable for H256 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}

impl Decodable for H256 {
    fn decode(buf: &[u8]) -> Result<Self> {
        Ok(H256::from_slice(buf))
    }
}

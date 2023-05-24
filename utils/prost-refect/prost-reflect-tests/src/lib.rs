use once_cell::sync::Lazy;
use prost_reflect::DescriptorPool;

#[cfg(test)]
mod arbitrary;
#[cfg(test)]
mod decode;
#[cfg(test)]
mod desc;
#[cfg(test)]
mod json;
#[cfg(test)]
mod text_format;

pub mod proto {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
    include!(concat!(env!("OUT_DIR"), "/test2.rs"));

    pub mod options {
        include!(concat!(env!("OUT_DIR"), "/custom.options.rs"));
    }
}

const DESCRIPTOR_POOL_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));

static TEST_DESCRIPTOR_POOL: Lazy<DescriptorPool> =
    Lazy::new(|| DescriptorPool::decode(DESCRIPTOR_POOL_BYTES).unwrap());

pub fn test_file_descriptor() -> DescriptorPool {
    TEST_DESCRIPTOR_POOL.clone()
}

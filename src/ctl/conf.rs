use std::sync::RwLock;
use crate::{Conf, Error};
use async_trait::async_trait;

pub struct Rconf{
    // (stripe_size, revision)
    stripes: RwLock<Vec<Option<u32>>>,
    // (chunk_size, revision)
    chunks: RwLock<Vec<Option<u32>>>,
}

#[async_trait]
impl Conf for Rconf{
    async fn oc_create_chunk_type(&self, chunk_size: u32) -> Result<u8, Error>{
        todo!();
    }

    async fn oc_create_stripe_type(&self, stripe_cnt: u32) -> Result<u8, Error>{
        todo!();
    }

    async fn oc_get(&self, stripe_type: u8, chunk_size: u8) -> Result<Option<(u32, u32)>, Error>{
        todo!();
    }

    async fn oc_get_stripe(&self, stripe_type: u8) -> Result<Option<u32>, Error>{
        todo!();
    }

    async fn oc_get_chunk(&self, chunk_type: u8) -> Result<Option<u32>, Error>{
        todo!();
    }

    async fn kv_get(&self, key: &str) -> Result<Option<String>, Error>{
        todo!();
    }

    async fn kv_put(&self, key: &str, value: &str) -> Result<(), Error>{
        todo!();
    }

    async fn kv_get_all(&self, prefix: &str) -> Result<Vec<(String, String)>, Error>{
        todo!();
    }
}

use raft::StorageError;
use std::error;
use super::peer_traits::{KvEngine, RaftEngine};
use super::common::*;

#[derive(Clone, Debug)]
pub struct Engines<K, R>{
    pub kv: K,
    pub raft: R,
}

impl<K: KvEngine, R: RaftEngine> Engines<K, R>{
    pub fn new(kv_engine: K, raft_engine: R) -> Self{
        Engines{
            kv: kv_engine,
            raft: raft_engine,
        }
    }
    
    pub fn write_kv(&self) -> Result<()>{
        todo!();
    }

}

pub struct RaftMessage{

}

pub struct RaftCmdRequest{

}

pub fn storage_error<E>(error: E)-> raft::Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    raft::Error::Store(StorageError::Other(error.into()))
}



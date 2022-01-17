use std::fs;
use std::path::Path;
use std::sync::Arc;
use rocksdb::{DBIterator, DB};
use raft::eraftpb::Entry;
use super::peer_traits::{KvEngine, Operation, RaftEngine};
use super::common::*;


#[derive(Clone, Debug)]
pub struct BasicEngine{
    db: Arc<DB>,
    shared_block_cache: bool,
}

impl BasicEngine{
    ///create a storage instance from RocksDB
    pub fn from_db(db: Arc<DB>) -> Self{
        BasicEngine{
            db,
            shared_block_cache: false,
        }
    }
    pub fn from_ref(db: &Arc<DB>) -> &Self{
        unsafe{&*(db as *const Arc<DB> as *const BasicEngine)}
    }

    ///get the inner RocksDB instance
    pub fn as_inner(&self) -> &Arc<DB>{
        &self.db
    }

    pub fn get_sync_db(&self) -> Arc<DB>{
        self.db.clone()
    }

    ///check whether the path exists
    pub fn exists(path: &str) -> bool{
        let path = Path::new(path);
        if !path.exists() || !path.is_dir(){
            return false;
        }
        fs::read_dir(&path).unwrap().next().is_some()
    }

    pub fn set_shared_block_cache(&mut self, enable: bool){
        self.shared_block_cache = enable;
    }
}

impl KvEngine for BasicEngine{

}

impl Operation for BasicEngine{
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>{
        match self.db.put(key, value){
            Ok(_) => Ok(()),
            Err(_) => Err(Error::Engine("put error".to_owned())),
        }
    }
    fn delete(&self, key: &[u8]) -> Result<()>{
        match self.db.delete(key){
            Ok(_) => Ok(()),
            Err(_) => Err(Error::Engine("delete error".to_owned())),
        }
    }
    fn delete_range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<()>{
        todo!("wrap rocksDB to support this range delection");
    }
    ///todo: unwrap error handle
    fn put_msg<M: protobuf::Message>(&self, key: &[u8], m: &M) -> Result<()>{
        self.put(key, &m.write_to_bytes().unwrap())
    }
}


impl RaftEngine for BasicEngine{
    fn get_entry(&self, raft_group_id: u64, index: u64)->Result<Option<Entry>>{
        todo!();
    }
    fn append(&mut self, raft_group_id: u64, entries: Vec<Entry>) -> Result<()>{
        todo!();
    }
    fn cut_logs(&mut self, raft_group_id: u64, from: u64, to: u64){
        todo!();
    }
}


#[cfg(test)]
mod tests{
    fn test_operation(){
        todo!("write basic operation tests");
    }
}




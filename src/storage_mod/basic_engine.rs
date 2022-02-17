use std::fs;
use std::path::Path;
use std::sync::Arc;
use rocksdb::{DBIterator, DB};
use raft::eraftpb::Entry;
use super::peer_traits::{KvEngine, RaftEngine};
use super::{common::*, utils::*};
use crate::{ClusterMapVersion, ChangeLog};

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
    fn get_range(&self, from: ClusterMapVersion, to: Option<ClusterMapVersion>,) ->std::result::Result<(Vec<ChangeLog>, i64), Error>{
        todo!();
    }
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>{
        match self.db.put(key, value){
            Ok(_) => Ok(()),
            Err(_) => Err(Error::Engine("put error".to_owned())),
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>>{
        match self.db.get(key){
            Ok(Some(value)) => return Some(value),
            Ok(None) => return None,
            Err(e) => return None,
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
    ///todo: error handle should be included
    fn put_msg<M: protobuf::Message>(&self, key: &[u8], m: &M) -> Result<()>{
        self.put(key, &m.write_to_bytes().unwrap())
    }

}


impl RaftEngine for BasicEngine{
    fn get_entry(&self, index: u64)->Result<Option<Entry>>{
        todo!();
    }
    fn append(&mut self, entries: Vec<Entry>) -> Result<()>{
        todo!();
    }
    fn cut_logs(&mut self, from: u64, to: u64){
        todo!();
    }
    fn fetch_entries_to(&self, begin: u64, end: u64, max_size: Option<usize>, to: &mut Vec<Entry>) -> Result<usize> {
        todo!();
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use tempfile::Builder;
    #[test]
    fn test_basic_op(){
        let path = Builder::new().prefix("var").tempdir().unwrap();
        let db = DB::open_default(path).unwrap();
        let engine = BasicEngine::from_db(Arc::new(db));
        let (key1, v1) = (b"key1", b"value1");
        let (key2, v2) = (b"key2", b"value2");
        let (key3, v3) = (b"key3", b"value3");
        engine.put(key1, v1).unwrap();
        assert_eq!("value1".to_string(), String::from_utf8(engine.get(key1).unwrap()).unwrap());
        engine.put(key2, v2).unwrap();
        engine.put(key3, v3).unwrap();
        assert_eq!("value2".to_string(), String::from_utf8(engine.get(key2).unwrap()).unwrap());
        assert_eq!("value3".to_string(), String::from_utf8(engine.get(key3).unwrap()).unwrap());
        engine.delete(key2).unwrap();
        assert_eq!(None, engine.get(key2));
    }

    #[test]
    fn test_rocksdb(){
        let path = Builder::new().prefix("var").tempdir().unwrap();
        let db = DB::open_default(path).unwrap();
        db.put(b"key", b"value").unwrap();
        assert_eq!("value".to_string(), String::from_utf8(db.get(b"key").unwrap().unwrap()).unwrap());
    }



    
}




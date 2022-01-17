use super::common::*;
use raft::eraftpb::Entry;

pub trait KvEngine{

}

pub trait RaftEngine: Sync + Send + Clone + 'static{
    fn get_entry(&self, raft_group_id: u64, index: u64)->Result<Option<Entry>>;
    fn append(&mut self, raft_group_id: u64, entries: Vec<Entry>) -> Result<()>;
    fn cut_logs(&mut self, raft_group_id: u64, from: u64, to: u64);
    
}


pub trait Operation{
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&self, key: &[u8]) -> Result<()>;
    fn delete_range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<()>;
    fn put_msg<M: protobuf::Message>(&self, key: &[u8], m: &M) -> Result<()>;
}


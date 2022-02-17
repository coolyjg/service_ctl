
use super::common::*;
use raft::eraftpb::Entry;
use crate::{ClusterMapVersion, ChangeLog};

pub trait KvEngine: Send + Sync{
    fn get_range(&self, from: ClusterMapVersion, to: Option<ClusterMapVersion>,) ->std::result::Result<(Vec<ChangeLog>, i64), Error>;
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn delete(&self, key: &[u8]) -> Result<()>;
    fn delete_range(&self, begin_key: &[u8], end_key: &[u8]) -> Result<()>;
    fn put_msg<M: protobuf::Message>(&self, key: &[u8], m: &M) -> Result<()>;
}

pub trait RaftEngine: Sync + Send + Clone + 'static{
    fn get_entry(&self, index: u64)->Result<Option<Entry>>;
    fn append(&mut self, entries: Vec<Entry>) -> Result<()>;
    fn cut_logs(&mut self, from: u64, to: u64);
    fn fetch_entries_to(&self, begin: u64, end: u64, max_size: Option<usize>, to: &mut Vec<Entry>,) -> Result<usize>;
    
}




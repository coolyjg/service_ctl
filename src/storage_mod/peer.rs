use super::basic_engine::*;
use super::peer_traits::*;
use raft::{self, RaftState, Ready, Storage, StorageError};

pub struct PeerStorage<EK, ER>
where  
    EK: KvEngine,
{
    pub engine: Engines<EK, ER>,
    peer_id: u64,
    applied_index_term: u64,
    last_term: u64,

    pub tag: String,

}

impl<EK, ER> Storage for PeerStorage<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    fn initial_state(&self) -> Result<RaftState>{
        todo!();
    }

    fn entries(&self, low: u64, high: u64, max_size: impl Into<Option<u64>>) -> Result<Vec<Entry>>{
        todo!();
    }

    fn term(&self, idx: u64) -> Result<u64>{
        todo!();
    }

    fn first_index(&self) -> Result<u64>{
        todo!();
    }

    fn last_index(&self) -> Result<u64>{
        todo!();
    }

    fn snapshot(&self, request_index: u64) -> Result<Snapshot>{
        todo!();
    }
}



use super::peer_traits::*;
use super::utils::*;
use super::common::Result;
use raft::{self, RaftState, Storage};
use raft::eraftpb::{Snapshot, Entry};
pub struct PeerStorage<EK, ER>
where  
    EK: KvEngine,
{
    pub engine: Engines<EK, ER>,
    //peer_id: u64,  //necessary?
    applied_index_term: u64,
    last_term: u64,

    ///this might be a human readable msg
    pub tag: String, 
}

impl<EK, ER> PeerStorage<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    pub fn new(
        engines: Engines<EK, ER>,
        //peer_id: u64,
        tag: String,
    ) -> Result<PeerStorage<EK, ER>>{
        let last_term = 5;
        let applied_index_term = 5; //Todo: get this two info. from engine
        Ok(PeerStorage{
            engine: engines,
            //peer_id: peer_id,
            applied_index_term: applied_index_term,
            last_term: last_term,
            tag: tag,
        })
    }
}


impl<EK, ER> Storage for PeerStorage<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    fn initial_state(&self) -> raft::Result<RaftState>{
        todo!();
    }

    fn entries(&self, low: u64, high: u64, max_size: impl Into<Option<u64>>) -> raft::Result<Vec<Entry>>{
        todo!();
    }

    fn term(&self, idx: u64) -> raft::Result<u64>{
        todo!();
    }

    fn first_index(&self) -> raft::Result<u64>{
        todo!();
    }

    fn last_index(&self) -> raft::Result<u64>{
        todo!();
    }

    fn snapshot(&self, request_index: u64) -> raft::Result<Snapshot>{
        todo!();
    }
}
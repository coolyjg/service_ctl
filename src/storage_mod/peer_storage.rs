use super::peer_traits::*;
use super::utils::*;
use super::common::Result;
use raft::StorageError;
use raft::eraftpb::HardState;
use raft::{self, RaftState, Storage};
use raft::eraftpb::{Snapshot, Entry};
use raft::Error as RaftError;

pub struct PeerStorage<EK, ER>
where  
    EK: KvEngine,
    ER: RaftEngine,
{
    pub engines: Engines<EK, ER>,
    //peer_id: u64,  //necessary?
    applied_index_term: u64,
    last_term: u64,
    raft_state: RaftState,
    
    first_index: u64,
    last_index: u64,

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
            //peer_id: peer_id,
            engines: engines,
            applied_index_term: applied_index_term,
            last_term: last_term,
            raft_state: Default::default(),
            first_index: 0,
            last_index: 0,
            tag: tag,
        })
    }

    pub fn hard_state(&self) -> &HardState{
        &self.raft_state.hard_state
    }

    fn check_range(&self, low: u64, high: u64)-> raft::Result<()>{
        if low > high{
            return Err(storage_error(format!(
                "low: {} is greater than high: {}",
                low, high
            )));
        }else if low <= self.first_index{
            return Err(RaftError::Store(StorageError::Compacted));
        }else if high > self.last_index + 1{
            return Err(storage_error(format!(
                "{} out of bound {}",
                high, self.last_index
            )))
        }
        Ok(())
    }



    pub fn initial_state(&self) -> raft::Result<RaftState>{
        Ok(self.raft_state.clone())
    }

    pub fn entries(&self, low: u64, high: u64, max_size: u64)-> raft::Result<Vec<Entry>>{
        self.check_range(low, high)?;
        let mut entries = Vec::with_capacity((high - low) as usize);
        if low == high{
            return Ok(entries);
        }
        self.engines.raft.fetch_entries_to(
            low,
            high,
            Some(max_size as usize),
            &mut entries,
        )?;
        return Ok(entries);
    }
    
    pub fn term(&self, idx: u64) -> raft::Result<u64>{
        //todo: add snapshot index/term jugdement
        self.check_range(idx, idx + 1)?;
        Ok(self.engines
            .raft
            .get_entry(idx)
            .unwrap()
            .unwrap()
            .get_term()
        )
    }


}


impl<EK, ER> Storage for PeerStorage<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    fn initial_state(&self) -> raft::Result<RaftState>{
        self.initial_state()
    }

    fn entries(&self, low: u64, high: u64, max_size: impl Into<Option<u64>>) -> raft::Result<Vec<Entry>>{
        self.entries(low, high, max_size.into().unwrap_or(u64::MAX))
    }

    fn term(&self, idx: u64) -> raft::Result<u64>{
        self.term(idx)
    }

    fn first_index(&self) -> raft::Result<u64>{
        Ok(self.first_index)
    }

    fn last_index(&self) -> raft::Result<u64>{
        Ok(self.last_index)
    }

    fn snapshot(&self, request_index: u64) -> raft::Result<Snapshot>{
        todo!();
    }
}
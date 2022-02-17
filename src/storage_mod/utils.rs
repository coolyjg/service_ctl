use raft::StorageError;
use raft::eraftpb::Snapshot;
use std::collections::VecDeque;
use std::error;
use std::f32::consts::E;
use super::peer_traits::{KvEngine, RaftEngine};
use super::common::*;
use time::Timespec;

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

#[derive(Debug)]
pub struct Proposal{
    pub is_conf_change: bool,
    pub index: u64,
    pub term: u64,
    pub propose_time: Option<Timespec>,
    pub must_pass_epoch_check: bool,
}

#[derive(Debug)]
struct ProposalQueue{
    tag: String,
    queue: VecDeque<Proposal>,
}

impl ProposalQueue{
    fn new(tag: String) -> ProposalQueue{
        ProposalQueue{
            tag,
            queue: VecDeque::new(),
        }
    }

    ///find the given term and index proposal
    fn pop(&mut self, term: u64, index: u64) -> Option<Proposal>{
        self.queue.pop_front().and_then(|p|{
            if (p.term, p.index) > (term, index){
                self.queue.push_front(p);
                return None;
            }
            Some(p)
        })
    }

    fn find_proposal(&mut self, term: u64, index: u64, current_term:u64) -> Option<Proposal>{
        while let Some(p) = self.pop(term, index){
            if p.term == term{
                if p.index == index{
                    return Some(p);
                }else{
                    panic!("expect index {}, find index {}",
                    index, p.index
                );}
            }else{
                todo!("reply stale request")
            }
        }
        None
    }

    fn push(&mut self, p: Proposal){
        if let Some(f) = self.queue.back(){
            assert!((p.term, p.index) > (f.term, f.index));
        }
        self.queue.push_back(p);
    }

    fn is_empty(&self) -> bool{
        self.queue.is_empty()
    }

    fn back(&self) -> Option<&Proposal>{
        self.queue.back()
    }


}



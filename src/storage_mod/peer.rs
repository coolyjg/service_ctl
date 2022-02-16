use super::basic_engine::*;
use super::peer_traits::*;
use super::peer_storage::*;
use super::common::*;
use raft::StateRole;
use raft::eraftpb;
use raft::{self, RaftState, Ready, Storage, StorageError, RawNode, Config};
use super::utils::*;
use std::collections::HashMap;
use madsim::time::Instant;
use madsim::net;


pub struct Peer<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    ///raft state machine of this peer
    pub raft_group: RawNode<PeerStorage<EK, ER>>,

    pub tag: String,
    pub peer_heartbeats: HashMap<u64, Instant>,

}

impl<EK, ER> Peer<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    pub fn new(
        cfg: &Config,
        engines: Engines<EK, ER>,
    ) -> Result<Peer<EK, ER>>{
        let logger = slog_global::get_global().new(slog::o!("invalid msg"=>""));
        let tag = format!("Invalid msg");
        let ps = PeerStorage::new(engines, tag)?;
        //todo: add error handle function to `unwrap()`
        let raft_group = RawNode::new(cfg, ps, &logger).unwrap();
        let tag = format!("raft peer");
        let peer = Peer{
            raft_group: raft_group,
            tag: tag.clone(),
            peer_heartbeats: HashMap::new(),
        };
        Ok(peer)
    }

    #[inline]
    fn next_proposal_index(&self) -> u64{
        self.raft_group.raft.raft_log.last_index() +1
    }
    
    #[inline]
    pub fn get_index_term(&self, idx: u64) -> u64{
        match self.raft_group.raft.raft_log.term(idx){
            Ok(t) => t,
            Err(e) => panic!("{} fail to load term for {} : {:?}", self.tag, idx, e),
        }
    }

    #[inline]
    pub fn leader_id(&self) -> u64{
        self.raft_group.raft.leader_id
    }

    #[inline]
    pub fn is_leader(&self) -> bool{
        self.raft_group.raft.state == StateRole::Leader
    }

    #[inline]
    pub fn get_role(&self) -> StateRole{
        self.raft_group.raft.state
    }

    #[inline]
    pub fn get_store(&self) -> &PeerStorage<EK, ER>{
        self.raft_group.store()
    }

    /// Leader broadcast heartbeat to followers
    pub fn ping(&mut self){
        if self.is_leader(){
            self.raft_group.ping();
        }
    }

    pub fn has_uncommitted_log(&self) -> bool{
        self.raft_group.raft.raft_log.committed < self.raft_group.raft.raft_log.last_index()
    }

    #[inline]
    /// get mutable reference to the store
    pub fn mut_store(&mut self) -> &mut PeerStorage<EK, ER>{
        self.raft_group.mut_store()
    }

    #[inline]
    pub fn has_pending_snapshot(&self) -> bool{
        self.get_pending_snapshot().is_some()
    }

    #[inline]
    pub fn get_pending_snapshot(&self) -> Option<&eraftpb::Snapshot>{
        self.raft_group.snap()
    }

    #[inline]
    pub fn send_raft_messages(&mut self, msgs: Vec<RaftMessage>,){
        todo!("redefine RaftMessage struct to complemente this");
    }

    #[inline]
    pub fn build_raft_messages(&mut self, msgs: Vec<eraftpb::Message>,) -> Vec<RaftMessage>{
        todo!();
    }

    ///drive raft process
    pub fn step(&mut self, mut m: eraftpb::Message,) -> Result<()>{
        todo!();
    }

    pub fn propose(&mut self, req: RaftCmdRequest) -> bool{
        todo!();
    }

    



}







use super::basic_engine::*;
use super::peer_traits::*;
use super::peer_storage::*;
use raft::{self, RaftState, Ready, Storage, StorageError, RawNode};
use super::utils::*;
use std::collections::HashMap;
use madsim::time::Instant;
use madsim::net::SocketAddr;



pub struct Peer<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    ///raft state machine of this peer
    pub raft_group: RawNode<PeerStorage<EK, ER>>,
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
        let ps = PeerStorage::new(engines, tag);
        let raft_group = RawNode::new(&cfg, ps, &logger);
        let peer = Peer{
            raft_group: raft_group,
            peer_heartbreats: HashMap::new(),
        };
        Ok(peer)
    }
}



#[madsim::test]
fn test(){

}




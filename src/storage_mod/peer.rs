use super::basic_engine::*;
use super::peer_traits::*;
use super::peer_storage::*;
use super::common::*;
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
    pub peer_heartbeats: HashMap<u64, Instant>,

}

impl<EK, ER> Peer<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    pub fn new(
        cfg: &Config,
        kengine: EK,
        rengine: ER,
    ) -> Result<Peer<EK, ER>>{
        let logger = slog_global::get_global().new(slog::o!("invalid msg"=>""));
        let tag = format!("Invalid msg");
        let ps = PeerStorage::new(rengine, kengine, tag)?;
        //todo: add error handle function to `unwrap()`
        let raft_group = RawNode::new(cfg, ps, &logger).unwrap();
        let peer = Peer{
            raft_group: raft_group,
            peer_heartbeats: HashMap::new(),
        };
        Ok(peer)
    }
}





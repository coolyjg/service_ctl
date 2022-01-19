use std::net::SocketAddr;
use std::sync::{RwLock, Arc};
use crate::{storage_mod::*, ClusterMap};


use uuid::Uuid;

use super::Rconf;


pub struct ServerCtl<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    uuid: Uuid,
    url: SocketAddr,
    raft_peer: Peer<EK, ER>,
    conf: Rconf,
    curr_map: RwLock<Arc<ClusterMap>>,

}






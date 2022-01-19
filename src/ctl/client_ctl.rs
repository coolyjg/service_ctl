use std::sync::{RwLock, Arc};

use tokio::sync::Mutex;

use crate::{storage_mod::{Peer, KvEngine, RaftEngine}, ClusterMap};

use super::Rconf;




pub struct ClientCtl<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    raft_peer: Peer<EK, ER>,
    curr_map: RwLock<Arc<ClusterMap>>,
    update_lock: Mutex<()>,
    conf: Rconf,

}


use std::sync::{RwLock, Arc};
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::{storage_mod::{Peer, KvEngine, RaftEngine}, ClusterMap, ClusterMapVersion, Error, Conf};
use super::Rconf;
use crate::ClientCtl;



pub struct RClientCtl<EK, ER>
where
    EK: KvEngine,
    ER: RaftEngine,
{
    raft_peer: Peer<EK, ER>,
    curr_map: RwLock<Arc<ClusterMap>>,
    update_lock: Mutex<()>,
    conf: Rconf,

}

#[async_trait]
impl<EK, ER> ClientCtl for RClientCtl<EK, ER>
where 
    EK: KvEngine,
    ER: RaftEngine,
{
    /// Get current cluster map.
    fn current_map(&self) -> Arc<ClusterMap>{
        todo!();
    }

    /// Update cluster map.
    async fn update_map(
        &self,
        version_hit: Option<ClusterMapVersion>,
    ) -> Result<Arc<ClusterMap>, Error>{
        todo!();
    }

    /// Get conf client.
    fn get_conf(&self) -> &dyn Conf{
        todo!();
    }

    /// Add all targets IN.
    ///
    /// Used in create fs, cnt_hint can use to check if all servers booted.
    async fn add_all_targets(&self, cnt_hint: Option<u32>) -> Result<Arc<ClusterMap>, Error>{
        todo!();
    }
}



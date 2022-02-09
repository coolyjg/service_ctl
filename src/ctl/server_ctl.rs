use std::net::SocketAddr;
use std::sync::{RwLock, Arc};
use crate::{storage_mod::{
    KvEngine, RaftEngine, Peer
}, ClusterMap, ServerCtl, Conf, ClusterMapVersion, Error};
use async_trait::async_trait;
use uuid::Uuid;
use super::Rconf;


pub struct RServerCtl<EK, ER>
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

#[async_trait]
impl<EK, ER> ServerCtl for RServerCtl<EK, ER>
where 
    EK: KvEngine,
    ER: RaftEngine,
{
    /// Wait a new cluster map.
    async fn wait_map(&self, prev_version: ClusterMapVersion) -> Arc<ClusterMap>{
        todo!()
    }

    /// Close server, stop background task and mark self as DOWN.
    async fn close(&self){
        todo!()
    }

    /// Get current cluster map.
    fn current_map(&self) -> Arc<ClusterMap>{
        todo!()
    }

    /// Update cluster map if we know new version exists.
    async fn update_map(&self) -> Result<Arc<ClusterMap>, Error>{
        todo!();
    }

    /// Get conf client.
    fn get_conf(&self) -> &dyn Conf{
        todo!();
    }

    /// Alloc some unique oid.
    async fn oid_alloc(&self, cnt: u64) -> Result<(u64, u64), Error>{
        todo!();
    }
}





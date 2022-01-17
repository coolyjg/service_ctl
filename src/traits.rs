use std::sync::Arc;

use crate::{ClusterMap, ClusterMapVersion, Error};
use async_trait::async_trait;

#[async_trait]
pub trait Conf: Send + Sync {
    async fn oc_create_chunk_type(&self, chunk_size: u32) -> Result<u8, Error>;

    async fn oc_create_stripe_type(&self, stripe_cnt: u32) -> Result<u8, Error>;

    async fn oc_get(&self, stripe_type: u8, chunk_size: u8) -> Result<Option<(u32, u32)>, Error>;

    async fn oc_get_stripe(&self, stripe_type: u8) -> Result<Option<u32>, Error>;

    async fn oc_get_chunk(&self, chunk_type: u8) -> Result<Option<u32>, Error>;

    async fn kv_get(&self, key: &str) -> Result<Option<String>, Error>;

    async fn kv_put(&self, key: &str, value: &str) -> Result<(), Error>;

    async fn kv_get_all(&self, prefix: &str) -> Result<Vec<(String, String)>, Error>;
}

#[async_trait]
pub trait ServerCtl: Send + Sync {
    /// Wait a new cluster map.
    async fn wait_map(&self, prev_version: ClusterMapVersion) -> Arc<ClusterMap>;

    /// Close server, stop background task and mark self as DOWN.
    async fn close(&self);

    /// Get current cluster map.
    fn current_map(&self) -> Arc<ClusterMap>;

    /// Update cluster map if we know new version exists.
    async fn update_map(&self) -> Result<Arc<ClusterMap>, Error>;

    /// Get conf client.
    fn get_conf(&self) -> &dyn Conf;

    /// Alloc some unique oid.
    async fn oid_alloc(&self, cnt: u64) -> Result<(u64, u64), Error>;
}

#[async_trait]
pub trait ClientCtl: Send + Sync {
    /// Get current cluster map.
    fn current_map(&self) -> Arc<ClusterMap>;

    /// Update cluster map.
    async fn update_map(
        &self,
        version_hit: Option<ClusterMapVersion>,
    ) -> Result<Arc<ClusterMap>, Error>;

    /// Get conf client.
    fn get_conf(&self) -> &dyn Conf;

    /// Add all targets IN.
    ///
    /// Used in create fs, cnt_hint can use to check if all servers booted.
    async fn add_all_targets(&self, cnt_hint: Option<u32>) -> Result<Arc<ClusterMap>, Error>;
}

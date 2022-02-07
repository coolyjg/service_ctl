mod cluster;
mod target;
mod traits;

pub mod storage_mod;
pub mod ctl;

pub use cluster::*;
pub use target::*;
pub use traits::*;


/// Error information
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error from madsim network.
    #[error("madsim RPC io error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error from `etcd_client`
    // #[error("etcd error: {0}")]
    // EtcdError(#[from] etcd_client::Error),

    /// Invalid arguments
    #[error("invalid arguments")]
    InvalidArg,

    /// Failed to modify cluster map because our cluster map are stale, update and retry
    #[error("txn failed with stale version, need fetch and retry, new version {0:?}")]
    StaleMapVersion(cluster::ClusterMapVersion),

    /// Target is still alive, can't make as DOWN
    #[error("target is still alive, can't make as DOWN")]
    TargetStillAlive(Vec<target::TargetId>),

    /// Transaction conflict, try again
    #[error("transaction conflict, try again")]
    TxnConflict,

    /// Transaction conflict too may times
    #[error("transaction conflict too may times.")]
    TxnMaxRetry,

    /// Watch return None
    #[error("watch return None, maybe already canceled")]
    WatchReturnNone,

    /// Server lost leadership
    #[error("server is not leader")]
    LeadershipLost,

    /// Target id exists
    #[error("target with same id exists")]
    TargetIdExists,

    /// Reach max stripe type
    #[error("reach max stripe types")]
    MaxStripeType,

    /// Max chunk type
    #[error("reach max chunk types")]
    MaxChunkType,

    /// Some targets not started
    #[error("some targets not started")]
    TargetsNotStarted,
}

use crate::target::{TargetId, TargetInfo};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display};
use uuid::Uuid;

/// Cluster map changelog.
#[derive(Serialize, Deserialize, Clone)]
pub struct ChangeLog {
    /// Version of this change
    pub version: ClusterMapVersion,
    /// Targets that changed.
    pub targets: Vec<TargetInfo>,
    /// Human readable information for this changelog entry
    pub info: String,
}

impl ChangeLog {
    pub fn new(version: ClusterMapVersion, targets: Vec<TargetInfo>, info: String) -> ChangeLog {
        ChangeLog {
            version,
            targets,
            info,
        }
    }
}

/// Version of cluster map.
///
/// Version consists of two parts: major and minor.
/// Major number incrase at major change (targets IN/OUT).
/// Minor number reset to 0 at major change and increase at minor change (targets UP/DOWN).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct ClusterMapVersion {
    /// major version
    pub major: u32,
    /// minor version
    pub minor: u32,
}

impl ClusterMapVersion {
    /// Create a cluster map version.
    pub fn new(major: u32, minor: u32) -> Self {
        ClusterMapVersion { major, minor }
    }

    /// Next major version.
    pub fn next_major(&self) -> Self {
        ClusterMapVersion {
            major: self.major + 1,
            minor: 0,
        }
    }

    /// Next minor version.
    pub fn next_minor(&self) -> Self {
        ClusterMapVersion {
            major: self.major,
            minor: self.minor + 1,
        }
    }

    pub fn is_next(&self, next: &ClusterMapVersion) -> bool {
        if self.major == next.major {
            next.minor == self.minor + 1
        } else {
            next.major == self.major + 1 && next.minor == 0
        }
    }
}

impl Display for ClusterMapVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.major, self.minor))
    }
}

/// Cluster map of specific version.
/// ClusterMap should be read only, any modifications on current map will generate a new ClusterMap.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClusterMap {
    pub version: ClusterMapVersion,
    pub uuid_map: BTreeMap<Uuid, TargetId>,
    pub targets: BTreeMap<TargetId, TargetInfo>,
    /// revision in etcd, can use this revision to watch changes
    pub revision: i64,
}

impl ClusterMap {
    /// Initial cluster map with version {0, 0}
    pub fn new_initial() -> ClusterMap {
        ClusterMap {
            version: ClusterMapVersion::new(0, 0),
            uuid_map: BTreeMap::new(),
            targets: BTreeMap::new(),
            revision: 0,
        }
    }

    /// Get target with `id`.
    pub fn get_target(&self, id: TargetId) -> Option<&TargetInfo> {
        self.targets.get(&id)
    }

    /// Get target by `uuid`.
    pub fn get_target_by_uuid(&self, uuid: &Uuid) -> Option<&TargetInfo> {
        self.uuid_map.get(uuid).and_then(|&id| self.get_target(id))
    }

    /// Get max id of all targets.
    pub fn target_max_id(&self) -> Option<TargetId> {
        self.targets.iter().next_back().map(|(&id, _)| id)
    }

    /// Apply changelog to current cluster map to generate next cluster map.
    pub fn apply_change(&self, log: &ChangeLog, revision: Option<i64>) -> ClusterMap {
        // check change log
        assert!(self.version.is_next(&log.version));
        for target in &log.targets {
            let id = target
                .get_id()
                .expect("target in changelog shouldn't be Init state!!");
            let uuid = target.uuid;
            match self.uuid_map.get(&uuid) {
                Some(&old_id) => assert_eq!(old_id, id),
                None => {
                    assert!(self.targets.get(&id).is_none());
                }
            }
        }

        let version = log.version;
        let mut uuid_map = self.uuid_map.clone();
        let mut targets = self.targets.clone();

        for target in &log.targets {
            let target_id = target.get_id().unwrap();
            uuid_map.insert(target.uuid, target_id);
            targets.insert(target_id, target.clone());
        }

        // check new target map
        for (target_id, target) in &targets {
            assert_eq!(Some(*target_id), target.get_id());
            assert_eq!(uuid_map.get(&target.uuid), Some(target_id));
        }

        ClusterMap {
            version,
            uuid_map,
            targets,
            revision: revision.unwrap_or(0),
        }
    }

    /// Apply all changelogs to generate new cluster map
    pub fn apply_all(&self, logs: &[ChangeLog], revision: Option<i64>) -> Option<ClusterMap> {
        let mut logs = logs
            .iter()
            .skip_while(|log| log.version <= self.version)
            .peekable();
        if logs.peek().is_none() {
            return None;
        }

        let mut targets = self.targets.clone();
        let mut uuid_map = self.uuid_map.clone();
        let mut prev_version = self.version;
        for log in logs {
            // log version should be continuous
            assert!(prev_version.is_next(&log.version));
            prev_version = log.version;

            // add targets
            for target in &log.targets {
                let target_id = target.get_id().unwrap();
                targets.insert(target_id, target.clone());
                uuid_map.insert(target.uuid, target_id);
            }
        }

        Some(ClusterMap {
            version: prev_version,
            uuid_map,
            targets,
            revision: revision.unwrap_or(0),
        })
    }
}

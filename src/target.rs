use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

/// Id of target.
///
/// Each target has a 32b-it id.
/// Target id can't change after creation.
/// If we don't specific id when creating targets,
/// the system will automatically assign one.
pub type TargetId = u32;

/// Target info that included in cluster map
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TargetInfo {
    pub uuid: Uuid,
    pub state: TargetState,
    pub location: TargetLocation,
}

impl TargetInfo {
    /// Create target info with state init.
    /// - uuid: target uuid.
    /// - url: target url.
    /// - id: id of the target(optional).
    pub fn new_init(uuid: Uuid, url: SocketAddr, id: Option<TargetId>) -> TargetInfo {
        TargetInfo {
            uuid,
            state: TargetState::Init(url, id),
            location: TargetLocation::new(),
        }
    }

    pub(crate) fn new(uuid: Uuid, id: TargetId, url: Option<SocketAddr>, in_: bool) -> TargetInfo {
        TargetInfo {
            uuid,
            state: TargetState::new(id, url, in_),
            location: TargetLocation::new(),
        }
    }

    pub fn add_in(&self) -> TargetInfo {
        let new_state = self.state.add_in().unwrap_or_else(|| self.state.clone());
        TargetInfo {
            state: new_state,
            ..self.clone()
        }
    }

    pub fn remove_out(&self) -> TargetInfo {
        let new_state = self
            .state
            .remove_out()
            .unwrap_or_else(|| self.state.clone());
        TargetInfo {
            state: new_state,
            ..self.clone()
        }
    }

    pub fn up(&self, url: SocketAddr) -> TargetInfo {
        let new_state = self.state.up(url).unwrap_or_else(|| self.state.clone());
        TargetInfo {
            state: new_state,
            ..self.clone()
        }
    }

    pub fn down(&self) -> TargetInfo {
        let new_state = self.state.down().unwrap_or_else(|| self.state.clone());
        TargetInfo {
            state: new_state,
            ..self.clone()
        }
    }

    pub fn init(&self, id: u32, up: bool, in_: bool) -> TargetInfo {
        let new_state = self.state.init(id, up, in_);
        TargetInfo {
            state: new_state,
            ..self.clone()
        }
    }

    pub fn is_init(&self) -> bool {
        self.state.is_init()
    }

    pub fn is_up(&self) -> bool {
        self.state.is_up()
    }

    pub fn is_in(&self) -> bool {
        self.state.is_in()
    }

    pub fn get_url(&self) -> Option<&SocketAddr> {
        self.state.get_url()
    }

    pub fn get_id(&self) -> Option<TargetId> {
        self.state.get_id()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Target state
/// - UP/DOWN: target is healthy(has a network url) or dead.
/// - IN/OUT: we only assign data to in targets.
pub enum TargetState {
    /// Initial state, `(url, Option<id>)`
    Init(SocketAddr, Option<TargetId>),
    /// Target is UP and IN, `(id, url)`
    UpIn(TargetId, SocketAddr),
    /// Target is DOWN but IN, `(id)`
    DownIn(TargetId),
    /// Target is UP but OUT, `(id, url)`
    UpOut(TargetId, SocketAddr),
    /// Target is DOWN but OUT, `(id)`
    DownOut(TargetId),
}

impl TargetState {
    fn new(id: TargetId, url: Option<SocketAddr>, in_: bool) -> TargetState {
        match (url, in_) {
            (None, true) => TargetState::DownIn(id),
            (None, false) => TargetState::DownOut(id),
            (Some(url), true) => TargetState::UpIn(id, url),
            (Some(url), false) => TargetState::UpOut(id, url),
        }
    }

    pub fn is_init(&self) -> bool {
        matches!(self, TargetState::Init(_, _))
    }

    pub fn is_up(&self) -> bool {
        matches!(self, TargetState::UpIn(_, _) | TargetState::UpOut(_, _))
    }

    pub fn is_in(&self) -> bool {
        matches!(self, TargetState::UpIn(_, _) | TargetState::DownIn(_))
    }

    pub fn get_url(&self) -> Option<&SocketAddr> {
        match self {
            TargetState::Init(url, _) => Some(url),
            TargetState::UpIn(_, url) => Some(url),
            TargetState::UpOut(_, url) => Some(url),
            _ => None,
        }
    }

    pub fn get_id(&self) -> Option<TargetId> {
        match *self {
            TargetState::UpIn(id, _) => Some(id),
            TargetState::DownIn(id) => Some(id),
            TargetState::UpOut(id, _) => Some(id),
            TargetState::DownOut(id) => Some(id),
            TargetState::Init(_, id) => id,
        }
    }

    pub fn init(&self, id: TargetId, up: bool, in_: bool) -> TargetState {
        let url = match self {
            TargetState::Init(url, _) => url.clone(),
            _ => unreachable!(),
        };
        match (up, in_) {
            (true, true) => TargetState::UpIn(id, url),
            (true, false) => TargetState::UpOut(id, url),
            (false, true) => TargetState::DownIn(id),
            (false, false) => TargetState::DownOut(id),
        }
    }

    pub fn up(&self, url: SocketAddr) -> Option<TargetState> {
        match self {
            TargetState::UpIn(target_id, old_url) => {
                if old_url == &url {
                    None
                } else {
                    Some(TargetState::UpIn(*target_id, url))
                }
            }
            TargetState::UpOut(target_id, old_url) => {
                if old_url == &url {
                    None
                } else {
                    Some(TargetState::UpOut(*target_id, url))
                }
            }
            TargetState::DownIn(target_id) => Some(TargetState::UpIn(*target_id, url)),
            TargetState::DownOut(target_id) => Some(TargetState::UpOut(*target_id, url)),
            _ => unreachable!(),
        }
    }

    pub fn down(&self) -> Option<TargetState> {
        if !self.is_up() {
            None
        } else {
            let target_id = self.get_id().unwrap();
            if self.is_in() {
                Some(TargetState::DownIn(target_id))
            } else {
                Some(TargetState::DownOut(target_id))
            }
        }
    }

    pub fn add_in(&self) -> Option<TargetState> {
        if self.is_in() {
            None
        } else {
            let target_id = self.get_id().unwrap();
            if self.is_up() {
                Some(TargetState::UpIn(
                    target_id,
                    self.get_url().unwrap().to_owned(),
                ))
            } else {
                Some(TargetState::DownIn(target_id))
            }
        }
    }

    pub fn remove_out(&self) -> Option<TargetState> {
        if !self.is_in() {
            None
        } else {
            let target_id = self.get_id().unwrap();
            if self.is_up() {
                Some(TargetState::UpOut(
                    target_id,
                    self.get_url().unwrap().to_owned(),
                ))
            } else {
                Some(TargetState::DownOut(target_id))
            }
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Init(_, _) => "Init",
            Self::UpIn(_, _) => "UpIn",
            Self::DownIn(_) => "DownIn",
            Self::UpOut(_, _) => "UpOut",
            Self::DownOut(_) => "DownOut",
        }
    }
}

/// Target location.
///
/// todo: may be should support host_id, rack_id, etc...
/// todo: handle location change in future
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TargetLocation {
    pub host: String,
}

impl TargetLocation {
    fn new() -> TargetLocation {
        let hostname = gethostname::gethostname().to_str().unwrap().to_owned();
        TargetLocation { host: hostname }
    }

    pub fn format_path(&self) -> String {
        format!("/{}", self.host)
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TargetChange {
//     pub uuid: uuid::Uuid,
//     pub op: TargetChangeOp,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum TargetChangeOp {
//     // new target crated, (id, url, location)
//     Creat(TargetId, String, TargetLocation),
//     // target mark self as UP, (url)
//     Up(String),
//     // target DOWN
//     Down,
//     // target IN
//     In,
//     // target OUT
//     Out,
//     // todo: add more operation, eg: move location ...
// }

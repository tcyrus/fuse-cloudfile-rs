use std::str::ParseBoolError;

// These xattrs are based on the NextCloud VFS implementation

pub trait XAttr {
    fn name() -> &[u8];
    fn value(&self) -> &[u8];
}

pub struct SyncHydrating(bool);

impl XAttr for SyncHydrating {
    const NAME: &[u8] = b"user.fuse.sync.hydrating";

    fn name() -> &[u8] {
        Self::NAME
    }

    fn value(&self) -> &[u8] {
        match self.0 {
            true => b"true",
            false => b"false",
        }
    }
}

// This is the Sync Setting for the object
pub enum SyncMode {
    Inherited,
    AlwaysLocal,
    OnlineOnly,
    Excluded,
    Unspecified,
};

impl XAttr for SyncMode {
    const NAME: &[u8] = b"user.fuse.sync.mode";

    fn name() -> &[u8] {
        Self::NAME
    }

    fn value(&self) -> &[u8] {
        match self {
            SyncMode::Inherited => b"Inherited",
            SyncMode::AlwaysLocal => b"AlwaysLocal",
            SyncMode::OnlineOnly => b"OnlineOnly",
            SyncMode::Excluded => b"Excluded",
            SyncMode::Unspecified => b"",
        }
    }
}

pub enum SyncStatus {
    AlwaysLocal,
    AllHydrated,
    Mixed,
    AllDehydrated,
    OnlineOnly,
};

impl XAttr for SyncStatus {
    const NAME: &[u8] = b"user.fuse.sync.status";

    fn name() -> &[u8] {
        Self::NAME
    }

    fn data(&self) -> &[u8] {
        match self {
            SyncStatus::AlwaysLocal => b"AlwaysLocal",
            SyncStatus::AllHydrated => b"AllHydrated",
            SyncStatus::Mixed => b"Mixed",
            SyncStatus::AllDehydrated => b"AllDehydrated",
            SyncStatus::AllDehydrated => b"OnlineOnly",
        }
    }
}

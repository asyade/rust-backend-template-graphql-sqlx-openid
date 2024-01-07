use rave_core_database::tables::asset::AssetKind;

use crate::prelude::*;

#[derive(Debug, Error)]
pub enum AssetError {
    #[error(transparent)]
    DatabaseError(#[from] CoreDatabaseError),
    #[error("failed to accquire database connection")]
    DatabaseUnavailable,
    #[error(transparent)]
    IoError(#[from] tokio::io::Error),
    #[error("invalid asset kind, expected {expected:?}, got {actual:?}")]
    KindMismatch {
        expected: AssetKind,
        actual: AssetKind,
    },
}

pub type AssetResult<T> = Result<T, AssetError>;

use std::fmt;

use meilisearch_types::error::{Code, ErrorCode};
use meilisearch_types::index_uid::IndexUidFormatError;
use meilisearch_types::internal_error;
use tokio::sync::mpsc::error::SendError as MpscSendError;
use tokio::sync::oneshot::error::RecvError as OneshotRecvError;
use uuid::Uuid;

use crate::{error::MilliError, index::error::IndexError, update_file_store::UpdateFileStoreError};

pub type Result<T> = std::result::Result<T, IndexResolverError>;

#[derive(thiserror::Error, Debug)]
pub enum IndexResolverError {
    #[error("{0}")]
    IndexError(#[from] IndexError),
    #[error("Index `{0}` already exists.")]
    IndexAlreadyExists(String),
    #[error("Index `{0}` not found.")]
    UnexistingIndex(String),
    #[error("A primary key is already present. It's impossible to update it")]
    ExistingPrimaryKey,
    #[error("An internal error has occurred. `{0}`.")]
    Internal(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("The creation of the `{0}` index has failed due to `Index uuid is already assigned`.")]
    UuidAlreadyExists(Uuid),
    #[error("{0}")]
    Milli(#[from] milli::Error),
    #[error("{0}")]
    BadlyFormatted(#[from] IndexUidFormatError),
}

impl<T> From<MpscSendError<T>> for IndexResolverError
where
    T: Send + Sync + 'static + fmt::Debug,
{
    fn from(other: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::Internal(Box::new(other))
    }
}

impl From<OneshotRecvError> for IndexResolverError {
    fn from(other: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::Internal(Box::new(other))
    }
}

internal_error!(
    IndexResolverError: milli::heed::Error,
    uuid::Error,
    std::io::Error,
    tokio::task::JoinError,
    serde_json::Error,
    UpdateFileStoreError
);

impl ErrorCode for IndexResolverError {
    fn error_code(&self) -> Code {
        match self {
            IndexResolverError::IndexError(e) => e.error_code(),
            IndexResolverError::IndexAlreadyExists(_) => Code::IndexAlreadyExists,
            IndexResolverError::UnexistingIndex(_) => Code::IndexNotFound,
            IndexResolverError::ExistingPrimaryKey => Code::PrimaryKeyAlreadyPresent,
            IndexResolverError::Internal(_) => Code::Internal,
            IndexResolverError::UuidAlreadyExists(_) => Code::CreateIndex,
            IndexResolverError::Milli(e) => MilliError(e).error_code(),
            IndexResolverError::BadlyFormatted(_) => Code::InvalidIndexUid,
        }
    }
}

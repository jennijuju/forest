// Copyright 2019-2023 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use thiserror::Error;

/// Database error
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid bulk write kv lengths, must be equal")]
    InvalidBulkLen,
    #[error("Cannot use unopened database")]
    Unopened,
    #[error(transparent)]
    Database(#[from] crate::db::db_engine::DbError),
    #[error("{0}")]
    Other(String),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;

        match (self, other) {
            (&InvalidBulkLen, &InvalidBulkLen) => true,
            (&Unopened, &Unopened) => true,
            (&Database(_), &Database(_)) => true,
            (Other(a), Other(b)) => a == b,
            _ => false,
        }
    }
}

impl From<Error> for String {
    fn from(e: Error) -> Self {
        e.to_string()
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Other(e.to_string())
    }
}

use crate::models::pokemon::PokemonData;
use reqwest;
use serde_json;
use std::sync::{MutexGuard, PoisonError};

#[derive(Debug)]
pub enum FetchError {
    Network(reqwest::Error),
    Parse(serde_json::Error),
    Poisoned(PoisonError<PokemonData>),
    PoisonedLock,
    ProgressError(std::io::Error),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FetchError::Network(err) => write!(f, "Network error: {}", err),
            FetchError::Parse(err) => write!(f, "Parsing error: {}", err),
            FetchError::Poisoned(err) => write!(f, "Poisoned error: {}", err),
            FetchError::ProgressError(err) => write!(f, "Progress error: {}", err),
            FetchError::PoisonedLock => write!(f, "Poisoned lock error"),
        }
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(err: reqwest::Error) -> FetchError {
        FetchError::Network(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Parse(err)
    }
}

impl From<PoisonError<PokemonData>> for FetchError {
    fn from(err: PoisonError<PokemonData>) -> FetchError {
        FetchError::Poisoned(err)
    }
}

impl From<PoisonError<MutexGuard<'_, PokemonData>>> for FetchError {
    fn from(_err: PoisonError<MutexGuard<'_, PokemonData>>) -> FetchError {
        FetchError::PoisonedLock
    }
}

impl From<std::io::Error> for FetchError {
    fn from(err: std::io::Error) -> FetchError {
        FetchError::ProgressError(err)
    }
}

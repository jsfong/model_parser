use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error;

use leptos::{
    prelude::{FromServerFnError, ServerFnErrorErr},
    server_fn::codec::JsonEncoding,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ModelError {
    ModelGraphBuildingError(String),
    ModelNotFound(String),
    InvalidInput,
    ServerFnError(ServerFnErrorErr),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::ModelGraphBuildingError(err) => {
                write!(f, "Error building model graph: {}", err)
            }
            ModelError::ModelNotFound(err) => write!(f, "Model {} not found", err),
            ModelError::ServerFnError(_server_fn_error_err) => Ok(()),
            ModelError::InvalidInput => Ok(()),
        }
    }
}

impl Error for ModelError {}

impl FromServerFnError for ModelError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        ModelError::ServerFnError(value)
    }
}

//! JSON-RPC IDs

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::uuid::Uuid;

/// JSON-RPC ID: request-specific identifier
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(untagged)]
pub enum Id {
    /// Numerical JSON ID
    Num(i64),
    /// String JSON ID
    Str(String),
    /// null JSON ID
    None,
}

impl Id {
    /// Create a JSON-RPC ID containing a UUID v4 (i.e. random)
    pub async fn uuid_v4() -> Self {
        Self::Str(Uuid::new().await.unwrap().to_string())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Id::Num(i) => write!(f, "{i}"),
            Id::Str(s) => write!(f, "{s}"),
            Id::None => write!(f, ""),
        }
    }
}

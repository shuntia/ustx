use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

/// Represents an error that can occur when working with `.ustx` files.
#[derive(Debug)]
pub enum Error {
    /// An error that occurred during YAML serialization or deserialization.
    Yaml(serde_yaml::Error),
    /// An error that occurred because the `.ustx` version is not supported.
    UnsupportedVersion(String),
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "yaml error: {err}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported ustx version: {version}")
            }
        }
    }
}

impl StdError for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Yaml(err) => Some(err),
            Self::UnsupportedVersion(_) => None,
        }
    }
}

impl From<serde_yaml::Error> for Error {
    #[inline]
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

impl Error {
    #[inline]
    #[must_use]
    pub fn unsupported_version<S: Into<String>>(version: S) -> Self {
        Self::UnsupportedVersion(version.into())
    }
}

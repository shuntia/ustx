use semver::Version as SemverVersion;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

pub const CURRENT_VERSION: Version = Version::new(0, 7, 0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    #[inline]
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    #[inline]
    #[must_use]
    pub fn parse(input: Option<&str>) -> Self {
        input
            .and_then(|raw| Self::from_str(raw).ok())
            .unwrap_or_else(Self::zero)
    }

    #[inline]
    #[must_use]
    pub const fn to_semver(self) -> SemverVersion {
        SemverVersion::new(self.major, self.minor, self.patch)
    }

    fn parse_with_fallback(input: &str) -> Result<Self, semver::Error> {
        match SemverVersion::parse(input) {
            Ok(version) => Ok(Self::from_semver(&version)),
            Err(original_error) => {
                if input.contains(['-', '+']) {
                    return Err(original_error);
                }
                let mut segments = input.split('.').collect::<Vec<_>>();
                if segments.len() >= 3 {
                    return Err(original_error);
                }
                segments.resize(3, "0");
                let normalized = segments.join(".");
                SemverVersion::parse(&normalized)
                    .map(|version| Self::from_semver(&version))
                    .map_err(|_| original_error)
            }
        }
    }

    #[inline]
    #[must_use]
    const fn from_semver(version: &SemverVersion) -> Self {
        Self::new(version.major, version.minor, version.patch)
    }
}

impl FromStr for Version {
    type Err = semver::Error;

    #[inline]
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Ok(Self::zero());
        }
        Self::parse_with_fallback(trimmed)
    }
}

impl Display for Version {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.patch > 0 {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::from_str(&raw).map_err(serde::de::Error::custom)
    }
}

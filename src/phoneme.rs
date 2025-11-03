use serde::{Deserialize, Serialize};

/// Represents a phoneme override.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PhonemeOverride {
    /// The index of the phoneme to override.
    #[serde(default)]
    pub index: i32,
    /// The new phoneme.
    #[serde(default)]
    pub phoneme: Option<String>,
    /// The new offset of the phoneme.
    #[serde(default)]
    pub offset: Option<i32>,
    /// The new preutterance delta of the phoneme.
    #[serde(default)]
    pub preutter_delta: Option<f32>,
    /// The new overlap delta of the phoneme.
    #[serde(default)]
    pub overlap_delta: Option<f32>,
}

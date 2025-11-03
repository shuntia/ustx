use serde::{Deserialize, Serialize};

/// Represents a tempo change in a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tempo {
    /// The position of the tempo change in ticks.
    #[serde(default)]
    pub position: i32,
    /// The new tempo in beats per minute.
    #[serde(default = "default_bpm")]
    pub bpm: f64,
}

impl Default for Tempo {
    #[inline]
    fn default() -> Self {
        Self {
            position: 0,
            bpm: default_bpm(),
        }
    }
}

/// Represents a time signature change in a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeSignature {
    /// The bar position of the time signature change.
    #[serde(default)]
    pub bar_position: i32,
    /// The number of beats per bar.
    #[serde(default = "default_beat_per_bar")]
    pub beat_per_bar: i32,
    /// The beat unit, which defines the note value that represents one beat.
    #[serde(default = "default_beat_unit")]
    pub beat_unit: i32,
}

impl Default for TimeSignature {
    #[inline]
    fn default() -> Self {
        Self {
            bar_position: 0,
            beat_per_bar: default_beat_per_bar(),
            beat_unit: default_beat_unit(),
        }
    }
}

#[inline]
const fn default_bpm() -> f64 {
    120.0
}

#[inline]
const fn default_beat_per_bar() -> i32 {
    4
}

#[inline]
const fn default_beat_unit() -> i32 {
    4
}

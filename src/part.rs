use serde::{Deserialize, Serialize};

use crate::expression::Curve;
use crate::note::Note;

/// Represents a voice part in an `OpenUtau` project.
///
/// A voice part contains a sequence of notes and expression curves.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VoicePart {
    /// The name of the voice part.
    #[serde(default = "default_part_name")]
    pub name: String,
    /// A comment for the voice part.
    #[serde(default)]
    pub comment: String,
    /// The track number that this part belongs to.
    #[serde(default)]
    pub track_no: i32,
    /// The position of the voice part in ticks.
    #[serde(default)]
    pub position: i32,
    /// The duration of the voice part in ticks.
    #[serde(default)]
    pub notes: Vec<Note>,
    /// A list of expression curves in the voice part.
    #[serde(default)]
    pub curves: Vec<Curve>,
}

/// Represents a wave part in an `OpenUtau` project.
///
/// A wave part contains a reference to an audio file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WavePart {
    /// The name of the wave part.
    #[serde(default = "default_part_name")]
    pub name: String,
    /// A comment for the wave part.
    #[serde(default)]
    pub comment: String,
    /// The track number that this part belongs to.
    #[serde(default)]
    pub track_no: i32,
    /// The position of the wave part in ticks.
    #[serde(default)]
    pub position: i32,
    /// The relative path to the audio file.
    #[serde(default)]
    pub relative_path: String,
    /// The duration of the audio file in milliseconds.
    #[serde(default)]
    pub file_duration_ms: f64,
    /// The number of milliseconds to skip at the beginning of the audio file.
    #[serde(default)]
    pub skip_ms: f64,
    /// The number of milliseconds to trim from the end of the audio file.
    #[serde(default)]
    pub trim_ms: f64,
}

#[inline]
fn default_part_name() -> String {
    String::from("New Part")
}

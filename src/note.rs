use serde::{Deserialize, Serialize};

use crate::expression::Expression;
use crate::phoneme::PhonemeOverride;

/// Represents a note in a voice part.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Note {
    /// The position of the note in ticks.
    #[serde(default)]
    pub position: i32,
    /// The duration of the note in ticks.
    #[serde(default = "default_note_duration")]
    pub duration: i32,
    /// The tone of the note, as a MIDI note number.
    #[serde(default)]
    pub tone: i32,
    /// The lyric of the note.
    #[serde(default = "default_note_lyric")]
    pub lyric: String,
    /// The pitch data for the note.
    #[serde(default)]
    pub pitch: Pitch,
    /// The vibrato data for the note.
    #[serde(default)]
    pub vibrato: Vibrato,
    /// A list of phoneme expressions for the note.
    #[serde(default)]
    pub phoneme_expressions: Vec<Expression>,
    /// A list of phoneme overrides for the note.
    #[serde(default)]
    pub phoneme_overrides: Vec<PhonemeOverride>,
    /// A list of phoneme indexes for the note.
    #[serde(default)]
    pub phoneme_indexes: Vec<i32>,
}

/// Represents the pitch data for a note.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Pitch {
    /// A list of pitch points.
    #[serde(default)]
    pub data: Vec<PitchPoint>,
    /// Whether to snap the first pitch point to the note's tone.
    #[serde(default = "default_snap_first")]
    pub snap_first: bool,
}

/// Represents a single point in a pitch curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PitchPoint {
    /// The x-coordinate of the pitch point, in ticks.
    pub x: f32,
    /// The y-coordinate of the pitch point, in cents relative to the note's tone.
    pub y: f32,
    /// The shape of the pitch point.
    #[serde(default)]
    pub shape: PitchPointShape,
}

/// Represents the shape of a pitch point.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PitchPointShape {
    /// Ease in and out.
    #[serde(rename = "io")]
    Io,
    /// Linear.
    #[serde(rename = "l")]
    L,
    /// Ease in.
    #[serde(rename = "i")]
    I,
    /// Ease out.
    #[serde(rename = "o")]
    O,
}

impl Default for PitchPointShape {
    #[inline]
    fn default() -> Self {
        Self::Io
    }
}

/// Represents the vibrato data for a note.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Vibrato {
    /// The length of the vibrato in milliseconds.
    #[serde(default)]
    pub length: f32,
    /// The period of the vibrato in milliseconds.
    #[serde(default = "default_vibrato_period")]
    pub period: f32,
    /// The depth of the vibrato in cents.
    #[serde(default = "default_vibrato_depth")]
    pub depth: f32,
    /// The fade-in time of the vibrato in milliseconds.
    #[serde(default = "default_vibrato_in")]
    pub r#in: f32,
    /// The fade-out time of the vibrato in milliseconds.
    #[serde(default = "default_vibrato_out")]
    pub out: f32,
    /// The shift of the vibrato in milliseconds.
    #[serde(default)]
    pub shift: f32,
    /// The drift of the vibrato in cents.
    #[serde(default)]
    pub drift: f32,
    /// The volume link of the vibrato.
    #[serde(default)]
    pub vol_link: f32,
}

impl Default for Vibrato {
    #[inline]
    fn default() -> Self {
        Self {
            length: 0.0,
            period: default_vibrato_period(),
            depth: default_vibrato_depth(),
            r#in: default_vibrato_in(),
            out: default_vibrato_out(),
            shift: 0.0,
            drift: 0.0,
            vol_link: 0.0,
        }
    }
}

#[inline]
const fn default_note_duration() -> i32 {
    120
}

#[inline]
fn default_note_lyric() -> String {
    String::from("あ")
}

#[inline]
const fn default_snap_first() -> bool {
    true
}

#[inline]
const fn default_vibrato_period() -> f32 {
    175.0
}

#[inline]
const fn default_vibrato_depth() -> f32 {
    25.0
}

#[inline]
const fn default_vibrato_in() -> f32 {
    10.0
}

#[inline]
const fn default_vibrato_out() -> f32 {
    10.0
}

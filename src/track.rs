use serde::{Deserialize, Serialize};

use crate::expression::Expression;

/// Represents the render settings for a track.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RenderSettings {
    /// The name of the renderer.
    #[serde(default)]
    pub renderer: Option<String>,
    /// The name of the resampler.
    #[serde(default)]
    pub resampler: Option<String>,
    /// The name of the wavtool.
    #[serde(default)]
    pub wavtool: Option<String>,
}

/// Represents a track in an `OpenUtau` project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Track {
    /// The name of the singer for this track.
    #[serde(default)]
    pub singer: String,
    /// The name of the phonemizer for this track.
    #[serde(default)]
    pub phonemizer: String,
    /// The render settings for this track.
    #[serde(default)]
    pub renderer_settings: RenderSettings,
    /// The name of the track.
    #[serde(default = "default_track_name")]
    pub track_name: String,
    /// The color of the track.
    #[serde(default = "default_track_color")]
    pub track_color: String,
    /// Whether the track is muted.
    #[serde(default)]
    pub mute: bool,
    /// Whether the track is soloed.
    #[serde(default)]
    pub solo: bool,
    /// The volume of the track, from -12.0 to 12.0 dB.
    #[serde(default)]
    pub volume: f64,
    /// The pan of the track, from -1.0 (left) to 1.0 (right).
    #[serde(default)]
    pub pan: f64,
    /// A list of expressions for this track.
    #[serde(default)]
    pub track_expressions: Vec<Expression>,
    /// A list of voice color names for this track.
    #[serde(default = "default_voice_color_names")]
    pub voice_color_names: Vec<String>,
}

impl Default for Track {
    #[inline]
    fn default() -> Self {
        Self {
            singer: String::new(),
            phonemizer: String::new(),
            renderer_settings: RenderSettings::default(),
            track_name: default_track_name(),
            track_color: default_track_color(),
            mute: false,
            solo: false,
            volume: 0.0,
            pan: 0.0,
            track_expressions: Vec::new(),
            voice_color_names: default_voice_color_names(),
        }
    }
}

#[inline]
fn default_track_name() -> String {
    String::from("New Track")
}

#[inline]
fn default_track_color() -> String {
    String::from("Blue")
}

#[inline]
fn default_voice_color_names() -> Vec<String> {
    vec![String::new()]
}

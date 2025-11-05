use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use crate::error::Error;
use crate::expression::ExpressionDescriptor;
use crate::part::{VoicePart, WavePart};
use crate::time::{Tempo, TimeSignature};
use crate::track::Track;
use crate::version::{CURRENT_VERSION, Version};

/// Represents an `OpenUtau` project file (`.ustx`).
///
/// This is the root object of a `.ustx` file. It contains all the project settings,
/// tracks, parts, and other data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    /// The name of the project.
    #[serde(default = "default_project_name")]
    pub name: String,
    /// A comment for the project.
    #[serde(default)]
    pub comment: String,
    /// The output directory for rendered audio.
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    /// The cache directory for temporary files.
    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
    /// The version of the `.ustx` file format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ustx_version: Option<Version>,
    /// The resolution of the project, in ticks per quarter note.
    #[serde(default = "default_resolution")]
    pub resolution: i32,
    /// The initial tempo of the project, in beats per minute.
    #[serde(default = "default_bpm")]
    pub bpm: f64,
    /// The number of beats per bar.
    #[serde(default = "default_beat_per_bar")]
    pub beat_per_bar: i32,
    /// The beat unit, which defines the note value that represents one beat.
    #[serde(default = "default_beat_unit")]
    pub beat_unit: i32,
    /// A map of expression names to their descriptors.
    #[serde(default)]
    pub expressions: BTreeMap<String, ExpressionDescriptor>,
    /// A list of expression selectors.
    #[serde(default = "default_exp_selectors")]
    pub exp_selectors: Vec<String>,
    /// The index of the primary expression.
    #[serde(default)]
    pub exp_primary: i32,
    /// The index of the secondary expression.
    #[serde(default = "default_exp_secondary")]
    pub exp_secondary: i32,
    /// The key of the project.
    #[serde(default)]
    pub key: i32,
    /// A list of time signatures in the project.
    #[serde(default = "default_time_signatures")]
    pub time_signatures: Vec<TimeSignature>,
    /// A list of tempos in the project.
    #[serde(default = "default_tempos")]
    pub tempos: Vec<Tempo>,
    /// A list of tracks in the project.
    #[serde(default = "default_tracks")]
    pub tracks: Vec<Track>,
    /// A list of voice parts in the project.
    #[serde(default)]
    pub voice_parts: Vec<VoicePart>,
    /// A list of wave parts in the project.
    #[serde(default)]
    pub wave_parts: Vec<WavePart>,
}

impl Project {
    /// Deserializes a `Project` from a YAML string.
    #[inline]
    pub fn from_yaml_str(input: &str) -> Result<Self, Error> {
        let mut documents = serde_yaml::Deserializer::from_str(input);
        let document = documents.next().ok_or_else(|| Error::MissingDocument)?;
        let project = Project::deserialize(document)?;
        Ok(project)
    }

    /// Serializes a `Project` to a YAML string.
    #[inline]
    pub fn to_yaml_string(&self) -> Result<String, Error> {
        serde_yaml::to_string(self).map_err(Error::from)
    }

    /// Deserializes a `Project` from a YAML string with compatibility upgrades.
    ///
    /// This function will attempt to upgrade the project from older formats to the
    /// current format.
    #[inline]
    pub fn from_yaml_str_with_compat(input: &str) -> Result<Self, Error> {
        let mut project = Self::from_yaml_str(input)?;
        project.convert_to(CURRENT_VERSION)?;
        Ok(project)
    }

    /// Serializes a `Project` to a YAML string with compatibility upgrades.
    #[inline]
    pub fn to_yaml_string_with_compat(&self) -> Result<String, Error> {
        let mut project = self.clone();
        project.convert_to(CURRENT_VERSION)?;
        serde_yaml::to_string(&project).map_err(Error::from)
    }

    /// Converts the project to the specified `target` version.
    pub fn convert_to(&mut self, target: Version) -> Result<(), Error> {
        let detected = self.ustx_version.unwrap_or_else(Version::zero);

        if target > CURRENT_VERSION {
            return Err(Error::unsupported_version(target.to_string()));
        }
        if detected > CURRENT_VERSION {
            return Err(Error::unsupported_version(detected.to_string()));
        }
        if detected >= target {
            self.ustx_version = Some(target);
            return Ok(());
        }

        if detected < VERSION_0_4 && target >= VERSION_0_4 {
            self.convert_pre_0_4();
        }
        if detected < VERSION_0_5 && target >= VERSION_0_5 {
            self.convert_pre_0_5();
        }
        if detected < VERSION_0_6 && target >= VERSION_0_6 {
            self.convert_pre_0_6();
        }
        if detected < VERSION_0_7 && target >= VERSION_0_7 {
            self.convert_pre_0_7();
        }

        self.ustx_version = Some(target);
        Ok(())
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_yaml_string_with_compat()
            .map_or(Err(fmt::Error), |text| f.write_str(&text))
    }
}

impl FromStr for Project {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_yaml_str_with_compat(s)
    }
}

#[inline]
fn default_project_name() -> String {
    String::from("New Project")
}

#[inline]
fn default_output_dir() -> String {
    String::from("Vocal")
}

#[inline]
fn default_cache_dir() -> String {
    String::from("UCache")
}

#[inline]
const fn default_resolution() -> i32 {
    480
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

#[inline]
fn default_exp_selectors() -> Vec<String> {
    vec![
        "dyn".into(),
        "pitd".into(),
        "clr".into(),
        "eng".into(),
        "vel".into(),
        "vol".into(),
        "atk".into(),
        "dec".into(),
        "gen".into(),
        "bre".into(),
    ]
}

#[inline]
const fn default_exp_secondary() -> i32 {
    1
}

#[inline]
fn default_time_signatures() -> Vec<TimeSignature> {
    vec![TimeSignature::default()]
}

#[inline]
fn default_tempos() -> Vec<Tempo> {
    vec![Tempo::default()]
}

#[inline]
fn default_tracks() -> Vec<Track> {
    vec![Track::default()]
}

const VERSION_0_4: Version = Version::new(0, 4, 0);
const VERSION_0_5: Version = Version::new(0, 5, 0);
const VERSION_0_6: Version = Version::new(0, 6, 0);
const VERSION_0_7: Version = Version::new(0, 7, 0);

const OLD_ACCENT_ABBR: &str = "acc";
const NEW_ACCENT_ABBR: &str = "atk";
const NEW_ACCENT_NAME: &str = "attack";
const DEFAULT_SELECTORS: [&str; 10] = [
    "dyn", "pitd", "clr", "eng", "vel", "vol", "atk", "dec", "gen", "bre",
];

impl Project {
    fn convert_pre_0_4(&mut self) {
        if self
            .expressions
            .get(OLD_ACCENT_ABBR)
            .is_none_or(|descriptor| descriptor.name != "accent")
        {
            return;
        }
        let Some(mut descriptor) = self.expressions.remove(OLD_ACCENT_ABBR) else {
            return;
        };
        descriptor.abbr = String::from(NEW_ACCENT_ABBR);
        descriptor.name = String::from(NEW_ACCENT_NAME);
        self.expressions
            .insert(String::from(NEW_ACCENT_ABBR), descriptor);

        for part in &mut self.voice_parts {
            for note in &mut part.notes {
                for expression in &mut note.phoneme_expressions {
                    if expression.abbr == OLD_ACCENT_ABBR {
                        expression.abbr = String::from(NEW_ACCENT_ABBR);
                    }
                }
            }
        }
    }

    fn convert_pre_0_5(&mut self) {
        for part in &mut self.voice_parts {
            for note in &mut part.notes {
                if note.lyric.starts_with("...") {
                    note.lyric = note.lyric.replace("...", "+");
                }
            }
        }
    }

    fn convert_pre_0_6(&mut self) {
        let beat_per_bar = if self.beat_per_bar > 0 {
            self.beat_per_bar
        } else {
            4
        };
        let beat_unit = if self.beat_unit > 0 {
            self.beat_unit
        } else {
            4
        };
        let bpm = if self.bpm > 0.0 { self.bpm } else { 120.0 };

        self.time_signatures = vec![TimeSignature {
            bar_position: 0,
            beat_per_bar,
            beat_unit,
        }];
        self.tempos = vec![Tempo { position: 0, bpm }];
    }

    fn convert_pre_0_7(&mut self) {
        if self.exp_selectors.len() >= DEFAULT_SELECTORS.len() {
            return;
        }
        let mut selectors = DEFAULT_SELECTORS
            .iter()
            .map(|&value| String::from(value))
            .collect::<Vec<_>>();
        for (index, existing) in self.exp_selectors.iter().enumerate() {
            if let Some(target) = selectors.get_mut(index) {
                target.clone_from(existing);
            } else {
                break;
            }
        }
        self.exp_selectors = selectors;
    }
}

#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::cognitive_complexity,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::too_many_lines
)]

//! The `ustx` crate provides a Rust implementation of the `OpenUtau` project file format.
//!
//! This crate allows you to read, write, and manipulate `OpenUtau` projects in Rust.
//! It provides data structures for all the components of a `.ustx` file,
//! including the project itself, tracks, parts, notes, and expressions.

pub mod error;
pub mod expression;
pub mod note;
pub mod part;
pub mod phoneme;
pub mod project;
pub mod time;
pub mod track;
pub mod version;

pub use error::Error;
pub use expression::{Curve, Expression, ExpressionDescriptor, ExpressionType};
pub use note::{Note, Pitch, PitchPoint, PitchPointShape, Vibrato};
pub use part::{VoicePart, WavePart};
pub use phoneme::PhonemeOverride;
pub use project::Project;
pub use time::{Tempo, TimeSignature};
pub use track::{RenderSettings, Track};
pub use version::{CURRENT_VERSION, Version};

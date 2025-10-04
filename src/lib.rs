//! # Phyla-Lang: Procedural Language Generation
//!
//! A Rust library that generates consistent, deterministic constructed languages (conlangs)
//! based on cultural personality traits and geographic influences.
//!
//! ## Core Concepts
//!
//! Languages **emerge** from parameterized cultural profiles. The same input parameters
//! always produce the same linguistic output (deterministic generation).
//!
//! ## Quick Start
//!
//! ```rust
//! use phyla_lang::{Language, CulturalProfile, Geography};
//!
//! // Create a language from cultural parameters
//! let coastal_culture = CulturalProfile {
//!     agreeableness: 4.0,
//!     openness: 3.0,
//!     conscientiousness: 2.0,
//!     extraversion: 3.0,
//!     honesty_humility: 3.0,
//!     emotionality: 4.0,
//! };
//!
//! let language = Language::from_culture(
//!     coastal_culture,
//!     Geography::Coastal,
//!     12345, // seed for deterministic generation
//! );
//!
//! // Translate words and phrases
//! let word = language.translate_word("house");
//! let phrase = language.translate_phrase("I bring the beer quickly");
//!
//! // The same input always produces the same output
//! assert_eq!(word, language.translate_word("house"));
//! ```

mod culture;
mod generation;
mod genome;
mod language;
mod morphology;
pub mod naming;
mod phonology;
mod seeded_rng;

pub use culture::{CulturalProfile, Geography};
pub use genome::{LinguisticGenome, MorphologyType, WordOrder};
pub use language::Language;
pub use morphology::{CombiningRule, Morpheme, MorphemeDatabase, MorphemeType};
pub use naming::{
    epithet::{Characteristic, EpithetContext},
    personal::PersonalNameContext,
    place::{PlaceNameContext, PlaceType},
    NamePattern, NamingSystem,
};
pub use phonology::{Consonant, PhonemeInventory, SyllableStructure, Vowel};


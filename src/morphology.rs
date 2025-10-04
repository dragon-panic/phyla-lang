//! Morphology: the building blocks of words and names.
//!
//! Morphemes are the smallest meaningful units in a language. This module provides
//! the infrastructure for generating and combining morphemes consistently.

use crate::culture::{CulturalProfile, Geography};
use crate::generation::generate_word;
use crate::genome::LinguisticGenome;
use crate::seeded_rng::SeededRng;
use std::collections::HashMap;

/// The semantic type of a morpheme - what it means conceptually.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MorphemeType {
    // Natural elements
    Fire,
    Water,
    Earth,
    Air,
    Stone,
    Mountain,
    River,
    Forest,
    Sea,
    Sky,
    Storm,
    Sun,
    Moon,
    Star,
    
    // Qualities
    Great,
    Small,
    Ancient,
    Young,
    Strong,
    Wise,
    Swift,
    Brave,
    Gentle,
    Dark,
    Bright,
    Cold,
    Warm,
    
    // Actions
    Strike,
    Protect,
    Create,
    Destroy,
    Walk,
    Fly,
    Swim,
    Speak,
    See,
    Hear,
    
    // Virtues
    Honor,
    Courage,
    Peace,
    War,
    Love,
    Hope,
    Faith,
    Truth,
    Justice,
    
    // Abstract
    Spirit,
    Soul,
    Heart,
    Mind,
    Power,
    Life,
    Death,
    Time,
    Fate,
}

impl MorphemeType {
    /// Get all morpheme types as a slice.
    pub fn all() -> &'static [MorphemeType] {
        &[
            // Elements
            MorphemeType::Fire, MorphemeType::Water, MorphemeType::Earth, MorphemeType::Air,
            MorphemeType::Stone, MorphemeType::Mountain, MorphemeType::River, MorphemeType::Forest,
            MorphemeType::Sea, MorphemeType::Sky, MorphemeType::Storm, MorphemeType::Sun,
            MorphemeType::Moon, MorphemeType::Star,
            // Qualities
            MorphemeType::Great, MorphemeType::Small, MorphemeType::Ancient, MorphemeType::Young,
            MorphemeType::Strong, MorphemeType::Wise, MorphemeType::Swift, MorphemeType::Brave,
            MorphemeType::Gentle, MorphemeType::Dark, MorphemeType::Bright, MorphemeType::Cold,
            MorphemeType::Warm,
            // Actions
            MorphemeType::Strike, MorphemeType::Protect, MorphemeType::Create, MorphemeType::Destroy,
            MorphemeType::Walk, MorphemeType::Fly, MorphemeType::Swim, MorphemeType::Speak,
            MorphemeType::See, MorphemeType::Hear,
            // Virtues
            MorphemeType::Honor, MorphemeType::Courage, MorphemeType::Peace, MorphemeType::War,
            MorphemeType::Love, MorphemeType::Hope, MorphemeType::Faith, MorphemeType::Truth,
            MorphemeType::Justice,
            // Abstract
            MorphemeType::Spirit, MorphemeType::Soul, MorphemeType::Heart, MorphemeType::Mind,
            MorphemeType::Power, MorphemeType::Life, MorphemeType::Death, MorphemeType::Time,
            MorphemeType::Fate,
        ]
    }
    
    /// Convert to a string key for word generation.
    pub fn as_str(&self) -> &'static str {
        match self {
            MorphemeType::Fire => "fire",
            MorphemeType::Water => "water",
            MorphemeType::Earth => "earth",
            MorphemeType::Air => "air",
            MorphemeType::Stone => "stone",
            MorphemeType::Mountain => "mountain",
            MorphemeType::River => "river",
            MorphemeType::Forest => "forest",
            MorphemeType::Sea => "sea",
            MorphemeType::Sky => "sky",
            MorphemeType::Storm => "storm",
            MorphemeType::Sun => "sun",
            MorphemeType::Moon => "moon",
            MorphemeType::Star => "star",
            MorphemeType::Great => "great",
            MorphemeType::Small => "small",
            MorphemeType::Ancient => "ancient",
            MorphemeType::Young => "young",
            MorphemeType::Strong => "strong",
            MorphemeType::Wise => "wise",
            MorphemeType::Swift => "swift",
            MorphemeType::Brave => "brave",
            MorphemeType::Gentle => "gentle",
            MorphemeType::Dark => "dark",
            MorphemeType::Bright => "bright",
            MorphemeType::Cold => "cold",
            MorphemeType::Warm => "warm",
            MorphemeType::Strike => "strike",
            MorphemeType::Protect => "protect",
            MorphemeType::Create => "create",
            MorphemeType::Destroy => "destroy",
            MorphemeType::Walk => "walk",
            MorphemeType::Fly => "fly",
            MorphemeType::Swim => "swim",
            MorphemeType::Speak => "speak",
            MorphemeType::See => "see",
            MorphemeType::Hear => "hear",
            MorphemeType::Honor => "honor",
            MorphemeType::Courage => "courage",
            MorphemeType::Peace => "peace",
            MorphemeType::War => "war",
            MorphemeType::Love => "love",
            MorphemeType::Hope => "hope",
            MorphemeType::Faith => "faith",
            MorphemeType::Truth => "truth",
            MorphemeType::Justice => "justice",
            MorphemeType::Spirit => "spirit",
            MorphemeType::Soul => "soul",
            MorphemeType::Heart => "heart",
            MorphemeType::Mind => "mind",
            MorphemeType::Power => "power",
            MorphemeType::Life => "life",
            MorphemeType::Death => "death",
            MorphemeType::Time => "time",
            MorphemeType::Fate => "fate",
        }
    }
    
    /// Check if this morpheme type is culturally salient based on geography.
    pub fn cultural_weight(&self, geography: &Geography, culture: &CulturalProfile) -> f32 {
        let mut weight: f32 = 1.0;
        
        // Geography influences
        match geography {
            Geography::Mountains => {
                match self {
                    MorphemeType::Mountain | MorphemeType::Stone | MorphemeType::Sky => weight += 2.0,
                    MorphemeType::Strong | MorphemeType::Cold => weight += 1.0,
                    _ => {}
                }
            }
            Geography::Coastal => {
                match self {
                    MorphemeType::Sea | MorphemeType::Water | MorphemeType::Storm => weight += 2.0,
                    MorphemeType::Swim | MorphemeType::Gentle => weight += 1.0,
                    _ => {}
                }
            }
            Geography::Desert => {
                match self {
                    MorphemeType::Sun | MorphemeType::Fire | MorphemeType::Stone => weight += 2.0,
                    MorphemeType::Warm | MorphemeType::Swift => weight += 1.0,
                    _ => {}
                }
            }
            Geography::Forest => {
                match self {
                    MorphemeType::Forest | MorphemeType::Earth | MorphemeType::Life => weight += 2.0,
                    MorphemeType::Gentle | MorphemeType::Wise => weight += 1.0,
                    _ => {}
                }
            }
            Geography::Plains | Geography::RiverValley => {
                match self {
                    MorphemeType::River | MorphemeType::Sky | MorphemeType::Walk => weight += 1.0,
                    _ => {}
                }
            }
        }
        
        // Personality influences
        // High openness = more abstract concepts
        if culture.normalized_openness() > 0.6 {
            match self {
                MorphemeType::Spirit | MorphemeType::Soul | MorphemeType::Fate | 
                MorphemeType::Time | MorphemeType::Mind => weight += 1.0,
                _ => {}
            }
        }
        
        // High agreeableness = gentle/peaceful concepts
        if culture.normalized_agreeableness() > 0.6 {
            match self {
                MorphemeType::Peace | MorphemeType::Love | MorphemeType::Hope |
                MorphemeType::Gentle => weight += 1.0,
                MorphemeType::War | MorphemeType::Destroy | MorphemeType::Strike => weight -= 0.5,
                _ => {}
            }
        }
        
        // Low agreeableness = martial concepts
        if culture.normalized_agreeableness() < 0.4 {
            match self {
                MorphemeType::War | MorphemeType::Strike | MorphemeType::Destroy |
                MorphemeType::Power | MorphemeType::Strong => weight += 1.0,
                _ => {}
            }
        }
        
        // High emotionality = emotional concepts
        if culture.normalized_emotionality() > 0.6 {
            match self {
                MorphemeType::Heart | MorphemeType::Love | MorphemeType::Hope |
                MorphemeType::Soul => weight += 1.0,
                _ => {}
            }
        }
        
        weight.max(0.1) // Minimum weight
    }
}

/// A morpheme - a sound paired with meaning and cultural weight.
#[derive(Debug, Clone)]
pub struct Morpheme {
    /// The sound form of this morpheme in this language
    pub form: String,
    /// The semantic type/meaning
    pub meaning: MorphemeType,
    /// Cultural salience (how important/common this concept is)
    pub weight: f32,
}

/// A database of morphemes for a language, indexed by meaning.
#[derive(Debug, Clone)]
pub struct MorphemeDatabase {
    morphemes: HashMap<MorphemeType, Morpheme>,
}

impl MorphemeDatabase {
    /// Generate a complete morpheme database for a language.
    pub fn from_genome(
        genome: &LinguisticGenome,
        culture: &CulturalProfile,
        geography: &Geography,
    ) -> Self {
        let mut morphemes = HashMap::new();
        
        for &meaning in MorphemeType::all() {
            let form = generate_word(genome, meaning.as_str());
            let weight = meaning.cultural_weight(geography, culture);
            
            morphemes.insert(meaning, Morpheme {
                form,
                meaning,
                weight,
            });
        }
        
        Self { morphemes }
    }
    
    /// Get a morpheme by its meaning type.
    pub fn get(&self, meaning: &MorphemeType) -> Option<&Morpheme> {
        self.morphemes.get(meaning)
    }
    
    /// Select a weighted random morpheme suitable for naming.
    pub fn select_weighted(&self, rng: &mut SeededRng, _geography: &Geography) -> &Morpheme {
        // Get all morphemes with their weights
        let morphemes: Vec<&Morpheme> = self.morphemes.values().collect();
        let weights: Vec<f32> = morphemes.iter().map(|m| m.weight).collect();
        
        let idx = rng.weighted_choice(&weights);
        morphemes[idx]
    }
    
    /// Get morphemes of specific types.
    pub fn select_from_types(&self, types: &[MorphemeType], rng: &mut SeededRng) -> Option<&Morpheme> {
        let available: Vec<&Morpheme> = types.iter()
            .filter_map(|t| self.get(t))
            .collect();
        
        if available.is_empty() {
            return None;
        }
        
        let weights: Vec<f32> = available.iter().map(|m| m.weight).collect();
        let idx = rng.weighted_choice(&weights);
        Some(available[idx])
    }
}

/// Rules for combining morphemes into names.
#[derive(Debug, Clone, Copy)]
pub enum CombiningRule {
    /// Simple concatenation: "Fire" + "Stone" = "Firestone"
    Concatenate,
    /// With separator: "Fire" + "Stone" = "Fire-Stone"
    Hyphenated,
    /// Genitive form: "Fire" + "Stone" = "Stone of Fire"
    Genitive,
}

impl CombiningRule {
    /// Determine the combining rule based on cultural traits.
    pub fn from_culture(culture: &CulturalProfile) -> Self {
        // High conscientiousness = more structured (hyphenated)
        if culture.normalized_conscientiousness() > 0.6 {
            CombiningRule::Hyphenated
        }
        // High openness = more complex (genitive)
        else if culture.normalized_openness() > 0.7 {
            CombiningRule::Genitive
        }
        // Default = simple concatenation
        else {
            CombiningRule::Concatenate
        }
    }
    
    /// Combine two morphemes according to this rule.
    pub fn combine(&self, first: &str, second: &str) -> String {
        match self {
            CombiningRule::Concatenate => format!("{}{}", first, second),
            CombiningRule::Hyphenated => format!("{}-{}", first, second),
            CombiningRule::Genitive => format!("{} of {}", second, first),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::culture::{CulturalProfile, Geography};
    use crate::genome::LinguisticGenome;

    #[test]
    fn test_morpheme_type_conversion() {
        assert_eq!(MorphemeType::Fire.as_str(), "fire");
        assert_eq!(MorphemeType::Mountain.as_str(), "mountain");
    }

    #[test]
    fn test_cultural_weights() {
        let culture = CulturalProfile::new(4.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        
        // Mountains should weight mountain morphemes higher than sea morphemes
        let mountain_weight_in_mountains = MorphemeType::Mountain.cultural_weight(&Geography::Mountains, &culture);
        let sea_weight_in_mountains = MorphemeType::Sea.cultural_weight(&Geography::Mountains, &culture);
        
        assert!(mountain_weight_in_mountains > sea_weight_in_mountains);
        
        // Coastal should weight sea morphemes higher than in mountains
        let sea_weight_in_coastal = MorphemeType::Sea.cultural_weight(&Geography::Coastal, &culture);
        assert!(sea_weight_in_coastal > sea_weight_in_mountains);
        
        // Coastal should weight sea morphemes higher than mountain morphemes
        let mountain_weight_in_coastal = MorphemeType::Mountain.cultural_weight(&Geography::Coastal, &culture);
        assert!(sea_weight_in_coastal > mountain_weight_in_coastal);
    }

    #[test]
    fn test_morpheme_database_generation() {
        let culture = CulturalProfile::new(4.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        let db = MorphemeDatabase::from_genome(&genome, &culture, &Geography::Coastal);
        
        // Should have all morpheme types
        assert!(db.get(&MorphemeType::Fire).is_some());
        assert!(db.get(&MorphemeType::Water).is_some());
        
        // Each morpheme should have a form
        let fire = db.get(&MorphemeType::Fire).unwrap();
        assert!(!fire.form.is_empty());
    }

    #[test]
    fn test_combining_rules() {
        let concat = CombiningRule::Concatenate;
        let hyphen = CombiningRule::Hyphenated;
        let genitive = CombiningRule::Genitive;
        
        assert_eq!(concat.combine("fire", "stone"), "firestone");
        assert_eq!(hyphen.combine("fire", "stone"), "fire-stone");
        assert_eq!(genitive.combine("fire", "stone"), "stone of fire");
    }

    #[test]
    fn test_deterministic_morpheme_generation() {
        let culture = CulturalProfile::new(4.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome1 = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        let genome2 = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        
        let db1 = MorphemeDatabase::from_genome(&genome1, &culture, &Geography::Coastal);
        let db2 = MorphemeDatabase::from_genome(&genome2, &culture, &Geography::Coastal);
        
        // Same seed should produce identical morphemes
        assert_eq!(
            db1.get(&MorphemeType::Fire).unwrap().form,
            db2.get(&MorphemeType::Fire).unwrap().form
        );
    }
}


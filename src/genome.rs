//! The linguistic genome - the complete "DNA" of a language.

use crate::culture::{CulturalProfile, Geography};
use crate::phonology::{Consonant, PhonemeInventory, ProsodicSystem, SyllableStructure, Vowel};

/// Word order patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordOrder {
    /// Subject-Verb-Object (English, Mandarin)
    SVO,
    /// Subject-Object-Verb (Japanese, Turkish)
    SOV,
    /// Verb-Subject-Object (Irish, Arabic)
    VSO,
    /// Verb-Object-Subject (Malagasy)
    VOS,
    /// Object-Verb-Subject (rare)
    OVS,
    /// Object-Subject-Verb (very rare)
    OSV,
}

/// Morphological type of the language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphologyType {
    /// Words are typically single morphemes (Chinese, Vietnamese)
    Isolating,
    /// Words are formed by stringing together morphemes (Turkish, Japanese)
    Agglutinative,
    /// Morphemes are fused together (Latin, Russian)
    Fusional,
}

/// The complete linguistic genome - all parameters needed to generate consistent output.
#[derive(Debug, Clone)]
pub struct LinguisticGenome {
    /// The phoneme inventory (available sounds)
    pub phoneme_inventory: PhonemeInventory,

    /// Allowed syllable patterns
    pub syllable_patterns: Vec<SyllableStructure>,

    /// Prosodic system (stress, tone)
    pub prosody: ProsodicSystem,

    /// Morphological type
    pub morphology_type: MorphologyType,

    /// Word order
    pub word_order: WordOrder,

    /// Generation seed for determinism
    pub seed: u64,
}

impl LinguisticGenome {
    /// Generate a genome from cultural parameters and geography.
    pub fn from_culture(culture: CulturalProfile, geography: Geography, seed: u64) -> Self {
        let phoneme_inventory = Self::generate_phoneme_inventory(&culture, &geography);
        let syllable_patterns = Self::generate_syllable_patterns(&culture, &geography);
        let word_order = Self::determine_word_order(&culture, seed);
        let morphology_type = Self::determine_morphology(&culture);

        Self {
            phoneme_inventory,
            syllable_patterns,
            prosody: ProsodicSystem::default(),
            morphology_type,
            word_order,
            seed,
        }
    }

    /// Generate phoneme inventory based on cultural traits and geography.
    fn generate_phoneme_inventory(
        culture: &CulturalProfile,
        geography: &Geography,
    ) -> PhonemeInventory {
        let agreeableness = culture.normalized_agreeableness();
        let openness = culture.normalized_openness();
        let _emotionality = culture.normalized_emotionality();

        // Base consonants that most languages have
        let mut stops = vec![
            Consonant::new("p"),
            Consonant::new("t"),
            Consonant::new("k"),
        ];
        let mut fricatives = vec![Consonant::new("s"), Consonant::new("h")];
        let mut nasals = vec![Consonant::new("m"), Consonant::new("n")];
        let liquids = vec![Consonant::new("l"), Consonant::new("r")];
        let glides = vec![];

        // Adjust based on geography
        match geography {
            Geography::Mountains => {
                // Add ejectives and glottal stops
                stops.push(Consonant::new("kʼ"));
                stops.push(Consonant::new("tʼ"));
                fricatives.push(Consonant::new("x"));
                fricatives.push(Consonant::new("ʃ"));
            }
            Geography::Coastal => {
                // More liquids and soft sounds
                fricatives.push(Consonant::new("f"));
                fricatives.push(Consonant::new("v"));
            }
            Geography::Desert => {
                // Guttural and emphatic consonants
                stops.push(Consonant::new("q"));
                fricatives.push(Consonant::new("ʃ"));
                fricatives.push(Consonant::new("x"));
                fricatives.push(Consonant::new("ħ"));
                fricatives.push(Consonant::new("ʕ"));
            }
            Geography::Forest => {
                // Softer sounds, more nasals
                fricatives.push(Consonant::new("f"));
                nasals.push(Consonant::new("ŋ"));
            }
            Geography::RiverValley | Geography::Plains => {
                // Balanced inventory
                stops.push(Consonant::new("b"));
                stops.push(Consonant::new("d"));
                stops.push(Consonant::new("g"));
                fricatives.push(Consonant::new("f"));
                fricatives.push(Consonant::new("v"));
                fricatives.push(Consonant::new("z"));
                fricatives.push(Consonant::new("ʃ"));
                fricatives.push(Consonant::new("ʒ"));
            }
        }

        // Base vowels
        let mut vowels = vec![
            Vowel::new("a"),
            Vowel::new("i"),
            Vowel::new("u"),
        ];

        // Add more vowels for high openness
        if openness > 0.5 {
            vowels.push(Vowel::new("e"));
            vowels.push(Vowel::new("o"));
        }

        if openness > 0.7 {
            vowels.push(Vowel::new("ə"));
        }

        // Weight calculation
        // High agreeableness = more nasals and liquids
        // Low agreeableness = more stops
        let nasal_weight = 0.15 + agreeableness * 0.15;
        let liquid_weight = 0.15 + agreeableness * 0.15;
        let stop_weight = 0.30 - agreeableness * 0.10;
        let fricative_weight = 0.25;
        let glide_weight = 0.10;

        let category_weights = vec![
            stop_weight,
            fricative_weight,
            nasal_weight,
            liquid_weight,
            glide_weight,
        ];

        PhonemeInventory {
            stops,
            fricatives,
            nasals,
            liquids,
            glides,
            vowels,
            category_weights,
        }
    }

    /// Generate syllable patterns based on cultural traits.
    fn generate_syllable_patterns(
        culture: &CulturalProfile,
        geography: &Geography,
    ) -> Vec<SyllableStructure> {
        let openness = culture.normalized_openness();
        let emotionality = culture.normalized_emotionality();
        let conscientiousness = culture.normalized_conscientiousness();

        let mut patterns = vec![
            SyllableStructure::CV,  // Universal - all languages have this
            SyllableStructure::CVC,
        ];

        // High emotionality = more vowels
        if emotionality > 0.5 {
            patterns.push(SyllableStructure::V);
            patterns.push(SyllableStructure::CVV);
        }

        // High openness = more complex patterns
        if openness > 0.5 {
            patterns.push(SyllableStructure::CCV);
            patterns.push(SyllableStructure::CCVC);
        }

        if openness > 0.7 {
            patterns.push(SyllableStructure::CVCC);
            patterns.push(SyllableStructure::VCC);
        }

        // Low conscientiousness = add some irregular patterns
        if conscientiousness < 0.3 {
            patterns.push(SyllableStructure::VC);
        }

        // Geography influences
        match geography {
            Geography::Mountains => {
                // Prefer closed syllables (ending in consonants)
                patterns.push(SyllableStructure::CVC);
                patterns.push(SyllableStructure::CCVC);
            }
            Geography::Coastal => {
                // More open syllables
                patterns.push(SyllableStructure::CV);
                patterns.push(SyllableStructure::V);
                patterns.push(SyllableStructure::CVV);
            }
            _ => {}
        }

        patterns
    }

    /// Determine word order based on culture.
    fn determine_word_order(culture: &CulturalProfile, seed: u64) -> WordOrder {
        use crate::seeded_rng::SeededRng;

        let conscientiousness = culture.normalized_conscientiousness();

        // High conscientiousness tends toward SOV (structured, verb at end)
        // Low conscientiousness tends toward VSO or VOS (verb-initial)
        // Medium tends toward SVO (most common cross-linguistically)

        let mut rng = SeededRng::new(seed.wrapping_mul(7919));

        if conscientiousness > 0.7 {
            // Prefer SOV
            if rng.next() < 0.8 {
                WordOrder::SOV
            } else {
                WordOrder::SVO
            }
        } else if conscientiousness < 0.3 {
            // Prefer VSO
            if rng.next() < 0.7 {
                WordOrder::VSO
            } else {
                WordOrder::VOS
            }
        } else {
            // Prefer SVO
            if rng.next() < 0.7 {
                WordOrder::SVO
            } else if rng.next() < 0.5 {
                WordOrder::SOV
            } else {
                WordOrder::VSO
            }
        }
    }

    /// Determine morphology type.
    fn determine_morphology(culture: &CulturalProfile) -> MorphologyType {
        let conscientiousness = culture.normalized_conscientiousness();
        let openness = culture.normalized_openness();

        // High conscientiousness + high openness = agglutinative (regular but complex)
        // High conscientiousness + low openness = isolating (regular and simple)
        // Low conscientiousness = fusional (irregular)

        if conscientiousness > 0.6 {
            if openness > 0.6 {
                MorphologyType::Agglutinative
            } else {
                MorphologyType::Isolating
            }
        } else {
            MorphologyType::Fusional
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genome_generation() {
        let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);

        assert!(!genome.phoneme_inventory.vowels.is_empty());
        assert!(!genome.syllable_patterns.is_empty());
    }

    #[test]
    fn test_deterministic_word_order() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome1 = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);
        let genome2 = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);

        assert_eq!(genome1.word_order, genome2.word_order);
    }

    #[test]
    fn test_different_geographies() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let coastal = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        let mountain = LinguisticGenome::from_culture(culture, Geography::Mountains, 12345);

        // Different geographies should produce different phoneme inventories
        // Mountains add ejectives to stops, so they should have more stops
        assert_ne!(
            coastal.phoneme_inventory.stops.len(),
            mountain.phoneme_inventory.stops.len()
        );
    }
}


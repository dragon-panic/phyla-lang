//! Word and phrase generation algorithms.

use crate::genome::LinguisticGenome;
use crate::phonology::PhonemeCategory;
use crate::seeded_rng::{hash_deterministic, SeededRng};

/// Generate a word for a given concept using deterministic generation.
pub fn generate_word(genome: &LinguisticGenome, concept: &str) -> String {
    let seed = hash_deterministic(concept, genome.seed);
    let mut rng = SeededRng::new(seed);

    // Determine syllable count based on concept length
    let syllable_count = if concept.len() < 4 {
        1 + rng.range(0, 2)
    } else {
        2 + rng.range(0, 2)
    };

    let mut word = String::new();

    for _ in 0..syllable_count {
        let syllable = generate_syllable(genome, &mut rng);
        word.push_str(&syllable);
    }

    word
}

/// Generate a single syllable following the language's patterns.
fn generate_syllable(genome: &LinguisticGenome, rng: &mut SeededRng) -> String {
    let pattern = rng.choice(&genome.syllable_patterns);
    let pattern_str = pattern.pattern();

    let mut syllable = String::new();

    for ch in pattern_str.chars() {
        match ch {
            'C' => {
                let consonant = choose_consonant(genome, rng);
                syllable.push_str(&consonant);
            }
            'V' => {
                let vowel = rng.choice(&genome.phoneme_inventory.vowels);
                syllable.push_str(&vowel.0);
            }
            _ => {}
        }
    }

    syllable
}

/// Choose a consonant based on weighted category probabilities.
fn choose_consonant(genome: &LinguisticGenome, rng: &mut SeededRng) -> String {
    let categories = genome.phoneme_inventory.available_categories();

    if categories.is_empty() {
        return String::new();
    }

    // Get weights for available categories
    let weights: Vec<f32> = categories
        .iter()
        .map(|cat| {
            let idx = match cat {
                PhonemeCategory::Stops => 0,
                PhonemeCategory::Fricatives => 1,
                PhonemeCategory::Nasals => 2,
                PhonemeCategory::Liquids => 3,
                PhonemeCategory::Glides => 4,
            };
            genome.phoneme_inventory.category_weights[idx]
        })
        .collect();

    let category_idx = rng.weighted_choice(&weights);
    let category = categories[category_idx];

    let consonants = genome.phoneme_inventory.get_category(category);
    let consonant = rng.choice(consonants);

    consonant.0.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::culture::{CulturalProfile, Geography};
    use crate::genome::LinguisticGenome;

    #[test]
    fn test_word_generation_determinism() {
        let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);

        let word1 = generate_word(&genome, "house");
        let word2 = generate_word(&genome, "house");

        assert_eq!(word1, word2);
    }

    #[test]
    fn test_different_concepts_different_words() {
        let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);

        let word1 = generate_word(&genome, "house");
        let word2 = generate_word(&genome, "tree");

        assert_ne!(word1, word2);
    }

    #[test]
    fn test_word_not_empty() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);

        let word = generate_word(&genome, "test");
        assert!(!word.is_empty());
    }

    #[test]
    fn test_syllable_generation() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);
        let mut rng = SeededRng::new(42);

        let syllable = generate_syllable(&genome, &mut rng);
        assert!(!syllable.is_empty());
    }
}


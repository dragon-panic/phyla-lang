//! The main Language struct and its public API.

use crate::culture::{CulturalProfile, Geography};
use crate::generation::generate_word;
use crate::genome::{LinguisticGenome, WordOrder};
use crate::naming::NamingSystem;
use std::collections::HashMap;
use std::sync::Mutex;

/// A complete language with its genome and optional caching.
pub struct Language {
    /// Unique identifier for this language
    pub id: String,

    /// The linguistic genome (complete language specification)
    pub genome: LinguisticGenome,

    /// Cultural profile used to generate this language
    culture: CulturalProfile,

    /// Geography
    geography: Geography,

    /// Naming system for generating names
    pub naming: NamingSystem,

    /// Optional cache for frequently-used words
    lexicon_cache: Mutex<HashMap<String, String>>,
}

impl Language {
    /// Create a new language from a cultural profile and geography.
    ///
    /// # Arguments
    ///
    /// * `culture` - The cultural personality profile
    /// * `geography` - The geographic environment
    /// * `seed` - Seed for deterministic generation
    ///
    /// # Example
    ///
    /// ```
    /// use phyla_lang::{Language, CulturalProfile, Geography};
    ///
    /// let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
    /// let language = Language::from_culture(culture, Geography::Coastal, 12345);
    /// ```
    pub fn from_culture(culture: CulturalProfile, geography: Geography, seed: u64) -> Self {
        let genome = LinguisticGenome::from_culture(culture, geography, seed);
        let naming = NamingSystem::new(genome.clone(), culture, geography);
        let id = format!("lang_{}", seed);

        Self {
            id,
            genome,
            culture,
            geography,
            naming,
            lexicon_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Create a language directly from a genome.
    ///
    /// Note: This requires providing culture and geography for the naming system.
    pub fn from_genome(
        genome: LinguisticGenome,
        culture: CulturalProfile,
        geography: Geography,
    ) -> Self {
        let naming = NamingSystem::new(genome.clone(), culture, geography);
        let id = format!("lang_{}", genome.seed);

        Self {
            id,
            genome,
            culture,
            geography,
            naming,
            lexicon_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Translate a single word/concept to this language.
    ///
    /// # Example
    ///
    /// ```
    /// use phyla_lang::{Language, CulturalProfile, Geography};
    ///
    /// let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
    /// let language = Language::from_culture(culture, Geography::Coastal, 12345);
    ///
    /// let word = language.translate_word("house");
    /// // The same input always produces the same output
    /// assert_eq!(word, language.translate_word("house"));
    /// ```
    pub fn translate_word(&self, concept: &str) -> String {
        let concept = concept.to_lowercase();

        // Check cache first
        {
            let cache = self.lexicon_cache.lock().unwrap();
            if let Some(cached) = cache.get(&concept) {
                return cached.clone();
            }
        }

        // Generate word
        let word = generate_word(&self.genome, &concept);

        // Cache it
        {
            let mut cache = self.lexicon_cache.lock().unwrap();
            cache.insert(concept, word.clone());
        }

        word
    }

    /// Translate a phrase to this language.
    ///
    /// This splits the phrase into words, translates each word,
    /// and applies the language's word order rules.
    ///
    /// # Example
    ///
    /// ```
    /// use phyla_lang::{Language, CulturalProfile, Geography};
    ///
    /// let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
    /// let language = Language::from_culture(culture, Geography::Coastal, 12345);
    ///
    /// let phrase = language.translate_phrase("I bring the beer quickly");
    /// assert!(!phrase.is_empty());
    /// ```
    pub fn translate_phrase(&self, phrase: &str) -> String {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        if words.is_empty() {
            return String::new();
        }

        // Translate each word
        let mut translated: Vec<String> = words.iter().map(|w| self.translate_word(w)).collect();

        // Apply word order transformation
        self.apply_word_order(&mut translated);

        translated.join(" ")
    }

    /// Apply the language's word order to a list of words.
    ///
    /// This is a simplified version that assumes Subject-Verb-Object pattern
    /// in the input and reorders according to the language's word order.
    fn apply_word_order(&self, words: &mut Vec<String>) {
        if words.len() < 3 {
            return; // Need at least 3 words for reordering
        }

        // Simple heuristic: assume format is S V O ...
        // In a real implementation, this would use proper syntactic parsing
        match self.genome.word_order {
            WordOrder::SVO => {
                // Already in SVO, no change needed
            }
            WordOrder::SOV => {
                // S V O ... -> S O V ...
                // Move verb (position 1) to after object (position 2)
                let verb = words.remove(1);
                words.insert(2, verb);
            }
            WordOrder::VSO => {
                // S V O ... -> V S O ...
                // Move verb (position 1) to front
                let verb = words.remove(1);
                words.insert(0, verb);
            }
            WordOrder::VOS => {
                // S V O ... -> V O S ...
                let subject = words.remove(0);
                let verb = words.remove(0); // Now at position 0 after previous removal
                words.insert(0, verb);
                words.push(subject);
            }
            WordOrder::OVS => {
                // S V O ... -> O V S ...
                let subject = words.remove(0);
                words.push(subject);
            }
            WordOrder::OSV => {
                // S V O ... -> O S V ...
                let subject = words.remove(0);
                let verb = words.remove(0);
                words.insert(0, verb);
                words.insert(0, subject);
            }
        }
    }

    /// Get the word order of this language.
    pub fn word_order(&self) -> WordOrder {
        self.genome.word_order
    }

    /// Clear the lexicon cache.
    pub fn clear_cache(&self) {
        let mut cache = self.lexicon_cache.lock().unwrap();
        cache.clear();
    }

    /// Get the number of cached words.
    pub fn cache_size(&self) -> usize {
        let cache = self.lexicon_cache.lock().unwrap();
        cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_creation() {
        let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let language = Language::from_culture(culture, Geography::Coastal, 12345);

        assert_eq!(language.id, "lang_12345");
    }

    #[test]
    fn test_word_translation() {
        let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let language = Language::from_culture(culture, Geography::Coastal, 12345);

        let word1 = language.translate_word("house");
        let word2 = language.translate_word("house");

        assert_eq!(word1, word2);
        assert!(!word1.is_empty());
    }

    #[test]
    fn test_phrase_translation() {
        let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let language = Language::from_culture(culture, Geography::Coastal, 12345);

        let phrase = language.translate_phrase("I bring the beer quickly");
        assert!(!phrase.is_empty());
    }

    #[test]
    fn test_cache() {
        let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let language = Language::from_culture(culture, Geography::Coastal, 12345);

        assert_eq!(language.cache_size(), 0);

        language.translate_word("house");
        assert_eq!(language.cache_size(), 1);

        language.translate_word("house");
        assert_eq!(language.cache_size(), 1); // Should still be 1 (cached)

        language.translate_word("tree");
        assert_eq!(language.cache_size(), 2);

        language.clear_cache();
        assert_eq!(language.cache_size(), 0);
    }

    #[test]
    fn test_different_languages() {
        let culture1 = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
        let culture2 = CulturalProfile::new(1.0, 2.0, 4.0, 2.0, 3.0, 2.0);

        let lang1 = Language::from_culture(culture1, Geography::Coastal, 12345);
        let lang2 = Language::from_culture(culture2, Geography::Mountains, 67890);

        let word1 = lang1.translate_word("house");
        let word2 = lang2.translate_word("house");

        // Different languages should produce different words
        assert_ne!(word1, word2);
    }
}


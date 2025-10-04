//! Naming system: generating culturally-consistent names for entities, places, and objects.
//!
//! This module provides the infrastructure for generating names that emerge from
//! the same cultural and linguistic foundations as the language itself.

pub mod personal;
pub mod place;
pub mod epithet;

use crate::culture::{CulturalProfile, Geography};
use crate::genome::LinguisticGenome;
use crate::morphology::{CombiningRule, MorphemeDatabase};
use crate::seeded_rng::{hash_deterministic, SeededRng};

/// The pattern for generating names in a culture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamePattern {
    /// Simple given name (e.g., "Aria")
    Simple,
    /// Patronymic: Given + Father's name (e.g., "Aran Thorson")
    Patronymic,
    /// Compound: Multiple morphemes (e.g., "Stormborn", "Ironheart")
    Compound,
    /// Elaborate: Title + Name + Lineage (e.g., "Lord Maxim the Third")
    Elaborate,
    /// Descriptive: Name + Characteristic (e.g., "Elara Brighteyes")
    Descriptive,
}

impl NamePattern {
    /// Determine the naming pattern from cultural traits.
    pub fn from_culture(culture: &CulturalProfile) -> Self {
        let openness = culture.normalized_openness();
        let conscientiousness = culture.normalized_conscientiousness();
        let honesty = culture.honesty_humility;
        
        // Low honesty-humility = elaborate names
        if honesty < 2.5 {
            return NamePattern::Elaborate;
        }
        
        // High openness = compound/descriptive names
        if openness > 0.7 {
            return NamePattern::Compound;
        }
        
        // High conscientiousness = structured patronymic
        if conscientiousness > 0.6 {
            return NamePattern::Patronymic;
        }
        
        // Default: simple names
        NamePattern::Simple
    }
}

/// Configuration for the naming system derived from culture.
#[derive(Debug, Clone)]
pub struct NamingSystem {
    /// The linguistic genome
    pub genome: LinguisticGenome,
    /// Cultural profile
    pub culture: CulturalProfile,
    /// Geography
    pub geography: Geography,
    /// Morpheme database
    pub morphemes: MorphemeDatabase,
    /// Naming pattern
    pub pattern: NamePattern,
    /// Combining rule for compound names
    pub combining_rule: CombiningRule,
    /// Average syllables per name component
    pub syllables_per_name: usize,
}

impl NamingSystem {
    /// Create a new naming system from cultural parameters.
    pub fn new(
        genome: LinguisticGenome,
        culture: CulturalProfile,
        geography: Geography,
    ) -> Self {
        let morphemes = MorphemeDatabase::from_genome(&genome, &culture, &geography);
        let pattern = NamePattern::from_culture(&culture);
        let combining_rule = CombiningRule::from_culture(&culture);
        
        // Name length influenced by geography and personality
        let syllables_per_name = Self::determine_name_length(&culture, &geography);
        
        Self {
            genome,
            culture,
            geography,
            morphemes,
            pattern,
            combining_rule,
            syllables_per_name,
        }
    }
    
    /// Determine typical name length based on culture.
    fn determine_name_length(culture: &CulturalProfile, geography: &Geography) -> usize {
        let mut syllables: usize = 2; // Base
        
        // High openness = longer names
        if culture.normalized_openness() > 0.6 {
            syllables += 1;
        }
        
        // Low honesty-humility = longer names
        if culture.honesty_humility < 2.5 {
            syllables += 1;
        }
        
        // Mountain cultures = shorter names (energy conservation)
        if matches!(geography, Geography::Mountains) {
            syllables = syllables.saturating_sub(1);
        }
        
        // Coastal cultures = longer, flowing names
        if matches!(geography, Geography::Coastal) {
            syllables += 1;
        }
        
        syllables.max(1).min(4)
    }
    
    /// Generate a simple given name using the language's phonology.
    pub fn generate_simple_name(&self, seed: u64) -> String {
        let concept = format!("name_{}", seed);
        let word_seed = hash_deterministic(&concept, self.genome.seed);
        let mut rng = SeededRng::new(word_seed);
        
        let mut name = String::new();
        
        for _ in 0..self.syllables_per_name {
            let syllable = self.generate_syllable(&mut rng);
            name.push_str(&syllable);
        }
        
        // Capitalize first letter
        if let Some(first) = name.chars().next() {
            name = first.to_uppercase().collect::<String>() + &name[first.len_utf8()..];
        }
        
        name
    }
    
    /// Generate a syllable for names (similar to word generation but tuned for names).
    fn generate_syllable(&self, rng: &mut SeededRng) -> String {
        let pattern = rng.choice(&self.genome.syllable_patterns);
        let pattern_str = pattern.pattern();
        
        let mut syllable = String::new();
        
        for ch in pattern_str.chars() {
            match ch {
                'C' => {
                    let consonant = self.choose_consonant(rng);
                    syllable.push_str(&consonant);
                }
                'V' => {
                    let vowel = rng.choice(&self.genome.phoneme_inventory.vowels);
                    syllable.push_str(&vowel.0);
                }
                _ => {}
            }
        }
        
        syllable
    }
    
    /// Choose a consonant for name generation.
    fn choose_consonant(&self, rng: &mut SeededRng) -> String {
        use crate::phonology::PhonemeCategory;
        
        let categories = self.genome.phoneme_inventory.available_categories();
        if categories.is_empty() {
            return String::new();
        }
        
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
                self.genome.phoneme_inventory.category_weights[idx]
            })
            .collect();
        
        let category_idx = rng.weighted_choice(&weights);
        let category = categories[category_idx];
        
        let consonants = self.genome.phoneme_inventory.get_category(category);
        let consonant = rng.choice(consonants);
        
        consonant.0.clone()
    }
    
    /// Generate a compound name from morphemes.
    pub fn generate_compound_name(&self, seed: u64, count: usize) -> String {
        let mut rng = SeededRng::new(seed ^ self.genome.seed);
        
        let morphemes: Vec<&str> = (0..count)
            .map(|_| {
                let m = self.morphemes.select_weighted(&mut rng, &self.geography);
                m.form.as_str()
            })
            .collect();
        
        if morphemes.is_empty() {
            return self.generate_simple_name(seed);
        }
        
        let mut name = morphemes[0].to_string();
        for morpheme in morphemes.iter().skip(1) {
            name = self.combining_rule.combine(&name, morpheme);
        }
        
        // Capitalize appropriately
        Self::capitalize_name(&name)
    }
    
    /// Capitalize a name appropriately.
    fn capitalize_name(name: &str) -> String {
        // For hyphenated names, capitalize each part
        if name.contains('-') {
            name.split('-')
                .map(|part| {
                    let mut chars = part.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join("-")
        } else if name.contains(" of ") {
            // For genitive forms, capitalize appropriately
            let parts: Vec<&str> = name.split(" of ").collect();
            if parts.len() == 2 {
                let first = Self::capitalize_first_letter(parts[0]);
                let second = Self::capitalize_first_letter(parts[1]);
                format!("{} of {}", first, second)
            } else {
                Self::capitalize_first_letter(name)
            }
        } else {
            Self::capitalize_first_letter(name)
        }
    }
    
    /// Capitalize the first letter of a string.
    fn capitalize_first_letter(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::morphology::MorphemeType;

    #[test]
    fn test_name_pattern_from_culture() {
        // High conscientiousness should give patronymic
        let culture = CulturalProfile::new(3.0, 3.0, 4.5, 3.0, 3.0, 3.0);
        assert_eq!(NamePattern::from_culture(&culture), NamePattern::Patronymic);
        
        // High openness should give compound
        let culture = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0);
        assert_eq!(NamePattern::from_culture(&culture), NamePattern::Compound);
        
        // Low honesty-humility should give elaborate
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 1.5, 3.0);
        assert_eq!(NamePattern::from_culture(&culture), NamePattern::Elaborate);
    }

    #[test]
    fn test_naming_system_creation() {
        let culture = CulturalProfile::new(4.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Coastal);
        
        assert!(!naming.morphemes.get(&MorphemeType::Fire).unwrap().form.is_empty());
    }

    #[test]
    fn test_simple_name_generation() {
        let culture = CulturalProfile::new(4.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Coastal);
        
        let name = naming.generate_simple_name(42);
        assert!(!name.is_empty());
        
        // Should be deterministic
        let name2 = naming.generate_simple_name(42);
        assert_eq!(name, name2);
        
        // Different seeds should give different names
        let name3 = naming.generate_simple_name(43);
        assert_ne!(name, name3);
    }

    #[test]
    fn test_compound_name_generation() {
        let culture = CulturalProfile::new(4.0, 4.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Mountains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Mountains);
        
        let name = naming.generate_compound_name(42, 2);
        assert!(!name.is_empty());
        
        // Should be deterministic
        let name2 = naming.generate_compound_name(42, 2);
        assert_eq!(name, name2);
    }

    #[test]
    fn test_name_length_by_geography() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        
        // Mountains should have shorter names
        let genome_mountain = LinguisticGenome::from_culture(culture, Geography::Mountains, 12345);
        let naming_mountain = NamingSystem::new(genome_mountain, culture, Geography::Mountains);
        
        // Coastal should have longer names
        let genome_coastal = LinguisticGenome::from_culture(culture, Geography::Coastal, 67890);
        let naming_coastal = NamingSystem::new(genome_coastal, culture, Geography::Coastal);
        
        assert!(naming_coastal.syllables_per_name >= naming_mountain.syllables_per_name);
    }
}


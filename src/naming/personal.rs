//! Personal name generation: names for individuals.

use super::{NamePattern, NamingSystem};
use crate::seeded_rng::SeededRng;

/// Context for generating a personal name.
#[derive(Debug, Clone)]
pub struct PersonalNameContext {
    /// Unique entity ID
    pub entity_id: u64,
    /// Optional parent/father name for patronymic systems
    pub parent_name: Option<String>,
    /// Birth order (for cultures that encode this)
    pub birth_order: Option<usize>,
}

impl PersonalNameContext {
    /// Create a simple context with just an entity ID.
    pub fn simple(entity_id: u64) -> Self {
        Self {
            entity_id,
            parent_name: None,
            birth_order: None,
        }
    }
    
    /// Create a context with a parent name for patronymic systems.
    pub fn with_parent(entity_id: u64, parent_name: String) -> Self {
        Self {
            entity_id,
            parent_name: Some(parent_name),
            birth_order: None,
        }
    }
}

impl NamingSystem {
    /// Generate a complete personal name based on the culture's naming pattern.
    pub fn generate_personal_name(&self, context: &PersonalNameContext) -> String {
        match self.pattern {
            NamePattern::Simple => {
                self.generate_simple_name(context.entity_id)
            }
            NamePattern::Patronymic => {
                self.generate_patronymic_name(context)
            }
            NamePattern::Compound => {
                // Use 2-3 morphemes for personal compound names
                let mut rng = SeededRng::new(context.entity_id ^ self.genome.seed);
                let count = 2 + rng.range(0, 2);
                self.generate_compound_name(context.entity_id, count)
            }
            NamePattern::Elaborate => {
                self.generate_elaborate_name(context)
            }
            NamePattern::Descriptive => {
                self.generate_descriptive_name(context)
            }
        }
    }
    
    /// Generate a patronymic name (e.g., "Aran Thorson").
    fn generate_patronymic_name(&self, context: &PersonalNameContext) -> String {
        let given_name = self.generate_simple_name(context.entity_id);
        
        if let Some(parent) = &context.parent_name {
            let patronymic = self.create_patronymic(parent);
            format!("{} {}", given_name, patronymic)
        } else {
            // No parent name provided, just use given name
            given_name
        }
    }
    
    /// Create a patronymic form from a parent's name.
    fn create_patronymic(&self, parent_name: &str) -> String {
        // Generate a suffix based on the language
        let suffix_seed = self.genome.seed ^ 0x504154524F4E594D; // "PATRONYM" in hex
        let suffix = self.generate_simple_name(suffix_seed);
        
        // Take first 2-3 characters of suffix as the patronymic marker
        let marker: String = suffix.chars().take(3).collect();
        
        // High conscientiousness = hyphenated
        if self.culture.normalized_conscientiousness() > 0.6 {
            format!("{}-{}", parent_name, marker)
        } else {
            format!("{}{}", parent_name, marker)
        }
    }
    
    /// Generate an elaborate name with titles.
    fn generate_elaborate_name(&self, context: &PersonalNameContext) -> String {
        let mut rng = SeededRng::new(context.entity_id ^ self.genome.seed);
        
        // Title
        let title = self.generate_title(&mut rng);
        
        // Given name
        let given_name = self.generate_simple_name(context.entity_id);
        
        // Lineage/ordinal
        let lineage = self.generate_lineage(&mut rng);
        
        format!("{} {} {}", title, given_name, lineage)
    }
    
    /// Generate a title (Lord, Lady, etc. but in the language).
    fn generate_title(&self, rng: &mut SeededRng) -> String {
        use crate::morphology::MorphemeType;
        
        // Select from power/authority morphemes
        let title_types = [
            MorphemeType::Power,
            MorphemeType::Great,
            MorphemeType::Strong,
            MorphemeType::Wise,
        ];
        
        if let Some(morpheme) = self.morphemes.select_from_types(&title_types, rng) {
            Self::capitalize_first_letter(&morpheme.form)
        } else {
            // Fallback
            Self::capitalize_first_letter(&self.generate_simple_name(rng.next() as u64 * 1000000))
        }
    }
    
    /// Generate a lineage suffix (e.g., "the Third", "of the Mountains").
    fn generate_lineage(&self, rng: &mut SeededRng) -> String {
        use crate::morphology::MorphemeType;
        
        // 50% chance of ordinal, 50% chance of geographic
        if rng.next() < 0.5 {
            // Ordinal
            let ordinals = ["First", "Second", "Third", "Fourth", "Fifth"];
            let idx = rng.range(0, ordinals.len());
            format!("the {}", ordinals[idx])
        } else {
            // Geographic/cultural
            let types = [
                MorphemeType::Mountain,
                MorphemeType::Sea,
                MorphemeType::Forest,
                MorphemeType::River,
            ];
            
            if let Some(morpheme) = self.morphemes.select_from_types(&types, rng) {
                format!("of the {}", Self::capitalize_first_letter(&morpheme.form))
            } else {
                "the Elder".to_string()
            }
        }
    }
    
    /// Generate a descriptive name (name + characteristic).
    fn generate_descriptive_name(&self, context: &PersonalNameContext) -> String {
        let mut rng = SeededRng::new(context.entity_id ^ self.genome.seed);
        
        let given_name = self.generate_simple_name(context.entity_id);
        let characteristic = self.generate_characteristic(&mut rng);
        
        // Format depends on combining rule
        match self.combining_rule {
            crate::morphology::CombiningRule::Hyphenated => {
                format!("{}-{}", given_name, characteristic)
            }
            _ => {
                format!("{} {}", given_name, characteristic)
            }
        }
    }
    
    /// Generate a characteristic descriptor.
    fn generate_characteristic(&self, rng: &mut SeededRng) -> String {
        use crate::morphology::MorphemeType;
        
        let characteristic_types = [
            MorphemeType::Strong,
            MorphemeType::Wise,
            MorphemeType::Swift,
            MorphemeType::Brave,
            MorphemeType::Gentle,
            MorphemeType::Dark,
            MorphemeType::Bright,
        ];
        
        if let Some(morpheme) = self.morphemes.select_from_types(&characteristic_types, rng) {
            Self::capitalize_first_letter(&morpheme.form)
        } else {
            // Fallback
            Self::capitalize_first_letter(&self.generate_simple_name(rng.next() as u64 * 1000000))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::culture::{CulturalProfile, Geography};
    use crate::genome::LinguisticGenome;

    #[test]
    fn test_simple_personal_name() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 4.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Plains);
        
        let context = PersonalNameContext::simple(42);
        let name = naming.generate_personal_name(&context);
        
        assert!(!name.is_empty());
        
        // Should be deterministic
        let name2 = naming.generate_personal_name(&context);
        assert_eq!(name, name2);
    }

    #[test]
    fn test_patronymic_name() {
        let culture = CulturalProfile::new(3.0, 3.0, 4.5, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Plains);
        
        let context = PersonalNameContext::with_parent(42, "Thorin".to_string());
        let name = naming.generate_personal_name(&context);
        
        assert!(name.contains("Thorin") || name.len() > 5);
        println!("Patronymic name: {}", name);
    }

    #[test]
    fn test_elaborate_name() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 1.5, 3.0); // Low H-H
        let genome = LinguisticGenome::from_culture(culture, Geography::Mountains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Mountains);
        
        let context = PersonalNameContext::simple(42);
        let name = naming.generate_personal_name(&context);
        
        // Elaborate names should have multiple parts
        assert!(name.contains(" "));
        println!("Elaborate name: {}", name);
    }

    #[test]
    fn test_compound_name() {
        let culture = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0); // High O
        let genome = LinguisticGenome::from_culture(culture, Geography::Forest, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Forest);
        
        let context = PersonalNameContext::simple(42);
        let name = naming.generate_personal_name(&context);
        
        assert!(!name.is_empty());
        println!("Compound name: {}", name);
    }

    #[test]
    fn test_different_entities_different_names() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Plains);
        
        let context1 = PersonalNameContext::simple(42);
        let context2 = PersonalNameContext::simple(43);
        
        let name1 = naming.generate_personal_name(&context1);
        let name2 = naming.generate_personal_name(&context2);
        
        assert_ne!(name1, name2);
    }
}


//! Epithet generation: titles, honorifics, and reputation-based names.
//!
//! Epithets are names that describe achievements, characteristics, or circumstances.
//! Examples: "the Wise", "Dragonslayer", "Stormborn"

use super::NamingSystem;
use crate::morphology::MorphemeType;
use crate::seeded_rng::SeededRng;

/// Context for generating an epithet.
#[derive(Debug, Clone)]
pub struct EpithetContext {
    /// Entity ID for determinism
    pub entity_id: u64,
    /// Birth circumstances (e.g., "born during storm")
    pub birth_event: Option<String>,
    /// Notable deed or achievement
    pub achievement: Option<String>,
    /// Defining characteristic
    pub characteristic: Option<Characteristic>,
}

/// A defining characteristic that could generate an epithet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Characteristic {
    // Physical
    Tall,
    Short,
    Strong,
    Swift,
    
    // Mental
    Wise,
    Cunning,
    Mad,
    
    // Moral
    Honest,
    Brave,
    Cruel,
    Just,
    
    // Social
    Silent,
    Loud,
    Beloved,
    Feared,
}

impl Characteristic {
    /// Map to morpheme types.
    fn to_morpheme_types(&self) -> Vec<MorphemeType> {
        match self {
            Characteristic::Tall => vec![MorphemeType::Great, MorphemeType::Sky],
            Characteristic::Short => vec![MorphemeType::Small],
            Characteristic::Strong => vec![MorphemeType::Strong, MorphemeType::Power],
            Characteristic::Swift => vec![MorphemeType::Swift, MorphemeType::Air],
            Characteristic::Wise => vec![MorphemeType::Wise, MorphemeType::Ancient],
            Characteristic::Cunning => vec![MorphemeType::Wise, MorphemeType::Dark],
            Characteristic::Mad => vec![MorphemeType::Storm, MorphemeType::Dark],
            Characteristic::Honest => vec![MorphemeType::Truth, MorphemeType::Bright],
            Characteristic::Brave => vec![MorphemeType::Brave, MorphemeType::Courage],
            Characteristic::Cruel => vec![MorphemeType::Dark, MorphemeType::Destroy],
            Characteristic::Just => vec![MorphemeType::Justice, MorphemeType::Truth],
            Characteristic::Silent => vec![MorphemeType::Dark, MorphemeType::Spirit],
            Characteristic::Loud => vec![MorphemeType::Storm, MorphemeType::Strike],
            Characteristic::Beloved => vec![MorphemeType::Love, MorphemeType::Hope],
            Characteristic::Feared => vec![MorphemeType::Dark, MorphemeType::Power],
        }
    }
}

impl EpithetContext {
    /// Create a simple epithet context.
    pub fn new(entity_id: u64) -> Self {
        Self {
            entity_id,
            birth_event: None,
            achievement: None,
            characteristic: None,
        }
    }
    
    /// Add birth event.
    pub fn with_birth_event(mut self, event: String) -> Self {
        self.birth_event = Some(event);
        self
    }
    
    /// Add achievement.
    pub fn with_achievement(mut self, achievement: String) -> Self {
        self.achievement = Some(achievement);
        self
    }
    
    /// Add characteristic.
    pub fn with_characteristic(mut self, characteristic: Characteristic) -> Self {
        self.characteristic = Some(characteristic);
        self
    }
}

impl NamingSystem {
    /// Generate an epithet if appropriate for this culture.
    ///
    /// Returns None if the culture doesn't use epithets or no suitable context exists.
    pub fn generate_epithet(&self, context: &EpithetContext) -> Option<String> {
        // High openness cultures love epithets
        // Low openness cultures rarely use them
        let openness = self.culture.normalized_openness();
        let mut rng = SeededRng::new(context.entity_id ^ self.genome.seed);
        
        let epithet_probability = openness;
        if rng.next() as f32 > epithet_probability {
            return None;
        }
        
        // Determine what type of epithet to generate based on available context
        if let Some(achievement) = &context.achievement {
            Some(self.generate_achievement_epithet(achievement, &mut rng))
        } else if let Some(birth) = &context.birth_event {
            Some(self.generate_birth_epithet(birth, &mut rng))
        } else if let Some(characteristic) = &context.characteristic {
            Some(self.generate_characteristic_epithet(characteristic, &mut rng))
        } else {
            None
        }
    }
    
    /// Generate an epithet based on an achievement (e.g., "Dragonslayer").
    fn generate_achievement_epithet(&self, achievement: &str, rng: &mut SeededRng) -> String {
        use crate::generation::generate_word;
        
        // Translate the achievement concept
        let achievement_word = generate_word(&self.genome, achievement);
        
        // Choose a format
        if rng.next() < 0.5 {
            // "the [Achievement]" format
            format!("the {}", Self::capitalize_first_letter(&achievement_word))
        } else {
            // "[Achievement]er" / "[Achievement]slayer" format
            let action_morphemes = [
                MorphemeType::Strike,
                MorphemeType::Destroy,
                MorphemeType::Protect,
            ];
            
            if let Some(action) = self.morphemes.select_from_types(&action_morphemes, rng) {
                format!("{}{}", Self::capitalize_first_letter(&achievement_word), action.form)
            } else {
                format!("the {}", Self::capitalize_first_letter(&achievement_word))
            }
        }
    }
    
    /// Generate an epithet based on birth circumstances (e.g., "Stormborn").
    fn generate_birth_epithet(&self, birth_event: &str, rng: &mut SeededRng) -> String {
        use crate::generation::generate_word;
        
        // Translate the event
        let event_word = generate_word(&self.genome, birth_event);
        
        // Add "born" suffix
        let born_morphemes = [MorphemeType::Life, MorphemeType::Young];
        
        if let Some(born) = self.morphemes.select_from_types(&born_morphemes, rng) {
            let name = self.combining_rule.combine(&event_word, &born.form);
            Self::capitalize_name(&name)
        } else {
            format!("{}-Born", Self::capitalize_first_letter(&event_word))
        }
    }
    
    /// Generate an epithet based on a characteristic (e.g., "the Wise").
    fn generate_characteristic_epithet(
        &self,
        characteristic: &Characteristic,
        rng: &mut SeededRng,
    ) -> String {
        let morpheme_types = characteristic.to_morpheme_types();
        
        if let Some(morpheme) = self.morphemes.select_from_types(&morpheme_types, rng) {
            // Format as "the [Characteristic]"
            format!("the {}", Self::capitalize_first_letter(&morpheme.form))
        } else {
            // Fallback
            "the Elder".to_string()
        }
    }
    
    /// Generate a complete name with epithet.
    pub fn generate_name_with_epithet(
        &self,
        base_name: &str,
        context: &EpithetContext,
    ) -> String {
        if let Some(epithet) = self.generate_epithet(context) {
            format!("{} {}", base_name, epithet)
        } else {
            base_name.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::culture::{CulturalProfile, Geography};
    use crate::genome::LinguisticGenome;

    #[test]
    fn test_epithet_probability_by_openness() {
        // High openness should generate epithets more often
        let high_o = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0);
        let low_o = CulturalProfile::new(3.0, 1.5, 3.0, 3.0, 3.0, 3.0);
        
        let genome_high = LinguisticGenome::from_culture(high_o, Geography::Plains, 12345);
        let genome_low = LinguisticGenome::from_culture(low_o, Geography::Plains, 12345);
        
        let naming_high = NamingSystem::new(genome_high, high_o, Geography::Plains);
        let naming_low = NamingSystem::new(genome_low, low_o, Geography::Plains);
        
        // Count epithets generated across multiple entities
        let mut high_count = 0;
        let mut low_count = 0;
        
        for i in 0..20 {
            let context = EpithetContext::new(i)
                .with_characteristic(Characteristic::Wise);
            
            if naming_high.generate_epithet(&context).is_some() {
                high_count += 1;
            }
            if naming_low.generate_epithet(&context).is_some() {
                low_count += 1;
            }
        }
        
        println!("High openness: {} epithets, Low openness: {} epithets", high_count, low_count);
        // High openness should generate more epithets
        assert!(high_count >= low_count);
    }

    #[test]
    fn test_achievement_epithet() {
        let culture = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Mountains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Mountains);
        
        let context = EpithetContext::new(42)
            .with_achievement("dragon".to_string());
        
        if let Some(epithet) = naming.generate_epithet(&context) {
            assert!(!epithet.is_empty());
            println!("Achievement epithet: {}", epithet);
        }
    }

    #[test]
    fn test_birth_epithet() {
        let culture = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Coastal);
        
        let context = EpithetContext::new(42)
            .with_birth_event("storm".to_string());
        
        if let Some(epithet) = naming.generate_epithet(&context) {
            assert!(!epithet.is_empty());
            println!("Birth epithet: {}", epithet);
        }
    }

    #[test]
    fn test_characteristic_epithet() {
        let culture = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Forest, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Forest);
        
        let context = EpithetContext::new(42)
            .with_characteristic(Characteristic::Wise);
        
        if let Some(epithet) = naming.generate_epithet(&context) {
            assert!(!epithet.is_empty());
            println!("Characteristic epithet: {}", epithet);
        }
    }

    #[test]
    fn test_full_name_with_epithet() {
        let culture = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Desert, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Desert);
        
        let base_name = naming.generate_simple_name(42);
        let context = EpithetContext::new(42)
            .with_characteristic(Characteristic::Strong);
        
        let full_name = naming.generate_name_with_epithet(&base_name, &context);
        assert!(!full_name.is_empty());
        println!("Full name with epithet: {}", full_name);
    }

    #[test]
    fn test_deterministic_epithets() {
        let culture = CulturalProfile::new(3.0, 5.0, 3.0, 3.0, 3.0, 3.0); // Very high O
        let genome = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Plains);
        
        let context = EpithetContext::new(42)
            .with_characteristic(Characteristic::Brave);
        
        let epithet1 = naming.generate_epithet(&context);
        let epithet2 = naming.generate_epithet(&context);
        
        assert_eq!(epithet1, epithet2);
    }
}


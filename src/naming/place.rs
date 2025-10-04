//! Place name generation: names for locations, settlements, and landmarks.

use super::NamingSystem;
use crate::culture::Geography;
use crate::morphology::MorphemeType;
use crate::seeded_rng::SeededRng;

/// The type of place being named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceType {
    /// Settlement (village, town, city)
    Settlement,
    /// Natural feature (mountain, river, forest)
    Natural,
    /// Constructed landmark (bridge, tower, fortress)
    Landmark,
    /// Region (valley, plains, territory)
    Region,
}

/// Context for generating a place name.
#[derive(Debug, Clone)]
pub struct PlaceNameContext {
    /// Unique place ID
    pub place_id: u64,
    /// Type of place
    pub place_type: PlaceType,
    /// Local geography (can differ from culture's primary geography)
    pub local_geography: Option<Geography>,
    /// Optional founder's name
    pub founder_name: Option<String>,
    /// Optional historical event
    pub historical_event: Option<String>,
}

impl PlaceNameContext {
    /// Create a simple place context.
    pub fn new(place_id: u64, place_type: PlaceType) -> Self {
        Self {
            place_id,
            place_type,
            local_geography: None,
            founder_name: None,
            historical_event: None,
        }
    }
    
    /// Add local geography information.
    pub fn with_geography(mut self, geography: Geography) -> Self {
        self.local_geography = Some(geography);
        self
    }
    
    /// Add founder information.
    pub fn with_founder(mut self, founder_name: String) -> Self {
        self.founder_name = Some(founder_name);
        self
    }
    
    /// Add historical event.
    pub fn with_event(mut self, event: String) -> Self {
        self.historical_event = Some(event);
        self
    }
}

/// Strategy for naming places.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlaceNamingStrategy {
    /// Descriptive of geographic features (Redmountain, Deepwater)
    Descriptive,
    /// Named after founder (Jamestown, Alexandria)
    Founder,
    /// Named after historical event (Battleford, Victory Bay)
    Historical,
    /// Mythological/poetic (Dragonspire, Moonhaven)
    Mythopoetic,
}

impl NamingSystem {
    /// Generate a complete place name based on cultural patterns and context.
    pub fn generate_place_name(&self, context: &PlaceNameContext) -> String {
        let mut rng = SeededRng::new(context.place_id ^ self.genome.seed);
        
        // Determine naming strategy based on culture and available context
        let strategy = self.determine_place_strategy(context, &mut rng);
        
        match strategy {
            PlaceNamingStrategy::Descriptive => {
                self.generate_descriptive_place_name(context, &mut rng)
            }
            PlaceNamingStrategy::Founder => {
                if let Some(founder) = &context.founder_name {
                    self.generate_founder_place_name(founder, context, &mut rng)
                } else {
                    // Fallback to descriptive
                    self.generate_descriptive_place_name(context, &mut rng)
                }
            }
            PlaceNamingStrategy::Historical => {
                if let Some(event) = &context.historical_event {
                    self.generate_historical_place_name(event, context, &mut rng)
                } else {
                    // Fallback to descriptive
                    self.generate_descriptive_place_name(context, &mut rng)
                }
            }
            PlaceNamingStrategy::Mythopoetic => {
                self.generate_mythopoetic_place_name(context, &mut rng)
            }
        }
    }
    
    /// Determine which naming strategy to use.
    fn determine_place_strategy(
        &self,
        context: &PlaceNameContext,
        rng: &mut SeededRng,
    ) -> PlaceNamingStrategy {
        // High openness cultures prefer mythopoetic names
        if self.culture.normalized_openness() > 0.7 {
            return PlaceNamingStrategy::Mythopoetic;
        }
        
        // High conscientiousness cultures prefer systematic descriptive names
        if self.culture.normalized_conscientiousness() > 0.7 {
            return PlaceNamingStrategy::Descriptive;
        }
        
        // If founder is available, sometimes use it
        if context.founder_name.is_some() && rng.next() < 0.4 {
            return PlaceNamingStrategy::Founder;
        }
        
        // If historical event is available, sometimes use it
        if context.historical_event.is_some() && rng.next() < 0.3 {
            return PlaceNamingStrategy::Historical;
        }
        
        // Default to descriptive
        PlaceNamingStrategy::Descriptive
    }
    
    /// Generate a descriptive place name based on geographic features.
    fn generate_descriptive_place_name(
        &self,
        context: &PlaceNameContext,
        rng: &mut SeededRng,
    ) -> String {
        // Select morphemes appropriate to the place type and geography
        let geography = context.local_geography.as_ref().unwrap_or(&self.geography);
        
        let feature_morpheme = self.select_geographic_morpheme(context.place_type, geography, rng);
        let quality_morpheme = self.select_quality_morpheme(rng);
        
        // Combine quality + feature (e.g., "Deep" + "Water" = "Deepwater")
        let name = self.combining_rule.combine(&quality_morpheme, &feature_morpheme);
        Self::capitalize_name(&name)
    }
    
    /// Select a morpheme appropriate to the geographic feature.
    fn select_geographic_morpheme(
        &self,
        place_type: PlaceType,
        geography: &Geography,
        rng: &mut SeededRng,
    ) -> String {
        let morpheme_types = match place_type {
            PlaceType::Settlement => {
                // Settlements often named after nearby features
                vec![
                    MorphemeType::River,
                    MorphemeType::Forest,
                    MorphemeType::Mountain,
                    MorphemeType::Stone,
                ]
            }
            PlaceType::Natural => {
                match geography {
                    Geography::Mountains => vec![
                        MorphemeType::Mountain,
                        MorphemeType::Stone,
                        MorphemeType::Sky,
                        MorphemeType::Cold,
                    ],
                    Geography::Coastal => vec![
                        MorphemeType::Sea,
                        MorphemeType::Water,
                        MorphemeType::Storm,
                    ],
                    Geography::Desert => vec![
                        MorphemeType::Sun,
                        MorphemeType::Stone,
                        MorphemeType::Fire,
                    ],
                    Geography::Forest => vec![
                        MorphemeType::Forest,
                        MorphemeType::Earth,
                        MorphemeType::Life,
                    ],
                    Geography::Plains | Geography::RiverValley => vec![
                        MorphemeType::River,
                        MorphemeType::Sky,
                        MorphemeType::Earth,
                    ],
                }
            }
            PlaceType::Landmark => {
                vec![
                    MorphemeType::Stone,
                    MorphemeType::Power,
                    MorphemeType::Protect,
                ]
            }
            PlaceType::Region => {
                vec![
                    MorphemeType::Earth,
                    MorphemeType::Sky,
                    MorphemeType::Great,
                ]
            }
        };
        
        if let Some(morpheme) = self.morphemes.select_from_types(&morpheme_types, rng) {
            morpheme.form.clone()
        } else {
            // Fallback
            self.generate_simple_name(rng.next() as u64 * 1000000)
        }
    }
    
    /// Select a quality/descriptor morpheme.
    fn select_quality_morpheme(&self, rng: &mut SeededRng) -> String {
        let quality_types = vec![
            MorphemeType::Great,
            MorphemeType::Ancient,
            MorphemeType::Dark,
            MorphemeType::Bright,
            MorphemeType::Cold,
            MorphemeType::Warm,
            MorphemeType::Strong,
        ];
        
        if let Some(morpheme) = self.morphemes.select_from_types(&quality_types, rng) {
            morpheme.form.clone()
        } else {
            // Fallback
            self.generate_simple_name(rng.next() as u64 * 1000000)
        }
    }
    
    /// Generate a place name based on a founder.
    fn generate_founder_place_name(
        &self,
        founder: &str,
        _context: &PlaceNameContext,
        rng: &mut SeededRng,
    ) -> String {
        // Different formats: "Foundersville", "Founder's Landing", "New Founder"
        let format_choice = rng.range(0, 3);
        
        match format_choice {
            0 => {
                // Add a suffix based on place type
                let suffix = match _context.place_type {
                    PlaceType::Settlement => self.translate_or_generate("town", rng),
                    PlaceType::Landmark => self.translate_or_generate("hold", rng),
                    _ => self.translate_or_generate("land", rng),
                };
                format!("{}{}", founder, suffix)
            }
            1 => {
                // Possessive form
                let feature = match _context.place_type {
                    PlaceType::Settlement => "Rest",
                    PlaceType::Landmark => "Tower",
                    PlaceType::Natural => "Vale",
                    PlaceType::Region => "Realm",
                };
                format!("{}'s {}", founder, feature)
            }
            _ => {
                // "New Founder" format
                format!("New {}", founder)
            }
        }
    }
    
    /// Generate a place name based on a historical event.
    fn generate_historical_place_name(
        &self,
        event: &str,
        _context: &PlaceNameContext,
        rng: &mut SeededRng,
    ) -> String {
        // Translate the event concept into the language
        let event_word = self.translate_or_generate(event, rng);
        
        // Add a geographic suffix
        let suffix = self.select_geographic_morpheme(_context.place_type, &self.geography, rng);
        
        let name = self.combining_rule.combine(&event_word, &suffix);
        Self::capitalize_name(&name)
    }
    
    /// Generate a mythopoetic/imaginative place name.
    fn generate_mythopoetic_place_name(
        &self,
        _context: &PlaceNameContext,
        rng: &mut SeededRng,
    ) -> String {
        // Combine abstract/powerful morphemes
        let mythic_types = vec![
            MorphemeType::Spirit,
            MorphemeType::Fate,
            MorphemeType::Star,
            MorphemeType::Moon,
            MorphemeType::Storm,
            MorphemeType::Power,
        ];
        
        let feature_types = vec![
            MorphemeType::Mountain,
            MorphemeType::Sky,
            MorphemeType::Sea,
            MorphemeType::Forest,
        ];
        
        let mythic = self.morphemes.select_from_types(&mythic_types, rng)
            .map(|m| m.form.as_str())
            .unwrap_or("mystic");
        
        let feature = self.morphemes.select_from_types(&feature_types, rng)
            .map(|m| m.form.as_str())
            .unwrap_or("place");
        
        let name = self.combining_rule.combine(mythic, feature);
        Self::capitalize_name(&name)
    }
    
    /// Translate a concept or generate a word for it.
    #[allow(unused_variables)]
    fn translate_or_generate(&self, concept: &str, rng: &mut SeededRng) -> String {
        // In a full implementation, this would use the language's lexicon
        // For now, generate based on concept
        use crate::generation::generate_word;
        generate_word(&self.genome, concept)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::culture::{CulturalProfile, Geography};
    use crate::genome::LinguisticGenome;

    #[test]
    fn test_descriptive_place_name() {
        let culture = CulturalProfile::new(3.0, 3.0, 4.5, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Mountains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Mountains);
        
        let context = PlaceNameContext::new(42, PlaceType::Natural)
            .with_geography(Geography::Mountains);
        
        let name = naming.generate_place_name(&context);
        assert!(!name.is_empty());
        println!("Descriptive mountain place: {}", name);
    }

    #[test]
    fn test_founder_place_name() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Plains, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Plains);
        
        let context = PlaceNameContext::new(42, PlaceType::Settlement)
            .with_founder("Thorin".to_string());
        
        let name = naming.generate_place_name(&context);
        assert!(!name.is_empty());
        println!("Founder-based settlement: {}", name);
    }

    #[test]
    fn test_mythopoetic_place_name() {
        let culture = CulturalProfile::new(3.0, 4.5, 3.0, 3.0, 3.0, 3.0); // High O
        let genome = LinguisticGenome::from_culture(culture, Geography::Forest, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Forest);
        
        let context = PlaceNameContext::new(42, PlaceType::Landmark);
        
        let name = naming.generate_place_name(&context);
        assert!(!name.is_empty());
        println!("Mythopoetic landmark: {}", name);
    }

    #[test]
    fn test_deterministic_place_names() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Coastal, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Coastal);
        
        let context = PlaceNameContext::new(42, PlaceType::Settlement);
        
        let name1 = naming.generate_place_name(&context);
        let name2 = naming.generate_place_name(&context);
        
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_different_place_types() {
        let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
        let genome = LinguisticGenome::from_culture(culture, Geography::Desert, 12345);
        let naming = NamingSystem::new(genome, culture, Geography::Desert);
        
        let settlement = PlaceNameContext::new(42, PlaceType::Settlement);
        let natural = PlaceNameContext::new(42, PlaceType::Natural);
        
        let name1 = naming.generate_place_name(&settlement);
        let name2 = naming.generate_place_name(&natural);
        
        // Same ID but different types should still produce names
        assert!(!name1.is_empty());
        assert!(!name2.is_empty());
        println!("Settlement: {}, Natural: {}", name1, name2);
    }
}


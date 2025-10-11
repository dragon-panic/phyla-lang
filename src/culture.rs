//! Cultural parameters that influence language generation.

/// HEXACO personality model scores (1-5 scale).
/// These traits map to linguistic features.
#[derive(Debug, Clone, Copy)]
pub struct CulturalProfile {
    /// Agreeableness: cooperation, empathy, patience.
    /// High → softer sounds (nasals, liquids), Low → harsher sounds (stops, clusters)
    pub agreeableness: f32,

    /// Openness to experience: creativity, curiosity, unconventionality.
    /// High → larger phoneme inventory, complex patterns, Low → simpler patterns
    pub openness: f32,

    /// Conscientiousness: organization, diligence, perfectionism.
    /// High → regular patterns, consistent rules, Low → irregular patterns
    pub conscientiousness: f32,

    /// Extraversion: social engagement, assertiveness, energy.
    /// High → louder consonants, Low → softer consonants
    pub extraversion: f32,

    /// Honesty-Humility: sincerity, fairness, modesty.
    /// Affects formality and politeness markers
    pub honesty_humility: f32,

    /// Emotionality: anxiety, sentimentality, fearfulness.
    /// High → more vowels, flowing sounds, Low → more consonants
    pub emotionality: f32,
}

impl CulturalProfile {
    /// Create a new cultural profile with all traits.
    pub fn new(
        agreeableness: f32,
        openness: f32,
        conscientiousness: f32,
        extraversion: f32,
        honesty_humility: f32,
        emotionality: f32,
    ) -> Self {
        Self {
            agreeableness,
            openness,
            conscientiousness,
            extraversion,
            honesty_humility,
            emotionality,
        }
    }

    /// Normalize a score to 0-1 range from 1-5 range.
    fn normalize(score: f32) -> f32 {
        (score - 1.0) / 4.0
    }

    /// Get normalized agreeableness (0-1).
    pub fn normalized_agreeableness(&self) -> f32 {
        Self::normalize(self.agreeableness.clamp(1.0, 5.0))
    }

    /// Get normalized openness (0-1).
    pub fn normalized_openness(&self) -> f32 {
        Self::normalize(self.openness.clamp(1.0, 5.0))
    }

    /// Get normalized conscientiousness (0-1).
    pub fn normalized_conscientiousness(&self) -> f32 {
        Self::normalize(self.conscientiousness.clamp(1.0, 5.0))
    }

    /// Get normalized extraversion (0-1).
    pub fn normalized_extraversion(&self) -> f32 {
        Self::normalize(self.extraversion.clamp(1.0, 5.0))
    }

    /// Get normalized emotionality (0-1).
    pub fn normalized_emotionality(&self) -> f32 {
        Self::normalize(self.emotionality.clamp(1.0, 5.0))
    }
}

/// Geographic environment that influences phonology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Geography {
    /// Mountains: glottal stops, ejectives, shorter words, abrupt sounds
    Mountains,
    /// Coast/Maritime: liquid consonants, flowing sounds, longer words
    Coastal,
    /// Deserts: emphatic consonants, guttural sounds, pharyngeal consonants
    Desert,
    /// Forests: softer sounds, breathy voice, nasal harmony
    Forest,
    /// Plains: balanced phoneme distribution, neutral features
    Plains,
    /// River valleys: balanced with slight bias toward liquids
    RiverValley,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cultural_profile_normalization() {
        let profile = CulturalProfile::new(1.0, 3.0, 5.0, 2.5, 4.0, 3.5);

        assert!((profile.normalized_agreeableness() - 0.0).abs() < 0.01);
        assert!((profile.normalized_openness() - 0.5).abs() < 0.01);
        assert!((profile.normalized_conscientiousness() - 1.0).abs() < 0.01);
    }
}






//! Phonological components: phonemes, syllable structures, and constraints.


/// A consonant sound.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Consonant(pub String);

impl Consonant {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// A vowel sound.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vowel(pub String);

impl Vowel {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Categories of consonants based on manner and place of articulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhonemeCategory {
    Stops,
    Fricatives,
    Nasals,
    Liquids,
    Glides,
}

/// The complete inventory of sounds available in a language.
#[derive(Debug, Clone)]
pub struct PhonemeInventory {
    pub stops: Vec<Consonant>,
    pub fricatives: Vec<Consonant>,
    pub nasals: Vec<Consonant>,
    pub liquids: Vec<Consonant>,
    pub glides: Vec<Consonant>,
    pub vowels: Vec<Vowel>,

    /// Weighted probabilities for each consonant category.
    /// Higher weights mean the category is used more frequently.
    pub category_weights: Vec<f32>,
}

impl PhonemeInventory {
    /// Get all consonants as a flat list.
    pub fn all_consonants(&self) -> Vec<&Consonant> {
        let mut all = Vec::new();
        all.extend(&self.stops);
        all.extend(&self.fricatives);
        all.extend(&self.nasals);
        all.extend(&self.liquids);
        all.extend(&self.glides);
        all
    }

    /// Get consonants by category.
    pub fn get_category(&self, category: PhonemeCategory) -> &[Consonant] {
        match category {
            PhonemeCategory::Stops => &self.stops,
            PhonemeCategory::Fricatives => &self.fricatives,
            PhonemeCategory::Nasals => &self.nasals,
            PhonemeCategory::Liquids => &self.liquids,
            PhonemeCategory::Glides => &self.glides,
        }
    }

    /// Get the categories that have consonants.
    pub fn available_categories(&self) -> Vec<PhonemeCategory> {
        let mut categories = Vec::new();
        if !self.stops.is_empty() {
            categories.push(PhonemeCategory::Stops);
        }
        if !self.fricatives.is_empty() {
            categories.push(PhonemeCategory::Fricatives);
        }
        if !self.nasals.is_empty() {
            categories.push(PhonemeCategory::Nasals);
        }
        if !self.liquids.is_empty() {
            categories.push(PhonemeCategory::Liquids);
        }
        if !self.glides.is_empty() {
            categories.push(PhonemeCategory::Glides);
        }
        categories
    }
}

/// Syllable structure patterns (e.g., CV, CVC, CCVC).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyllableStructure {
    /// Vowel only: "a", "i"
    V,
    /// Consonant-Vowel: "ma", "to"
    CV,
    /// Vowel-Consonant: "am", "it"
    VC,
    /// Consonant-Vowel-Consonant: "mat", "tok"
    CVC,
    /// Consonant-Consonant-Vowel: "pra", "kli"
    CCV,
    /// Vowel-Consonant-Consonant: "amp", "ost"
    VCC,
    /// Consonant-Consonant-Vowel-Consonant: "prak", "klin"
    CCVC,
    /// Consonant-Vowel-Consonant-Consonant: "mask", "tors"
    CVCC,
    /// Double vowel for vowel-heavy languages
    CVV,
}

impl SyllableStructure {
    /// Get the pattern as a string (e.g., "CVC").
    pub fn pattern(&self) -> &'static str {
        match self {
            Self::V => "V",
            Self::CV => "CV",
            Self::VC => "VC",
            Self::CVC => "CVC",
            Self::CCV => "CCV",
            Self::VCC => "VCC",
            Self::CCVC => "CCVC",
            Self::CVCC => "CVCC",
            Self::CVV => "CVV",
        }
    }
}

/// Prosodic system (stress, tone, intonation).
#[derive(Debug, Clone)]
pub struct ProsodicSystem {
    /// Stress pattern: None, Initial, Final, Penultimate, etc.
    pub stress_pattern: StressPattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StressPattern {
    /// No distinctive stress
    None,
    /// Stress on first syllable
    Initial,
    /// Stress on last syllable
    Final,
    /// Stress on second-to-last syllable
    Penultimate,
}

impl Default for ProsodicSystem {
    fn default() -> Self {
        Self {
            stress_pattern: StressPattern::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phoneme_inventory() {
        let inventory = PhonemeInventory {
            stops: vec![Consonant::new("p"), Consonant::new("t"), Consonant::new("k")],
            fricatives: vec![Consonant::new("s"), Consonant::new("h")],
            nasals: vec![Consonant::new("m"), Consonant::new("n")],
            liquids: vec![Consonant::new("l"), Consonant::new("r")],
            glides: vec![],
            vowels: vec![Vowel::new("a"), Vowel::new("i"), Vowel::new("u")],
            category_weights: vec![0.25, 0.25, 0.25, 0.25, 0.0],
        };

        assert_eq!(inventory.stops.len(), 3);
        assert_eq!(inventory.vowels.len(), 3);
        assert_eq!(inventory.all_consonants().len(), 9);
    }

    #[test]
    fn test_syllable_structure() {
        assert_eq!(SyllableStructure::CVC.pattern(), "CVC");
        assert_eq!(SyllableStructure::CV.pattern(), "CV");
    }
}


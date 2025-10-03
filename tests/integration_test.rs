//! Integration tests for the phyla-lang library.

use phyla_lang::{CulturalProfile, Geography, Language};

#[test]
fn test_basic_translation() {
    let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
    let language = Language::from_culture(culture, Geography::Coastal, 12345);

    let word = language.translate_word("house");
    assert!(!word.is_empty());
}

#[test]
fn test_determinism_across_instances() {
    let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);

    let lang1 = Language::from_culture(culture, Geography::Coastal, 12345);
    let lang2 = Language::from_culture(culture, Geography::Coastal, 12345);

    let word1 = lang1.translate_word("water");
    let word2 = lang2.translate_word("water");

    assert_eq!(word1, word2);
}

#[test]
fn test_phrase_translation_consistency() {
    let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
    let language = Language::from_culture(culture, Geography::Coastal, 12345);

    let phrase1 = language.translate_phrase("I bring the beer quickly");
    let phrase2 = language.translate_phrase("I bring the beer quickly");

    assert_eq!(phrase1, phrase2);
}

#[test]
fn test_multiple_cultures() {
    let coastal = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
    let mountain = CulturalProfile::new(1.0, 2.0, 4.0, 2.0, 3.0, 2.0);
    let desert = CulturalProfile::new(2.0, 2.0, 3.0, 3.0, 3.0, 2.0);

    let lang_coastal = Language::from_culture(coastal, Geography::Coastal, 111);
    let lang_mountain = Language::from_culture(mountain, Geography::Mountains, 222);
    let lang_desert = Language::from_culture(desert, Geography::Desert, 333);

    let word_coastal = lang_coastal.translate_word("sun");
    let word_mountain = lang_mountain.translate_word("sun");
    let word_desert = lang_desert.translate_word("sun");

    // All three should be different
    assert_ne!(word_coastal, word_mountain);
    assert_ne!(word_coastal, word_desert);
    assert_ne!(word_mountain, word_desert);
}

#[test]
fn test_word_orders() {
    // Create languages with different personalities that should yield different word orders
    let svo_culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
    let sov_culture = CulturalProfile::new(3.0, 3.0, 5.0, 3.0, 3.0, 3.0); // High conscientiousness
    let vso_culture = CulturalProfile::new(3.0, 3.0, 1.0, 3.0, 3.0, 3.0); // Low conscientiousness

    let lang_svo = Language::from_culture(svo_culture, Geography::Plains, 1000);
    let lang_sov = Language::from_culture(sov_culture, Geography::Plains, 2000);
    let lang_vso = Language::from_culture(vso_culture, Geography::Plains, 3000);

    // Just verify they were created successfully
    assert!(!lang_svo.translate_word("test").is_empty());
    assert!(!lang_sov.translate_word("test").is_empty());
    assert!(!lang_vso.translate_word("test").is_empty());
}

#[test]
fn test_geographic_variation() {
    let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);

    let geographies = [
        Geography::Mountains,
        Geography::Coastal,
        Geography::Desert,
        Geography::Forest,
        Geography::Plains,
        Geography::RiverValley,
    ];

    let languages: Vec<_> = geographies
        .iter()
        .enumerate()
        .map(|(i, &geo)| Language::from_culture(culture, geo, 5000 + i as u64))
        .collect();

    // Translate the same word in all languages
    let translations: Vec<_> = languages
        .iter()
        .map(|lang| lang.translate_word("mountain"))
        .collect();

    // Verify all translations are different (they should be due to different seeds and geographies)
    for (i, trans1) in translations.iter().enumerate() {
        for (j, trans2) in translations.iter().enumerate() {
            if i != j {
                assert_ne!(trans1, trans2);
            }
        }
    }
}

#[test]
fn test_empty_phrase() {
    let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
    let language = Language::from_culture(culture, Geography::Plains, 9999);

    let phrase = language.translate_phrase("");
    assert_eq!(phrase, "");
}

#[test]
fn test_single_word_phrase() {
    let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
    let language = Language::from_culture(culture, Geography::Plains, 9999);

    let phrase = language.translate_phrase("hello");
    assert!(!phrase.is_empty());
}

#[test]
fn test_cache_functionality() {
    let culture = CulturalProfile::new(3.0, 3.0, 3.0, 3.0, 3.0, 3.0);
    let language = Language::from_culture(culture, Geography::Plains, 7777);

    assert_eq!(language.cache_size(), 0);

    let _ = language.translate_word("first");
    assert_eq!(language.cache_size(), 1);

    let _ = language.translate_word("second");
    assert_eq!(language.cache_size(), 2);

    // Translating the same word shouldn't increase cache size
    let _ = language.translate_word("first");
    assert_eq!(language.cache_size(), 2);

    language.clear_cache();
    assert_eq!(language.cache_size(), 0);
}


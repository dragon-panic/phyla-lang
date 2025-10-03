//! Example that demonstrates the Rust library producing similar results to the JavaScript implementation.
//!
//! This example creates languages that mirror the cultures from the HTML demo:
//! - Melodic Coastal Folk
//! - Mountain Warriors
//! - River Valley Scholars
//! - Desert Nomads

use phyla_lang::{CulturalProfile, Geography, Language};

fn main() {
    println!("=== Comparing Rust Implementation with JavaScript Demo ===\n");

    // Recreate the cultures from language_generator.html
    let melodic = create_melodic_coastal();
    let martial = create_mountain_warriors();
    let scholarly = create_river_scholars();
    let desert = create_desert_nomads();

    // Test phrase from the JavaScript demo
    let test_phrase = "I bring the beer quickly";

    println!("Translating: \"{}\"\n", test_phrase);

    println!("1. Melodic Coastal Folk (Agreeable, Emotional, Open)");
    println!("   Geography: Coastal plains");
    println!("   Word Order: {:?}", melodic.word_order());
    println!("   Translation: \"{}\"", melodic.translate_phrase(test_phrase));
    println!();

    println!("2. Mountain Warriors (Disagreeable, Conscientious, Brave)");
    println!("   Geography: Mountains");
    println!("   Word Order: {:?}", martial.word_order());
    println!("   Translation: \"{}\"", martial.translate_phrase(test_phrase));
    println!();

    println!("3. River Valley Scholars (Open, Conscientious, Intellectual)");
    println!("   Geography: River valleys");
    println!("   Word Order: {:?}", scholarly.word_order());
    println!("   Translation: \"{}\"", scholarly.translate_phrase(test_phrase));
    println!();

    println!("4. Desert Nomads (Independent, Hardy, Traditional)");
    println!("   Geography: Arid deserts");
    println!("   Word Order: {:?}", desert.word_order());
    println!("   Translation: \"{}\"", desert.translate_phrase(test_phrase));
    println!();

    // Demonstrate individual word translations
    println!("=== Individual Word Translations ===\n");

    let words = ["water", "sun", "mountain", "warrior", "peace"];

    for word in &words {
        println!("{:12} → Coastal: {:12} | Mountain: {:12} | Scholar: {:12} | Desert: {:12}",
            word,
            melodic.translate_word(word),
            martial.translate_word(word),
            scholarly.translate_word(word),
            desert.translate_word(word),
        );
    }

    println!("\n=== Phonological Characteristics ===\n");

    println!("Melodic Coastal Folk:");
    print_phonology(&melodic);

    println!("\nMountain Warriors:");
    print_phonology(&martial);

    println!("\nRiver Valley Scholars:");
    print_phonology(&scholarly);

    println!("\nDesert Nomads:");
    print_phonology(&desert);
}

fn create_melodic_coastal() -> Language {
    // Based on JavaScript: agreeableness: 4, openness: 3, conscientiousness: 2
    let culture = CulturalProfile::new(
        4.0, // agreeableness
        3.0, // openness
        2.0, // conscientiousness
        3.0, // extraversion
        3.0, // honesty_humility
        4.0, // emotionality (high for emotional culture)
    );

    Language::from_culture(culture, Geography::Coastal, 1001)
}

fn create_mountain_warriors() -> Language {
    // Based on JavaScript: agreeableness: 1, openness: 2, conscientiousness: 4
    let culture = CulturalProfile::new(
        1.0, // agreeableness (disagreeable)
        2.0, // openness
        4.0, // conscientiousness
        4.0, // extraversion (brave, assertive)
        3.0, // honesty_humility
        2.0, // emotionality (stoic)
    );

    Language::from_culture(culture, Geography::Mountains, 1002)
}

fn create_river_scholars() -> Language {
    // Based on JavaScript: agreeableness: 3, openness: 4, conscientiousness: 4
    let culture = CulturalProfile::new(
        3.0, // agreeableness
        4.0, // openness (intellectual, creative)
        4.0, // conscientiousness (organized, precise)
        3.0, // extraversion
        4.0, // honesty_humility (modest scholars)
        3.0, // emotionality
    );

    Language::from_culture(culture, Geography::RiverValley, 1003)
}

fn create_desert_nomads() -> Language {
    // Based on JavaScript: agreeableness: 2, openness: 2, conscientiousness: 3
    let culture = CulturalProfile::new(
        2.0, // agreeableness (independent)
        2.0, // openness (traditional)
        3.0, // conscientiousness
        3.0, // extraversion
        3.0, // honesty_humility
        2.0, // emotionality (hardy, resilient)
    );

    Language::from_culture(culture, Geography::Desert, 1004)
}

fn print_phonology(language: &Language) {
    let genome = &language.genome;
    let inventory = &genome.phoneme_inventory;

    println!("  Stops: {:?}", inventory.stops.iter().map(|c| &c.0).collect::<Vec<_>>());
    println!("  Fricatives: {:?}", inventory.fricatives.iter().map(|c| &c.0).collect::<Vec<_>>());
    println!("  Nasals: {:?}", inventory.nasals.iter().map(|c| &c.0).collect::<Vec<_>>());
    println!("  Liquids: {:?}", inventory.liquids.iter().map(|c| &c.0).collect::<Vec<_>>());
    println!("  Vowels: {:?}", inventory.vowels.iter().map(|v| &v.0).collect::<Vec<_>>());
    println!("  Morphology: {:?}", genome.morphology_type);
}


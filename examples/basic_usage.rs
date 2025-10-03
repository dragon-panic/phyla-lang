//! Basic usage example of the phyla-lang library.

use phyla_lang::{CulturalProfile, Geography, Language};

fn main() {
    println!("=== Phyla-Lang: Procedural Language Generation ===\n");

    // Define different cultures
    let coastal_folk = CulturalProfile::new(
        4.0, // High agreeableness - cooperative, empathetic
        3.0, // Moderate openness
        2.0, // Lower conscientiousness - more flexible
        3.0, // Moderate extraversion
        3.0, // Moderate honesty-humility
        4.0, // High emotionality - sensitive, expressive
    );

    let mountain_warriors = CulturalProfile::new(
        1.0, // Low agreeableness - more competitive
        2.0, // Lower openness - traditional
        4.0, // High conscientiousness - disciplined
        3.0, // Moderate extraversion
        3.0, // Moderate honesty-humility
        2.0, // Lower emotionality - stoic
    );

    let river_scholars = CulturalProfile::new(
        3.0, // Moderate agreeableness
        4.0, // High openness - innovative, curious
        4.0, // High conscientiousness - organized
        3.0, // Moderate extraversion
        4.0, // High honesty-humility - modest, sincere
        3.0, // Moderate emotionality
    );

    // Create languages
    let melodic = Language::from_culture(coastal_folk, Geography::Coastal, 1001);
    let harsh = Language::from_culture(mountain_warriors, Geography::Mountains, 1002);
    let scholarly = Language::from_culture(river_scholars, Geography::RiverValley, 1003);

    println!("1. MELODIC COASTAL LANGUAGE (Agreeable, Emotional, Coastal)");
    println!("   Word order: {:?}", melodic.word_order());
    demonstrate_language(&melodic);

    println!("\n2. MOUNTAIN WARRIOR LANGUAGE (Disagreeable, Conscientious, Mountains)");
    println!("   Word order: {:?}", harsh.word_order());
    demonstrate_language(&harsh);

    println!("\n3. RIVER VALLEY SCHOLAR LANGUAGE (Open, Conscientious, River Valley)");
    println!("   Word order: {:?}", scholarly.word_order());
    demonstrate_language(&scholarly);

    // Demonstrate determinism
    println!("\n=== DETERMINISM DEMONSTRATION ===");
    println!("Creating two identical languages with the same parameters...\n");

    let lang1 = Language::from_culture(coastal_folk, Geography::Coastal, 5555);
    let lang2 = Language::from_culture(coastal_folk, Geography::Coastal, 5555);

    let word1 = lang1.translate_word("forever");
    let word2 = lang2.translate_word("forever");

    println!("Language 1 translates 'forever' as: {}", word1);
    println!("Language 2 translates 'forever' as: {}", word2);
    println!("Are they the same? {}", word1 == word2);

    // Demonstrate word variation
    println!("\n=== VOCABULARY SAMPLE ===");
    let concepts = vec![
        "water", "fire", "earth", "wind", "sun", "moon", "star", "tree", "stone", "mountain",
    ];

    println!("\nCoastal Language:");
    for concept in &concepts {
        println!("  {} → {}", concept, melodic.translate_word(concept));
    }

    println!("\nMountain Language:");
    for concept in &concepts {
        println!("  {} → {}", concept, harsh.translate_word(concept));
    }

    println!("\nScholar Language:");
    for concept in &concepts {
        println!("  {} → {}", concept, scholarly.translate_word(concept));
    }
}

fn demonstrate_language(language: &Language) {
    let phrases = vec![
        "I bring the beer quickly",
        "the sun rises in the east",
        "water flows down the mountain",
    ];

    for phrase in phrases {
        println!("   \"{}\" → \"{}\"", phrase, language.translate_phrase(phrase));
    }
}


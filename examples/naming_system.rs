//! Comprehensive example of the naming system integrated with phyla-lang.
//!
//! This example demonstrates how cultural traits and geography influence
//! personal names, place names, and epithets in a unified linguistic system.

use phyla_lang::{
    Characteristic, CulturalProfile, EpithetContext, Geography, Language, PersonalNameContext,
    PlaceNameContext, PlaceType,
};

fn main() {
    println!("=== Phyla-Lang Naming System Demo ===\n");

    // Example 1: Coastal Culture - High Agreeableness, High Emotionality
    println!("--- Coastal Culture (Gentle, Flowing Names) ---");
    let coastal_culture = CulturalProfile::new(
        4.5, // High agreeableness -> soft sounds
        3.0, // Medium openness
        2.5, // Low conscientiousness
        4.0, // High extraversion
        4.0, // High honesty-humility -> humble names
        4.5, // High emotionality -> flowing, emotional
    );
    let coastal_lang = Language::from_culture(coastal_culture, Geography::Coastal, 12345);

    // Generate personal names
    for i in 0..5 {
        let context = PersonalNameContext::simple(i);
        let name = coastal_lang.naming.generate_personal_name(&context);
        println!("  Person #{}: {}", i + 1, name);
    }

    // Generate place names
    let settlement = PlaceNameContext::new(1, PlaceType::Settlement)
        .with_geography(Geography::Coastal);
    println!("  Settlement: {}", coastal_lang.naming.generate_place_name(&settlement));

    let natural = PlaceNameContext::new(2, PlaceType::Natural)
        .with_geography(Geography::Coastal);
    println!("  Natural Feature: {}", coastal_lang.naming.generate_place_name(&natural));

    println!();

    // Example 2: Mountain Culture - Low Agreeableness, High Conscientiousness
    println!("--- Mountain Culture (Strong, Structured Names) ---");
    let mountain_culture = CulturalProfile::new(
        2.0, // Low agreeableness -> harsh sounds
        2.5, // Low-medium openness
        4.5, // High conscientiousness -> structured patterns
        3.0, // Medium extraversion
        3.5, // Medium-high honesty
        2.0, // Low emotionality
    );
    let mountain_lang = Language::from_culture(mountain_culture, Geography::Mountains, 54321);

    // Generate patronymic names (high conscientiousness)
    let child_context = PersonalNameContext::with_parent(10, "Throk".to_string());
    let child_name = mountain_lang.naming.generate_personal_name(&child_context);
    println!("  Child of Throk: {}", child_name);

    for i in 11..14 {
        let context = PersonalNameContext::simple(i);
        let name = mountain_lang.naming.generate_personal_name(&context);
        println!("  Person #{}: {}", i - 10, name);
    }

    // Mountain place names
    let peak = PlaceNameContext::new(100, PlaceType::Natural)
        .with_geography(Geography::Mountains);
    println!("  Mountain Peak: {}", mountain_lang.naming.generate_place_name(&peak));

    let fortress = PlaceNameContext::new(101, PlaceType::Landmark);
    println!("  Fortress: {}", mountain_lang.naming.generate_place_name(&fortress));

    println!();

    // Example 3: Desert Culture - Low Honesty-Humility (Elaborate Names)
    println!("--- Desert Culture (Elaborate, Ostentatious Names) ---");
    let desert_culture = CulturalProfile::new(
        2.5, // Low-medium agreeableness
        4.0, // High openness
        3.0, // Medium conscientiousness
        4.5, // High extraversion
        1.5, // Low honesty-humility -> elaborate names!
        3.0, // Medium emotionality
    );
    let desert_lang = Language::from_culture(desert_culture, Geography::Desert, 99999);

    // Generate elaborate names with titles
    for i in 20..23 {
        let context = PersonalNameContext::simple(i);
        let name = desert_lang.naming.generate_personal_name(&context);
        println!("  Noble #{}: {}", i - 19, name);
    }

    // Desert place names
    let oasis = PlaceNameContext::new(200, PlaceType::Settlement)
        .with_geography(Geography::Desert);
    println!("  Oasis Settlement: {}", desert_lang.naming.generate_place_name(&oasis));

    println!();

    // Example 4: Forest Culture - High Openness (Compound/Mythopoetic Names)
    println!("--- Forest Culture (Compound, Nature-Themed Names) ---");
    let forest_culture = CulturalProfile::new(
        4.0, // High agreeableness
        4.8, // Very high openness -> compound/complex names
        3.0, // Medium conscientiousness
        3.0, // Medium extraversion
        3.5, // Medium-high honesty
        3.5, // Medium-high emotionality
    );
    let forest_lang = Language::from_culture(forest_culture, Geography::Forest, 77777);

    // Generate compound names
    for i in 30..34 {
        let context = PersonalNameContext::simple(i);
        let name = forest_lang.naming.generate_personal_name(&context);
        println!("  Person #{}: {}", i - 29, name);
    }

    // Forest place names (mythopoetic)
    let grove = PlaceNameContext::new(300, PlaceType::Natural)
        .with_geography(Geography::Forest);
    println!("  Sacred Grove: {}", forest_lang.naming.generate_place_name(&grove));

    println!();

    // Example 5: Epithets (High Openness Cultures)
    println!("--- Epithets and Honorifics ---");

    // Achievement-based epithet
    let warrior = PersonalNameContext::simple(40);
    let warrior_name = forest_lang.naming.generate_personal_name(&warrior);
    let epithet_context = EpithetContext::new(40)
        .with_achievement("dragon".to_string());

    if let Some(epithet) = forest_lang.naming.generate_epithet(&epithet_context) {
        println!("  Warrior: {} {}", warrior_name, epithet);
    } else {
        println!("  Warrior: {}", warrior_name);
    }

    // Birth circumstance epithet
    let prophet = PersonalNameContext::simple(41);
    let prophet_name = forest_lang.naming.generate_personal_name(&prophet);
    let birth_context = EpithetContext::new(41)
        .with_birth_event("storm".to_string());

    if let Some(epithet) = forest_lang.naming.generate_epithet(&birth_context) {
        println!("  Prophet: {} {}", prophet_name, epithet);
    } else {
        println!("  Prophet: {}", prophet_name);
    }

    // Characteristic epithet
    let sage = PersonalNameContext::simple(42);
    let sage_name = forest_lang.naming.generate_personal_name(&sage);
    let char_context = EpithetContext::new(42)
        .with_characteristic(Characteristic::Wise);

    if let Some(epithet) = forest_lang.naming.generate_epithet(&char_context) {
        println!("  Sage: {} {}", sage_name, epithet);
    } else {
        println!("  Sage: {}", sage_name);
    }

    println!();

    // Example 6: Founder-based place names
    println!("--- Founder-Based Place Names ---");

    let founder_name = desert_lang.naming.generate_personal_name(&PersonalNameContext::simple(50));
    let founded_city = PlaceNameContext::new(500, PlaceType::Settlement)
        .with_founder(founder_name.clone());

    println!("  Founder: {}", founder_name);
    println!("  Their City: {}", desert_lang.naming.generate_place_name(&founded_city));

    println!();

    // Example 7: Determinism demonstration
    println!("--- Determinism (Same Input = Same Output) ---");

    let context = PersonalNameContext::simple(999);
    let name1 = coastal_lang.naming.generate_personal_name(&context);
    let name2 = coastal_lang.naming.generate_personal_name(&context);

    println!("  First generation: {}", name1);
    println!("  Second generation: {}", name2);
    println!("  Are they identical? {}", name1 == name2);

    println!();

    // Example 8: Geographic influence on place names
    println!("--- Geographic Influence on Place Names ---");

    let place_id = 777;

    let mountain_settlement = PlaceNameContext::new(place_id, PlaceType::Natural)
        .with_geography(Geography::Mountains);
    println!("  Mountain Feature: {}", 
        coastal_lang.naming.generate_place_name(&mountain_settlement));

    let coastal_settlement = PlaceNameContext::new(place_id, PlaceType::Natural)
        .with_geography(Geography::Coastal);
    println!("  Coastal Feature: {}", 
        coastal_lang.naming.generate_place_name(&coastal_settlement));

    let desert_settlement = PlaceNameContext::new(place_id, PlaceType::Natural)
        .with_geography(Geography::Desert);
    println!("  Desert Feature: {}", 
        coastal_lang.naming.generate_place_name(&desert_settlement));

    println!();

    // Example 9: Integration with language translation
    println!("--- Names and Language Working Together ---");

    let person = coastal_lang.naming.generate_personal_name(&PersonalNameContext::simple(100));
    let place = coastal_lang.naming.generate_place_name(
        &PlaceNameContext::new(100, PlaceType::Settlement)
            .with_geography(Geography::Coastal)
    );

    // Translate a phrase about them
    let greeting = coastal_lang.translate_word("hello");
    let from = coastal_lang.translate_word("from");

    println!("  Name: {}", person);
    println!("  Home: {}", place);
    println!("  Introduction: {} {} {} {}", greeting, person, from, place);

    println!("\n=== Demo Complete ===");
}


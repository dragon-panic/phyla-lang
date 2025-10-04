# Phyla-Lang: Procedural Language Generation

A Rust library that generates consistent, deterministic constructed languages (conlangs) based on cultural personality traits and geographic influences.

## Features

- **Deterministic Generation**: Same inputs always produce the same outputs
- **Cultural Personality Mapping**: HEXACO traits influence phonology, morphology, and syntax
- **Geographic Influences**: Mountains, coasts, deserts, etc. shape sound systems
- **Integrated Naming System**: Generate personal names, place names, and epithets from the same cultural DNA
- **Infinite Scalability**: Generate unlimited unique languages without storing dictionaries
- **Memory Efficient**: Store only generation parameters (~3KB per language), not full lexicons
- **Thread-Safe**: Language objects are Send + Sync for multi-threaded use

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
phyla-lang = "0.1.0"
```

### Basic Usage

```rust
use phyla_lang::{Language, CulturalProfile, Geography};

// Define a cultural profile (HEXACO personality traits, 1-5 scale)
let coastal_culture = CulturalProfile::new(
    4.0, // Agreeableness - cooperative, empathetic
    3.0, // Openness - creative, curious
    2.0, // Conscientiousness - organized, disciplined
    3.0, // Extraversion - social, assertive
    3.0, // Honesty-Humility - sincere, modest
    4.0, // Emotionality - sensitive, expressive
);

// Create a language
let language = Language::from_culture(
    coastal_culture,
    Geography::Coastal,
    12345, // seed for deterministic generation
);

// Translate words
let word = language.translate_word("house");
println!("'house' in this language: {}", word);

// Translate phrases (applies word order rules)
let phrase = language.translate_phrase("I bring the beer quickly");
println!("Translated phrase: {}", phrase);

// Determinism: same input always produces same output
assert_eq!(word, language.translate_word("house"));
```

### Name Generation

```rust
use phyla_lang::{Language, CulturalProfile, Geography, PersonalNameContext, PlaceNameContext, PlaceType};

let culture = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
let language = Language::from_culture(culture, Geography::Coastal, 12345);

// Generate personal names
let context = PersonalNameContext::simple(entity_id);
let name = language.naming.generate_personal_name(&context);
println!("Character name: {}", name);

// Generate place names
let place_context = PlaceNameContext::new(place_id, PlaceType::Settlement)
    .with_geography(Geography::Coastal);
let place_name = language.naming.generate_place_name(&place_context);
println!("Settlement name: {}", place_name);

// Names share the same phonology as the language
let greeting = language.translate_word("hello");
println!("{} from {}!", greeting, place_name);
```

## How It Works

### 1. Cultural Parameters → Linguistic Features

The library maps personality traits and geography to linguistic properties:

**Personality Traits:**
- **High Agreeableness** → More nasals (m, n), liquids (l, r), softer sounds
- **Low Agreeableness** → More stops (p, t, k), harsh fricatives, consonant clusters
- **High Openness** → Larger phoneme inventory, complex syllable structures
- **High Conscientiousness** → Regular patterns, SOV word order
- **High Emotionality** → More vowels, flowing sounds

**Geography:**
- **Mountains** → Glottal stops, ejectives (k', t'), harsh sounds
- **Coastal** → Liquids, flowing sounds, vowel-heavy syllables
- **Desert** → Guttural consonants (ħ, ʕ, x), emphatic sounds
- **Forest** → Softer sounds, breathy voice, nasals
- **Plains/River Valleys** → Balanced phoneme distribution

### 2. Deterministic Word Generation

1. Hash concept string + language seed → deterministic seed
2. Use seeded RNG to determine syllable count
3. For each syllable:
   - Choose syllable pattern (CV, CVC, CCVC, etc.)
   - Fill with phonemes weighted by category probabilities
4. Return generated word (always the same for same inputs)

### 3. Grammar Application

- Supports 6 word orders: SVO, SOV, VSO, VOS, OVS, OSV
- Word order is determined by cultural conscientiousness
- Phrase translation automatically applies word order rules

## Examples

### Different Cultures, Different Languages

```rust
use phyla_lang::{Language, CulturalProfile, Geography};

// Agreeable, emotional coastal folk
let coastal = CulturalProfile::new(4.0, 3.0, 2.0, 3.0, 3.0, 4.0);
let lang1 = Language::from_culture(coastal, Geography::Coastal, 1001);

// Disagreeable, disciplined mountain warriors
let mountain = CulturalProfile::new(1.0, 2.0, 4.0, 2.0, 3.0, 2.0);
let lang2 = Language::from_culture(mountain, Geography::Mountains, 1002);

println!("Coastal: {}", lang1.translate_word("sun"));  // Flowing, soft
println!("Mountain: {}", lang2.translate_word("sun")); // Harsh, abrupt
```

Run the examples:

```bash
cargo run --example basic_usage
cargo run --example naming_system
```

### Naming System

The naming system generates culturally-consistent names based on the same linguistic foundations:

**Personal Names:**
- Simple names: "Aria", "Krag"
- Patronymic: "Aran Thorson" (high conscientiousness cultures)
- Compound: "Stormborn", "Ironheart" (high openness cultures)
- Elaborate: "Lord Maxim the Third" (low honesty-humility cultures)

**Place Names:**
- Descriptive: "Deepwater", "Redmountain"
- Founder-based: "Jamestown", "Alexandria"
- Historical: "Battleford", "Victory Bay"
- Mythopoetic: "Dragonspire", "Moonhaven" (high openness cultures)

**Epithets:**
- Achievement: "Dragonslayer", "the Conqueror"
- Birth circumstance: "Stormborn", "of the Winter"
- Characteristic: "the Wise", "the Brave"

Names use the same phonology, morphemes, and cultural weights as the language, ensuring coherence.

### Use Cases

1. **Game Worlds**: Generate distinct languages and naming conventions for factions/cultures
2. **Worldbuilding**: Create realistic language families with consistent naming patterns
3. **Procedural Content**: Generate character names, place names, dialogue
4. **Simulation**: Deterministic name generation for reproducible worlds

## Architecture

### Core Components

- `LinguisticGenome`: Complete specification of a language (phonology, syntax, morphology)
- `PhonemeInventory`: Available sounds (consonants and vowels)
- `SyllableStructure`: Patterns like CV, CVC, CCVC
- `WordOrder`: SVO, SOV, VSO, etc.
- `MorphemeDatabase`: Semantic building blocks weighted by cultural importance
- `NamingSystem`: Generate personal, place, and epithet names
- `Language`: Public API for word/phrase translation and name generation

### Performance

- Word generation: < 100 microseconds
- Memory per language: < 5KB (genome only)
- Optional caching for frequently-used words
- Thread-safe for concurrent use

## Testing

Run unit tests:

```bash
cargo test
```

Run integration tests:

```bash
cargo test --test integration_test
```

Run all tests with output:

```bash
cargo test -- --nocapture
```

## Design Philosophy

Languages **emerge** from parameterized cultural profiles rather than being manually designed. This enables:

1. **Infinite scalability**: Generate unlimited languages without storing dictionaries
2. **Narrative consistency**: Same concept always translates to same word
3. **Memory efficiency**: Store only parameters, not full lexicons
4. **Emergent authenticity**: Languages feel real because they follow consistent rules

## Future Enhancements

- ✅ **Naming System** (completed)
- Writing system generation (orthography)
- Historical sound changes and language evolution
- Language family relationships (proto-languages, daughter languages)
- Dialectal variation
- Full morphological analysis (affixes, inflections)
- Phonotactic constraints (rules about sound combinations)
- Object naming (tools, weapons, artifacts)
- Family/clan names with inheritance

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.


//! Deterministic random number generator for consistent language generation.

/// A simple linear congruential generator (LCG) for deterministic randomness.
/// This ensures that the same seed always produces the same sequence of numbers.
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    /// Create a new seeded RNG with the given seed.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Generate the next random number in [0, 1).
    pub fn next(&mut self) -> f64 {
        // LCG parameters (same as JavaScript implementation)
        self.state = (self.state.wrapping_mul(9301).wrapping_add(49297)) % 233280;
        self.state as f64 / 233280.0
    }

    /// Choose a random element from a slice.
    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        let index = (self.next() * items.len() as f64) as usize;
        &items[index.min(items.len() - 1)]
    }

    /// Choose an index based on weighted probabilities.
    /// Returns the index of the selected item.
    pub fn weighted_choice(&mut self, weights: &[f32]) -> usize {
        let total: f32 = weights.iter().sum();
        let mut rand = self.next() as f32 * total;

        for (i, &weight) in weights.iter().enumerate() {
            if rand < weight {
                return i;
            }
            rand -= weight;
        }

        weights.len() - 1
    }

    /// Generate a random integer in the range [min, max).
    pub fn range(&mut self, min: usize, max: usize) -> usize {
        min + (self.next() * (max - min) as f64) as usize
    }
}

/// Hash a string to a deterministic u64 seed.
pub fn hash_string(s: &str) -> u64 {
    let mut hash: i32 = 0;
    for ch in s.chars() {
        hash = ((hash << 5).wrapping_sub(hash)).wrapping_add(ch as i32);
    }
    hash.unsigned_abs() as u64
}

/// Create a deterministic seed from a concept and language seed.
pub fn hash_deterministic(concept: &str, language_seed: u64) -> u64 {
    let concept_hash = hash_string(concept);
    // Combine the two hashes in a deterministic way
    concept_hash.wrapping_mul(31).wrapping_add(language_seed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinism() {
        let mut rng1 = SeededRng::new(12345);
        let mut rng2 = SeededRng::new(12345);

        for _ in 0..100 {
            assert_eq!(rng1.next(), rng2.next());
        }
    }

    #[test]
    fn test_hash_determinism() {
        let hash1 = hash_string("hello");
        let hash2 = hash_string("hello");
        assert_eq!(hash1, hash2);

        let hash3 = hash_string("world");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_weighted_choice() {
        let mut rng = SeededRng::new(42);
        let weights = vec![0.5, 0.3, 0.2];

        let mut counts = vec![0; 3];
        for _ in 0..1000 {
            let choice = rng.weighted_choice(&weights);
            counts[choice] += 1;
        }

        // First option should be chosen most often (roughly 50% of the time)
        assert!(counts[0] > counts[1]);
        assert!(counts[1] > counts[2]);
    }
}


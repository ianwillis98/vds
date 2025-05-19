#![cfg(feature = "generate")]

use vds::{VDGenerator, VDS_ALLOWED};
use proptest::prelude::*;
use rand::SeedableRng;
use rand::rngs::SmallRng;

proptest! {
    #[test]
    fn generator_produces_valid_strings(len in 1usize..20) {
        let mut rng = SmallRng::seed_from_u64(42);
        let code = VDGenerator::new().length(len).generate(&mut rng).unwrap();
        prop_assert_eq!(code.len(), len);
        prop_assert!(code.as_vdchars().iter().all(|c| VDS_ALLOWED.contains(&c.as_char())));
    }

    #[test]
    fn generator_respects_no_adjacent_repeats(len in 2usize..20) {
        let mut rng = SmallRng::seed_from_u64(99);
        let code = VDGenerator::new().length(len).no_adjacent_repeats().generate(&mut rng).unwrap();
        for w in code.as_vdchars().windows(2) {
            prop_assert_ne!(w[0], w[1]);
        }
    }

    #[test]
    fn generator_respects_no_repeats(len in 1usize..=VDS_ALLOWED.len()) {
        let mut rng = SmallRng::seed_from_u64(7);
        let code = VDGenerator::new().length(len).no_repeats().generate(&mut rng).unwrap();
        let mut seen = std::collections::HashSet::new();
        for ch in code.as_vdchars() {
            prop_assert!(seen.insert(ch.as_char())); // ensures no duplicates
        }
    }
}

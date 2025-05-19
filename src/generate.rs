extern crate alloc;
use alloc::vec::Vec;

use rand_core::RngCore;
use crate::{VDChar, VDString};
use crate::vdchar::VDS_ALLOWED;

/// Error returned when [`VDGenerator::generate`] is called with an invalid configuration.
///
/// This type is only available when the `generate` feature is enabled.
#[cfg_attr(docsrs, doc(cfg(feature = "generate")))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VDGeneratorError {
    /// The requested output length exceeds the number of unique characters available.
    ///
    /// This error occurs when `no_repeats` is enabled and `length > VDS_ALLOWED.len()`.
    LengthExceedsUniqueSet {
        /// The requested number of characters.
        requested: usize,
        /// The number of distinct characters available.
        available: usize,
    },
}

/// A builder-style configuration for generating random [`VDString`]s.
///
/// This generator creates strings made up of [`VDChar`]s — characters from a curated
/// uppercase set that excludes ambiguous glyphs like `O`, `0`, `I`, and `1`.
///
/// You can customize the output length and control whether repeated or adjacent characters
/// are allowed.
///
/// This type is only available when the `generate` feature is enabled.
///
/// # Examples
///
/// ```
/// use rand::rngs::SmallRng;
/// use rand::SeedableRng;
/// use vds::{VDGenerator, VDString};
///
/// let mut rng = SmallRng::seed_from_u64(123);
///
/// let result: VDString = VDGenerator::new()
///     .length(8)
///     .no_adjacent_repeats()
///     .no_repeats()
///     .generate(&mut rng)
///     .unwrap();
///
/// assert_eq!(result.len(), 8);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "generate")))]
pub struct VDGenerator {
    len: usize,
    no_adjacent_repeats: bool,
    no_repeats: bool,
}

#[cfg_attr(docsrs, doc(cfg(feature = "generate")))]
impl VDGenerator {
    /// Creates a new generator with default settings.
    ///
    /// Defaults:
    /// - Length: 6
    /// - Adjacent repeats: allowed
    /// - Any repeats: allowed
    pub fn new() -> Self {
        Self {
            len: 6,
            no_adjacent_repeats: false,
            no_repeats: false,
        }
    }

    /// Sets the length of the generated string.
    pub fn length(mut self, len: usize) -> Self {
        self.len = len;
        self
    }

    /// Forbids adjacent repeated characters in the output.
    pub fn no_adjacent_repeats(mut self) -> Self {
        self.no_adjacent_repeats = true;
        self
    }

    /// Forbids any repeated characters in the output.
    ///
    /// Useful when aiming for high visual uniqueness in short codes.
    /// Has an upper bound of `VDS_ALLOWED.len()` characters.
    pub fn no_repeats(mut self) -> Self {
        self.no_repeats = true;
        self
    }

    /// Generates a [`VDString`] based on the current configuration and RNG.
    ///
    /// Returns a [`VDGeneratorError`] if the configuration is invalid.
    ///
    /// # Errors
    /// - [`VDGeneratorError::LengthExceedsUniqueSet`] if `no_repeats` is enabled and
    ///   `length > VDS_ALLOWED.len()`.
    pub fn generate<R: RngCore + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<VDString, VDGeneratorError> {
        if self.no_repeats && self.len > VDS_ALLOWED.len() {
            return Err(VDGeneratorError::LengthExceedsUniqueSet {
                requested: self.len,
                available: VDS_ALLOWED.len(),
            });
        }

        let mut result = Vec::with_capacity(self.len);

        if self.no_repeats {
            // Sample without replacement by shuffling
            let mut pool: Vec<VDChar> = (0..VDS_ALLOWED.len())
                .map(|i| VDChar(i as u8))
                .collect();

            // Fisher-Yates shuffle (partial)
            for i in 0..self.len {
                let j = i + (rng.next_u32() as usize % (pool.len() - i));
                pool.swap(i, j);
            }

            result.extend_from_slice(&pool[..self.len]);

            if self.no_adjacent_repeats {
                // Rotate until no adjacent duplicates, up to `len` tries
                for _ in 0..self.len {
                    if result.windows(2).any(|w| w[0] == w[1]) {
                        result.rotate_left(1);
                    } else {
                        break;
                    }
                }
            }

            return Ok(VDString::new(result));
        }

        // With replacement sampling
        let mut last: Option<VDChar> = None;

        while result.len() < self.len {
            let idx = (rng.next_u32() as usize) % VDS_ALLOWED.len();
            let ch = VDChar(idx as u8);

            if self.no_adjacent_repeats && last == Some(ch) {
                continue;
            }

            result.push(ch);
            last = Some(ch);
        }

        Ok(VDString::new(result))
    }
}

#[cfg(test)]
#[cfg(feature = "generate")]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::vec;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    fn seeded_rng() -> SmallRng {
        SmallRng::seed_from_u64(42)
    }

    #[test]
    fn generates_expected_length() {
        let mut rng = seeded_rng();
        let code = VDGenerator::new().length(8).generate(&mut rng).unwrap();
        assert_eq!(code.len(), 8);
    }

    #[test]
    fn no_adjacent_repeats_enabled() {
        let mut rng = seeded_rng();
        let code = VDGenerator::new()
            .length(16)
            .no_adjacent_repeats()
            .generate(&mut rng)
            .unwrap();

        for pair in code.as_vdchars().windows(2) {
            assert_ne!(pair[0], pair[1], "adjacent repeat found");
        }
    }

    #[test]
    fn no_repeats_enabled() {
        let mut rng = seeded_rng();
        let code = VDGenerator::new()
            .length(16)
            .no_repeats()
            .generate(&mut rng)
            .unwrap();

        let mut seen = vec![];
        for ch in code.as_vdchars() {
            assert!(!seen.contains(ch), "repeat found: {:?}", ch);
            seen.push(*ch);
        }
    }

    #[test]
    fn no_repeats_exceeds_allowed_panics() {
        let mut rng = seeded_rng();
        let result = VDGenerator::new()
            .length(VDS_ALLOWED.len() + 1)
            .no_repeats()
            .generate(&mut rng);

        assert!(matches!(
            result,
            Err(VDGeneratorError::LengthExceedsUniqueSet { .. })
        ));
    }

    #[test]
    fn combined_flags_hold() {
        let mut rng = seeded_rng();
        let code = VDGenerator::new()
            .length(12)
            .no_adjacent_repeats()
            .no_repeats()
            .generate(&mut rng)
            .unwrap();

        let mut seen = vec![];
        for (i, ch) in code.as_vdchars().iter().enumerate() {
            if i > 0 {
                assert_ne!(code[i - 1], *ch, "adjacent repeat");
            }
            assert!(!seen.contains(ch), "repeat");
            seen.push(*ch);
        }
    }
}

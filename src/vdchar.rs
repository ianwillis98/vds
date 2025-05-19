use core::fmt;

/// Allowed characters for [`VDChar`].
///
/// This list excludes commonly ambiguous glyphs like `O`, `0`, `I`, and `1`
/// to ensure visual clarity in user-facing strings (e.g. codes, IDs).
///
/// All characters are uppercase Latin letters or digits, and are stored in a
/// fixed order. Characters not in this set (including lowercase) are rejected.
pub const VDS_ALLOWED: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
    'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7', '8', '9',
];

/// A single visibly distinguishable character from a restricted set.
///
/// Internally stores an index into [`VDS_ALLOWED`], a curated set of
/// uppercase characters chosen for visual clarity. Use this type to ensure
/// consistent readability when displaying codes in a UI, printing them on
/// packaging, or transmitting over error-prone mediums.
///
/// [`VDChar`] is also used to construct [`VDString`](crate::VDString), which represents a
/// sequence of validated `VDChar`s.
///
/// # Examples
///
/// ```
/// use vds::VDChar;
///
/// assert!(VDChar::new('A').is_some());
/// assert!(VDChar::new('o').is_none()); // lowercase rejected
/// assert!(VDChar::new('O').is_none()); // O is excluded for clarity
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VDChar(pub(crate) u8);

impl VDChar {
    /// Attempts to create a [`VDChar`] from a `char`.
    ///
    /// Returns `None` if the input character is not in [`VDS_ALLOWED`],
    /// including lowercase characters or excluded ambiguous glyphs.
    ///
    /// # Examples
    /// ```
    /// use vds::VDChar;
    ///
    /// assert!(VDChar::new('X').is_some());
    /// assert!(VDChar::new('x').is_none()); // lowercase
    /// assert!(VDChar::new('0').is_none()); // excluded
    /// ```
    pub fn new(c: char) -> Option<Self> {
        VDS_ALLOWED.iter().position(|&x| x == c).map(|i| Self(i as u8))
    }

    /// Returns the underlying `char` represented by this `VDChar`.
    ///
    /// # Examples
    /// ```
    /// use vds::VDChar;
    /// let c = VDChar::new('V').unwrap();
    /// assert_eq!(c.as_char(), 'V');
    /// ```
    pub fn as_char(self) -> char {
        VDS_ALLOWED[self.0 as usize]
    }
}

impl fmt::Display for VDChar {
    /// Formats the `VDChar` as its character value.
    ///
    /// Equivalent to calling `.as_char()`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;
    use alloc::string::ToString;
    
    #[test]
    fn valid_vdchar_constructs() {
        assert!(VDChar::new('A').is_some());
        assert!(VDChar::new('Z').is_some());
        assert!(VDChar::new('2').is_some());
        assert!(VDChar::new('9').is_some());
    }

    #[test]
    fn excluded_chars_are_rejected() {
        assert!(VDChar::new('O').is_none()); // intentionally excluded
        assert!(VDChar::new('I').is_none()); // ambiguous
        assert!(VDChar::new('0').is_none()); // looks like 'O'
        assert!(VDChar::new('1').is_none()); // looks like 'I'
    }

    #[test]
    fn lowercase_chars_are_rejected() {
        assert!(VDChar::new('a').is_none());
        assert!(VDChar::new('z').is_none());
        assert!(VDChar::new('o').is_none());
    }

    #[test]
    fn as_char_returns_original_char() {
        for &c in VDS_ALLOWED {
            let vd = VDChar::new(c).expect("should be allowed");
            assert_eq!(vd.as_char(), c);
        }
    }

    #[test]
    fn display_matches_as_char() {
        let ch = VDChar::new('X').unwrap();
        assert_eq!(ch.to_string(), "X");
    }
}

extern crate alloc;
use alloc::{vec::Vec, string::String};

use core::{fmt, ops::{Deref, Index}};
use core::str::FromStr;

use crate::VDChar;

/// Error returned when constructing or parsing a [`VDString`].
///
/// This error occurs when an input string contains characters not in the
/// allowed visible set defined by [`VDS_ALLOWED`](crate::VDS_ALLOWED).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VDStringError {
    /// A character in the input was not part of the allowed set.
    InvalidChar(char),
}

/// A validated, immutable string composed entirely of [`VDChar`]s.
///
/// All characters are guaranteed to come from [`VDS_ALLOWED`](crate::VDS_ALLOWED),
/// a curated uppercase set that avoids visually ambiguous glyphs like `0`, `O`, `1`, `I`.
///
/// Internally backed by a `Vec<VDChar>` and a cached `String`, `VDString` ensures both:
/// - **Safe rendering** in user interfaces or printed material
/// - **Fast access** to the string representation
///
/// Construct via `.parse()`, `TryFrom<&str>`, or from a list of `VDChar`s.
///
/// # Examples
/// ```
/// use vds::VDString;
///
/// let code: VDString = "AB29XY".parse().unwrap();
/// assert_eq!(&*code, "AB29XY");
///
/// for ch in &code {
///     print!("{},", ch);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VDString {
    chars: Vec<VDChar>,
    cache: String,
}

impl VDString {
    /// Creates a new `VDString` from a validated list of [`VDChar`]s.
    ///
    /// Caches the string representation for formatting and fast lookup.
    ///
    /// # Examples
    /// ```
    /// use vds::{VDChar, VDString};
    ///
    /// let chars = ['A', 'B', '2'].iter().filter_map(|&c| VDChar::new(c)).collect();
    /// let code = VDString::new(chars);
    /// assert_eq!(&*code, "AB2");
    /// ```
    pub fn new(chars: Vec<VDChar>) -> Self {
        let cache = chars.iter().map(|c| c.as_char()).collect();
        Self { chars, cache }
    }

    /// Returns a slice of the internal [`VDChar`] list.
    ///
    /// Useful for inspecting, transforming, or re-encoding the raw data.
    pub fn as_vdchars(&self) -> &[VDChar] {
        &self.chars
    }
}

impl Deref for VDString {
    type Target = str;

    /// Allows `VDString` to behave like a `&str` (e.g., `&*vdstring == "ABC"`).
    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl fmt::Display for VDString {
    /// Displays the cached string of visible characters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.cache)
    }
}

impl Index<usize> for VDString {
    type Output = VDChar;

    /// Indexes into the underlying [`VDChar`] list.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// use vds::VDString;
    ///
    /// let s: VDString = "B7X".parse().unwrap();
    /// assert_eq!(s[1].as_char(), '7');
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.chars[index]
    }
}

impl<'a> IntoIterator for &'a VDString {
    type Item = VDChar;
    type IntoIter = core::iter::Copied<core::slice::Iter<'a, VDChar>>;

    /// Returns an iterator over the [`VDChar`]s in this string.
    ///
    /// # Examples
    /// ```
    /// use vds::VDString;
    ///
    /// let s: VDString = "3MV".parse().unwrap();
    /// let chars: Vec<_> = s.into_iter().map(|c| c.as_char()).collect();
    /// assert_eq!(chars, vec!['3', 'M', 'V']);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.chars.iter().copied()
    }
}

impl FromStr for VDString {
    type Err = VDStringError;

    /// Parses a `&str` into a `VDString`, validating each character.
    ///
    /// Returns a [`VDStringError::InvalidChar`] if any character is invalid.
    ///
    /// # Examples
    /// ```
    /// use vds::VDString;
    ///
    /// let valid = "7ZPQ".parse::<VDString>();
    /// assert!(valid.is_ok());
    ///
    /// let invalid = "7ZPQ!".parse::<VDString>();
    /// assert!(invalid.is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| VDChar::new(c).ok_or(VDStringError::InvalidChar(c)))
            .collect::<Result<Vec<_>, _>>()
            .map(VDString::new)
    }
}

impl TryFrom<&str> for VDString {
    type Error = VDStringError;

    /// Tries to convert a string slice into a `VDString`.
    ///
    /// Equivalent to `VDString::from_str`.
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        VDString::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use alloc::{string::ToString, vec, vec::Vec};
    use super::*;

    fn vd(c: char) -> VDChar {
        VDChar::new(c).unwrap()
    }

    #[test]
    fn construct_from_vec() {
        let chars = vec![vd('A'), vd('B'), vd('2')];
        let s = VDString::new(chars.clone());
        assert_eq!(s.as_vdchars(), &chars[..]);
        assert_eq!(&*s, "AB2");
    }

    #[test]
    fn parse_valid_string() {
        let s: VDString = "M29W".parse().unwrap();
        assert_eq!(&*s, "M29W");
        assert_eq!(s.len(), 4);
        assert_eq!(s[0], vd('M'));
        assert_eq!(s[3].as_char(), 'W');
    }

    #[test]
    fn parse_invalid_string() {
        let err = "HELLO!".parse::<VDString>();
        assert!(err.is_err());

        let err2 = "O0I1".parse::<VDString>();
        assert!(err2.is_err());
    }

    #[test]
    fn index_returns_correct_char() {
        let s: VDString = "5K7".parse().unwrap();
        assert_eq!(s[0], vd('5'));
        assert_eq!(s[1].as_char(), 'K');
        assert_eq!(s[2].to_string(), "7");
    }

    #[test]
    fn iterates_over_chars() {
        let s: VDString = "X2Z".parse().unwrap();
        let collected: Vec<char> = s.into_iter().map(|c| c.as_char()).collect();
        assert_eq!(collected, vec!['X', '2', 'Z']);
    }

    #[test]
    fn from_str_and_try_from_match() {
        let a = "Q4V";
        let parsed = a.parse::<VDString>().unwrap();
        let tried = VDString::try_from(a).unwrap();
        assert_eq!(parsed, tried);
    }
}

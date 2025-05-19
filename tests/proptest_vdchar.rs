use vds::{VDChar, VDS_ALLOWED};
use proptest::prelude::*;

// Ensures that `VDChar::new` only accepts characters in the allowed set.
proptest! {
    #[test]
    fn vdchar_new_matches_allowed_set(c in any::<char>()) {
        let actual = VDChar::new(c).is_some();
        let expected = VDS_ALLOWED.contains(&c);
        prop_assert_eq!(actual, expected);
    }
}

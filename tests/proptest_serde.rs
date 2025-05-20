#![cfg(feature = "serde")]

use vds::{VDChar, VDString, VDS_ALLOWED};
use proptest::{prelude::*, sample::select};
use serde_json;

proptest! {
    #[test]
    fn vdchar_json_roundtrip(c in select(VDS_ALLOWED)) {
        let vd = VDChar::new(c).unwrap();
        let json = serde_json::to_string(&vd).unwrap();
        let parsed: VDChar = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(vd, parsed);
    }

    #[test]
    fn vdstring_json_roundtrip(chars in proptest::collection::vec(select(VDS_ALLOWED), 1..20)) {
        let s: String = chars.iter().collect();
        let vd: VDString = s.parse().unwrap();
        let json = serde_json::to_string(&vd).unwrap();
        let parsed: VDString = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(vd, parsed);
    }

    #[test]
    fn vdstring_json_fails_on_invalid(s in ".*") {
        // Only test strings with at least one invalid character
        if s.chars().any(|c| !VDS_ALLOWED.contains(&c)) {
            let result = serde_json::from_str::<VDString>(&format!("\"{}\"", s));
            prop_assert!(result.is_err());
        }
    }
}

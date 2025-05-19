use vds::{VDString, VDS_ALLOWED};
use proptest::{prelude::*, sample::select};

proptest! {
    #[test]
    fn vdstring_parse_rejects_invalid(s in ".*") {
        let res = s.parse::<VDString>();
        if res.is_ok() {
            prop_assert!(s.chars().all(|c| VDS_ALLOWED.contains(&c)));
        }
    }

    #[test]
    fn vdstring_roundtrip_valid_string(s in proptest::collection::vec(select(VDS_ALLOWED), 1..20)) {
        let input: String = s.iter().collect();
        let parsed: VDString = input.parse().unwrap();
        prop_assert_eq!(&*parsed, input);
    }
}

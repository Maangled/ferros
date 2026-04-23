use ferros_core::{foundation_ready, FOUNDATION_MARKER};

#[test]
fn trivial_foundation_smoke_test() {
    assert_eq!(1 + 1, 2);
    assert!(foundation_ready());
    assert_eq!(FOUNDATION_MARKER, "foundation-ready");
}

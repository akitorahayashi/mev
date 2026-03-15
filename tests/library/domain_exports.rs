//! Verify public API surfaces remain accessible.

#[test]
fn domain_tag_resolution_is_public() {
    let tags = mev::domain::tag::resolve_tags("rust");
    assert_eq!(tags, vec!["rust-platform", "rust-tools"]);
}


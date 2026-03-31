//! Verify public API surfaces remain accessible.

#[test]
fn identity_resolves_identities() {
    use mev::domain::identity::SwitchIdentity;
    let identity = mev::domain::identity::resolve_switch_identity("p");
    assert_eq!(identity, Some(SwitchIdentity::Personal));
}

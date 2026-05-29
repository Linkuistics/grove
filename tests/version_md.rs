use grove::version_md::{read_version, write};
use tempfile::TempDir;

#[test]
fn write_canonicalises_v_prefixed_tag() {
    // The git tag `v3.0.1` is stored canonically (no leading `v`) — the stamp
    // is the version's identity, matching CARGO_PKG_VERSION. See ADR-0008.
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "claude", "v3.0.1").unwrap();

    let got = read_version(tmp.path()).unwrap();
    assert_eq!(got, "3.0.1");
}

#[test]
fn write_leaves_already_canonical_version_untouched() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "claude", "0.4.2").unwrap();

    let got = read_version(tmp.path()).unwrap();
    assert_eq!(got, "0.4.2");
}

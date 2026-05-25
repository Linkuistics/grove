use grove::version_md::{read_version, write};
use tempfile::TempDir;

#[test]
fn roundtrips_version_string() {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "claude", "v0.4.2").unwrap();

    let got = read_version(tmp.path()).unwrap();
    assert_eq!(got, "v0.4.2");
}

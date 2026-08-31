// Smoke test for the `grove-llm` binary's clap wiring: the LLM-driven verbs
// are enumerated by `grove-llm --help`, and the human-facing launcher/admin
// verbs do not bleed into that surface (the audience-split of cli-binary-split).

use assert_cmd::Command;

#[test]
fn help_lists_only_llm_verbs() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    // A representative sample of the surviving task-tree verbs.
    for verb in ["pick", "root-init", "leaf-add", "complete"] {
        assert!(s.contains(verb), "missing grove-llm verb `{verb}`: {s}");
    }
    // Launcher and admin verbs must not bleed into grove-llm's surface.
    for forbidden in [
        "start",
        "continue",
        "takeover",
        "install",
        "update",
        "report-turn",
    ] {
        assert!(
            !s.contains(&format!(" {} ", forbidden)),
            "grove-llm --help should not list `{}`: {s}",
            forbidden
        );
    }
}

/// **`grove-llm --version` is grove's version, not this package's.**
///
/// One workspace, one release version (`docs/specs/module-decomposition.md`,
/// decision 1). `loop-crate-verbs-k21` made `grove-llm` its own package, and a
/// package carries its own `version` field — so clap's bare `version` attribute
/// silently started answering `0.1.0`, a number naming nothing an operator can
/// install. The two binaries have to agree, because a skew between them is
/// exactly what an operator reaches for them to diagnose.
#[test]
fn the_two_binaries_report_one_version() {
    let llm = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--version")
        .output()
        .unwrap();
    let human = Command::cargo_bin("grove")
        .unwrap()
        .arg("--version")
        .output()
        .unwrap();
    let llm = String::from_utf8_lossy(&llm.stdout).into_owned();
    let human = String::from_utf8_lossy(&human.stdout).into_owned();
    assert_eq!(
        llm.trim().strip_prefix("grove-llm "),
        human.trim().strip_prefix("grove "),
        "the two binaries must publish one version: {llm:?} vs {human:?}"
    );
    assert_ne!(
        llm.trim().strip_prefix("grove-llm "),
        Some("0.1.0"),
        "grove-llm is answering with its own package version: {llm:?}"
    );
}

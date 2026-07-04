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
    for forbidden in ["start", "continue", "takeover", "install", "update"] {
        assert!(
            !s.contains(&format!(" {} ", forbidden)),
            "grove-llm --help should not list `{}`: {s}",
            forbidden
        );
    }
}

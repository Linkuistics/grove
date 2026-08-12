# shared-skill-dir-clobber-integrate-k19

**Integrates:** shared-skill-dir-clobber-review-k16

## Goal

Apply the actionable findings from `shared-skill-dir-clobber-review-k16` before
`one-build-owns-a-session-k17` implements the design. Rework
`docs/adr/one-build-owns-a-session.md` and every reconciled current-state
document in place; update the implementation leaf where the corrected decision
changes its mandate.

## Context

The central diagnosis survives: the checked sibling path is discarded, crate
version cannot identify an edited checkout at the same version, and a
compile-time methodology identity can keep `grove-llm` from linking the
embedded content. The review also confirmed that a flag can stay outside the
agent verb grammar, that per-iteration stamp re-verification is useful before a
next launch, and that the new ADR is registered with working durable links.

Apply these findings:

1. **P1 — The fatal `PATH` preflight cannot identify the CLI an opaque
   configured session will run.** The ADR says driver-side `PATH` resolution is
   "the way the session will" resolve `grove-llm`, but the complete-session
   configuration contract deliberately supports opaque wrappers, login shells,
   `ssh`, and containers that may re-derive `PATH`. Such a valid target can be
   refused while correct or admitted while wrong. The old version guard did
   not make the same ordinary-case assumption: it preferred
   `current_exe()`'s sibling and consulted PATH only on fallback; its returned
   path is bound to `_grove_llm` and never reaches the launched config-expanded
   argv. Choose a coherent correction before implementation: either narrow the
   supported launch contract and reconcile `complete-session-configuration`,
   or make the proxy probe advisory/best-effort and state that the actual CLI
   behind an opaque target is observable only when it runs. Reconsider the
   shim/expected-identity alternatives rather than rejecting them solely by
   citing an ADR that may itself need in-place amendment.
2. **P2 — `cargo install --path .` is not a one-command universal remedy.** In
   the reviewed environment `/opt/homebrew/bin` precedes `~/.cargo/bin`; Cargo
   installation would not replace or outrank the Homebrew pair. Explicitly
   invoking the new Cargo-installed driver would still resolve the old
   Homebrew `grove-llm`, refuse, and prescribe the install already performed.
   State the real requirement — the intended pair must occupy active PATH
   resolution, or an explicitly supported equivalent — make the diagnostic
   name the resolved mismatching path, and remove the claim that Cargo install
   alone fixes every grove on the machine.
3. **P2 — The config-session spec still specifies the old guard.** Reconcile
   `docs/specs/config-driven-sessions.md:374-380` (sibling-first resolution and
   crate-version check) and lines 1288-1289 (the stale test-seam requirement)
   with the corrected decision.
4. **P3 — The proposed identity omits directory structure.**
   `src/provision.rs:172-189` hashes only file paths and bytes, while
   `include_dir` 0.7.4 embeds a `DirEntry::Dir` for every directory, including
   empty ones. The current tree has no symlinks or empty directories and hidden
   files/symlink targets otherwise follow the expected traversal, but an empty
   directory can change the embed and extraction without changing the stamp,
   and both proposed traversals can omit it while their equality test passes.
   Either hash typed directory paths or narrow "embedded-tree identity" to the
   semantically relevant file payload and explicitly make empty directories
   non-identity. Keep the build-vs-runtime equality test for file traversal.
5. **P3 — Keep the glossary terms separate but remove mechanism.**
   **Methodology identity** names a value; **Build pairing** names an invariant,
   so collapsing them loses useful vocabulary. Trim `CONTEXT.md:29-36,56-64`
   to those definitions and useful `_Avoid_` guards; keep `build.rs`, the flag,
   and the three-check inventory in the ADR/architecture.

## Done when

- The design no longer claims a fatal driver-side proxy observes an opaque
  configured session's actual CLI, and its supported/unsupported boundary is
  consistent with `complete-session-configuration`.
- Installation guidance is actionable under multiple installed prefixes and
  diagnostics identify what actually mismatched.
- The config-session spec and `one-build-owns-a-session-k17` task agree with the
  revised decision, with no stale sibling/version requirements.
- Methodology identity either covers directories or states its file-payload
  grain accurately.
- The two glossary definitions stay distinct and definition-only.
- This integration task changes design/docs/task instructions only. It does not
  implement or verify the Rust changes owned by `one-build-owns-a-session-k17`.

## Notes

The review was inspection-only: no build, test, lint, or format commands ran.
Its exact findings and source citations remain in
`shared-skill-dir-clobber-review-k16`.

# grove-native-provisioning-k21

## Goal

Remove automatic Codex snapshot provisioning while keeping native-only Grove
launches and explicit installation usable with Ravel 1.1.

## Context

Use the delivery specification and its evidence document's consumer map.
Recheck `cli::execute`, `provision`, the generated build inventory and binary
module, provisioning/lifecycle tests and package checks. Do not infer a shared
dependency is dead merely because the provisioner used it.

## Done when

- Remove the startup writer, private provisioner and embedded corpus/build
  coupling together. Retain manual installers and marketplace sources; add no
  fallback, skill presence gate or automatic legacy cleanup.
- Replace obsolete installation assertions with behavioral no-mutation checks
  covering fresh, owned and foreign entries, aliases, modes, current pointers,
  failure and an active old reader. Verify a native-only lifecycle with complete
  explicit skills and directory injection disabled against actual Ravel 1.1.
- Update installation/plugin guidance, architecture/glossary, relevant
  source-exact overview books/indexes and formula caveats/checks alongside code.
  Reconcile the old provisioning plan. Preserve the archive's executable/assets
  boundary; startup and package upgrade leave old installations untouched.
- Supply optional manual cleanup in `docs/USAGE.md`, linked from plugin guidance.
  Exercise it in disposable legacy layouts with stopped writers/readers,
  owned/foreign links, aliases, uncertain provenance and retained references.
  Path shape alone never authorizes removal.
- Run relevant tasks (`check:skills`, affected book and installed-layout checks)
  and the full `check`/`bash scripts/check.sh` with explicit compatible Ravel
  inputs. Record exact build identities and limits on this task. Decompose if
  necessary rather than leaving code and required consumers inconsistent.

## Notes

This is the whole writer-removal increment, not a preparatory dead layer.
No owner settings, shared native links or installed binaries are changed.
Mandate wording and real implicit-discovery trials have separate later work.

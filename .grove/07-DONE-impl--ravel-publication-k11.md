# ravel-publication-k11

## Goal

Publish and install the independently verified Ravel release and Homebrew
formula without combining release-instrument development with a public cut.

## Context

- Requires ravel-delivery-k4's committed recipes, exact manifests and passing
  unreleased installed-layout checks for all three supported targets.
- Work in `/Users/antony/Development/Ravel` and the existing Homebrew tap.
  Recheck source, remote tags and tap state; preparation evidence is not a lock.

## Done when

- Run Ravel's validated `task release:minor` from k3's 0.0.0 to first release
  0.1.0, after rechecking tags and any partial cut. Verify notes,
  tag, all three archives/digests, formula and `brew test ravel`; do not change
  unrelated tap formulas or the installed Grove/legacy dispatch pair.
- The Homebrew-linked `ravel` finds its own matching worker from its real path.
  Generic fake-agent selection and opaque-result round trips work without Grove
  or host Bun/Node, with the copied personal config where applicable.
- Record the published source/version, immutable archive URLs and digests for
  every target, installation prefix and test evidence in the cross-project
  handoff. These are k6's Linux/QEMU dependency inputs where Homebrew is absent.
- If a cut or publication partly succeeds, resume documented unfinished steps;
  never rerun a bumping task. Return here to retire/commit only after delivery
  and installed checks pass.

## Notes

Independent Ravel publication is already in the agreed scope. This leaf does
not release Grove, retire this grove or modify its live ending channel.


## Ready handoff — ravel-delivery-k4 closed

Consume sealed Ravel main `64b1e6e7b92ebd3eac28c9765e6c705d40c3e95d` (clean empty
successor). Both k24 and k25 are terminal; all of k4's preparation criteria are
met. Read Ravel's `docs/release-lifecycle-evidence.md` and `docs/RELEASING.md`,
plus k25's terminal handoff, before a public cut. This source owns the tested
independent Taskfile lifecycle; cargo-release 1.1.2 is its validated version.

All current three-target 0.0.0 archives/floor/manifest/formula and 134 runtime
checks passed. Exact fresh local digests and individual routes/logs are in that
evidence; these are preparation artifacts, not publishable inputs. Version is
still 0.0.0, with no local/public version cut. Remote recheck shows only
structured-plan-migration; first minor remains 0.1.0. Recheck tags/partial state
again, then run the actual `task release:minor` from Ravel's default colocated
workspace, rebuilding/smoking every tagged artifact before any publication.

All prerequisites were available at handoff. The tap is clean at e7416e57,
ravel-lite.rb intact, ravel.rb absent; copied owner configuration and original
installed 22.1.0 Grove/dispatch are individually digest-verified unchanged.
The lifecycle tests cover missing prerequisites, dirty/detached/diverged taps,
smoke ordering, notes/replacements, partial-cut refusal and tap-only recovery.
If publication partly succeeds, resume the documented boundary without another
bump. This leaf owns the real GitHub/tap/installed proofs and immutable URLs,
not another release-instrument development increment.

## Decisions (running log)

- 2026-10-13: Rechecked Ravel's clean source at sealed main
  `64b1e6e7b92ebd3eac28c9765e6c705d40c3e95d`; local and remote tags contain only
  the historical structured-plan-migration tag, with no GitHub releases or
  installed ravel keg. The synchronized tap passes preflight. Doctor passes
  every prerequisite with Homebrew Bash 5.3.20. Preserve the existing reviewed
  Unreleased prose and execute the authorized first minor release task.
  Resume individual documented steps if any cut boundary partly succeeds.
- 2026-10-13: The task passed all 134 runtime tests and cut local v0.1.0 at
  `bbab823803bd2f2796f8bd9570f732dce900efa2`, preserving the reviewed release
  notes under the version heading and seeding an empty Unreleased section.
  A cut now exists: any failure must resume from this identity, never rerun
  the bumping task. Fresh tagged archive builds are in progress.
- 2026-10-13: All fresh tagged target/floor/formula checks passed. The task
  stopped before publication because main@origin was non-tracking in Ravel's
  jj state. Confirmed main is the intended cut and the remote has no version
  tag; track the existing remote bookmark and resume only the main/tag pushes,
  publisher and installation under the documented partial-release procedure.
  No source/instrument change or repeated bump is needed.
- 2026-10-13: Main/tag/GitHub/tap publication succeeded. All three authenticated
  downloads exactly match the smoked archives and GitHub SHA-256 values; notes
  match the versioned changelog. Homebrew's anonymous fetch fails with 404:
  Linkuistics/Ravel is PRIVATE. Asked the owner before changing visibility,
  because that exposes retired history as well as the new release. Continue
  reversible installed checks from the verified cached archive while awaiting
  that decision; public accessibility is still unresolved.
- 2026-10-13: The cached Homebrew install and brew test passed at
  `/opt/homebrew/Cellar/ravel/0.1.0`. Its linked front and all nine installed
  bin/libexec files match the release archive. The owner policy passed all 81
  shell fake-agent sessions and three refusals with no Grove/Bun/Node on the
  child PATH; a deliberate argument mismatch control failed as expected.
  All protected config/tap/original Grove subjects match; brew update changed
  148 Homebrew Ruby subjects, separately recorded. Remote tap formula bytes
  match the validated formula at tap commit
  `bd5aa37b721aaff4a72b01fb966785392812b6a8`. Durable evidence is prepared at
  `/Users/antony/Development/Ravel/docs/publication-evidence.md`. Keep this leaf
  live until the owner resolves the private repository/public URL mismatch.
- 2026-10-13: The owner explicitly approved making Linkuistics/Ravel public,
  including its full history and retired branches. Apply that visibility
  change to the existing release, then verify anonymous bytes for every target
  and Homebrew's actual fetch without another cut or archive rebuild.
- 2026-10-13: Public visibility is confirmed. Every anonymous archive download
  matches the tagged/tested bytes and GitHub SHA-256; Homebrew's forced public
  fetch and fresh brew test pass. Sealed/pushed Ravel evidence main is
  `3916a4a1d17fef925db7e9c730bd4c23d6df3ac8`, followed by a clean empty change.
  All k11 acceptance criteria are met; retain the v0.1.0 identity and original
  installed Grove/dispatch pair for the continuing driver.

## Handoff

Ravel's first public release is **0.1.0**, source/peeled tag
`bbab823803bd2f2796f8bd9570f732dce900efa2`; annotated tag object
`7a1415fdfb5d9cc87e1e0801d81f8ced43756a06`. Public release:
https://github.com/Linkuistics/Ravel/releases/tag/v0.1.0.
Sealed/pushed main `3916a4a1d17fef925db7e9c730bd4c23d6df3ac8` adds durable
`docs/publication-evidence.md` to that exact release source and has a clean
empty successor. Tap main is synchronized and clean at
`bd5aa37b721aaff4a72b01fb966785392812b6a8`; only Formula/ravel.rb was added.

The version-specific dependency URLs and SHA-256 pins are:

| Target | Archive URL | SHA-256 |
|---|---|---|
| aarch64-apple-darwin | https://github.com/Linkuistics/Ravel/releases/download/v0.1.0/ravel-v0.1.0-aarch64-apple-darwin.tar.xz | `7e33ae0c56a93e6a5cff256c5a8a981dec4eb4d114c1f636ce8ee23d1f5d1059` |
| aarch64-unknown-linux-gnu | https://github.com/Linkuistics/Ravel/releases/download/v0.1.0/ravel-v0.1.0-aarch64-unknown-linux-gnu.tar.xz | `19a20dc895a09440cbe3d8da1ce80b96311fbc1c3e6be5e2bf1eb18bb6cc0f7e` |
| x86_64-unknown-linux-gnu | https://github.com/Linkuistics/Ravel/releases/download/v0.1.0/ravel-v0.1.0-x86_64-unknown-linux-gnu.tar.xz | `b9e3443dc428923d6ab45dff7d3837def16bd36985ddd5900f75489a7a4b96e8` |

Both authenticated and anonymous downloads were compared byte-for-byte with
the fresh tagged archives after all manifest/formula/native/QEMU/glibc/CPU-floor
checks passed. GitHub's digests and the remote tap formula match. These pins
must be checked by k6's Linux/QEMU consumers; GitHub reports immutable=false,
so the published release itself has no repository-enforced lock. The source
identity, version URL and digest together fix each dependency's tested bytes.

Homebrew-linked `/opt/homebrew/bin/ravel` resolves to
`/opt/homebrew/Cellar/ravel/0.1.0/bin/ravel` and its matching
`libexec/ravel/ravel-policy`. All nine installed files match the archive.
The public Homebrew forced fetch and brew test pass. All 81 owner-policy shell
fake-agent sessions and three refusals pass with no Grove/Bun/Node on the
child PATH, including ordinary/empty/multiline/flag-like opaque results and
workspace grants. No model launched. Full runtime checks passed 134 tests.

Original Grove/grove-llm/harness-dispatch still resolve to their 22.1.0 keg
with all protected bytes unchanged, as do copied owner configuration and all
pre-existing tap formulas including ravel-lite. Release version/changelog
changes and brew update's 148 Homebrew Ruby subject changes are individually
classified in the recorded digest comparison. Cleanup was disabled.

Complete logs and per-file/per-session evidence are enumerated in Ravel's
`docs/publication-evidence.md` under the sealed source above. The task required
two documented recoveries: tracking the existing jj main@origin bookmark after
the first push refusal, then changing the private repository to public on the
owner's explicit approval after Homebrew returned 404. No second bump, source
fix, archive rebuild or republished tag ran. No partial release step remains.
Return to the continuing Grove work; this is no Grove release or teardown,
and this session retains the original harness-dispatch exit ending.

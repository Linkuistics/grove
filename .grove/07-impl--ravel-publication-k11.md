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

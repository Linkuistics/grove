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
  Generic fake-agent selection, run and record cases work without Grove or host
  Bun/Node, with the copied personal config where applicable.
- Record the published source/version, immutable archive URLs and digests for
  every target, installation prefix and test evidence in the cross-project
  handoff. These are k6's Linux/QEMU dependency inputs where Homebrew is absent.
- If a cut or publication partly succeeds, resume documented unfinished steps;
  never rerun a bumping task. Return here to retire/commit only after delivery
  and installed checks pass.

## Notes

Independent Ravel publication is already in the agreed scope. This leaf does
not release Grove, retire this grove or modify its live ending channel.

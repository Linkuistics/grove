# promotion-controls-decision-k336

## Goal

Run the six promotion controls against the rehearsal instrument, see each
fail, and decide whether finish can promote.

## Context

- The node brief, `_promotion-upgrade-rehearsal.md`, and the instrument and
  record of k333, k334 and k335.
- `docs/specs/coordinated-release.md`, "Acceptance and limits" and the last
  paragraph of "Compatible tap states".

## Done when

- Each control is seen to fail. The controls are a wrong channel, a
  `versioned-2` cut after promotion and mismatched archives. They are also
  silent schema-3 reuse, a removed retained resource and promotion before
  compatible Grove.
- The record states the decision: proceed to promotion in finish, or stay in
  state C. It gives the evidence for that decision.
- A stay in C cuts the packaging repair as live leaves before
  cross-repo-docs-reconciliation-k319.
- The record is committed, and the parent node closes.

## Decisions (running log)

- The six controls run as one new section of `scripts/promotion-rehearsal.sh`,
  after upgrade-a-d and before the protected-subject digest. They reuse the
  run's scratch origin, its Ravel clone and its scenario prefixes, so the
  digest covers them. Each control drives the real Ravel entry point that
  owns the guard:
  - Wrong channel: `task --dry release:patch` with no or an unknown
    `CHANNEL`, and `release-prepare.sh --channel unversioned` at state C.
  - `versioned-2` after promotion: `release-prepare.sh --channel versioned-2`
    at state D.
  - Mismatched archives: `release:promote` with three faults. The receipt's
    digests are not the cut's. The evidence's digest is not the public
    asset's. The tap's `ravel@2.rb` names other archives. The instrument also
    checks that `ravel.rb` and `ravel@2.rb` at D name the same archives.
  - Silent schema-3 reuse: a `SCHEMA_VERSION = 3` copy of the working
    `config.rhai` at D, with and without `"skills": {"sources": []}`.
  - Removed retained resource: `release-install.sh` at D, with a `brew`
    wrapper that removes the 1.1.0 keg's `libexec/ravel/ravel-policy` after
    the real upgrade. Its `same_keg` check must fail too.
  - Promotion before compatible Grove: `release:promote` with no receipt, and
    with the tap at state B under a receipt that names the candidate.
  Each refusal must leave the scratch origin, the tap clone and Ravel's
  `main` unchanged. A stub `brew` proves that the early refusals ran no
  Homebrew command. A paired command in the right state shows that each
  channel refusal comes from the channel, not from the environment.
- A pre-test of the probes found that the instrument's 1.1.0 front check is
  blind to a removed worker. The front still reports 1.1.0 and refuses
  `end-session`, but a launch refuses with `worker_missing` and exit 5. The
  keg manifest is the check that catches it.
- `release-prepare.sh --channel unversioned` at state C refuses with the
  baseline message "tap Formula/ravel.rb is not at v2.0.0 … resume formula
  publication". Ravel's own fixture expects that text. The control accepts it,
  because `release-publish.sh` refuses an unversioned 2.x publication again
  with the channel message, as Ravel's fixture shows.
- The measured run passed on 2026-11-12 with the instrument at
  `de41b99e33d3…`. Each of the six controls refused through its real Ravel
  entry point and left the scratch origin, its tap clone and the Ravel
  clone's `main` unchanged. After the run, only the summary message of
  `untouched` changed, because the stub `brew` is all that it checks.
- Decision: proceed to promotion in finish. Every rehearsed route to D gives a
  usable pair, every real upgrade kept the retained kegs, and every control
  fails. The defects found are in documentation, not in packaging, and
  homebrew-tap-trust-docs-k337 and promotion-upgrade-remedy-docs-k338 own
  them before finish. The spec requires the remedy "upgrade Grove first" to
  be published before promotion, so k338 is a precondition of the promotion.
  Ravel's readiness check binds the receipt to the tap's Grove version only.
  finish-handoff-k324 must therefore state that finish writes the receipt
  after the installed verification of the released Grove's three routes.

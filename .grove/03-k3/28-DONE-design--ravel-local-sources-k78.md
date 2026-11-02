# ravel-local-sources-k78

## Goal

Narrow the skill-discovery contract of Ravel to local sources. Record why
Ravel does not fetch, install or update a source. Settle the permanent outcome
of a settings record with `type: git`.

## Context

- During ravel-git-acquire-k77 on 2026-11-02, the human chose to narrow Ravel
  to local sources. Their words: "The process of 'installing' skills and
  keeping them up to date should not be part of Ravel."
- Work in the workspace `Ravel.skill-directory-delivery`. The unpushed bookmark
  `skill-directory-delivery` is at `c2e24d371db2`. That commit backs out the
  Git acquisition of ravel-git-acquire-k70, and its tree equals the local
  stage `1d50dae5d29e`.
- `docs/specs/skill-discovery.md` still states the pinned-Git contract. The
  affected parts are the `git` settings record and "Exact Git acquisition and
  offline reuse". They also include `skill_revision_unavailable`, the `git`
  descriptor record and the `reused` and `acquired` dispositions. The Git
  acceptance rows and the Homebrew Git decision are affected too.
- The local build refuses a Git source with status 3 and no stable code. The
  local stage made that refusal temporary.
- The running log of ravel-git-acquire-k77 holds the six reproduced defects of
  the Git path. They are evidence for the decision record.

## Done when

- The specification, its design evidence and `CONTEXT.md` describe local
  sources only. No clause requires Ravel to run Git.
- The specification states the public outcome of a `type: git` record: the
  code, the stage, the exit status and the remedy.
- A Ravel ADR records the decision, the rejected Git acquisition and what
  would reopen it.
- The Ravel change is committed through jj, and the bookmark advances without
  a push. Record the output commit here and in the Grove commit.

## Notes

This leaf is design only. skill-loading-replan-k80 grows the implementation.
That implementation removes the temporary refusal, its fixture and the Git
parts of the book and `CHANGELOG.md`. Earn a focused review if the change of
the specification is not mechanical.

## Decisions (running log)

**Base.** On 2026-11-02 the workspace `Ravel.skill-directory-delivery` had an
empty working copy on `c2e24d371db2`, the bookmark `skill-directory-delivery`.
The local build at that tree refuses a valid Git record in `refuse_git`, with
the internal code `skill_git_unsupported` and status 3. It parses and checks
the Git fields first, so a malformed Git record refuses as `settings_invalid`
at that field.

**Outcome of a `type: git` record.** `type` accepts only `local`. Any other
value, `git` included, refuses as `settings_invalid`, stage `cli`, exit 2, with
the location `skills.sources[N].type`. Ravel checks `type` before the other
fields of the record. The remedy tells the owner to put a checkout on disk
with a tool outside Ravel and to name its directory as a `local` source. The
refusal comes before capture, cache work and the worker, at both public
boundaries.

Reason: no release accepted a Git source, so no published settings
file holds one. The closed schema already refuses a malformed value, and one
rule covers every unknown type with no special case.

**The `type` field stays.** It is required, with the one value `local`. The
descriptor and request records keep `type: "local"`, and the notice keeps
`disposition: "captured"`. The local build already writes those shapes, so the
implementation only removes Git code. Dropping `type` would change three
public shapes and give a former Git record a generic unknown-field remedy.

**Local behaviour stated without Git.** Delivery runs no VCS command and makes
no network request. A checkout is an ordinary local source.

Capture leaves out the reserved entries, includes uncommitted changes and
records no VCS revision. A submodule directory is captured as its files,
without its reserved entry. An LFS pointer is captured as its bytes.

These sentences describe the local build as it is. The tests
`skills_resolve.rs`, `skills_lifetime.rs` and `skills_bounds.rs` cover the
reserved entries, the LFS pointer, quiet manual removal and cancellation
during capture.

**Scope in Grove.** Three Grove documents restate the withdrawn Ravel Git rules
as current requirements, and no sibling leaf owns them. This leaf reconciles
them: the delivery specification `codex-skill-provisioning.md`, the stage list
of `coordinated-release.md` and two rows of the evidence documents. The
"source-pack pins" of the release order are the owner's pack revisions, so
owner-pack-checkouts-k79 keeps them. `owner-skill-delivery.md` also stays with
k79.

**Review.** The specification change is mostly a removal. It also adds the
type refusal, one acceptance row and a wider corrupt-object rule. This leaf
therefore spent its one in-session reviewer, not a review leaf. The reviewer
read the Ravel and Grove diffs and the build code, without my conclusions.

**Triage of the eleven findings.** All eleven are valid. One is an unclear
contract, and the other ten are actionable defects:

- 1: the lifetime view kept "exact commit" and "configured pin". My pattern
  sweep did not list those words.
- 2: the `--config` clause described a check of a stored origin record. The
  build checks each live source against the entry directory before capture.
  The public test in `skills_launch.rs` proves it, with a control.
- 3 and 8: the status named only the `git` gap. The build also gives other
  types the generic remedy, and the book and changelog still describe Git.
- 4: the checkout remedy misleads for a typo. My change of "revision" to
  "source" in the failures section also made a conflict.
- 5, unclear contract: the repeated-member check of the whole file comes
  before the type check.
- 6 and 7: the record said all six findings changed the fetch, left out the
  `ext` allow setting and linked the wrong section.
- 9 and 10: Grove kept "revision" and "acquisition", and credited
  `822c0ed2` with the narrowed set.
- 11: the handoff left out two Git branches in code and a remedy assertion.
  The local stage record still named the pinned stage.

Each fix is a wording correction inside the settled rule. The code, stage,
exit and location of the refusal do not change. A public test covers fix 2. No
fix needs a second review.

**Checks.** After the last edit, both prose checkers found no hard finding in
any changed file, and planted controls failed each one. The scratch link check
resolved every local link in the changed Ravel documents. A stale Git anchor
failed it.

The vocabulary sweep took each Git word of the base specification
that the new one lacks. It found only the removal itself, unrelated senses and
historical records. The same sweep found the old lifetime labels in the base.

Both changed views rendered in a browser with their new labels. Grove's
document tests passed. They are `reference_navigation`,
`corpus_exception_inventory`, `native_skill_delivery` and `no_kind_admission`.
They also include `help_surfaces`, `composition_guidance` and the
`grove-loop` prompt tests. No product test ran.

**Ravel output.** The prose conversion is commit `31e32945544e` above
`c2e24d371db2`. The narrowing is commit
`7a894c7820372b753775ce1c6778023b26f99fb4`, and the local bookmark
`skill-directory-delivery` now points at it. Nobody pushed it. Ravel `main`
stays at `24a545236185`.

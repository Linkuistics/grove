# ravel-git-acquire-k77

**Integrates:** ravel-git-acquire-k76

## Goal

Triage the findings of `ravel-git-acquire-k76` against the online Git
acquisition implementation. Apply the valid findings, record the disposition
of each one, and verify the resulting public behaviour.

## Context

- The review names Ravel commit `1898dc8603a0` and specification `822c0ed2`.
  Read its findings and evidence from the committed review leaf.
- Work in `Ravel.skill-directory-delivery`, on the unpushed bookmark
  `skill-directory-delivery`. Follow that repository's instructions.
- This integration runs before `ravel-git-offline-k71`. Offline reuse,
  transport bounds and confinement keep their existing later leaves.

## Done when

- Each review finding has a reasoned disposition. Valid findings are fixed
  with meaningful verification at the applicable public seam.
- Affected documentation and checks agree with the resulting behaviour.
- The Ravel result is committed through jj and the local bookmark advances
  without a push. Record its output commit here and in the Grove commit.

## Notes

The review's counterexamples were derived from source. It ran no
reproduction, test, build, lint or format command. This integration owns
reproductions, fixes and post-fix verification against the supported Git
executables; it can reject a finding with evidence.

## Decisions (running log)

**Reproduction.** On 2026-11-02 a scratch probe ran each counterexample with
an isolated HOME against Apple Git 2.54 and Homebrew Git 2.56. Both behave
the same unless this log says otherwise.

**F1 is a real issue.** Git fetched the protected bare repository through four
routes. They are a percent-encoded `file://` locator and a missing `alias` with
a symlink `alias.git`. They are also a directory with a `.git` file that names
the repository, and an HTTPS locator that a global `insteadOf` rewrites.

A `/~` path does not expand on either version, so it is not an added route.
Fix: interpret a local locator as Git does. Test every path that Git can open
from it. Test again after Git's rewrite rules.

**F2 is a real issue.** The parser admits a query and a fragment, and its
refusal echoes the complete URL. Fix: refuse `?` and `#` in every URL with a
scheme, and do not echo the value.

**F3 is a real issue.** A global `protocol.ext.allow=always` with a rewrite to
`ext::` ran the helper. A command-line `protocol.ext.allow=never` stops it.
Fix: deny each protocol that Git knows other than the three, and refuse an
effective locator of any other shape. A sibling route is a global remote
section whose name is the locator, since Git treats the locator as a remote
name. Ravel refuses that too.

**F4 is a real issue.** A global `fetch.bundleURI` imported the pin from a
bundle when the configured repository did not hold it. The advertised-ref
fetch then succeeded. A command-line `fetch.bundleURI=` disables it with no
warning on both versions.

**F5 is a real issue.** The cause is wider than the hook. Git removes
`GIT_CONFIG_PARAMETERS` before it starts a local `upload-pack`, so no
command-line setting of Ravel reaches the local server side. Fix: for a local
locator, run `upload-pack` with no system or global configuration. The local
server side needs no credential. Trade-off: a `safe.directory` entry in the
owner's global configuration no longer applies to a local source repository.

**F6 is a real issue.** Protocol 0 reports "Server does not support this
repository's object format" in both directions, and Ravel gives the transport
remedy. Fix: match whole lines that the Git client prints, in the three known
forms. Server text arrives with a `remote:` or `remote error:` prefix. Raw SSH
stderr can still imitate a line, which changes only the remedy.

**Human decision.** The fixes above were half written when the human asked
why Ravel works with Git at all. I gave two options with a recommendation.
The human chose to narrow Ravel to local sources. Their words: "The process
of 'installing' skills and keeping them up to date should not be part of Ravel."

The planned
fixes are therefore not applied. Removal of the Git path resolves all six
findings, because no Git child remains.

**Ravel output.** A jj backout of `1898dc8603a0` is commit
`c2e24d371db26a8729349f35df3fae76479d67c7` on the unpushed bookmark
`skill-directory-delivery`. `jj diff` from `1d50dae5d29e` to it is empty. The
control diff from `1d50dae5d29e` to `1898dc8603a0` shows 13 files. The tree is
therefore the reviewed local stage, and the checks of ravel-local-handoff-k61
cover it.

I discarded the partial fixes with `jj restore` before the
backout. Ravel `main` stays at `24a545236185`.

**Tree changes.** The human's choice authorised the prune of
ravel-git-offline-k71, ravel-git-bounds-k72, ravel-git-confinement-k73,
ravel-git-install-k74 and ravel-pinned-handoff-k75. Each one served only Git
acquisition. The root brief now records the decision and supersedes the
pinned-Git clauses below it. Three leaves precede grove-dual-ravel-k27:
ravel-local-sources-k78, owner-pack-checkouts-k79 and skill-loading-replan-k80.
The ADR of the decision belongs to k78, in the Ravel set.

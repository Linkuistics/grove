# In-repo issue trackers — post-mortem survey

A focused review of prior systems that embedded an issue-tracker-lite, work
queue, or collaborative annotation store **inside the git repo alongside the
code**, organised around **what went wrong** after contact with real
multi-person, multi-machine, multi-year use.

This is a sibling, not a replacement, of `seed-capture-prior-art.md`. The
prior survey mapped the paradigm space to pick a shape; this one stress-tests
the shape grove has now agreed on (markdown files on a dedicated `grove-meta`
branch, CLI-mediated writes, optional remote sync — see ADR-0002 and
ADR-0003) by reading the post-mortems of similar designs.

The audience is four open planning leaves:

- **060 — sync semantics + inbox shape** (remote-sync semantics; single
  file vs directory-of-files).
- **070 — `grove meta` rename + `meta init`** (branch architecture
  + materialisation).
- **080 — LLM/CLI boundary audit** (which deterministic steps belong in
  CLI verbs).
- **090 — TUI navigator** (whether and how to ship an interactive view).

Each system section ends with a *takeaway for grove* pointing at the leaf(es)
its lesson informs. The final **Synthesis** section answers the
leaf-specific questions in one place.

---

## git-bug

**One-paragraph summary.** [git-bug/git-bug](https://github.com/git-bug/git-bug)
(MichaelMure, since 2018, ~9.8k stars, Go) stores issues as op-chains under
`refs/bugs/*` rather than as files in the working tree. Capture via
`git bug add`; sync via explicit `git bug push/pull`. Architecturally it is
the closest mainstream precedent for what grove is building — same
"auxiliary data on dedicated refs in the same repo, CLI-mediated" shape —
and the most operationally instructive: it has demonstrated the adoption,
sync, and tooling-fragility headwinds the grove design must plan for.

**Architecture (one screen).** Entities live under `refs/<namespace>/<id>`
(`refs/bugs/*`, `refs/identities/*`), not on a working branch. Each entity
is a chain of `OperationPack`s — JSON blobs aggregated under git Tree objects
(`/ops`, `/media`). Concurrent edits form a DAG; merge order is resolved by
Lamport-clock values encoded in tree-entry names, lexicographic ID as
tiebreaker. Pushes are fast-forward where possible; on divergence git-bug
constructs the equivalent of a merge commit joining the two op-chain heads.
Sync is explicit: `git bug push` / `git bug pull` (plus `git bug bridge pull`
for GitHub/GitLab/Jira mirroring). Sources:
[`doc/design/data-model.md`](https://github.com/git-bug/git-bug/blob/master/doc/design/data-model.md),
retrospective issue
[#212](https://github.com/git-bug/git-bug/issues/212).

**Failure modes.**

- **Multi-writer merge is the headline pitch but the least-exercised path.**
  The maintainer's own 2019 retrospective admits the centralized topology
  was the only well-exercised one: "this method works well with a
  centralized sharing topology… it's much less clear to me what happens
  with multiple repositories push/pulling on each other"
  ([#212](https://github.com/git-bug/git-bug/issues/212)). Early versions
  enforced ff-only and required pull-then-replay before push. No primary
  report of a wedged real-world merge was found in three searches, but the
  threat-model doc is itself the signal: hub-and-spoke is the trusted path;
  mesh is aspirational.
- **Adoption drop-off.** Sustained adoption outside git-bug's own repo is
  essentially absent. HN 2022 thread consensus:
  "around 2010–2015 there were a whole bunch of these distributed git
  bugtrackers and none of them took off" ([HN
  33730417](https://news.ycombinator.com/item?id=33730417)). No primary
  source documenting a well-known third-party project standardising on
  git-bug.
- **Tooling fragility.** [#178](https://github.com/git-bug/git-bug/issues/178)
  — bare repos initially rejected despite `refs/bugs/*` being present;
  [#1335](https://github.com/git-bug/git-bug/issues/1335) — `git bug push`
  fails when SSH config differs from what git-bug introspects; identity
  edge cases pile up in
  [#130](https://github.com/git-bug/git-bug/issues/130),
  [#1003](https://github.com/git-bug/git-bug/issues/1003),
  [#1139](https://github.com/git-bug/git-bug/issues/1139).
- **Sync semantics in real use.** Strongest signal: the project's own
  dogfood ticket
  [#1221](https://github.com/git-bug/git-bug/issues/1221). The most active
  contributor besides the maintainer writes (May 2026): "most of my
  commenting and issue opening/closing occurs offline… periodically, i do
  need to go to github.com… my prior comment about automating it is simply
  about adding a pipeline that runs something like `git bug bridge pull &&
  git bug push` on a schedule." Translation: even maintainers don't trust
  themselves to remember to sync; without scheduled automation, edits
  silently stay local. No post-commit-hook recipe ships with the tool.
- **Opacity / walk-away.** Data is JSON in git tree objects under
  `refs/bugs/*` — invisible to `grep` on a working checkout, invisible to
  GitHub's web UI, only readable through git-bug itself or by walking refs
  with `git cat-file -p`. The schema is documented, so a determined user
  *could* read it raw, but no script-your-own-reader recipe surfaces in
  search. No primary report of users losing access after abandoning the
  tool — but adoption is too low for that failure mode to have been
  road-tested.

**Walk-away check.** Partial. `refs/bugs/*` are normal git refs; the schema
is documented; `git for-each-ref refs/bugs/ | xargs -n1 git cat-file -p`
recovers the data even with git-bug uninstalled. But grep-on-checkout sees
nothing, GitHub renders nothing, and most users would have no idea the refs
exist. The walk-away is technical, not ergonomic.

**Takeaway for grove.** Two leaves. **060 (sync semantics)** is the
highest-leverage lesson: even git-bug's own maintainers admit they forget to
push/pull and want a cron — grove must ship the scheduled-fetch story in
the box, not as a TODO. **080 (CLI-boundary audit)** is next: git-bug's
identity, bare-repo, and SSH-config edge cases all stem from coupling CLI
assumptions to working-tree state; grove's dedicated-branch inbox should
make those CLI assumptions explicit. Secondary signal for **090 (TUI)**:
git-bug shipped TUI + Web UI + CLI and the maintenance backlog
([#1213](https://github.com/git-bug/git-bug/issues/1213),
[#1533](https://github.com/git-bug/git-bug/pull/1533)) shows the cost of
plural front-ends on a tiny maintainer team.

**Maintenance status as of 2026-05.** Active but thin. Default branch
`trunk`; last push 2026-05-24; last release v0.10.1 on 2025-05-19; 165 open
issues; 9.8k stars. After a 2.5-year gap between v0.8 (Nov 2022) and v0.9
(May 2025) the cadence resumed but is dominated by housekeeping. Bus-factor
is still effectively one maintainer plus `sudoforge`.

---

## Fossil tickets / wiki

**One-paragraph summary.** Fossil (D. Richard Hipp / SQLite team) ships
tickets, wiki, forum, and chat in a single binary, persisting all of them
in one SQLite repository file. Tickets are not files in the source tree;
they live as immutable "change artifacts" in the SQLite blob table and
current state is computed by replaying every artifact for a ticket-id in
timestamp order ([bugtheory.wiki](https://fossil-scm.org/home/doc/tip/www/bugtheory.wiki),
[tickets.wiki](https://fossil-scm.org/home/doc/tip/www/tickets.wiki)). The
design has held internally (SQLite, Tcl/Tk, pikchr genuinely use it) but
has not crossed back into git-land.

**Architecture (one screen).** A repository is one SQLite file (`*.fossil`).
All durable data is a row in the `blob` table, zlib-delta-compressed,
addressed by SHA hash. Auxiliary tables are derived (re-crosslinkable)
views. Sync ships artifacts; ticket "conflicts" don't exist because every
change is just another append. Tickets are deliberately *not* in the
working tree — Hipp rejected that because checkins are immutable so
tickets-as-files couldn't be added to past checkins, and the volume would
clutter source.

**Failure modes.**

- **Clock-skew dependency.** "If a timestamp on a ticket change artifact
  is off by months or years, it can seriously confuse the replay algorithm
  for determining the current ticket state"
  ([tickets.wiki](https://fossil-scm.org/home/doc/tip/www/tickets.wiki)).
  Append-only-in-timestamp-order is fragile to misconfigured machines.
- **Web-UI primacy.** "Meant to be managed from the web interface"; the
  CLI is an afterthought. Long-standing threads on fossil-users from
  people trying to drive tickets from the command line
  ([fossil-users: ticket system from command line](https://fossil-users.fossil-scm.narkive.com/TEfqOy24/using-ticket-system-from-command-line)).
- **Wiki/ticket boundary leaks.** Automatic cross-linking was rejected as
  ambiguous; users invented ad-hoc conventions
  ([fossil-users: ticket link in the wiki](https://fossil-users.fossil-scm.narkive.com/vffTocVR/how-to-create-a-ticket-link-in-the-wiki)).
- **Moderation/sync interaction.** Unmoderated changes do not sync until
  accepted ([changes.wiki](https://www.fossil-scm.org/index.html/doc/bf1b9972/www/changes.wiki))
  — a "I wrote it, why doesn't anyone see it?" failure mode specific to
  the integrated model.
- **No spread back to git-land.** Despite the well-known Hipp argument,
  git-bug and successors all chose plain-files-in-refs over Fossil's
  SQLite-blob approach. Public comparisons frame Fossil as
  "violates the unix philosophy / does more than is needed"
  ([HN 11428973](https://news.ycombinator.com/item?id=11428973)).
- **Opaque store.** Ticket data is zlib-compressed delta-encoded artifacts
  in a SQLite blob table; the format is documented but reading without
  `fossil` means re-implementing crosslink/replay. No widely-used
  third-party reader exists.

**Walk-away check.** Partial. The SQLite file opens in any SQLite tool but
tickets need the crosslink/replay algorithm to render. Code checkouts
survive (`fossil export` to git is routine).

**Takeaway for grove.** The append-only-replay model is mathematically
clean but binds correctness to clock truth and a specific replay
implementation — argues *against* leaf **060** adopting a log-of-deltas
inbox; whole-file-per-item is more walk-away-able. The wiki/ticket
cross-link confusion argues for leaf **080** making the CLI surface a
single noun (`inbox`) rather than splitting kinds. The CLI-as-afterthought
story argues for leaf **080** treating CLI parity as a P0.

**Maintenance status as of 2026-05.** Healthy. Fossil 2.26 (2025-02);
SQLite, Tcl, Tk, pikchr continue to use tickets in production
([core.tcl-lang.org/tk/reportlist](https://core.tcl-lang.org/tk/reportlist),
[sqlite.org/src/rptview](https://sqlite.org/src/rptview?rn=1)).

---

## bugs-everywhere (BE)

**One-paragraph summary.** Chris Ball / W. Trevor King's polyglot
DVCS-agnostic bug tracker. Bugs as UUID-named directory trees in `.be/`.
Real ambition (LWN coverage, FSF directory, Debian packaging) but its
primary hosting on Gitorious died with Gitorious, the maintainer drifted,
and the current mirror's last real commit is Nov 2016.

**Architecture (one screen).** `.be/<dir-uuid>/bugs/<bug-uuid>/values` plus
`comments/<uuid>/body`. Each bug is a directory tree of small text files,
one key per file (`status`, `severity`, `summary`, `assigned`). No central
index — listing walks the tree. Globally unique UUIDs, not monotonic.
Capture: `be new "summary"`. Sync: whatever the host VCS does.

**Failure modes.**

- **Severity-merge conflicts hand-edited.** "Changing the severity of a
  bug in two branches and merging the result creates a conflict… resolved
  by hand-editing"
  ([LWN, Distributed bug tracking, 2008](https://lwn.net/Articles/281849/)).
- **Wrong-branch loss and commit pollution.** "I have managed to file
  bugs to incorrect branches and completely loose them," plus complaints
  about "a lot of mess BE did in my commits"
  ([Cepl, Current state of distributed issue tracking](https://matej.ceplovi.cz/blog/current-state-of-the-distributed-issue-tracking.html)).
- **Non-developer adoption wall.** "Most users are unlikely to be impressed
  by a message like 'set up a git repository and run these commands to
  file or comment on a bug'" ([LWN, ibid.](https://lwn.net/Articles/281849/)).

**Why it died.** Mostly social and infrastructural. Hosting collapse
(Gitorious 2015) removed the centre of gravity; the maintainer moved on.
Design did contribute: many-small-files-per-issue produced noisy commits
that polluted the code log — Cepl cites this explicitly.

**Walk-away check.** Excellent. Plain-text key/value files. `find .be -type
f -name body | xargs cat` and `grep -r severity .be/` both work.

**Takeaway for grove.** The combination "many small files per item +
working-tree resident + commit-as-publish" pollutes the code's `git log`.
Grove already dodges this by putting the inbox on a *dedicated branch*
rather than the default branch (ADR-0002). The lesson is to *protect* that
choice — leaf **060** must keep inbox commits off the default branch even
when it gets tempting; leaf **080** should bundle add+commit into the
capture verb so users never half-capture.

**Maintenance status as of 2026-05.** Dead. Last meaningful commit Nov 2016
on `aaiyer/bugseverywhere`.

---

## ditz

**One-paragraph summary.** William Morgan's Ruby tracker (2008). One YAML
file per issue in a `bugs/` directory plus a single `bugs/project.yaml`.
Strongest Unix-y appeal of the four abandoned trackers — clean
line-oriented YAML, no VCS coupling, Debian-packaged — and the most
pointed retrospective from its own maintainer base: "I still love ditz
and I'm totally not using it."

**Architecture (one screen).** `bugs/project.yaml` (releases + components)
plus `bugs/issue-<sha1>.yaml` (one file per issue, line-oriented YAML so
per-field diffs work). Capture: `ditz add` drops into an interactive prompt
sequence (title, description, type, component, release). Sync: none of its
own — issues are just files under whatever VCS you use.

**Failure modes.**

- **Interactive-capture friction.** "I dislike ditz, because it is
  constantly nagging me with questions"
  ([Zwinkau](https://beza1e1.tuxen.de/articles/later_bug_tracker.html)).
  Canonical anti-pattern for mid-flow capture.
- **"Done = dead with fewer bugs."** "The development status of ditz is
  'done', which is similar to 'dead', but with less bugs"
  ([Zwinkau, ibid.](https://beza1e1.tuxen.de/articles/later_bug_tracker.html)).
- **Maintainer attrition.** "Ditz is kind of abandonware right now as the
  original author has gone on to other things… I still love ditz and I'm
  totally not using it"
  ([Matt Katz, official maintainer](http://www.morelightmorelight.com/2011/03/15/ditz-the-distributed-issue-tracker/)).
- **`project.yaml` central index** is the obvious merge-conflict candidate
  but **no primary source found** explicitly diagnosing this; recorded as
  inferred risk, not documented failure.

**Why it died.** Almost entirely social. PyDitz (a Python re-implementation
on sourcehut) confirms the architecture was fine — the *project* failed,
not the *model*.

**Walk-away check.** Best of the four abandoned trackers. `cat
bugs/issue-*.yaml`, `grep status bugs/*.yaml`, or open in `$EDITOR`.
Line-oriented YAML diffs and merges cleanly per field.

**Takeaway for grove.** Two strong signals. Leaf **080 (CLI boundary)** —
avoid an interactive capture wizard at all costs; `grove inbox add` must
accept the whole item non-interactively (flags or stdin) so it can be
invoked mid-flow without breaking concentration. Leaf **060 (sync semantics
+ shape)** — `project.yaml` is the warning that *any* centralised index
file becomes a merge-conflict hotspot; prefer derived indexes over
committed ones.

**Maintenance status as of 2026-05.** Dead. `jashmenn/ditz` last push
2011-04-01.

---

## ticgit / ticgit-ng

**One-paragraph summary.** Scott Chacon's 2008 demo of "what you could do
with git" — tickets as files committed to an orphan `ticgit` branch in
the same repo, leaving the working tree untouched. Adoption was always
thin; jeffWelling forked `ticgit-ng` to namespace the gem, then it too
went quiet (last commit Feb 2014). Notably, Chacon **relaunched the name
in May 2026** with a brand-new Rust implementation on top of `git-meta` —
primary evidence that the original architecture wasn't worth iterating on.

**Architecture (one screen).** All ticket data lives on an orphan branch
called `ticgit` (or `ticgit-ng`) — never checked out into the working
tree. Tickets are directories of small files (title, state, comments) keyed
by SHA. Capture: `ti new "title"` does an isolated commit on the orphan
branch. Sync: `git push origin ticgit` — the orphan branch is just another
ref. Chacon's explicit design claim: "there should not ever be any merge
conflicts, since I never edit files"
([Chacon, 2008](https://schacon.github.io/2008/03/23/ticgit-and-a-new-git-gem.html)).

**Failure modes.**

- **Project declared dead on its own wiki.** Cepl, surveying the
  distributed-tracker landscape, notes the GitHub wiki "declares project
  as dead"
  ([Cepl](https://matej.ceplovi.cz/blog/current-state-of-the-distributed-issue-tracking.html)).
- **"Never edit files" doesn't hold up.** Status changes, assignment
  changes, comment edits all eventually mean editing files. The
  conflict-free promise was contingent on never mutating state, which no
  real workflow can sustain.
- **Orphan-branch invisibility.** Tickets are *not* in normal
  `git log`/`git status` output. Workflows feel disconnected from the code.
  Chacon's 2026 README — "Tickets live in the repository as structured
  `git-meta` metadata" — is an implicit retreat from the orphan-branch
  design toward a git-notes-style metadata model, which is itself a
  retrospective.

**Why it died.** Mostly social — Chacon went to GitHub Inc.; jeffWelling's
-ng fork stalled. But there is a technical signal: orphan-branch refs sit
outside the default refspec; `git pull` doesn't bring them. Adoption
required user effort beyond `git pull`. Chacon's 2026 reboot uses a
different data model (git-meta, JSON schema, agent-oriented) — the
strongest possible retrospective: the original author rebuilt from scratch
rather than reviving the codebase.

**Walk-away check.** Possible but awkward. `git checkout ticgit` then
`find . -type f | xargs cat` works, but if you didn't fetch the branch you
can't see anything. After deleting the tool, you need to know the magic
ref name to recover content.

**Takeaway for grove.** The orphan-branch convention is the closest analogue
to grove's `grove-meta` branch architecture and ticgit is its cautionary
tale. Leaf **060 (sync semantics)** must explicitly answer "how does the
user know inbox content exists and how do they fetch it?" — do not assume
`git fetch` brings it; configure a refspec or wrap it in a `grove sync`
verb. Leaf **070 (rename + init)** should make the worktree at
`<repo>/.grove-meta/` the *visible* anchor so the data is not invisible
the way ticgit's orphan branch was. And ticgit's "no edits → no
conflicts" reasoning is the failure pattern to study: as soon as state
mutates, append-only-by-convention isn't enough; mutation needs an
explicit conflict story.

**Maintenance status as of 2026-05.** Original `schacon/ticgit` Ruby
project is dead; `jeffWelling/ticgit` (-ng) last push 2014-02-09. But
Chacon has just relaunched `schacon/ticgit` as a Rust binary `ti` with a
v0.3.1 release on 2026-05-20 using git-meta — explicitly designed for
agentic workflows (`--json`, `--markdown`, `ti agent`). The *name* is
back; the original code is not.

---

## artemis

**One-paragraph summary.** Dmitriy Morozov's Mercurial extension (alpha
Git support). Each issue is a Maildir directory under `.issues/`, metadata
in RFC-822 headers, body in the message. Most obscure of the abandoned
trackers; never had visible adoption.

**Architecture (one screen).** `.issues/<issue-id>/` is a Maildir (a root
email-style message plus `new/`, `cur/`, `tmp/` for comments-as-replies).
Properties live in RFC-822 headers; body is free-form prose. Capture: `hg
iadd` opens `$EDITOR` on a templated message. Sync: `hg push` — issues are
just tracked files. Source:
[Mercurial wiki ArtemisExtension](https://wiki.mercurial-scm.org/ArtemisExtension).

**Why it died.** Almost entirely ecosystem. Last commit 2018-12-14 "update
to 4.8 API" — maintenance commits tracking Mercurial's extension API,
not feature work. Mercurial's broader decline took artemis's host platform
with it. No "we are stopping" notice; just gradual abandonment.
**No primary post-mortem source found** in this round of searching.

**Walk-away check.** Excellent — arguably the best of the abandoned set
for archaeology. `cat .issues/*/new/*` reads issues; any maildir tool
(mutt, mu, notmuch) can browse them; headers are grep-friendly. The format
predates and outlasts the tool.

**Takeaway for grove.** Borrow the format-outlasts-tool insight. Leaf
**070 (CLI boundary)** — inbox items should be readable by *some* widely
deployed external tool even with grove uninstalled. Markdown with optional
frontmatter is exactly this (works in any editor, in `gh issue create
--body-file`, in any markdown renderer). Don't invent a bespoke envelope.

**Maintenance status as of 2026-05.** Dormant. Last code change 2018-12-14.

---

## git-appraise

**One-paragraph summary.** Google's `git-appraise` (Omar Jarjur et al.) is
a fully-distributed code-review system that stores every artefact — review
requests, comments, CI results, robot analyses — as JSON blobs inside git
*notes*, partitioned across `refs/notes/devtools/*`. The design's elegance
is also its fatal flaw: git's defaults do not fetch or push `refs/notes/*`,
GitHub silently strips/hides them, and rebases detach notes from their
commits — so the very feature that makes it forge-agnostic also makes it
invisible to the forges that won.

**Architecture (one screen).**

- `refs/notes/devtools/reviews` — review requests, attached to the first
  revision in a series.
- `refs/notes/devtools/discuss` — human review comments.
- `refs/notes/devtools/analyses` — robot/static-analysis comments.
- `refs/notes/devtools/ci` — CI results.

Each note is one-JSON-object-per-line so git's built-in `cat_sort_uniq`
notes-merge strategy gives idempotent dedupe-merge across writers. Sync is
explicit: `git appraise pull [<remote>]` and `git appraise push [<remote>]`
install their own refspecs because `git fetch`/`git push` do not touch
`refs/notes/*` by default. "Latest wins by timestamp" for review-request
updates; comments and CI accumulate (multi-writer is additive).

**Failure modes.**

- **The `refs/notes` non-default-fetch problem** is real and well-documented:
  Tyler Cipriani's
  [Git Notes: git's coolest, most unloved feature](https://tylercipriani.com/blog/2022/11/19/git-notes-gits-coolest-most-unloved-feature/)
  and Wouter J's walk-through
  ([wouterj.nl/2024/08/git-notes](https://wouterj.nl/2024/08/git-notes))
  both call out that fetching/pushing has to be re-taught to every remote
  and that `--force-with-lease` interacts badly with notes refs
  ("[rejected] refs/notes/commits (stale info)").
- **GitHub renders nothing for `refs/notes/*`** — no PR-diff overlay, no
  search index, no UI — turning the review thread into dark matter on the
  forge most teams actually use.
- **HN discussion** ([item 37084575](https://news.ycombinator.com/item?id=37084575))
  surfaces the discovery problem ("So what, I just write code reviews to
  myself?") and rebase fragility — notes detach from commits on rewrite;
  git-appraise has shipped its own rebase wrappers and an open
  "more sophisticated rebase support" issue since 2016
  ([#45](https://github.com/google/git-appraise/issues/45)).
- **No primary source found** stating Google itself adopted or migrated
  away from `git-appraise` internally; Google's actual internal review
  tool is Critique.

**Walk-away check.** Notes objects live in git's object database but not
in the working tree, so `grep` over a checkout finds nothing — they are
invisible the way `refs/notes/git-bug/*` data is invisible. The binary's
disappearance leaves the data in `.git/` but readable only via
`git notes show` + `jq` plumbing. No primary-source account of a team
recovering review history after tool loss was found.

**Takeaway for grove.** Three lessons. (1) **Leaf 060 (sync semantics)** —
accept the cost of explicit `grove inbox push` / `grove inbox pull` as
git-appraise did; users tolerate it when the tool owns the refspec.
(2) **Leaf 070 (rename + init)** — a dedicated branch holding plain
markdown files *in the working tree of `.grove-meta/`* is strictly better
than `refs/notes/*` for humans grepping and for forges rendering;
`git-appraise` was forge-invisible and that killed adoption. Grove's
existing choice to materialise the branch as a sibling worktree at
`<repo>/.grove-meta/` is exactly the right reaction to this failure mode
— guard it. (3) **Leaf 060 (shape)** — lean hard on append-mostly,
one-record-per-line, deterministically-sortable artefacts so merge
reduces to `cat | sort -u` even across writers. This is the *one* design
choice from `git-appraise` that aged well.

**Maintenance status as of 2026-05.** Not archived but effectively dormant.
Latest release is the April 2021 "rollup"; ~26 open issues; companion
projects `git-appraise-eclipse` and `git-appraise-web` similarly inactive.
Treat as a design reference, not a living dependency.

---

## Radicle COBs

**One-paragraph summary.** Radicle's Collaborative Objects (COBs) are
CRDTs encoded as git commit DAGs under `refs/cobs/<type-namespace>/<oid>`,
with each change a signed git commit. Issues
(`xyz.radicle.issue`), patches (`xyz.radicle.patch`), and CI jobs are all
COBs. Sync is plain git fetch — peers union DAGs and state is a
deterministic causal replay. Compared with Fossil, the model is per-object
and signature-rooted rather than repo-global and timestamp-rooted. The
architecture is elegant; the lived experience is dominated by
infrastructure (seed nodes, NAT, key-per-device) more than by data-model
issues.

**Architecture (one screen).** COBs live under
`refs/cobs/<reverse-DNS-type-name>/<object-id>`. Each "change" is a git
commit whose tree contains the CRDT operation payload and whose parents
are the commits the author observed. Commits are signed by the author's
node key. State is computed by topologically sorting the DAG and applying
ops in causal order; the merge of two peers' views is the union of
commits. There is no merge conflict at the COB layer; ordinary git file
conflicts still apply to the code side. Sync via the `rad` CLI talking to
a seed node ([radicle.dev/guides/protocol](https://radicle.dev/guides/protocol),
[radicle.dev/guides/user](https://radicle.dev/guides/user)).

**Failure modes.**

- **Walk-away is effectively zero.** "Interacting with a Radicle
  repository, even just to report issues, requires using Radicle"
  ([radicle.dev/faq](https://radicle.dev/faq)). The data is git objects
  but the CRDT schema is implicit in the `radicle-cob` Rust crate; no
  production third-party readers found.
- **Key-per-device, no recovery.** "If a private key is either lost or
  compromised, one will have to generate a new node identity"
  ([radicle.dev/guides/user](https://radicle.dev/guides/user)). Backup or
  sync of keystores is explicitly discouraged.
- **Sync is decoupled from push.** "Changes pushed by collaborators may
  not reach your node immediately… `rad sync --fetch`"
  ([radicle.dev/guides/user](https://radicle.dev/guides/user)). A coherent
  local view can be silently behind the network — a different shape of
  leaf 060's "users forgot to pull before drain" pathology.
- **Seed nodes are the practical SPOF.** Public seeds may be slow or
  unavailable; running your own is recommended
  ([Seeder's Guide](https://hackmd.io/@radicle/r1Zejbx5a)). NAT traversal
  is not implemented — peers without public IPs rely on third-party seeds.
- **Read-only without a node.** "Unless you run your own Radicle node and
  open its web interface, you can't currently make changes"
  ([blog.liw.fi, Jan 2026](https://blog.liw.fi/posts/2026/radicle-status-quo-01/)).
  Drive-by contribution costs an install.
- **Churn.** Operator report: "Radicle has been unstable for years, with
  APIs and configurations changing virtually every other month"
  ([Mic92/radicle-sync README](https://github.com/Mic92/radicle-sync/blob/main/README.md)).
- **CRDT-merge surprises** — **no primary source found** for concrete
  field-stomping or comment-order anomalies in three searches. The model
  makes "field stomp" structurally possible (last-writer-wins per field
  resolved by node-id tiebreak) but no user complaints surfaced in the
  public record. Flagged as unverified-but-plausible.

**Walk-away check.** Effectively zero. Code is plain git; COBs are git
objects requiring the `radicle-cob` schema to render.

**Takeaway for grove.** The COB model is the strongest "no merge conflict
ever" prior art and validates DAG-of-changes designs — but the walk-away
cost is total and key-per-device is heavy even for insiders. **Leaf 060**:
keep the inbox as plain markdown (no CRDT); keep "drain" as a deliberate
human-mediated step; don't pursue signed-by-key authorship. **Leaf 080**:
the CLI is the *only* surface; don't assume an out-of-band node service.
Confirms (against Fossil) that signed-author models work technically but
cost users.

**Maintenance status as of 2026-05.** Very active. 1.6 (Jan 2026), 1.7
(Mar 2026), 1.9 (May 2026), ~12 paid contributors funded by Radworks
Treasury.

---

## Sapling / Jujutsu metadata

**One-paragraph summary.** Both Sapling and Jujutsu attach metadata to
commits that git doesn't natively carry — stable change-ids that survive
rewrites, predecessor/evolution chains, op-logs. The design choice that
matters for grove: **almost none of this is replicated between users by
default.** Sapling's `MetaLog` and jj's `.jj/repo/` live local-only; the
*one* piece that must cross machines (jj's change-id) is shipped via a
standardised `change-id:` git commit header, not via an out-of-band ref
or sidecar.

**Architecture (one screen).**

- **jj:** `.jj/repo/store/extra/` holds change-ids + predecessor lists as
  a `StackedTable`; `.jj/repo/op_store/` holds the operation log;
  bookmarks and "views" live in custom backends, not in git. In a
  colocated repo the docs explicitly tell you to `echo '/*' > .jj/.gitignore`
  — **`.jj/` is never committed**
  ([architecture](https://docs.jj-vcs.dev/latest/technical/architecture/),
  [git-compatibility](https://docs.jj-vcs.dev/latest/git-compatibility/)).
- **Sapling:** `MetaLog` is a local-only, content-addressed blob store +
  key-log that tracks bookmarks, remote bookmarks, visible heads, etc.,
  with an in-process merge function for concurrent writers
  ([sapling-scm.com/docs/dev/internals/metalog](https://sapling-scm.com/docs/dev/internals/metalog/)).
- **The propagated piece:** a 32-char reverse-hex `change-id` stored as a
  git commit *header*, standardised across Jujutsu, GitButler, and Gerrit
  ([gerritcodereview.com/design-docs/support-jujutsu-use-cases](https://www.gerritcodereview.com/design-docs/support-jujutsu-use-cases.html)).

**Failure modes.**

- Two jj users on the same upstream branch will have **different operation
  logs, different predecessors, possibly different change-ids** for
  commits that arrived via git. jj falls back to bit-reversed commit-id
  as change-id for git-imported commits — two clones that import the same
  commit converge, but independent rewrites diverge in predecessor
  history. The design *assumes* this, treats it as not-a-bug.
- jj is explicit that the storage layer must survive rsync/Dropbox copy
  of the whole repo, but this is whole-repo replication of one user's
  state, not multi-writer merge.
- Sapling's MetaLog uses an application-supplied merge function and
  **refuses to commit** on merge failure rather than silently picking a
  winner — same "surface drift, don't paper over it" stance.
- **No primary source found** describing a real team running jj's *native*
  (non-git) backend in shared/multi-writer mode; in practice everyone
  uses the git backend and lets git be the transport.

**Walk-away check.** Local metadata is a cache. Deleting `.jj/` rebuilds
it from the git repo with fresh deterministic change-ids. Sapling's
MetaLog likewise reconstructs from refs + ZStore. The only piece that
*cannot* be regenerated — the human-meaningful change-id of an in-flight
commit — has been pushed into the commit itself via the standardised
header so it survives `git push`.

**Takeaway for grove.** The Sapling/jj pattern argues against trying to
replicate grove's cross-grove inbox state via a dedicated-ref-with-special-fetch
(the git-appraise failure mode); instead, **put the small, identity-bearing
piece inside artefacts that ride normal git transport, and let everything
else be a per-clone cache.** Concretely for **leaves 060/070**:
(1) grove's inbox-branch + markdown-files approach is already closer to
"put it where git already syncs it" than to git-appraise's ref-namespace
trick — keep it that way; (2) treat any future per-grove derived index
(e.g. a "what's pending across all groves" rollup) as MetaLog-style
local-only cache, regenerable from the inbox branch, never the source of
truth — this is also a hint for **leaf 090 (TUI)**: the TUI's state is a
cache, not a source; (3) borrow MetaLog's "merge function returns failure
→ refuse to commit" stance for the inbox-branch write path. jj is
explicitly motivated by Mercurial-evolve's mistakes
([jj-init essay](https://v5.chriskrycho.com/essays/jj-init/}) — most
importantly that evolution data must not require a server-side
obsolescence-marker concept that users have to push and pull, which is
exactly the trap a naive grove cross-grove sync could fall into.

**Maintenance status as of 2026-05.** Both active. jj has regular releases
and an in-flight `change-id` git-header standardisation effort with
Gerrit/GitButler. Sapling continues to ship from Meta.

---

## TODO/FIXME comments

The dominant lived experience: TODOs accumulate, never get resolved, and
rot. Pieces like
[TODO Comments Considered Harmful (c2)](https://wiki.c2.com/?TodoCommentsConsideredHarmful=)
and
[Stop Letting Your TODO Comments Go to Die](https://dev.to/varun_m_a19ae9585eab6a659/stop-letting-your-todo-comments-go-to-die-57p1)
describe the same pathology — "later" buckets nobody empties; owners
drift; `grep TODO` returns hundreds of stale entries indistinguishable
from live ones. Grep-as-aggregation works mechanically at any scale but
loses signal-to-noise above a few hundred entries; the common mitigations
(require an issue link, TTL, lint-fail on bare `TODO:`) all reintroduce a
separate tracking system, defeating the inline-comment premise.

**Walk-away check.** Trivially perfect.

**Takeaway for grove.** TODOs are the *floor* — the baseline that grove's
seed convention exists to beat. Two specific lessons. (1) **Drain must be
mandatory and visible.** TODOs rot because there is no forcing function;
grove's "drain on bootstrap" rule (ADR-0002) is precisely that forcing
function — leaf **060** must keep drain as bootstrap-mandatory, never
optional. (2) The lesson is *not* "categorise more rigorously" — every
attempt at TODO taxonomy has died. Leaf **080** should keep the capture
verb single-noun (`grove inbox add`) rather than introducing
`grove inbox add --type=bug | --type=question | --type=…`.

---

## CHANGELOG.md / RELEASE_NOTES.md (and the directory-of-files workarounds)

**One-paragraph summary.** A single shared markdown file that many PRs
append to is git's three-way-merge worst case. The whole industry —
JavaScript (Changesets), Python (towncrier), Go (changie), Rust (git-cliff),
and GitLab itself — independently converged on the same answer: **don't
append; drop one file per change into a directory and assemble at release
time.** This is the *exact* shape leaf 060 needs to consider.

**Architecture variants.**

- **Single-file append (CHANGELOG.md, Keep a Changelog).** Every PR edits
  `## [Unreleased]`. Conflicts on nearly every merge.
- **`merge=union` gitattributes.** Stopgap that concatenates both sides
  instead of conflicting. Works locally; **silently breaks ordering on
  non-unique lines**; ignored by GitHub's web merge UI
  ([GitHub Community #9288](https://github.com/orgs/community/discussions/9288)).
- **Directory-of-fragments** (Changesets, towncrier, changie, git-cliff
  with `--include-path`, GitLab's `CHANGELOG/unreleased/`, logchange).
  Each change is its own file with a random or PR-derived name. The
  release tool walks the directory, emits CHANGELOG.md, deletes the
  fragments.
- **Generate-from-git-log** (git-cliff, Conventional Commits). Orthogonal:
  the changelog is a *view* over commit messages, not a tracked artifact.

**Failure modes.**

- **Single-file append.** "Every PR merged to develop automatically creates
  a conflict for all other PRs"
  ([GitLab CHANGELOG conflict crisis](https://about.gitlab.com/blog/2018/07/03/solving-gitlabs-changelog-conflict-crisis/),
  [HN 17452978](https://news.ycombinator.com/item?id=17452978)).
- **`merge=union`.** "Works only when every line is unique… causes silent
  merging of similar strings"
  ([Kiselev — On reducing Changelog merge conflicts](https://medium.com/@nettsundere/on-reducing-changelog-merge-conflicts-1eb23552630b)).
- **Conventional Commits alone** is machine-readable but loses
  human framing — `fix:` is a category, not an end-user sentence.

**The migration triggers.** GitLab is the cleanest documented migration.
They started with single-file CHANGELOG.md, tried `merge=union`
([commit 4377ba1c](https://gitlab.com/gitlab-org/gitlab-foss/-/commit/4377ba1c360cf6f4d15e3b5ad2a7ed7bc41f795e)),
found it insufficient, moved to `CHANGELOG/unreleased/<entry>.yml` with a
release-time merger script
([issue #17826 "Fix the Great CHANGELOG Conflict Crisis"](https://gitlab.com/gitlab-org/gitlab-foss/-/issues/17826),
[blog](https://about.gitlab.com/blog/2018/07/03/solving-gitlabs-changelog-conflict-crisis/)).
Trigger: contributor friction at scale — rebases dominated review cycles.
towncrier was created in Twisted for the same reason
([towncrier docs](https://towncrier.readthedocs.io/en/stable/index.html)).
Changesets' rationale: "all the xxx-changeset.md files are distributed,
so teams can easily collaborate on merge/rebase/cherry-pick without any
worry about conflict"
([changesets/changesets#719](https://github.com/changesets/changesets/issues/719)).
**No primary source found** for any tool that migrated *away* from
directory-of-files back to single-file — the migration is unidirectional.

**Does the directory become the new conflict surface?** Largely no, but
not perfectly. Random naming (Changesets uses human-readable slugs like
`tasty-otters-jump.md`) makes filename collisions astronomically rare;
even if two PRs picked the same slug, git reports an add/add conflict on
a *new* file — trivially resolved by renaming. Real residual issues are
(a) directory-level operations (release-time `changeset version`
consuming fragments) still produce churn, and (b) Changesets' pre-release
mode *does* re-introduce conflicts because it writes to a shared
`pre.json` ([#719](https://github.com/changesets/changesets/issues/719)).

**Walk-away check.** Directory-of-fragments has near-perfect walk-away:
orphan fragments are inert markdown. Single-file append fails walk-away
— an abandoned branch's CHANGELOG edit becomes a conflict the next
person inherits.

**Takeaway for grove.** **This is the central post-mortem for leaf 060.**
Every team that tried "many writers append to one markdown file"
eventually moved to "many writers each drop one small file into a
directory, assembled later." The convergence is across language
ecosystems, across years, with independent invention. If grove's current
inbox shape is *one markdown file per inbox* (`inboxes/<name>.md`) with
parallel appenders across worktrees, it is replicating the exact
pathology Changesets/towncrier/changie/git-cliff/GitLab/logchange all
explicitly walked away from. The proven shape is
`inboxes/<name>.d/<timestamp>-<slug>.md` (directory per grove, one file
per observation), with optional consumer-side concatenation for a human
view. `merge=union` is a tempting shortcut but is a documented dead-end
at platform-UI level and silently corrupts on non-unique lines.

---

## `docs/` markdown notes

**Common adoption story.** ADRs (`docs/adr/NNNN-title.md`, one file per
decision) work well — they mirror the directory-of-fragments pattern.
Free-form `docs/` notes and design RFCs work when each note is its own
file with a stable name; they break when teams treat a single
`docs/NOTES.md` or `docs/TODO.md` as a shared scratchpad — same
append-conflict pathology as CHANGELOG.

**Obsidian-in-repo.** Real friction. Persistent conflicts on
`.obsidian/workspace.json` and plugin state files
([obsidian-git #114](https://github.com/denolehov/obsidian-git/issues/114),
[forum: Synchronization conflicts in obsidian folder](https://forum.obsidian.md/t/synchronization-conflicts-in-obsidian-folder/77080)),
iCloud-vs-git interaction issues, worktree corruption requiring fix
scripts. The fix is invariably `.gitignore` on workspace/plugin files —
i.e. exclude the shared-mutable state from version control entirely.

**Walk-away check.** Perfect for plain markdown; poor for
plugin-state-coupled directories.

**Takeaway for grove.** Mirrors the CHANGELOG lesson: shared mutable
state in git is the antipattern; per-actor or per-event files are the
pattern. Concretely for **leaf 060** — if grove ever adds a per-grove
derived view ("what's pending across all groves"), it should be
*computed* and gitignored, not committed alongside the inbox files. For
**leaf 090 (TUI)** — the TUI's local cache must not be committed; treat
it like `.jj/` (always gitignored, regenerable from the inbox branch).

---

## Synthesis

The four planning-leaf-specific questions, answered with citations.

### For leaf 060 — sync semantics + inbox shape

**Q: Which surveyed tools used single-file vs directory-of-files shapes?
What did they say about merge conflict frequency under multi-writer?**

- **Single-file shapes:** CHANGELOG.md
  ([GitLab](https://about.gitlab.com/blog/2018/07/03/solving-gitlabs-changelog-conflict-crisis/)),
  ditz's `project.yaml`, shared `docs/NOTES.md` — all report frequent
  merge conflicts. GitLab's experience was severe enough to be named "the
  Great CHANGELOG Conflict Crisis".
- **Directory-of-files shapes:** Changesets, towncrier, changie, git-cliff,
  logchange, GitLab's `CHANGELOG/unreleased/`, BE's `.be/<uuid>/`,
  artemis's `.issues/<id>/`, ditz's per-issue YAML files (separate from
  its central index). All report near-zero textual merge conflicts;
  residual issues are limited to directory-level churn and shared-mutable
  *non-fragment* files (Changesets' `pre.json`).
- **The grove inbox today is single-file** (`inboxes/<name>.md`). The
  prior art is consistent and emphatic: this shape will produce conflicts
  under parallel-writer load.

**Q: For tools that migrated from single-file to directory-of-files, what
triggered the migration?**

- **GitLab** migrated explicitly because rebases dominated review cycles
  at contributor scale
  ([issue #17826](https://gitlab.com/gitlab-org/gitlab-foss/-/issues/17826)).
- **towncrier** was *created* (rather than a migration *to* it) in
  Twisted because the maintainers had tired of conflict resolution
  ([docs](https://towncrier.readthedocs.io/en/stable/index.html)).
- **Changesets** was designed conflict-free from day one
  ([rationale](https://github.com/changesets/changesets/issues/719)).
- **No primary source found** for any tool migrating from
  directory-of-files *back* to single-file. The migration is
  unidirectional.

**Q: For tools that auto-pushed on every write, what went wrong? For tools
that left push manual, what went wrong differently?**

- **No surveyed tool auto-pushed on every write.** Even Radicle, the most
  sync-aggressive, requires explicit `rad sync` against a seed node and
  has "your local view is silently behind" as a known pathology
  ([radicle.dev/guides/user](https://radicle.dev/guides/user)).
- **Manual-push tools** all suffer the same failure: users forget. git-bug
  maintainers admit it in their own dogfood thread
  ([#1221](https://github.com/git-bug/git-bug/issues/1221)) and want a
  cron. git-appraise leaves push manual; HN consensus is "I just write
  reviews to myself"
  ([HN 37084575](https://news.ycombinator.com/item?id=37084575)).
- **The robust answer is between the two:** keep the write local and
  fast (manual semantics) but ship a scheduled/post-action sync verb out
  of the box so the user has a one-liner to enable. *Don't* auto-push on
  every write — that couples write latency to network availability and
  is unrecoverable when offline.

**Q: Did any tool encounter a "users forgot to pull before drain"
pathology? How did they address it?**

- **git-bug, git-appraise, Radicle** all manifest this in slightly
  different shapes. None ship a satisfying solution; the closest is
  Radicle's `rad sync --fetch` ergonomic shorthand and the still-open
  git-bug feature request for scheduled bridge-pull
  ([#1221](https://github.com/git-bug/git-bug/issues/1221)).
- **The Sapling/jj pattern is the strongest answer**
  ([MetaLog](https://sapling-scm.com/docs/dev/internals/metalog/)): treat
  local state as a cache, rebuild deterministically from the source of
  truth on every read, and refuse to commit on merge failure. For grove
  this maps to: drain should `fetch` the `grove-meta` branch as its first
  step (refusing to drain stale state is acceptable; silently draining
  stale state is not).

**Synthesis for 060.** The inbox-shape decision is heavily pre-judged by
the prior art. Move from `inboxes/<name>.md` (single file) to
`inboxes/<name>.d/<timestamp>-<slug>.md` (directory of observation
files). Sync semantics: keep the default "no remote" stance; offer an
opt-in CLI verb to configure one; have `grove start` / `grove continue`
do a `fetch` (refusing to drain on stale state) but never auto-push.

### For leaf 080 — LLM/CLI boundary audit

**Q: For tools whose workflow was prose-coded vs CLI-coded, was there a
difference in user error rates?**

- **Prose-coded workflows fail repeatedly.** Fossil tickets's
  CLI-as-afterthought produced years of fossil-users threads from users
  trying to drive tickets from the command line. git-bug's identity
  edge cases are all consequences of CLI assumptions that worked in the
  documented workflow but broke under bare repos, custom SSH config, etc.
- **Interactive-capture is the canonical anti-pattern.** ditz's `ditz
  add` wizard was explicitly cited as the friction that drove users
  away ([Zwinkau](https://beza1e1.tuxen.de/articles/later_bug_tracker.html)).
- The grove rule should be: **the capture verb takes the full item
  non-interactively** (flags or stdin), and **process-correctness invariants
  belong in the CLI, not in prose in SKILL.md**. The featured grove example
  (`grove leaf insert` for renumbering subsequent leaves) is exactly this
  shape — a prose-coded "do these git steps in order" pattern promoted
  to a single-shot verb.
- **No primary source found** for a controlled comparison of prose-coded
  vs CLI-coded error rates; the evidence is qualitative — every surveyed
  tool that ended up with significant CLI gaps has issues from users
  saying so.

### For leaf 090 — TUI

**Q: Which surveyed tools provided a TUI / interactive view, and which
did not? Did that affect adoption?**

- **TUI shipped:** git-bug (TUI + Web UI + CLI), Fossil (Web UI is the
  primary surface), Radicle Desktop (shipped June 2025 specifically
  because issue/patch UX in the CLI was a barrier
  ([Radicle Desktop launch](https://radicle.xyz/2025/06/13/radicle-desktop))).
- **No TUI:** ditz, BE, ticgit, artemis, git-appraise. All abandoned or
  dormant.
- **The TUI does not save adoption.** git-bug's TUI exists and the
  project still has the adoption-drop-off problem
  ([HN 33730417](https://news.ycombinator.com/item?id=33730417)). Radicle's
  Desktop is recent and adoption data is not yet available.
- **TUI does increase maintenance burden visibly.** git-bug's backlog
  ([#1213](https://github.com/git-bug/git-bug/issues/1213),
  [PR #1533](https://github.com/git-bug/git-bug/pull/1533) for "webui v2")
  shows the cost of plural front-ends on a small maintainer team.

**Q: Of the tools with a TUI, were there cases where the TUI drifted
from on-disk state? What caused the drift?**

- **git-bug** has a recurring complaint thread about web-UI desync
  against the local refs
  ([#409](https://github.com/git-bug/git-bug/issues/409)). Cause: the web
  UI caches; the local refs change underneath it.
- **The Sapling/jj precedent** is the cleanest design answer
  ([MetaLog](https://sapling-scm.com/docs/dev/internals/metalog/)): the
  UI's state is a regenerable cache, never the source of truth; drift is
  surfaced by refusing to commit rather than papered over.

**Synthesis for 090.** The grove TUI scope should be modest: per-repo,
filesystem-watch, no cached source-of-truth. The TUI's local state must
be a regenerable cache (gitignored, like `.jj/`). Don't ship a TUI to
fix an adoption problem — git-bug shows it doesn't. Ship a TUI only if
it makes the *day-to-day inbox triage gesture* materially faster than
`grove start` + bootstrap drain.

### For leaf 070 — `grove meta` rename + `meta init`

**Q: The dedicated-branch architecture is most similar to git-bug's
`refs/bugs/*` approach. What does the git-bug post-mortem say about
that choice in particular?**

- The 2019 retrospective
  ([#212](https://github.com/git-bug/git-bug/issues/212)) acknowledges
  the architecture works for centralized topologies and is "much less
  clear" for mesh. The choice has not been retracted in v0.9 / v0.10
  release notes — but no user has stress-tested the mesh case at scale,
  so the implicit verdict is "centralized is fine; mesh is unknown."
- The opacity problem is the lesson that *applies most directly to
  grove's rename leaf*: git-bug's data is in git but invisible on
  checkout. Grove's choice to materialise the branch as a sibling
  worktree at `<repo>/.grove-meta/` is the **direct, deliberate fix**
  for this failure mode. Leaf 070 should *protect* the worktree
  materialisation as a primary feature, not as a convenience —
  `grove meta init` exists exactly so users with pre-feature repos can
  get the worktree without rebuilding the repo.

**Q: Are there any reported pathologies around branch materialisation as
a worktree (vs hidden refs)?**

- **No primary source found** describing a *worktree-specific* pathology
  for in-repo metadata. The closest is Obsidian-in-repo
  ([obsidian-git #114](https://github.com/denolehov/obsidian-git/issues/114)),
  where worktrees and plugin state files interacted badly — but the
  cause was committing shared-mutable state, not the worktree itself.
- ticgit's orphan-branch invisibility (no worktree, no `git pull`
  default refspec) is the *opposite* pathology and the strongest case
  *for* grove's worktree choice.

**Synthesis for 070.** The rename to `grove-meta` (broader scope) is
well-supported by the Fossil and git-bug experiences — both projects
discovered after the fact that the "coordination surface" wanted more
than its initial scope. The `meta init` verb is supported by every
surveyed tool's pre-feature-repo migration story (or lack thereof —
most surveyed tools never had one, which is why pre-feature users were
stranded). Make `meta init` idempotent and call it implicitly from
`install`/`update`; expose it explicitly so users with pre-feature repos
have a one-shot fix.

---

## Cross-cutting findings

Three patterns visible across the whole survey, each one a direct
endorsement of a grove design choice:

1. **Death by maintainer attrition, not by design defect.** Every primary
   retrospective surfaced ("ditz is kind of abandonware," BE's quiet
   fade, ticgit's wiki declaration) names *the maintainer moved on*
   before it names a technical flaw. **Grove's seed convention is
   convention-only, not a tool to maintain** — there is no separate
   "inbox tracker" project to abandon. The CLI is one verb namespace in
   `grove`; the data is plain markdown on a plain git branch. If the
   `grove` CLI disappears tomorrow, the inboxes remain legible. This is
   ADR-0002's walk-away argument, vindicated by every surveyed system's
   death.

2. **The central index file is the conflict hotspot.** Ditz's
   `project.yaml`, BE's bug-dir bookkeeping, ticgit's branch tip, every
   single-file CHANGELOG — wherever there's one mutable file that every
   operation touches, that file collects merge conflicts. **Grove must
   compute any cross-grove view from on-disk content rather than
   committing it.** This bears on the inbox shape decision in 060 and on
   the TUI's cache discipline in 090.

3. **Mid-flow capture must be one non-blocking gesture.** Ditz died
   partly because `ditz add` was a wizard; BE survived to be
   adopt-considerable because `be new "summary"` is one shot.
   **`grove inbox add` must accept full content non-interactively**
   (flags or stdin) so it can be invoked from inside another flow
   without breaking concentration. Leaf 080.

---

## Notes on confidence and gaps

- **High-confidence** on architecture and primary-source quotes for
  git-bug, Fossil, BE, ditz, ticgit, git-appraise, Sapling/jj, and the
  CHANGELOG/Changesets/towncrier family.
- **Medium-confidence** on Radicle COBs — architecture is well-documented
  but **no primary source found** for concrete CRDT-merge surprises
  (search returned silence rather than evidence-of-absence; flagged in
  the section).
- **Lowest-confidence** on artemis — most obscure of the four abandoned
  trackers; **no primary post-mortem source found**; section is
  deliberately short per the brief's time-box instruction.
- **Other "no primary source found" notes** are flagged inline: ditz's
  `project.yaml` merge-conflict pain (structurally inferred, not
  documented); Google's internal use or non-use of git-appraise;
  multi-writer jj native-backend operation; CRDT-merge surprises in
  COBs.

The grove convention is **not invalidated** by anything in this survey.
The closest to a disqualifying signal is the systematic preference for
directory-of-files over single-file under parallel-writer load — which
is a *shape* refinement for leaf 060, not a *substrate* rejection. The
substrate choice (plain markdown on a dedicated branch, CLI-mediated
writes, drain at bootstrap) is consistent with every successful pattern
in the survey and an explicit reaction against every failure pattern.

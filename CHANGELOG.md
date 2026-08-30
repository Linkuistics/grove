# Changelog

One file, one entry style, for everything this repo ships (*skills-monorepo*).

**Versioned sections are grove's.** A `## v<N>.<m>.<p>` heading is a release of
the `grove` binary — that is the only artifact this repo tags and ships by
version. Entries under it are grouped `### Added` / `### Changed` / `### Fixed`
where a release has enough of each to be worth grouping, and a flat list
otherwise.

**`## Unreleased` accumulates what the next cut will carry.** A session logs its
change when it makes it, so the heading has to exist before the release that
closes it does — the rule below already presupposes exactly that ("the section of
the grove release it *lands before*"), and with no standing heading it can only be
obeyed retroactively, by whoever cuts the release and no longer has the context.
Appending to the top `## v<N>.<m>.<p>` section instead is not the alternative:
that heading is tagged, and adding to it falsifies a release that shipped without
the change. **The cut closes the heading for you** — `release.toml`'s
`pre-release-replacements` renames `## Unreleased` to the version being released
and puts a fresh empty one back, so the standing heading is structural rather than
a step someone has to remember. Two consequences for anyone editing this file.
Leave the heading exactly as `## Unreleased` on a line of its own: the replacement
anchors on it, and a cut that cannot find it aborts before writing anything. And
every *other* mention of the string in this file is safe **because** it is inline —
never start a line with a bare `## Unreleased` in prose, or the release aborts on
an ambiguous match.

**A change to anything the grove binary does not carry is logged in the section
of the grove release it lands before**, prefixed with the component it touched —
e.g. *"`linkuistics` / `using-jujutsu`: …"*. It gets no `##` heading of its own:
this file is not those components' release ledger, it is the record of what
changed and when. A grove release that happens to contain only such changes
still gets a version, because the binary is what was cut.

**A decision earns its entry when its behaviour lands, not when it is recorded.**
A `design` session that reworks the ADR set and changes nothing a reader runs has
shipped nothing yet, so an ADR-only commit is deliberately absent from this file;
the entry belongs to the change that enacts the decision, and names the ADR there.
The reversal then reads as one entry rather than as a promise followed by a
delivery.

**A skills entry names no version, because the plugins carry none.** Neither
`plugins/<name>/.claude-plugin/plugin.json` declares a `version`: both are
versioned by commit SHA, so every push delivers and there is no bump to record
(*skills-monorepo* has the trade, and adding a `version` would quietly undo it).
The skills reaching codex, gemini and pi by `plugins/install.sh` are symlinks,
so they were never in a version's path either.

The section at the foot of this file is the `Linkuistics/skills` changelog as it
stood at the graft — a closed record, not part of the versioned sequence above.

## Unreleased

- **`grove-llm leaf-insert`'s cross-reference lint takes a shared lock, and
  holds none while it prints.** The lint reads the tree the renumber left and
  writes nothing to it, so it no longer takes the store's exclusive lock: a
  concurrent `pick`, `kind` or `brief-chain` used to wait out a whole-tree
  content scan for a guarantee only writers needed. And the hits are now
  *returned* to the CLI rather than written to a sink under the lock — a stderr
  that has stopped draining blocks the printing process alone, where before it
  wedged every grove process on the worktree behind the held lock. A hit that
  cannot be printed is still dropped rather than failing the command: the insert
  has already landed. What is given up is a claim that never survived the call's
  own return — that each hit was printed while the tree was held; what is kept,
  and now asserted, is that every hit comes from one consistent snapshot.

- **The launched session is a job of its own** — its own process group, holding
  the terminal, with default signal dispositions
  (`docs/adr/the-launched-child-is-a-job.md`). Three defects go with it, all of
  which predate the runner's extraction:
  - **Ctrl-C reaches the session again.** The driver ignores SIGINT so a typed
    interrupt cannot kill the loop, and an *ignored* disposition is the one kind
    that survives `execve` — so the session inherited it, and so did everything
    it spawned and every wrapper its command template named. Under
    `sh -lc 'claude …'` Ctrl-C did nothing at all and `/exit` was the only way
    out. `keyed_launch::run` now resets the terminal-generated signals to their
    defaults across the spawn, so the driver's policy stays the driver's.
  - **The kill escalation reaps descendants.** It signalled the session's pid, so
    a tool subprocess, a language server or an in-flight `grove-llm` survived the
    SIGKILL and stayed attached to the terminal. It now signals the process
    group. The expensive case that closes: a surviving `grove-llm` holds shared
    epoch admission, so the driver's post-reap invalidation waited out its full
    30-second bound and then turned a session that had finished correctly into a
    fatal error with its completion token discarded uninterpreted.
  - **A signalled driver no longer exits 0.** `grove` killed by SIGTERM or
    SIGHUP mid-grove restores the default disposition and dies of the same
    signal after its cleanup, so a systemd unit, a `timeout(1)` or a shell
    `wait` reads `128 + N` rather than a clean finish. `LoopOutcome` gained
    `Interrupted(signal)` to carry it, and `End::Interrupted` now names the
    signal it forwarded.

- **The driver is a crate, both binaries are thin, and the repository root is no
  longer a package.** `driver_lease`, `loop_driver`, `prompt` and
  `session_config` moved out of the root `src/` into `crates/grove-loop`, and
  `src/` is gone. What was the root package is now `crates/grove`: a binary with
  no library at all, three steps long — parse an empty command line, resolve the
  workspace, take the lease — and then `grove_loop::run`, which is the whole loop
  (`docs/specs/module-decomposition.md`, decision 9). Both binaries are now
  packages over `grove-loop` rather than targets inside it, so *the binary is
  thin* is compiler-enforced for the human's as it already was for the session's.
- **`run(workspace, lease, templates)` is the loop, and `LoopOutcome` says why
  it ended.** The lease is taken by value, so it is released
  exactly when the loop that justified holding it returns. `templates` is a
  `TemplateSource` — *where* the launch configuration is read from — rather than
  a loaded configuration, because the loop re-reads it once per iteration: a
  session that adds a kind to `config.kdl` is launched from the document as it
  stands, and the just-in-time presence rule is asked against the document as it
  stood **before** the tree was mutated
  (`docs/adr/complete-session-configuration.md`).
- **`DriverLease::acquire` takes a resolved workspace instead of a path.** The
  binary resolves the working tree once and hands that value to both the lease
  and the loop, so one resolution stands behind the lease, the configuration
  delta's search order, `${repo}` and the version control the prompt states —
  where two derivations could previously only be checked by eye. `main_repo()`
  goes with it, and composing the prompt became infallible: it used to resolve a
  workspace it had already proved was there.
- **One release version is a manifest fact.** `[workspace.package] version` is
  the only place a version is written and every member takes
  `version.workspace = true`, so a `cargo release` cut rewrites one field and
  moves all six together. That is what lets `grove_loop::VERSION` be grove's
  published release version rather than a library's own `0.1.0` — the constant
  the prompt publishes (decision 10) and both binaries answer `--version` with.
  `release.toml`'s changelog replacement now resolves against `crates/grove`, so
  it names `../../CHANGELOG.md`; written the old way a cut aborts, loudly but
  only at release time.
- **The shared test helpers live at `testing/support.rs`.** They were the root
  package's `tests/support/mod.rs`, and there is no root package; they are about
  the repository rather than any of the three packages that use them, so each
  keeps a one-line shim naming the single file by path. The repository-surface
  suites — reference navigation, plugin citations, the two guidance checks — moved
  to `crates/grove/tests/`, and the human CLI's two clap-model assertions became
  unit tests beside the `Cli` they are about, which is the only place a
  binary-only package's model is reachable from.

- **The task tree and its twelve verbs are a crate: `grove-loop`.** `task_name`,
  `task_tree`, `task_grow`, `tree_lifecycle` and the completion signal moved out
  of the `grove` package into `crates/grove-loop`, behind the surface
  `docs/specs/module-decomposition.md` decision 9 fixes: `read` / `write`
  answering `Reading` / `Writing`, a `Reference` covering `.`, a key, a handle
  and a path, a `Selection`, and `verbs::` — twelve of them, each taking the
  lock it needs in its signature (a `Tree` to read, a `TreeWrite` to write),
  each answering the store's `Sought` where a search can match nothing, and each
  returning the paths it wrote. Opening mirrors the store's one level up, so
  `root-init` takes the `Vacancy` and **cannot** run over a live grove: the
  refusal to clobber is the shape of the types rather than a check inside them.
- **`grove-llm` is its own crate, not a second `[[bin]]`.** A binary target can
  reach its own library's private items, so *the binary is thin* stopped being
  compiler-enforced the moment it was one. It is now a package over `grove-loop`,
  and every verb in it is one `grove_loop::verbs::` call plus rendering.
- **`grove-loop` answers one opaque `Error` for the whole crate**, under the
  obligation the runner's and the VCS seam's are under: every one names what is
  wrong *and* what fixes it. `anyhow` stops at that boundary, so a consumer takes
  on no error library of grove's.
- **The finish teardown deletes through the store.** `finish-commit` was the last
  caller keeping its own `remove_dir_all` spelling of an operation
  `root-lifecycle-belongs-to-the-store` had already moved across; it now consumes
  its write guard through `WriteGuard::delete`, which refuses a root spelled
  through a link and reports what went. The commit it takes is unchanged.
- **`grove-llm resolve .` answers the grove root.** `.` was previously read as a
  bare slug and reported as matching nothing; it is the root, which is the one
  reference that names no entry (decision 9). An empty or blank reference is now
  refused rather than searched for.
- **`grove-llm pick` / `brief-chain` / `kind` / `resolve` refuse a worktree with
  no grove, and say how to make one.** They always refused it; the message now
  carries the remedy. A grove that is not there is still not a grove that is
  finished — a session run one directory off is told so rather than told its work
  is done.
- **`grove-llm leaf-prune` that stops partway now names what it already marked.**
  A subtree prune is *N* renames under *N* guards
  (`docs/adr/bulk-marks-are-not-atomic.md`), accepted on the argument that the
  marks *are* the state so rerunning converges — and that argument is only
  available to an operator who can see the residue. A stopped run now lists the
  leaves already marked `ABANDONED` and says to rerun the same command.
- **A second tree operation under one opening announces its own wait.** It
  reopens the lock rather than keeping one, so a contender arriving in the gap
  used to block the command with nothing printed. The two commands with a second
  operation are `leaf-insert` (the cross-reference lint) and `leaf-prune` on a
  node.
- No release is cut for any of this and nothing is reinstalled: the change is
  which package a module compiles in, and the tree, its grammar and the verb set
  are untouched. The leaf's own `## Why this leaf does not install anything`
  carries the measured matrix.

## v19.6.0

- **A session kind is an open token: Grove holds no list of kinds.** `Kind` was a
  compiled enum of nineteen variants with a parse arm, a label arm and a
  hand-maintained `ALL` roster; it is now a validated token — non-empty,
  lowercase ASCII letters, digits and single dashes, no `--`, not a reserved
  marker — and nothing else. The kinds that *exist* are the `grove-<kind>` skills
  the installed methodology ships, so adding one is authoring a skill rather than
  editing and releasing a binary. A leaf whose kind no skill declares **parses
  and launches**, and the failure is reported by the session that could not load
  the skill. Grove spells exactly two kind tokens, at the two places it writes a
  leaf itself with no session to delegate to: `requirements` for root scaffolding
  and `finish` for the teardown sentinel
  (`docs/adr/a-kind-is-an-open-token.md`, `docs/specs/module-decomposition.md`
  decision 5).
- **Breaking: `--kind` is now required on `grove-llm leaf-add` and
  `grove-llm leaf-insert`.** Both defaulted to `impl` — a kind literal under a
  friendlier name, and the one that produced a *wrong* leaf rather than an
  error, since a forgotten flag yielded a well-formed `impl` leaf and no
  complaint.
- **Breaking: `grove-llm leaf-add-pair` is removed; `leaf-add` takes an ordered
  list of kinds.** `leaf-add <parent> <stem> --kind research-a --kind research-b
  --kind combine-research` lands the research pair as one unit, at consecutive
  positions with consecutive keys, or lands none of it. The verb was the last
  place the machinery held a list of kinds. It was **generalised rather than
  deleted**: telling the skill to call `leaf-add` three times would reintroduce
  the live-prefix hazard the atomic run exists to exclude. Twelve verbs, not
  thirteen.
- **Changed: an unrecognised kind is no longer refused by the grammar.** The
  refusal that listed all nineteen labels is now a **shape** refusal naming the
  character it refused (`TaskNameError::UnknownKind` → `BadKind`), and a kind no
  launch template declares is refused before the tree is mutated, naming the kind
  and the file that must declare it. `--kind work` is an ordinary token now: the
  `work` → `impl` rename alias is gone, because refusing a well-formed token by
  naming a replacement is Grove holding an opinion about what a kind means.
- **Removed: `src/leaf.rs`.** It held `Kind` alone, apart from the grammar's
  three other validated components, because the set was closed and the label
  doubled as a configuration key. With the set open, `Kind` is what `Slug` is —
  and the *canonicity* of a leaf filename depends on the two obeying the same
  shape rule — so both are `task_name`'s, validated by one function.
  `SlugError` becomes `TokenError`, shared by both.
- `grove` / methodology: the spine's decomposition procedure now spells a
  research pair as a kind list, both grow verbs are documented as requiring
  `--kind`, and `TASK-FORMAT.md` states that its nineteen are the methodology's
  set rather than the binary's.

## v19.5.0

- **Provisioning is deleted: Grove writes no skill directory, embeds no
  methodology, and keeps no harness registry.** The binary used to compile
  `content/` into itself with `include_dir!` and sweep it, on every invocation,
  into `~/.claude/skills/grove`, `~/.codex/skills/grove` and
  `~/.pi/agent/skills/grove`. The methodology now ships as the `grove` plugin,
  installed the way this repo's other two plugins are, and all twenty of its
  skills declare `harnesses: [any]` rather than `[claude-code]`. Deleted with the
  sweep: `src/provision.rs`, `src/harness.rs`, `src/methodology.rs`, `content/`,
  `build.rs`, and the three dependencies they were the only consumers of —
  `include_dir`, `sha2`, and `tempfile` outside dev
  (`docs/specs/module-decomposition.md`, decision 11).
- **Removed: `grove-llm --content-hash`, and the driver's per-iteration build
  pairing report.** Both had the same two operands — a hash of the binary's
  embedded corpus, and the same hash stamped on a skill directory — and neither
  operand exists. Gone with them: the per-verb warning that an installed
  directory carried another build's methodology, the per-launch repair of a
  clobbered one, and the report that no known harness root existed at all.
  `grove-llm` takes a verb or prints help; it has no metadata argument left.
- **Retired: `one-build-owns-a-session` and `skill-delivers-the-methodology`.**
  Every mechanism either record argued was machinery for making one *shared
  mutable global directory* safe. A plugin has its own install route, so there is
  no directory for two builds to contend over and no pairing to report.
- **The cost is recorded rather than argued away.** Grove no longer guarantees
  the methodology is present, so a session can be launched pointing at a skill
  that is not installed. That is a message, not machinery: the prompt states the
  version Grove is and names the skill. A harness with a skill-loading affordance
  is unaffected; one without loses its fallback, and the reopen condition is a
  session that cannot reach the methodology by the affordance alone. Serving the
  methodology over MCP is **rejected** in the same place: it would not remove the
  delivery machinery, only change what is served. A harness registry row for a
  further harness is answered by deletion — there is no registry left to hold one.
- **Test-side: three corpus suites deleted, three re-homed.**
  `tests/lifecycle_invariants.rs`'s coverage walk and
  `tests/loaded_path_budgets.rs`'s per-kind word budgets measured the path the
  binary composed out of `content/`, and `docs/specs/corpus-rule-ownership.md`
  described it; all three went, and `plugins/grove/conformance.sh` — whose
  fourth, temporary assertion holding the spine and `content/` byte-identical
  went too — is the only standing delivery instrument. `tests/methodology.rs`'s
  one claim whose subject survives is now `tests/instructed_verbs.rs`: the
  shipped methodology instructs no `grove-llm` verb the CLI lacks, read off
  `plugins/grove/skills/` rather than off an embed. `tests/plugin_fallback.rs`
  and `tests/session_kind_guidance.rs` were repointed at the same files.
- **`plugins/install.sh`'s workspace guard and non-symlink refusal both stand.**
  A directory an older grove build swept into place is not a symlink, so the
  script refuses it by name and says so; remove it once by hand and re-run.

- **The launch prompt names one `grove-<kind>` skill and publishes Grove's
  version, and the driver interprets a kind nowhere.** `${prompt}` is three
  driver-authored parts and reads nothing: an imperative naming that kind's
  skill in both spellings of the one target — bare, and `grove:grove-<kind>` on a
  harness that namespaces plugin components — the runtime facts, now the selected
  handle, the stated version control **and the workspace's release version**, and
  Grove's own signalling contract. `grove --version` renders the same value and
  stays as a fallback rather than as the mechanism: a verb needs the CLI on
  `PATH` and fires only if the session thinks to run it, which is the deferred
  read `docs/research/wording-micro-test.md` measured, while a value in the
  prompt cannot fail. Deleted with the content dependency: `reference_file`'s
  nineteen-to-ten `match`, `ending_file`'s nineteen-to-two one, the
  provisioned-directory list, and `src/prompt.rs`'s call to
  `methodology::embed()` — so composition is infallible and there is no launch
  left to fail. `compose` now takes a `Mandate`
  (`docs/specs/module-decomposition.md`, decisions 9 and 10).
- **A `finish` session's three endings moved into `grove-finish`, and the prompt
  states a mechanism instead of an instruction.** Two embedded signal files used
  to serve the nineteen kinds so that `finish` — which chooses its ending by what
  it did — never read *run `grove-llm complete`* last in the prompt of a session
  that may have just deleted the task tree. One contract serves all nineteen now,
  and the ending a kind takes is inline in that kind's skill, which is where the
  spine already said it belongs. **The kind's own ending is the contract's
  subject and the ordinary verb is subordinate to it** — *your kind's skill
  states how this session ends … ordinarily it is `grove-llm complete`* — so no
  prompt ends on a bare imperative for an action that is wrong for the session
  reading it. The residue that remains is recorded in `src/prompt.rs` and in
  decision 9 rather than argued away: a `finish` session whose skill is missing
  or unread meets the default and nothing contradicting it, where the old prompt
  alone was fail-safe for that kind whatever was installed. The reopen condition
  is a `finish` session observed signalling `complete` after a teardown.
  `content/references/finish.md` stopped claiming the outcomes ride the prompt,
  which this change would otherwise have made false for every session the still-
  live provisioning path reaches.
- **`grove` / conformance: `finish-three-endings` is asserted again**, against
  `grove-finish/SKILL.md`, where it used to be one of two rows the manifest owned
  to `${prompt}` and checked nowhere. One row is left there,
  `signal-is-the-last-action`, and it now binds all nineteen kinds. The mirror
  column's emphasised `none` reads its note as prose rather than matching one
  hard-coded sentence, since the two rows no longer share one.
- **`tests/lifecycle_invariants.rs` and `tests/loaded_path_budgets.rs` now
  measure `content/` as a corpus rather than as something the prompt routes
  into.** Nothing routes a session into `content/` once the prompt names a plugin
  skill, so the static path they walk is the corpus's own three entry files —
  `SKILL.md`, the kind's reference file, the kind's signal file — derived from
  each kind's own label rather than from a table a twentieth kind would never
  appear in. Every budget row was re-fitted to what that measures; the static
  paths fell (`impl` 1,334 → 1,268 words) and the reachable ones rose to what the
  corpus had already grown to (`impl` 11,926 → 12,539). Both suites still go with
  `content/` at `delete-provisioning-k19`.

- **One `grove-<kind>` skill per session kind now ships beside the spine.** The
  five producers each with their `review-` and `integrate-review-` steps, the two
  research halves, `combine-research` and `finish` — and no list of them anywhere:
  a kind exists **iff** a skill of that name exists, so nothing in `src/` gained a
  manifest and the conformance runner still asserts nothing about how many there
  are. The fatness rule ships in its two halves rather than collapsed into one.
  A rule owned by one kind is **inline in that kind's own `SKILL.md`**, which is
  what makes it a one-hop read; a rule owned by a *family* — the five reviews, the
  five integrations, the two research halves — stays as **one file in the spine**
  and each member's skill directs a load of it by name without restating a word.
  Seven kinds are fat that way and twelve are thin. `grilling.md` moved into
  `grove-requirements` and now points at that skill rather than at a reference
  file that no longer exists. Nothing is removed from the binary: `content/` is
  untouched and dies at `delete-provisioning-k19`.
- **`grove` / conformance: a rule's owner is now written shipped-set-relative**
  (`grove/references/execute.md`, `grove-impl/SKILL.md`) rather than
  skill-relative. With one skill the two grains agreed; with a skill per kind they
  do not — every kind owns a file called `SKILL.md`, so a sweep reporting sites
  skill-relative collapses two kinds stating one rule into a single site and reads
  clean over exactly the duplicate assertion 2 exists to find. The new control
  *one kind-owned rule stated by a second kind is caught* is watched failing
  against that case, which is what credits the grain. The runner also memoises its
  loaded-path closures and normalised files; without that, nineteen kinds against
  a 146-row manifest takes minutes.
- **`grove` / conformance: two rules are now owned by `${prompt}` and asserted
  nowhere** — the signal step and a `finish` session's three endings, whose bytes
  the driver inlines into the launch prompt. A runner over an installed skill set
  cannot read a prompt, so they are reported as prompt-delivered instead of
  *pending*: pending promises a later leaf that ships the file, and no leaf will.
  Every other row's owner is now shipped, so the pending count is zero.
- **`tests/rule_ownership.rs` is deleted**, its subject being assertion 2 of
  `plugins/grove/conformance.sh`: its 68 pinned wordings and 3 removed
  paraphrases are `conformance/rules.tsv`'s rows, verbatim, and its three
  controls are `conformance.test.sh`'s. The other Rust suites that overlap the
  runner are **mixed** and stay — `tests/lifecycle_invariants.rs` holds the
  behavioural coverage walk and `tests/loaded_path_budgets.rs` the load column and
  the per-kind word budgets, neither of which has a home in the runner. Deleting
  them now would retire an assertion whose subject is still shipping.
- **`docs/adr/grove-binds-without-the-plugin.md` is reworked and renamed
  `a-skill-states-what-binds-without-its-dependencies.md`.** Its opening was that
  the binary sweeps `content/` into every harness's skill directory, which is no
  longer how the methodology arrives; the obligation it records was never about
  that, so its subject is now what binds when a *skill's* own dependencies are
  absent. `behavioural-coverage-asserts-delivery`, `corpus-rules-have-one-owner`
  and `restatement-declares-its-class` are amended in place: the rules survive
  unchanged, and what moves is the register (the plugin spine, not an embedded
  `content/`), the reachability edge (the composed loaded path, not a module of
  the binary), and the instrument (the shell runner).
- **The second hop is not measured, and that is written where a session meets
  it** — [`plugins/grove/README.md`](plugins/grove/README.md). The wording
  micro-test measured one hop, from a prompt naming one target; nothing measures
  the hop from a `grove-<kind>` skill to the spine. What is inline is unaffected;
  what is in the spine loses the guarantee. The reopen condition is a session
  observed acting without a spine rule.
- **The methodology now ships as a Claude Code plugin: `plugins/grove/`.** A
  third marketplace entry beside `linkuistics` and `testanyware`, carrying one
  `grove` **spine** skill — the seven constraints, the bootstrap, execution,
  decomposition, retirement and commit procedures, the review / integration /
  research family files, and the five format documents. One `grove-<kind>` skill
  per session kind lands beside it (see the entry above); a kind exists **iff** a
  skill of that name exists, and nothing here enumerates them.
  Nothing the binary does changes: it still compiles `content/` in and still
  provisions `~/.claude/skills/grove` and its two siblings. **The spine is the
  source and `content/`'s copy of those files is what gets deleted** with
  provisioning; which file is which, and where the rest of `content/` lands, is
  in [`plugins/grove/README.md`](plugins/grove/README.md). The one file that is
  *split* rather than shared is `SKILL.md` — `content/`'s is a kind router, the
  spine's is a condition register — and the two are expected to differ.
- **`grove` / conformance: the methodology's delivery assertion moves to a
  dependency-free shell runner**, `plugins/grove/conformance.sh`, with its
  controls in `conformance.test.sh`. Over the shipped skill set it asserts that
  every behavioural rule is present on the composed loaded path of every kind
  that binds it, that no rule has two owners, and that every file a skill names
  by path exists — and nothing about how many kinds there are. Its inventory is
  `plugins/grove/conformance/rules.tsv`, all 146 rows of
  `docs/specs/corpus-rule-ownership.md`'s.
  Why a shell runner: two of the four things
  [`behavioural-coverage-asserts-delivery`](docs/adr/behavioural-coverage-asserts-delivery.md)'s
  walk covers stop existing in the binary, so the assertion cannot stay in the
  Rust suite. Those suites are untouched here and are deleted only once their
  assertion has a home. A spine with no kind skills beside it is a legitimate
  intermediate state: the runner reports it and stays green.
- **`plugins/install.sh` refuses rather than warns when a real file or directory
  already sits at a target path.** The path is still left untouched and every
  other skill still installs, but the collision is an `error`, the closing report
  names each blocked path and the two ways out, and the run exits non-zero.
  Why: the previous `warn` line was read once among the `ok` lines, and an
  uninstalled skill then reads as a successful install — the silent, delayed
  failure the script's own workspace guard already refuses for. The live case is
  grove itself, which provisions directories into `~/.codex/skills/grove` and
  `~/.pi/agent/skills/grove`; the spine declares `harnesses: [claude-code]` so
  those paths are not contested until the binary stops writing them.

## v19.4.0

- **A leaf filename separates its session kind from its slug with `--`.** The
  grammar is now
  `NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md`; the middle splits at
  the **first** `--`, and neither token may contain one (`Slug::new` refuses it,
  and no kind label carries it). Node directory names, the `DONE`/`ABANDONED`
  infixes, the terminal `-k<key>` and the handle are all unchanged, so a handle
  is still `<slug>-k<key>` and still a contiguous terminal substring of the
  name. **This renames every existing leaf, and a name without the separator is
  refused** — with the canonical form in the message, because the recovery is a
  `mv` per leaf and the advice is part of the error. A tree still on the old
  spelling fails loudly at its first name rather than being read wrongly.
  Why: with a single `-`, a hyphenated kind beside a hyphenated slug had more
  than one reading — `02-design-decomposition-k2.md` is kind `design` + slug
  `decomposition` *and* kind `design-decomposition` + no slug — and only a
  longest-label match against the closed set of nineteen kinds resolved it. That
  is one filename naming two entries whose **handles** differ, and the closed set
  that papered over it is what `open-kind-k20` removes next. The separator makes
  the boundary notation rather than inference, so `Kind::split_filename_prefix`
  is deleted and the parser reads the kind token by exact equality.
  A leaf whose kind is unknown now quotes the offending token, which the old
  grammar could not do — there was no single token to name — and a leaf with no
  separator gets its own refusal naming the grammar
  (`docs/specs/module-decomposition.md` decision 3 and its two scenarios;
  `docs/adr/task-names-are-canonical.md`, amended; `grammar-separator-k15`).
- **`.grove/FORMAT` is no longer written, read or required.** `delete-migration-k6`
  removed every reader and writer but left the file in trees so the shipped build
  could keep driving; with this release nothing on any build looks at it, and it
  can be deleted from any tree that still carries one.
- **One type owns the name, and the handle is a rendering of it.**
  `task_name::Handle` — the position-free `<slug>-k<key>` identity that crosses
  every module boundary — is a type with `of`, `parse`, `slug`, `key` and a
  `Display`, and it is now the only thing that spells that grammar. `Handle::render`
  is the single `write!` it appears in and **both of `TaskName`'s renderings end
  in a call to it**, so a filename and a handle disagreeing is not expressible
  rather than merely reviewed against; `task_name::peel_key` is the single place
  the terminal `-k<digits>` is taken apart, shared by `Handle::parse` and the
  filename parser. Four hand-rolled produce sites and the duplicate peel are
  gone — `task_tree::live_leaf`'s `format!`, `tree_lifecycle::finish_handle`,
  `tree_lifecycle::append_brief_suffix_in_file`'s title match,
  `task_grow::task_template_body`, and `task_tree::handle_key`, whose own comment
  conceded it "mirrors the filename grammar" — and the renderer's own two inline
  arms went with them. `SelectedLeaf::handle` is a
  `Handle` rather than a `String`, and `finish-commit` now reads its argument
  through `Handle::parse`, so a malformed one is told what a handle is instead of
  being reported as a mismatch (`docs/specs/module-decomposition.md`, decision 4;
  `name-ownership-k14`). `Kind` keeps its closed set — `open-kind-k20` removes
  it, and not before the separator lands.
- **`finish-commit` accepts a lenient key spelling and still records the
  canonical handle.** `finish-k0002` now names the live `finish-k2` rather than
  being reported as a mismatch, because a handle is a reference an operator
  types and not a name on disk. The teardown commit and the refusals were
  separated to make that safe: the commit message renders the *tree's* handle,
  so a permanent record can no longer name a handle no leaf ever wore, while a
  refusal quotes the argument exactly as it was typed.
- **Grove's second lock layer is gone.** `src/tree_access.rs` is deleted, and
  with it Grove's own `flock` on the directory containing the task tree. Every
  tree operation now opens through the store's `read` / `write` and takes its
  guard from there, so the deadlock the layer existed to be kept away from —
  two open file descriptions on one directory do not share a lock, so a verb
  holding Grove's guard that called into the library blocked on *itself* — is
  gone rather than routed around. The three recorded reasons for the layer
  dissolved at once and only now: `open-shape-k25` made an absent tree a shape,
  `delete-migration-k6` left no legacy shape, and `delete-finish-transaction-k8`
  left no transaction (`docs/specs/module-decomposition.md`, decision 2).
- **A grove is created whole, in one store operation.** `root-init` and the
  driver's lifecycle transition each take one `task_tree::write_or_vacancy` and
  hand the vacancy to `ordinal_fs_tree::fs::Vacancy::initialize`, which creates
  the root, its `BRIEF.md` and the first `requirements` leaf under the exclusive
  lock the vacancy already holds. The two-phase `Classification` / `settle`
  dance is deleted, and so is the window it opened between the phases.
- **A taskless root is refused, not repaired.** `complete_partial_root_unlocked`
  — the last of `minimalism-k1`'s roughly twenty-five auto-repair functions
  still standing — is deleted with the anomaly it existed for. A root holding
  its charter and no task is no longer a shape Grove produces, so it is now
  something else's doing, and it stops with a sentence naming `jj undo`
  (principle 2). `CurrentTransition::RootInitRecovered` goes with it.
- **A transition over a name Grove refuses now states the refusal itself**,
  in the domain's own words, instead of answering *already current* and leaving
  the next read to say it. Grove classified the root off its own listing while
  it held its own guard; the store halts the whole tree on such a name, so the
  transition carries the message. One refusal where there were a success and a
  refusal.
- **`task_tree::restate` orders `RootIsNotATree` ahead of its absence clause**,
  because `is_dir` reads a dangling symbolic link at the root as *absent* and
  the store's own sentence says what is there and that a tree is a directory.
- `tests/tree_access.rs` is `tests/tree_lock.rs`, and carries a second
  enumeration beside the one that holds the store's lock to a single module:
  **no `flock` Grove takes in production ever blocks**. Waiting belongs to the
  store, on the store's own lock; a blocking acquisition anywhere in `src/` is
  the second layer growing back, and its symptom is a hang rather than a
  failure.

- **The tree store can delete a tree root, and says what it destroyed.**
  `ordinal_fs_tree::fs::WriteGuard::delete` removes the root and everything
  beneath it under the exclusive lock the guard already holds, and answers with
  `Removed { root, entries }` — every path that went, in the order it went,
  children before the level that held them (`docs/specs/module-decomposition.md`,
  decision 2). It is the only operation in this library that removes anything.

  **It follows no symbolic link, and that is a security property rather than a
  detail.** Descent is decided by the same unfollowed look a snapshot is read
  through — `DirEntry::file_type`, which is `symlink_metadata` — so a link naming
  a directory, including one naming a directory *outside* the root, is unlinked
  as a link and its target is untouched. The bound on that claim is stated where
  the walk is: the look and the descent are two syscalls, and the writer who
  could exploit the gap is the writer who ignores the advisory lock, which is
  already outside what this library defends against.

  **Why it reports paths where every other mutation reports names.** Deletion
  acts on the root, so it acts on everything beneath it — including the entries
  the domain deliberately declines to parse as its own, which the walk already
  skips and which no `N` could describe. A third bucket of names would report
  less than the paths do while looking like it reported more.

  **`delete` is on the write guard, so deleting a vacancy does not compile** —
  the third of the three ill-formed calls the opening shape removes from the
  language, with a `compile_fail` doc test on `Vacancy` as the proof.

- **`delete` is the one operation that constrains how the root was *spelled*.**
  Every other operation uses the root as the directory things are in, so letting
  the kernel resolve its last component is right — a symbolic link naming a
  directory is an accepted spelling, and two spellings of one tree take one
  lock. A deletion acts on the root as an object, where a link and what it names
  are two things and only one is the tree; and it removes the very components a
  path is built from, so a spelling that descends into the tree and comes back
  out through `..` stops resolving partway through its own removal. Both are
  refused before anything goes, as `Error::RootIsNotSpelledDirectly`. The `..`
  rule is coarser than the danger and says so: a `..` cancelling a component
  above the tree is harmless and refused with the rest, because separating them
  means resolving the path and nothing here resolves anything. A leading `..` is
  accepted, so `../course` still works.

- **A removal has nothing to unwind, so it has a failure answer of its own:
  `Error::RemovalStopped`.** `Error::Failed` promises *the tree is as it was
  found — every effect this operation had applied was undone*, and an unlinked
  file cannot be put back; `Error::FailedPartiallyRolledBack` reports an unwind
  that failed, and here none is ever attempted. The new variant claims neither:
  it carries what stopped the removal and the paths that had already gone, and
  whether the tree is as it was found is read off that list being empty rather
  than asserted. `docs/ordinal-fs-tree/ARCHITECTURE.md`'s *Plan atomicity* now
  says explicitly that it is a property of plans and does not reach this
  operation.

- **`syllabus delete --yes` is the CLI's fourteenth verb, and its only
  destructive one.** stdout is one record — `.` and the root, the subject —
  and the entries go to stderr as their own trace in the order they went, which
  is the split every other mutating verb already makes between its subject and
  the consequences. The confirmation is a **flag** and not a prompt, because the
  binary's consumers are contract tests and scripts and an interactive `[y/N]`
  would make the one destructive verb the one thing they cannot drive. No new
  exit code: `RemovalStopped` lands on `6` when nothing had gone and `7` when
  something had, which is exactly the *as it was found* / *neither state*
  distinction those two rows already draw, and
  `RootIsNotSpelledDirectly` lands on `4`, whose definition it already fits.
  One consequence worth knowing: the default `--root .` does not delete, so the
  destructive verb is the one that cannot be run by leaving the flag off.

- **`root-lifecycle-stays-with-its-receipt` is reversed, and is now
  [`root-lifecycle-belongs-to-the-store`](docs/adr/root-lifecycle-belongs-to-the-store.md).**
  That record rejected library ownership of root destruction on three arguments,
  all of them about coordinating a destroy with an external effect through a
  four-point callback — machinery `delete-finish-transaction-k8` deleted. What
  is left is a destroy whose only verdict is the filesystem's own, which is the
  record's own stated reopen condition, met by the consumer it was written
  about. Its creation half had already been overtaken by `Vacancy::initialize`.
  The receipt argument survives and is narrower than it was read to be: grove's
  proof that a teardown happened is still a commit in the repository's history,
  which decides where the receipt lives and not who performs the removal —
  `Removed` is a postcondition, not a receipt. The record also moves context
  owner in `CONTEXT-MAP.md`, from grove to `ordinal-fs-tree`, and that move *is*
  the decision. `entries-are-never-removed` gains the clause distinguishing
  removing an **entry**, which stays impossible, from deleting the **root**,
  which ends the allocation its argument is about.

- **The tree store answers *is there a tree here* as a shape, and the vacancy can
  create one.** `ordinal_fs_tree::fs::read` and `fs::write` no longer return a
  guard directly: they return `Reading` (`Tree` / `Vacant`) and `Writing`
  (`Tree` / `Vacancy`), and `Vacancy::initialize` creates the root, its
  distinguished child and a first run of entries under the exclusive lock the
  vacancy already holds (`docs/specs/module-decomposition.md`, decision 2).

  **Why a shape and not an `exists()`.** A predicate beside an opening is a
  check-then-act split, and the act it splits from is creating a tree: between
  *there is no tree* and *make one*, another writer fits. One lock acquisition
  now answers the question and hands back the only operation valid for the
  answer — which also removes a class of call from the language rather than
  refusing it at run time. `initialize` is on the vacancy and nowhere else, so
  **initializing over a live tree does not compile**; the mutations are on the
  write guard, so **mutating a tree that is not there does not compile** either.
  Two `compile_fail` doc tests on `Vacancy` are the proof, and a compile-fail is
  the honest form of *not expressible* where a run-time test could only show that
  one particular bad call was refused.

  **Why `initialize` takes bytes rather than an entry.** `NewEntry` describes a
  positioned entry, and the distinguished child is the one entry that cannot be
  described that way — no parts, no ordinal, no key, and its name is
  `EntryName::distinguished()`. The library already writes one exactly like this
  when a promotion moves a leaf's bytes into a new node, so root creation reuses
  that primitive and the seam gains **no trait method**:
  `docs/adr/entry-name-is-the-only-seam.md` is untouched. Without it the consumer
  would write the root's own content itself, outside the lock and outside the
  store, at the first operation of every fresh tree. `None` makes a root with no
  distinguished child and empty bytes make an empty one — different trees, and
  the choice is the domain's. Bytes in a domain whose `distinguished()` is `None`
  are the **same refusal a promotion gives**, which is why
  `Refusal::PromoteNoDistinguished` is now `Refusal::NoDistinguishedChild
  { promoting: Option<Key> }`: one condition, two operations, and a root
  initialization names no entry because the root is not one.

  **A root that is neither a tree nor nothing is an error carrying what was
  found** — `Error::RootIsNotATree { root, found }` — and not a third variant. A
  regular file, a socket or a symbolic link naming one is not an answer a caller
  can act on, and the library will not move aside something it did not put there.
  A **dangling** link is that error too, not a vacancy: it resolves to nothing
  while plainly occupying the name, so `initialize` sent at it would collide.

  **The lock still covers everything, including the tree's creation**, because it
  has always been taken on the directory *containing* the root — and the route to
  that directory turns on **whether the kernel follows the last component**, not
  on whether the root resolves. Where the last component is a plain name (nothing
  at it, a regular file, a socket) the lexical parent *is* the directory holding
  it, so the two routes cannot disagree; where it is a symbolic link they can, so
  a link naming a directory takes `<root>/..` like any other accepted spelling
  and a **dangling** one is refused before any lock is taken — locking its
  lexical parent would put a caller through the link and a caller through the
  target path on two different locks over one tree, which is the defect
  `reading-k19` closed, re-entering through the door absence opened.

  Migration is mechanical: every `fs::read`/`fs::write` call site matches the new
  enum, or takes `expect_tree` where it has already established the tree exists.
  `syllabus` gains an `init` verb — the manual's *there is no `init`: an empty
  directory is an empty tree* was true about the format and wrong about the
  operator, since `mkdir` leaves the root's own OVERVIEW to be written by hand —
  and every other verb now refuses a root holding no tree rather than creating
  one on the way past, so a mistyped `--root` is a refusal and not a second
  course. Inside grove the three call sites in `src/task_tree.rs` raise the
  existing *grove root not found* diagnostic on the vacant arm; Grove still
  creates its own root outside the store, and moving that inside is
  `collapse-tree-access-k13`. `models/operations.qnt` gains `Initialize` as a
  transition — the only plan that *creates* a distinguished child rather than
  renaming an inode onto one — and `models/structure.als` gains the
  vacancy/tree/neither trichotomy; both runners are green.
  `docs/adr/grove-does-not-stage-its-own-renames.md` and
  `docs/adr/bulk-marks-are-not-atomic.md` were re-checked against a store that
  now owns root creation: the first is unchanged and says why a `mkdir` is
  covered by the same reasoning as a rename, and the second drops its claim that
  the library's operation set is closed, which this change falsified.

- **The tree store has a word for a search that matched nothing.**
  `ordinal-fs-tree` gains `Sought<T>` (`Match` / `Nothing`), and it is what
  *every* search on the public surface answers: `Snapshot::find` is now
  `Snapshot::seek`, and `Snapshot::by_key` returns a `Sought` rather than an
  `Option` (`docs/specs/module-decomposition.md`, decision 2, the fourth
  operation). The rename comes with the type rather than beside it — `find` is
  `Iterator`'s word, it is right there on `Walk`, and two operations one
  character apart answering in two vocabularies is the confusion *one word for
  one concept* exists to prevent.

  **Not a refusal, and that is the whole content of the type.** Every one of
  `Refusal`'s variants is a refusal to *mutate*; a search asked for no change,
  and a tree holding no match is not a damaged tree. A store whose only word for
  *matched nothing* is `None` makes each consumer invent its own — which is
  exactly what grove's `Option<SelectedLeaf>` is. The line drawn is between a
  **search**, which takes a criterion and scans, and an **accessor**, which reads
  an attribute off something already in hand: `Entry::key` and `Entry::contents`
  keep their `Option`, because their absence is a fact about the entry and not
  about any scan. `Sought::into_option` and `From` in both directions are the
  door back out, so a consumer's control flow stays its own while `Option`
  appears in no signature the library owns. `into_option` is the reliable
  spelling of the outward direction: `core`'s own blanket
  `impl<T> From<T> for Option<T>` also applies against an inferred `Option<_>`,
  so `sought.into()` is ambiguous unless the target type is written out — the
  impl says so where a reader meets it.

  The reference CLI is the worked example of the policy being the consumer's:
  `show 99` builds a `Refusal::TargetMissing` from the same answer that
  `list --first` renders as an empty listing. `docs/ordinal-fs-tree/CONTEXT.md`
  gains **search** and **sought** and the accessor distinction; `ARCHITECTURE.md`
  and `CLI.md` are reconciled. **Neither model moved**: a search adds no state
  transition, and `operations.qnt` already resolved a key with `leastId`'s
  `-1` — the in-band sentinel a typed answer replaces. Grove is
  *adapted*, not migrated: `task_tree.rs`'s one `by_key` call site maps
  `Sought::Nothing` onto the `Lookup::NotFound` it already produced, and grove's
  own `Option<SelectedLeaf>` stays where it is until `collapse-tree-access-k13`.

- **The runner now runs what it expanded: the channel, the spawn and the kill
  escalation are `crates/keyed-launch` too.** The crate gains `Channel`
  (`allocate` / `path` / `read` / `discard` / `discard_abandoned`), the free
  `signal(path, token)` its child end writes through, an opaque `Token`, and
  `run(Launch) -> Ended` — which spawns the expanded argv **directly, with no
  shell**, hands the child the caller's environment minus a caller-supplied
  scrub list plus the fresh channel path under a caller-chosen variable name,
  and supervises it to one of three ends: the child exits, the token appears
  (grace → SIGTERM → kill-grace → SIGKILL), or the launcher itself is signalled.
  `Argv` still has no constructor, so *nothing reaches a spawn that a template
  did not author* is now a fact about the types rather than a convention grove
  keeps. `crates/keyed-launch/tests/launch.rs` drives the whole of it end to end
  against a fake `sh` child with no grove anywhere in sight (test seam 1); the
  crate takes `libc` for `kill(2)` and `signal(2)`, which `std` does not reach —
  `Child::kill` sends SIGKILL and nothing else, so there is no graduated
  escalation without it.

  **`src/launch.rs` is gone**, absorbed into `src/loop_driver.rs`: `bare_grove`
  and the `LOOP_CONTROL_ENV` scrub list, which is now handed to the runner as
  `Launch::scrub` rather than applied by a grove helper — the one spawn allowed
  to *grant* the channel therefore cannot be written without first removing what
  it inherited. `src/driver_lease.rs` loses the signal-channel third (allocation,
  the `signal-<128-bit>` grammar, abandoned-channel cleanup, its nonce draw and
  their unit tests) and exposes the control directory instead: *which* directory
  is grove's is the lease's to say, and everything about what lives in it is the
  runner's. What the driver keeps is the four things a loop must choose and a
  runner cannot — the control directory, the variable name, the scrub list, and
  the two graces. `src/complete.rs` writes through `keyed_launch::signal` and its
  `read_signal` becomes `interpret(Option<&Token>)`: the runner carries the token
  opaquely and grove's stake in its content is one match, in one place.

  **Behaviour change, small and deliberate**: SIGTERM/SIGHUP are caught by the
  runner for the duration of a launch rather than by the driver for the whole
  loop. A signal arriving while a session runs behaves exactly as before —
  forwarded to the child, reaped through the same escalation, reported as
  `End::Interrupted`. One arriving *between* iterations is now collected by the
  driver at the top of its loop (`keyed_launch::take_interrupt`) and stops it,
  where before it was latched and spent on the *next* session — which had
  signalled nothing and was killed on its first poll. SIGINT stays the driver's,
  because what a loop does about the human's Ctrl-C is the loop's policy and not
  a runner's mechanism.

  **Five defects an adversarial read of the new supervision found, fixed here.**
  `End::Signalled` now means the escalation actually ran, not merely that a token
  appeared — which `token` already reported, so the two fields no longer say the
  same thing while one of them claims something stronger; a child that signals
  and exits inside its own grace comes back `Exited` with a token. A failing
  `try_wait` SIGKILLs and reaps the child before returning the error instead of
  leaving an interactive one holding the terminal with nothing left to reap it. A
  driver interrupt no longer restarts a kill grace that is already counting down,
  which used to *extend* a stuck teardown by five seconds per signal. An empty
  channel file — what a child killed between creating the file and writing to it
  leaves — is no longer read back as an empty token that grove's
  anything-unrecognised rule turns into a relaunch. And the terminal is restored
  on every path out of the loop body, not only the successful one, so an error is
  legible in the shell that has to display it.

  Three findings from the same read are **not** fixed here and are
  `child-signal-disposition-k31`: `SIG_IGN` for SIGINT is inherited across `exec`
  by the session and everything it spawns; the escalation signals the child and
  not its process group, so grandchildren survive; and a SIGTERM'd driver exits
  0. All three predate this change and all three alter what an interactive
  session experiences, which is not a thing to change blind.

  **`cargo clippy --workspace --all-targets` is the gate**, and the manifest now
  says so. This root is also a package, so the bare `cargo clippy --all-targets`
  the comment used to prescribe lints `grove` alone — which is how
  `crates/keyed-launch` came to carry two `clippy::err_expect` errors unnoticed
  since it was created, with the workspace `deny` baseline in force the whole
  time. Both are fixed.

- **The runner's template half is a crate, and it has never heard of a session.**
  `crates/keyed-launch` owns a configuration of `key -> complete command
  template`: the KDL grammar, whole-document validation against a *slot
  vocabulary* the consumer supplies at load, aggregate diagnostics with source
  locations, whole-word expansion into an argv, and a conformance kit
  (`keyed_launch::conformance::check`) that holds a consumer's configuration to
  the crate's contract from outside the consumer's own suite. A key is an opaque
  string and a slot is a name the consumer declares; the crate holds no set of
  either. `src/session_config.rs` falls from 673 lines to the part that could not
  move — the personal file's path, grove's four slots (`prompt`, `session_name`,
  `worktree`, `repo`), and the configuration delta's search and trackedness
  rules, which are questions about grove's worktree answered through grove's
  version control seam. Grove drops `kdl` and `shell-words` as direct
  dependencies.

  **Configuration presence is now per kind and just-in-time.** The personal file
  no longer has to declare all nineteen session kinds. The whole of both
  documents is still validated eagerly — before every tree mutation and again
  before every launch, for syntax, duplicates, node shape and every template
  rule, so a malformed entry for a kind this run will never reach still fails
  before anything is spawned. What moved is *presence*: before Grove writes a
  leaf of kind K, and before it launches K, K must resolve to exactly one
  complete template read whole out of one file. **What is lost is exactly the
  early warning for a kind not yet reached** — a stale configuration now fails at
  the first `leaf-add` of that kind rather than at the next tree mutation of any
  kind. What is bought is that adding a kind no longer wedges every operation in
  every stale configuration until each owner edits their file. The five verbs
  that write a leaf — `leaf-add`, `leaf-add-pair`, `leaf-insert`,
  `leaf-decompose`, `root-init` — now check before mutating, so a refusal leaves
  the tree byte-identical.

  **A key resolves only if the primary file declares it.** The all-nineteen rule
  was what made a partial second source safe: a delta could only ever override a
  kind the operator had already written down. That argument goes with the
  quantifier, so it is restated one kind at a time — the delta overrides and
  never supplies, and where only the delta declares a kind, the kind does not
  resolve and the refusal names it and the personal file that must declare it.
  `docs/adr/untracked-configuration-delta.md` now states that as its own property
  rather than borrowing one.

  **`unknown session kind` is no longer a diagnostic.** It was a claim about a
  closed key set, and neither the crate nor grove holds one; a key nobody uses
  costs nothing, and a key that is used must resolve. Configuration diagnostics
  say *key* rather than *session kind* for the same reason. `Kind` itself stays a
  closed enum for filenames.

- **The version control seam is a crate, and it has never heard of grove.**
  `crates/jj-workspace` is the workspace's third package and the whole of what
  grove knows about Jujutsu: resolve a workspace, refuse a working tree that is
  not one, hand a consumer a namespaced control directory, answer what is
  tracked, take a path-scoped commit. `src/repo.rs` and `src/repo/finish_commit.rs`
  (349 lines) are deleted, and with them every `jj` spawn in grove's production
  code — each child that speaks to the version control system is now started
  inside the crate.
  It takes **no dependencies**: `Refusal` is opaque and implements
  `std::error::Error`, which `anyhow` swallows through `?` without either crate
  knowing about the other.

  **Domain-freedom is enforced at a method, not asserted in a sentence.**
  `control_dir` takes the *consumer's* namespace — grove passes `"grove"`, and
  gets `.jj/grove/` back, created if absent and guaranteed shared with nothing
  else. The implementation that moved reached that path by hard-coding a
  grove-named directory inside jj's administrative one, which cannot be stated
  as a postcondition without naming the consumer; handing back the administrative
  directory raw was rejected too, since `driver.lease` and `session.epoch` are
  generic names in a directory jj owns and may extend, so the collision would be
  one release away and silent. `crates/jj-workspace/tests/` links the crate
  **without grove** — 28 tests over the public interface plus a separate binary
  for the one claim that has to mutate the process environment — which is the
  compiler-enforced form of the claim rather than a doc comment making it.

  **Two behaviours changed, both deliberately.** Resolving a workspace now also
  answers which workspace holds the repository, because the interface makes that
  answer infallible; it costs no subprocess in the common case, since jj marks a
  *borrowed* repository with a `.jj/repo` pointer file and one it holds with the
  repository itself — the same file-versus-directory shape Git uses for `.git`.
  And `is_tracked` now lets jj snapshot first, so the answer is about the tree as
  it is on disk rather than as it was at the last snapshot: a caller reads it to
  decide whether removing a file could be undone, and a stale answer is wrong for
  that. Measured (jj 0.44.0) rather than assumed — a snapshotting probe records
  an operation only when the working copy actually changed, which is the same
  snapshot the next jj command would take anyway, taken earlier and not twice.
  The finish gate is the caller that needed it: an untracked `.grove/` is refused
  before deletion, and that refusal is now correct for a tree committed but not
  re-snapshotted.

  **Grove keeps only what grove can say.** The refusal wording for `.grove/`,
  the name of its control namespace, and why a tracked configuration delta is
  refused stayed here; jj's remedies — `jj git init --colocate`, `jj undo`,
  `jj op log` — moved into the crate, where every refusal variant names what is
  wrong, where, and what fixes it. `scrub_internal_child_env` and the repository
  selector list went with them, leaving `launch::scrub_loop_control_env` as
  grove's own half: it removes the session-ending authority `GROVE_*` carries,
  which `jj` does not read.

  **Records reworked in place.** `one-live-driver-per-working-tree` — the lease
  survives; the Git-or-jj control-directory derivation, the same-device gate and
  the lost-result retry path do not, and the control directory becomes the
  namespace the seam hands back. `untracked-configuration-delta` and
  `jj-is-the-only-lane` are reconciled to the seam's probe and refusal.
  `docs/specs/module-decomposition.md` decision 8 lands as written.

- **The hand-built finish transaction is deleted; the version control system
  owns the transaction.** `grove-llm finish-commit` now deletes `.grove/` and
  takes one path-scoped `jj commit`, and grove implements no witness, manifest,
  rollback proof, index image, quarantine or recovery path around it. jj
  snapshots the working copy before every command and its operation log *is* the
  transaction record, so every guarantee the 10,400-line transaction hand-built
  was already available from the tool that owns it. Measured before it was
  relied on (jj 0.44.0, colocated): `rm -rf .grove/` with no jj command run then
  `jj restore .grove` returned every file, and a partial deletion then `jj undo`
  reported *"Added 2 files"* — exactly the missing ones.

  **What goes.** `src/finish_transaction.rs` (3,645), `src/finish_cleanup.rs`
  and `src/finish_cleanup/` (1,439), `src/test_barrier.rs` (the publication rule
  those seams shared), and the whole proof half of `src/repo/finish_commit.rs`
  (608 → 129): the `FINISHING-`/`PREPARING-FINISH-` witnesses, the evacuation
  manifest, the finish-attempt identity, the start anchor and deletion
  fingerprint, the three-way commit disposition, the rollback, the quarantine
  and its reaper. With them go the two witness prefixes as reserved names in the
  filename grammar, `TaskNameError::PendingFinish`, `tree_access::refuse_pending`
  and every reader's refusal of a tree "mid transaction" — a leftover
  `FINISHING-*` directory is now a foreign entry every verb walks past, the same
  answer `delete-migration-k6` reached for a stray `.grove/FORMAT`. The
  workspace-layout preflight goes too — `ensure_supported_workspace_layout`,
  `control_directory_device` and `repo::measured_device` with its
  `GROVE_TEST_FOREIGN_FILESYSTEM` seam — because it existed only to prove the
  quarantine rename would be same-device. `tests/finish_lifecycle.rs` (2,861) and
  `tests/workspace_layout.rs` (679) are replaced by `tests/finish_commit.rs`
  (424). `serde` and `serde_json` leave the dependency set with the manifest that
  was the only thing serialising anything. Net across the whole change: 7,026
  lines out of `src/` and 3,160 out of the suite.

  **What survives, and it is the point.** The teardown's *tree and VCS facts* are
  not what was deleted — the transaction around them is. `finish-commit` still
  revalidates, under the exclusive tree lock, that the live leaf is the
  driver-owned finish leaf the caller named and that no ordinary work appeared;
  still refuses a `.grove` that is a symlink, unfollowed; and still deletes and
  commits only `.grove/`, so unrelated working-copy changes stay uncommitted. One
  precondition is added rather than kept: an untracked task tree is refused,
  because the operation log can only restore what it tracks, and deleting one
  would be the unrecoverable kind. That is a gate that makes the VCS's guarantee
  applicable, not a transaction — it promises nothing and repairs nothing.

  **What a failure says now.** It stops with a message naming the operation-log
  command that puts the tree back — `jj restore .grove` if the deletion is what
  failed, `jj undo` if the commit is — and **no grove-authored recovery runs**.
  An absent `.grove/` is a plain refusal naming `jj op log`, no longer routed to
  a proof that some earlier attempt's commit was this launch's.

  **Records retired.** Four, not two. `task-tree-transactions-fail-closed`, whose
  witness protocol this supersedes — not by the reopen condition it named (a
  durable finish receipt) but because the version control system owns the
  transaction — and `supported-workspace-layouts`, whose whole subject was the
  same-device rename, are the two `docs/specs/module-decomposition.md` assigns
  here. Two more are retired because this change makes their decisions false:
  `finish-keeps-a-cleanup-layer-it-has-not-proved-forced`, which decided that the
  three nested crash-safe transactions all stay, and
  `success-is-proved-by-the-ticket-not-the-tree`, whose attempt-bound correlation
  ticket no longer exists. Their citations are reconciled in
  `one-live-driver-per-working-tree`, `untracked-configuration-delta`,
  `jj-is-the-only-lane`, `root-lifecycle-stays-with-its-receipt`, both specs,
  `CONTEXT.md`, `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`, `docs/USAGE.md`,
  `content/references/finish.md` and three historical documents. `CONTEXT.md`
  loses seven glossary terms — *Finish transaction*, *Finish-attempt identity*,
  *Evacuation manifest*, *Correlation ticket*, *Finish disposition*, *Recovery
  pending / Ownership conflict*, *Quarantine* and *Workspace layout preflight* —
  which have no referent left.

  **What is deliberately not deleted.** `complete_partial_root_unlocked`, the
  last of `minimalism-k1`'s roughly twenty-five auto-repair functions. The
  anomaly it repairs is one grove itself creates, in the two-phase root-init
  dance `collapse-tree-access-k13` deletes; turning it into a refusal now would
  make bare `grove` stop on a state its own `root_init` produced. It is noted on
  that leaf.

- **jj is the only lane.** Grove drives Jujutsu and refuses everything else: a
  working tree with no `.jj/` directory at or above it is now stopped by one
  precondition gate, before any mutation, with `jj git init --colocate` named as
  the remedy. Nothing downstream branches on which version control owns the tree,
  because nothing else can own it, and a `.git` beside a `.jj` is a colocated
  repository that is jj's business — grove never reads it and never spawns `git`.
  Recorded as `docs/adr/jj-is-the-only-lane.md`, which carries the rejected
  alternative that had to be argued rather than dismissed: narrowing the safety
  principle to *where the version control system can*, keeping the hand-built
  finish transaction alive on one lane.

  **What goes with it.** The plain-Git finish commit and its proof; the
  `GitIndexBackup` family and the whole colocated-index auxiliary protocol
  (`src/finish_cleanup/auxiliary*`, artifact-and-marker replacement, its ten
  rebind checkpoints and their two test seams); the empty-internal-hooks-path
  rule; the `Vcs` enum, `ControlMarker`'s three variants and the gitfile
  indirection behind them; and the `git ls-files` trackedness probe with the
  `GIT_INDEX_FILE` hazard that made it load-bearing. About 6,570 net lines out of
  `src/` and 1,720 out of the suite. `repo::vcs_of` becomes
  `require_jj_workspace`, a gate returning one refusal.

  **What this changes for an operator.** A plain-Git checkout cannot run grove
  until someone runs `jj git init --colocate` in it — one command, which keeps the
  Git history and leaves every Git tool working. Grove no longer promises anything
  about a colocated repository's Git index during teardown; jj owns it. The
  workspace-layout preflight survives and is simpler: `.jj/grove/` is always
  inside the working tree, so the only cross-device layout left is one where
  `.jj/` has itself been put on another filesystem, and the refusal names that
  directory.

  **Records reconciled in place.** `grove-does-not-stage-its-own-renames` loses
  its Git-lane consequences and gets simpler — jj snapshots the working copy, so
  a `DONE` mark is one whole rename with nothing to stage.
  `supported-workspace-layouts`, `task-tree-transactions-fail-closed`,
  `untracked-configuration-delta` and `one-live-driver-per-working-tree` lose the
  clauses this change falsifies. `content/references/commit.md` drops the Git
  boundary it can no longer produce, and `docs/USAGE.md`,
  `docs/ARCHITECTURE.md`, `docs/CONFIGURATION.md`, `CONTEXT.md` and
  `CONTEXT-MAP.md` stop describing a lane grove refuses.

- **The last of the formal-methods apparatus is deleted.**
  `crates/grove-finish/models/` — `finish.qnt`, `finish.als`,
  `finish-controls.qnt` and its `README.md`, 16,398 lines carrying the `FN-`
  obligation family — goes, and the directory `crates/grove-finish/` with it: it
  was never a crate, only a model directory borrowing the `crates/` path to name
  a context. The entry below deleted this column's only runner
  (`models/run.sh` mapped scope `finish` here) and enumerated everything except
  the column itself, so it had been left as apparatus with nothing to run it.
  Same terms, same record: `docs/adr/evidence-outlives-the-instrument.md`.

  **Nine records are reworked in place and none loses its finding.** Seven under
  `docs/adr/` cited these files as evidence —
  `finish-keeps-a-cleanup-layer-it-has-not-proved-forced`,
  `success-is-proved-by-the-ticket-not-the-tree`,
  `root-lifecycle-stays-with-its-receipt`,
  `a-shared-safety-claim-names-the-role-not-the-artifact`,
  `a-refusal-leaves-nothing-standing`,
  `a-closed-partition-is-over-outcomes-not-states` and
  `obligations-follow-context-not-artifact` — and each now says in
  `## What enforces it` what checks it today.
  `success-is-proved-by-the-ticket-not-the-tree` is the only one with a shipped
  answer, and it is a real one: `tests/finish_lifecycle.rs`'s three rootless-retry
  tests decide success by the teardown commit's attempt-bound message and refuse
  to read it off the tree. The other six name a reader or state plainly that
  nothing mechanical checks them, which
  `docs/adr/evidence-outlives-the-instrument.md` now records as the honest form.
  `evidence-outlives-the-instrument` and
  `a-witnessless-root-refuses-what-it-cannot-account-for` are the two others the
  sweep reached.

  **Two findings stop being open work.**
  `finish-keeps-a-cleanup-layer-it-has-not-proved-forced` reopened Q1 on a run
  that needs `sweep-ownership-k81` and `alloy-candidate-k82`; with no column to
  mirror and no `README.md` to start from, Q1 cannot be reopened on the terms it
  states, and it says so rather than leaving a reader to discover it. The
  `FN-24.a` / `FN-28` Alloy repair
  `a-shared-safety-claim-names-the-role-not-the-artifact` left outstanding is
  likewise not work anyone will do.

  **`obligations-follow-context-not-artifact` is kept, and the call is recorded.**
  Its whole subject — prefixes, the catalogue, the runner's placement rule, three
  model directories — is now gone, so deletion was the live alternative under
  `ADR-FORMAT.md`'s current-state rule. It stays because
  `docs/driving-a-checkable-loop.md` classifies four records as being about
  *evidence rather than about Grove* and this is one of them; its sibling
  `a-lifecycle-claim-says-what-it-is-over` lost its models one leaf earlier and
  was kept; and it is cited from `CONTEXT-MAP.md` and twice from
  `docs/formalism-findings.md`, once as the recorded outcome of a hand-off.
  Whether all four belong under `docs/adr/` at all is one question about the set
  rather than four about records, and is `spec-to-current-state-k23`'s.

  **The link repair is mechanical and the prose is not.** Fourteen relative links
  into the deleted files become plain names, which is what
  `tests/reference_navigation.rs` enforces; the backticked model paths in
  `docs/formalism-findings.md` stay, because they are accurate as history. The
  standing notes in `formalism-findings.md`, `candidate-lessons.md` and
  `driving-a-checkable-loop.md` now name both deletion leaves, `CONTEXT.md`'s
  *Root and lifecycle semantics* section says which leaf took which column, and
  `CONTEXT-MAP.md` records the finish column's departure beside the catalogue's.

- **The formal-methods campaign's apparatus is deleted; its evidence is kept.**
  About 23,800 lines go: `models/` (the `SY-` lifecycle column in Quint and Alloy,
  its controls, the whole-repository runner `models/run.sh` and its control
  harness `run-controls.sh`), `crates/grove-task-tree/models/` (the `TT-` column,
  and with it a directory that was never a crate), the two report generators
  `scripts/loop-record.py` and `scripts/review-yield.py`, and the claim catalogue
  `docs/specs/semantic-contract.md` that all of them were checked against. The
  campaign's value is the lessons it produced and the `linkuistics` skills those
  lessons became, and the runnable machinery had no consumer left. The rule is
  recorded as `docs/adr/evidence-outlives-the-instrument.md`, which carries the
  rejected alternative — deleting the records too — and why it was refused.

  **What was kept, and why.** Every record under `docs/` — `formalism-findings.md`,
  `candidate-lessons.md`, `review-yield.md`, `loop-record.md`,
  `preservation-baseline.md`, `driving-a-checkable-loop.md` and
  `results-of-formal-methods-trial.md` — because `plugins/linkuistics`' skills cite
  them for their authority, one through a public URL, and **a rule whose evidence
  has been deleted is a rule nothing can falsify**. Also kept:
  `docs/ordinal-fs-tree/models/`, which the catalogue delegated to rather than
  owned, whose own runners are self-contained and whose crate is live.

  **Every citation into the deleted set is reconciled rather than dropped.** Seven
  records under `docs/adr/` gain the paragraph
  `a-witnessless-root-refuses-what-it-cannot-account-for` established at
  `delete-migration-k6` — the decision outlives the instrument, and the citation
  moves to whatever checks it now. Four kept records gain a standing note that
  their model paths are descriptions rather than links. `docs/loop-record.md` and
  `docs/review-yield.md` are relabelled **derived and now frozen**, naming what
  that costs: `review-yield.md`'s judged half is no longer re-assertable, because
  its classification table lived inside the deleted script. `CONTEXT.md` loses
  **Obligation / sub-identity**, whose whole subject was the retired coverage
  matrix, and its *Formal contract* section is renamed for the terms that outlived
  it. `CONTEXT-MAP.md` drops `semantic-contract` from grove's specs, four to
  three. `docs/ARCHITECTURE.md`'s *Documentation ownership* gains the
  `loop-record.md` and `review-yield.md` rows the previous entry claimed but did
  not land, and its Alloy-against-Quint row no longer says the models are here.

- **The embedded methodology gains three rules, all measured on this
  repository's own formal-modelling campaign.** `references/execute.md` now
  carries *a control that has never been seen to fail is not a control* — show a
  positive or cross-tree control coming back dirty before crediting its clean
  read — and a new *The provenance of a measurement* section: finish every edit
  before measuring, digest every subject either side, one measurement one writer,
  never read a launcher's return as a background job's completion, and confirm a
  re-run item by item rather than by matching totals. `references/decompose.md`
  and `references/integrate-review.md` land the third: **an integration's body
  carries the review's handle, never its findings**, because a body that *is* the
  finding list makes that list the integration's charter and leaves it no
  structural place to reject one. `decompose.md`'s *"or the findings verbatim"*
  is gone; the producer's-doubt half of that sentence stands. Six rows added to
  `docs/specs/corpus-rule-ownership.md` and its single-source sweep. Evidence and
  falsifiers: `docs/candidate-lessons.md`, `docs/review-yield.md`.

- **`docs/ARCHITECTURE.md` now owns the campaign records.** *Documentation
  ownership* gains rows for `candidate-lessons.md`, `loop-record.md`,
  `review-yield.md`, `formalism-findings.md` and `preservation-baseline.md`,
  which were reachable only from `.grove/` — a directory `grove finish` deletes.
  The paragraph below the table no longer counts the files under `docs/`.

- **Grove no longer migrates task trees, and `.grove/FORMAT` is gone.** The
  session-kind migration, its fail-closed transaction, the
  `MIGRATING-session-kinds` witness, its focused commit and the format witness
  are all deleted — `src/tree_migrate.rs`, `src/tree_migration_transaction.rs`,
  `src/tree_format.rs` and `src/repo/migration_commit.rs` — about 1,800 lines of
  non-test source and 1,600 of tests, and around 5,500 lines net across the
  change. A tree whose names the current grammar cannot spell is now **refused
  by name**: `TaskNameError` already carries the filename on disk and the shape
  it should have had, which is what an operator needs in order to rename it or
  start a fresh grove. Existing current-format trees are unaffected and need no
  action; a tree still carrying body `**Kind:**` markers must be brought current
  by an older `grove` before upgrading, or renamed by hand.

  Three consequences worth knowing:

  - **`.grove/FORMAT` is no longer written, read or required.** The filenames are
    the format. Deleting it from an existing tree is optional and can wait: a
    stray `FORMAT` is an ordinary foreign entry every reader ignores, which is
    also what lets a tree stay readable by both this build and the one before it
    while an upgrade is in flight. `grove-llm root-init` now prints two paths —
    the charter, then the leaf — where it printed three.
  - **A partial `root-init` is recognised by shape rather than by content.** A
    root holding its charter and no keyed entry is the one thing bare `grove`
    completes; the byte-exact match against the deterministic fresh-tree content
    existed only to tell that shape apart from a legacy tree, and went with the
    thing it was discriminating against.
  - **A root holding no grove entry at all is refused rather than scaffolded
    over.** The withdrawn `NNN-slug/` + `done/` and v1-flat layouts are positioned
    but unkeyed, so every one of their names is *foreign* — invisible to the
    reader rather than refused by it — and such a tree would otherwise read as an
    empty grove and take the driver's finish sentinel. The refusal names the
    entries it disclaimed and the grammar it does read. It no longer names *which*
    withdrawn layout this is: that per-layout classifier was migration's.

  Also gone with it: the `work` → `impl` and bare-`research` read-side aliases
  (both were migration's lenient parser; `--kind` still names the replacement
  when you type the old word), and the driver's second configuration load within
  one iteration is no longer separately covered — nothing mutates configuration
  between the two now that the transition takes no commit. Decisions:
  `docs/adr/task-tree-transactions-fail-closed.md` loses its migration half;
  `docs/ARCHITECTURE.md#no-migration` states what replaces it.

## v19.3.0

- **`leaf-retire` and `leaf-prune` no longer stage their rename, and on Git that
  changes what you see before the commit.** Both verbs now mark through
  `ordinal-fs-tree`'s `rewrite`, which renames with `rename(2)` and detects no
  repository, so a tracked leaf marked `DONE` shows in `git status` as an
  unstaged deletion at the live name beside an untracked file at the marked one,
  where a `git mv` previously showed a staged rename. The commit is unaffected
  **provided it stages the tree** — Git infers renames at diff time by content
  similarity — so `git add -A` before committing, which is what
  `content/references/commit.md` now says; `git commit -a` records the deletion
  alone. Jujutsu is unaffected and always was. Decision:
  `docs/adr/grove-does-not-stage-its-own-renames.md`.

- **`leaf-prune` on a node directory is no longer one critical section.** A
  library mutation consumes its write guard, so a subtree of *N* live leaves is
  *N* guards. The whole subtree is still planned and validated against the first
  guard before any rename, so a prune that cannot complete still fails with
  nothing renamed; what is lost is the window between guards, where a concurrent
  writer or a filesystem fault can now stop the run part way. The verb is
  re-runnable — an already-`ABANDONED` leaf is skipped — which is the repair.
  Decision: `docs/adr/bulk-marks-are-not-atomic.md`.

- **Fixed: a tree with a duplicated key marked the wrong entry and reported
  success.** A mark names its target by permanent key, and two entries under one
  key — a hand edit, or a rollback that failed — made the choice between them
  arbitrary. Both verbs now refuse such a key before doing anything, naming both
  paths and what to do about it.

## v19.2.0

- **A project may override personal launch policy per session kind, with an
  untracked `.grove.kdl` configuration delta.** It is searched at the worktree
  root and then the main repository root — the two paths `${worktree}` and
  `${repo}` expand to — the first one found is *the* delta, and the two are never
  merged with each other. It declares any subset of the nineteen kinds and each
  one it declares replaces the personal file's entry outright, so a kind's launch
  is still one complete template string read whole out of one file; every kind it
  omits falls through, and `~/.config/grove/config.kdl` stays mandatorily
  complete and fully validated. The motivating case is balancing account usage
  across vendors within one workstream
  ([#10](https://github.com/Linkuistics/grove/issues/10)). Decisions:
  `docs/adr/untracked-configuration-delta.md`, and the rework of
  `docs/adr/complete-session-configuration.md` it split from.

- **A tracked delta is refused rather than trusted to an ignore rule.** The file
  names a program to execute, so a repository that could ship one would choose
  what Grove spawns in any checkout of it; Grove asks the VCS owning the
  candidate one read-only question, jj-first and `--ignore-working-copy`, and
  only when a candidate file exists. Unreadable, unparseable, invalid, tracked,
  or unanswerable-by-probe all fail closed at **both** load points — before every
  tree mutation and again before every launch — with the same aggregate
  diagnostics the personal file gets, reported against the delta's own path, line
  and column. Grove still writes no ignore rule and creates or edits no
  configuration file; `docs/CONFIGURATION.md` names the `/.grove.kdl` line to add.

- **Fixed: a spawn failure named the personal config path unconditionally.** It
  now names the file the failing kind actually resolved from, which after a delta
  need not be the personal one.

## v19.1.0

- **`content/SKILL.md` is a condition register, not a methodology.** It drops
  from 3,152 words to 867: eight rules it owns outright — the routing table, the
  numbered spine, one-task-is-one-session, the bootstrap order, the mandate, no
  second pick, the stated VCS statement, the HITL mark — plus 26 canonical
  trigger sentences of at most 25 words each, and nothing else. A rule bound to
  one kind or one family now has no statement here at all, which is most of the
  drop and deletes nothing from the corpus: every such rule is stated by the file
  the ownership inventory assigns it, and reached by a trigger. Four rules that
  had lived only in `SKILL.md` moved to their owners rather than being orphaned —
  the one-focused-commit scope and the Retire-first reason to
  `references/commit.md`, node-close-is-implicit and pruning-is-HITL's agent-side
  half to `references/retire.md`. The budget is asserted rather than intended:
  a 900-word ceiling with no floor, exactly 26 triggers matched against the 31
  canonical rows both ways (every row claims one sentence, every sentence is
  claimed by one row), and the eight owned rows named individually — so a
  dropped rule fails the suite even when the word count still looks healthy.

- **The ten session-kind references are incremental.** Each states its
  deliverable, its permissions, its special verification and its unique human
  gate, and nothing a sibling, loop-step or format file already carries.
  `requirements.md` sheds the glossary-inline mirror and a duplicated always-form
  bullet; `design.md` loses the OR-form ADR test outright, because
  `ADR-FORMAT.md` owns that test; `planning.md` and `combine-research.md` each
  state a rule once where they had stated it twice; `research.md`, `impl.md`,
  `integrate-review.md` and `prototype.md` gain the rows they were missing.
  `review.md` and `finish.md` needed no change.

- **The requirements interview threshold is resolved, and the resolution is
  subordination rather than a compromise.** A `requirements` session always
  establishes *what* — that is unconditional and stays so — but the full
  one-question-at-a-time grilling procedure runs only for three or more
  interdependent open questions; below it, record the decisions and proceed.
  Stated once, in `content/references/requirements.md`, with `execute.md`'s
  always-form restatement removed in the same commit. Its behavioural test pins
  the off-by-one ("only above three") and the reversal as named counterexamples,
  because a topic match on this rule passes for both.

- **The seven loop-step references are rehomed onto the ownership map.**
  `execute.md` loses *What each kind produces* entirely — a nineteen-kind summary
  of material the kind references own, written for a session the driver has
  already routed — and gains the repo-claim disciplines, decisions-land-as-they-
  settle, escalation-names-the-trade-off and the review budget's predicate and
  per-kind table. `decompose.md` gains fog-or-ticket, vertical-slice,
  expand-contract, the prior-art-research rules and the two shape-selection
  rules, each stated once in place of unowned duplicates. `retire.md` takes
  triage-picks-the-verb, prune-scopes-to-the-whole-path and
  finish-is-the-drivers-to-discover, whose occasion set makes Retire the earlier
  step. `grove.md`'s argued spine and glossary rationale move to
  `docs/ARCHITECTURE.md`, and its build-pairing section now separates the build
  boundary — real, and enforced — from the pairing, which Grove reports and
  cannot enforce, so a skew diagnostic reads as a finding rather than as a
  contradiction of `one-build-owns-a-session`.

- **The lifecycle invariants are held by behavioural tests rather than by prose
  bulk.** `tests/lifecycle_invariants.rs` asserts the inventory's behavioural
  rules across nine areas — no second pick, no VCS reprobe, stale launch, the
  decomposition boundary, human-only pruning, retire → commit → complete, the
  review budget, all three finish-signal outcomes, and the interview threshold —
  over the loaded path a session of each kind actually opens, composed from
  `src/prompt.rs` rather than enumerated beside it. What "behavioural" means for
  a `cargo test` is decided rather than left open: it asserts *delivery*, not
  conduct, recorded in `docs/adr/behavioural-coverage-asserts-delivery.md`. A
  claim counts as delivered only when it is stated on the static path or in a
  paragraph that states that rule's own situation, so two triggers naming one
  owner no longer stand in for each other; every claim carries its own near-miss
  control, verified red against the pre-fix file.

- **`plugins/install.sh` honours declared harness compatibility.** Every bundled
  `SKILL.md` declares `harnesses:` — `[any]` for the fifteen portable skills,
  `[claude-code]` for `guardrail`, whose whole mechanism is a Claude Code
  `PreToolUse` hook that a codex or pi session cannot act on. The installer scans
  both plugins rather than `linkuistics` alone and reports every skip with the
  declaration that caused it. Absent metadata installs nowhere and says so: 15 of
  16 skills are portable, so installing everywhere would mis-install exactly the
  one that cannot work, and installing Claude-Code-only would withhold the other
  fifteen. A re-run now reconciles rather than only adding — a link whose skill
  was deleted, renamed, or made ineligible for that harness is removed and
  reported.

- **No Grove rule defers silently to the optional `linkuistics` plugin.** The
  audit was re-derived against the corpus as it now stands rather than executed
  from the design's table: the rewrites above took 14 citations across 9 files
  down to 6 across 4, discharging four of them on the way. Two sites were
  genuinely open, and both failed the same way — `references/commit.md` said
  Grove states "only where its boundary falls" and `SPEC-FORMAT.md` said it "only
  says where the agreement gets recorded", while both already carried the
  complete operative rule. A reader could not tell a complete statement from a
  teaser, which is a silent dependency in its purest form; the repair is the
  sentence rather than new content.

- **`linkuistics`: the skill corpus is rationalised — two skills gone, two turned
  into routers, three compressed.** The loaded path is what was too large, not the
  corpus, so the cuts fall where a skill was paying standing context for detail a
  session rarely needs. `using-jujutsu` (1,908) and `git-to-jj-mapping` (1,421)
  become **one** skill of 1,214 words over two reference files: the body keeps only
  what decides behaviour — which interface the repo picks, working-copy-as-commit,
  the describe-early lane, that a bookmark never advances onto a sealed change and
  must be repointed before a push, non-interactive discipline, the human-gated list,
  and the colocated git-read-only policy — and the command surface plus the whole
  git→jj lookup table move to `references/`. The split is by *what a session would
  do differently*, not by topic: a fact that decides an action stays in the body
  even where its commands live in the reference. `using-codebase-memory` (2,609 → 849 +
  a reference) keeps the mechanics of making a *correct* call (which surface, the
  required `project`, `jq -n` argument building, the `cm` guard wrapper) and routes
  the failure-mode catalogue, which is detail you read when a result looks
  surprising rather than context you carry all session.

  `codebase-design` (1,536 → 992), `decision-records` (1,345 → 935) and
  `simplify-project` (1,191 → 1,155) are compressed by deleting passages, not by
  paraphrasing them smaller — two ASCII diagrams, a *Relationships* section that
  restated the glossary above it, and two *Rejected framings* lists whose entries
  each restated a rule already stated. Both bodies grove's methodology defers to
  survive intact: the ADR **AND** test and the minimum-coherent-set discipline in
  `decision-records`, and what a seam is and how to judge one — the deletion test,
  two-adapters-mean-a-real-seam, and the dependency-category table — in
  `codebase-design`.

  Generic **`coding-style` is deleted**. It carried `paths: "**/*"`, so it was the
  one skill auto-loading on every file in Claude Code, and its content was either
  advice a model already follows ("use descriptive names", "keep files small") or
  overlap with the six language skills — with two exceptions, both rehomed rather
  than dropped. Naming one concept one way across a codebase is now a
  `codebase-design` principle, where naming *is* interface surface; **test-first**
  was not overlap at all (no language skill stated it) and is now the first line of
  each of the six language skills' *Testing* sections — Rust gained the section it
  had never had. The consequence for a language with no skill here — Go, Java, C —
  is that it triggers no linkuistics coding standard at all, which
  `plugins/README.md` now says outright rather than leaving a deleted universal
  skill to imply otherwise. Each language skill gains a leading sentence making the
  **repo's own configuration the authority**, naming that language's real config
  files, with the house defaults demoted to the fallback for a repo that has not
  decided; asserting them unconditionally was wrong in any repo that had.

  `cli-tool-design` gains an **Applicability** section instead of losing words
  (1,403 → 1,675): a rule asserted for every tool in every language is either
  trivially true or frequently wrong, so the checklist now names the three things
  that excuse a line — the tool's shape, its audience, and an established
  convention it already follows — and the mandates state what they apply to. The
  audit reference is scoped the same way, because a finding against a guideline the
  tool was never bound by is noise.

  Two consequences for anyone with the symlink install. The set is now **15
  skills**, so `./plugins/install.sh` should be re-run; and because it links what
  exists and never unlinks what doesn't, the links for `coding-style` and
  `git-to-jj-mapping` survive that re-run pointing at nothing. A dangling link
  reads as "skill not installed" rather than as an error, so nothing reports it —
  `plugins/README.md` now carries the one-line `find` that lists them.

- **`content/driving.md` is gone, and the supporting corpus is split along
  grammar / policy / rationale.** The habits file was the corpus's largest (5,817
  words) and on **no session's loaded path** — nothing under `content/` named it,
  so only `SKILL.md` and the ten kind reference files are ever reached statically
  and a rule surviving only there was already deleted in effect. Its surviving
  imperatives were rehomed first, row by row, against
  `docs/specs/corpus-rule-ownership.md`'s relocation table; the two sections
  addressed to the **human** driving a grove — *ask the LLM "WDYT"*, *ask for
  pushback* — moved to `docs/USAGE.md`, which already has that audience; the rest
  was argument, worked example and provenance, and is in the VCS.

  `TASK-FORMAT.md` (3,012 → 935 words) keeps only what constrains bytes on disk:
  the five-field name grammar, the nineteen kinds, the body shape and
  *convention-not-grammar*. The composition shapes, the doubt-budget table, the
  kind disciplines, *a leaf never names a harness*, the research output-path table
  and the commit-subject imperative each state a rule some other file owns, and
  each left for it. Two global rules that were never format grammar left with
  them — *one task is one session* to `SKILL.md`, and the pruning duplicate to
  `references/retire.md`.

  The **ADR when-to-write test now has exactly one statement in the corpus**, in
  `ADR-FORMAT.md`, which also takes the minimum-coherent-set discipline and the
  research→ADR bridge locally rather than by citation — a test that is only cited
  is no bar of Grove's own, and the AND/OR contradiction this workstream opened on
  is what a cited-only test produced. `CONTEXT-FORMAT.md` takes the
  glossary-sharpening rules, and `grilling.md` becomes the interrogation procedure
  plus a Grove-authored entry condition, its bundled `<what-to-do>` block
  byte-intact and its four duplicate sections pointing at their owners rather than
  restating them.

- **The corpus is held to per-kind loaded-path budgets, and two prose-shape
  alarms are gone.** `tests/methodology.rs`'s 500-line ceiling on `SKILL.md`'s body
  and its 100-line alarm on the loop section are deleted; `tests/loaded_path_budgets.rs`
  measures what a session actually reads instead. Neither was *dominated* by the
  900-word ceiling that survives — a word limit bounds no line count — so the
  ground for deleting them is that a **line** is not a unit anyone reads and a
  **section** is not a unit anyone loads: a section budget is discharged by moving
  prose across a heading, a line budget by rewrapping. A **loaded path** is per-kind and
  has two halves: the *static* path — the guaranteed core, `SKILL.md`, and that
  kind's reference file, read unconditionally — and the *reachable* path, which adds
  the transitive closure of the pointer graph the rule-ownership inventory records.
  Nineteen kinds share ten reference files, so the budget is a table of nineteen
  rows rather than a number.

  **It measures through the runtime, not beside it.** The core comes from
  `prompt::compose` and the reference file from `prompt::reference_file`, so a
  budget cannot drift from the prompt a session is actually handed; the kind's
  signal file is identified by matching `prompt::ending_of`'s bytes against the
  embed, which reads that seam without widening it (`src/prompt.rs` is unchanged).
  Each budget is asserted from **both** sides — the corpus stays under the ceiling,
  and the ceiling stays within measurement + 25% — so a limit nothing approaches
  fails as loudly as a path that outgrew one. That two-sidedness is the specific
  thing the 500-line ceiling could not do: it sat at 500 over a body the rewrite
  had estimated at ~200 lines, and passing it was evidence of nothing. The band
  has real width on purpose: set at +10% and allowed to +25%, a ceiling tolerates
  ordinary editing in both directions, where a band with no width made the two
  checks contradict each other — the over-budget failure says "move a procedure
  out", and doing so shrank the measurement and tripped the other one.

  Budgets are in **words**, not tokens, and the reason is stated rather than
  implied: a reproducible token count needs a vendored tokenizer and vocabulary,
  and a budget that needs a download stops running. The limit is stated too — a
  word count cannot price a register change, so the reading is always "this path
  grew".

  Four assertions over the inventory's **load-predicate** column ride with it.
  `static(K)` is checked against the runtime, so a row claiming a session reads a
  file unconditionally that the runtime never puts in front of it fails — the check
  the superseded `always(19)` labelling would not have survived. Reachability is
  asserted as an **edge**: the triggering file must literally name the owner's path,
  every chain must terminate at a static path without a cycle, and every non-static
  owner must have at least one incoming edge — which is what catches an owner
  nothing points at, the state `content/references/driver.md` was left in. The
  conditional rows are **partitioned first**: 45 of the 92 record an in-file
  condition rather than a transition, and running the cycle check over the
  unpartitioned set would make each a self-loop and fail on half the design. Two
  schema checks come free — a row reached from `SKILL.md` declares the `trigger`
  class, and every cited trigger sentence number exists in the canonical 26.

  **The acceptance comparison is recorded in `docs/ARCHITECTURE.md`**, which now
  carries the before/after table per kind rather than a claim about it. Every
  kind's unconditional read is between a quarter and two-fifths of what it was —
  835–1,585 words excluding the guaranteed core, against a measured 3,108–3,944.
  That before-range is itself a correction: the workstream's brief carried
  "roughly 3,200–3,700", which is wrong at both ends, so the ratios are computed
  per kind against measured figures rather than against an estimate. What none of
  it establishes is that the paths still carry the rules: that is the behavioural
  coverage's job, and a green budget over a corpus that lost a rule is a smaller
  path that teaches less.

  The routing-table check stays, on grounds the other two did not have: it is not a
  shape measure but a reachability claim about the first screen, asserting that ten
  kind→file pairs resolve.

- **`docs/RELEASING.md` spells out the manual cut.** `release.toml` already said
  that a refused `cargo release` is a per-session fact about the harness rather
  than anything inherent, and to fall back to the constituent commands — but it
  named those only for the publish step. The v19.0.0 cut hit the refusal and had
  to derive the rest, so it is written down: the three edits `cargo release`
  makes, and the `jj describe` / `bookmark set` / `jj new` / `git tag` sequence
  that produces the same two artifacts. The `jj new` is the part worth having in
  writing — a colocated `HEAD` follows the working copy's *parent*, so without it
  the tag lands one commit early and `release-build.sh` refuses with a message
  about the tag rather than about the cause.

## v19.0.0

- **Breaking. The v1-flat task-tree layout is no longer migrated either, and
  migration is now a rename in place.** With the `NNN-slug/` layout already
  withdrawn (below), `<position>-[<key>]-<slug>[.BRIEF|.DONE].md` was the second
  of three legacy inputs and the other one that **relocated** entries: its flat
  keyed files became node directories, and every `# <dotted>-[<key>]-<slug>`
  header was rewritten down to the position-free handle. Withdrawing it takes the
  reader, the renderer that placed entities into directories, the header
  rewriter, and the whole `src/leaf_id.rs` module — the v1 name parser, which was
  deliberately *kept whole* under a rule that a frozen grammar is not trimmed to
  what its one caller happens to use. That rule was right while something read
  the grammar; once nothing did, keeping a 500-line parser to answer a yes/no
  question was the reverse of what it argued for. Same treatment as before: the
  shape is still **classified**, so bare `grove` names it and refuses rather than
  reading it as a tree with nothing in it, and what survives of each withdrawn
  reader is a private name matcher in `tree_migrate`.

  **One legacy input remains** — a tree already in v2 directories whose leaves
  predate filename kinds. It is the shape the migration was always going to end
  at, and it needs no relocation: each leaf is renamed *inside its own directory*
  to gain the kind its body's `**Kind:**` marker declared, and the marker lines
  come out of the body. So **no migration creates or removes a directory** any
  more. That is now an asserted property rather than an implicit one, because two
  successive versions of the same transaction test — one asserting directory
  removal, one asserting directory creation — each went vacuous as its layout was
  withdrawn.

  The migration transaction keeps its rollback destination-directory sweep even
  though nothing this build plans can trigger it. The asymmetry with the forward
  sweep removed below is deliberate: recovery runs off the manifest, never off a
  fresh plan, so a witness left by an older, relocating build can still be
  finished or rolled back by this one — and what each sweep would leave behind
  differs in kind. An unswept *source* directory is `done/` or `020-node`, which
  no reader parses, so it is inert litter; an unswept *destination* directory is
  `NN-<slug>-k<key>/`, which every reader parses, and an empty one is a node with
  no brief and no children — a malformed tree.

- **Breaking. The original `NNN-slug/` + `done/` task-tree layout is no longer
  migrated.** It was the most expensive of the three legacy readers and the one
  furthest from anything in use: converting it meant building a unified forest —
  merging the parallel `done/` mirror into the live tree by logical path, since a
  retired leaf lived physically outside the node it belonged to — and then
  assigning every key fresh in DFS pre-order, because that layout carries none.
  Roughly 200 lines of reader, plus the fixtures that pinned its exact key
  assignment. **v1-flat and kind-less v2 trees migrate exactly as before**; only
  this one input is withdrawn.

  **The shape is still recognised, and that is the change rather than a
  leftover.** Deleting the detection along with the reader would have let such a
  tree classify as having nothing to migrate, whereupon migration installs the
  `.grove/FORMAT` witness over it — and every entry in that tree is then foreign,
  so `pick` reports a finished grove and a workstream is silently gone. So bare
  `grove` classifies the layout, refuses before any mutation, and names the
  entries that put the tree in that class, in the same shape as the existing
  ambiguous-layout and unknown-kind refusals. What survives of the reader is
  `leaf::split_prefix`, a prefix recogniser whose result nothing consumes as tree
  content. Converting an affected tree by hand and re-running is safe: the
  refusal writes nothing, commits nothing, and launches nothing.

  One further removal follows from it rather than being bundled with it: the
  migration transaction's *forward* sweep for emptied source directories is gone,
  because that layout's `done/` mirror and `NNN-slug/` nodes were the only
  sources that were ever directories. Every remaining input has flat sources, so
  the sweep could find nothing. The rollback sweep — over *destinations*, which
  are directories in both remaining inputs — is untouched and is where
  `remove_empty_directories` keeps its coverage.

- **The repository carried two current-state design corpora, and now carries
  one.** `docs/specs/config-driven-sessions.md` and
  `docs/specs/skill-delivered-methodology.md` were the design records for the
  v17 and v18 increments. Both shipped, and both went on describing the same
  running system as `docs/ARCHITECTURE.md` — one grain finer, in parallel prose,
  with no rule saying which one a reader should believe when they disagreed. Two
  owners for one subject is the drift risk, and it does not need a measured
  disagreement to be worth closing. Everything still binding already had a home:
  the too-late test is a section of ADR *skill-delivers-the-methodology*, the
  leaf grammar's non-prefix invariant and the reserved-not-blocking finish rule
  are in the task-tree data model, and the specs' `## Requirements` were long
  since discharged as the tests that cite them. What only the specs carried moves
  to the architecture guide: the corpus's condition/procedure split and why the
  routing table opens the skill (*The corpus's shape*), the three size alarms —
  500-line body, 100-line loop section, 4 KiB prompt — and the argument that each
  is an alarm rather than a budget, and the embed test seam that explains why
  `methodology` and `prompt` are `pub`. Their `## Problem` narratives and
  `## Removed surfaces` inventories are history, which this file already holds.
  `docs/specs/doubt-grove-review-mechanics.md` stays: it describes a composition
  *between* two contexts and so outlives the increment that wrote it, which is
  what `CONTEXT-MAP.md` now records as the membership rule for the set.

- **The documentation prose sweep is deleted.** `tests/legacy_claim_sweep.rs`
  enumerated every occurrence of the removed launch vocabulary across the docs
  and required each to sit inside a committed quotation judging it a refutation
  rather than a claim. That was the right shape while the removal was landing.
  Two releases later what it still bought was a build that breaks when the
  *wording* of a documentation sentence changes — the table hard-coded some thirty
  sentences, which is why the fold above would otherwise have been a test
  failure rather than an edit. The behavioural half of the same claim stays and
  is where the value always was: `tests/removed_surface.rs` drives a real bare
  `grove` launch with the whole legacy environment set to values that would be
  catastrophic if read, in a Git worktree and a jj workspace, and asserts the
  launch is byte-identical to a clean run. Its `GROVE_SESSION_TARGET` row went
  with the sweep, reported by its own stale-entry check — the name reached that
  table only because the sweep had to spell it.

- **The module-visibility exemption list said two and the sweep reports three.**
  `src/lib.rs` and `docs/ARCHITECTURE.md` both stated that, after the visibility
  pass, exactly two surfaces survive the production reachability check. Running
  the technique they document — copy `src/` to a scratch crate, make every module
  private but `cli` and `llm_cli`, read the compiler's warnings — reports
  `leaf_id`, `tree_lifecycle::transition_to_current`, **and**
  `methodology::markdown_files`. The third was never unargued; its own doc
  comment claims the seam exemption by name, and it was simply missing from the
  two tallies. Both now say *two kinds of surface*, with every reported item
  falling under one — which is the claim that was always true and the one that
  survives the next module joining the list.

## v18.4.0

- **`${prompt}` carries a guaranteed core, and the provisioned skill delivers the
  methodology again.** A session no longer receives ~49 KiB of sliced
  methodology; it receives about 1.9 KiB — an instruction to load the `grove`
  skill and this kind's reference file, named by path, with the provisioned
  directories given as absolute paths; the two facts the driver resolved (the
  selected handle and the working tree's version control); and the session-ending
  text, which is its kind's signal file's own bytes inlined byte-exact. The order is
  the session's own timeline. Both failures this sits between are **measured**,
  not argued: the wall degrades behaviour (sessions finish and fail to signal,
  stalling the loop), and the skill alone was demonstrably not read. What earns a
  place is settled by the **too-late test** — a sentence rides `${prompt}` only if
  its failure mode is one the skill cannot repair — closed on the word *fact*, so
  the runtime facts arrive as bare values and every normative consequence of them
  (*the pick is authoritative*, *do not probe for the version control*) stays in
  `content/SKILL.md`, which now states both. The load instruction's wording is the
  micro-test's winning arm verbatim, ablated to the three elements that arm
  measured ([`wording-micro-test`](docs/research/wording-micro-test.md)).
  *The skill delivers the methodology* is that decision, reworked in place from
  the record it reverses. (That record was retired at `delete-provisioning-k19`,
  which is why this entry names it rather than linking to it.)

- **Two signal files serve the nineteen kinds, and every prompt still has three
  parts.** Eighteen kinds end exactly one way, and `content/SIGNAL.md` says so.
  A `finish` session has three endings chosen by what it did, so inlining that
  file for it would put a fixed *run `grove-llm complete`* last in the prompt of
  the one session that may have just torn the grove down — which the driver then
  re-scaffolds. Dropping the part instead would answer that by removing the very
  too-late-shaped instruction the core exists to carry, in the session where a
  forgotten signal costs most: teardown completes, `--done` is forgotten, and the
  loop waits on a session that will not end. So the three-outcome table gets an
  embedded source of its own, `content/SIGNAL-FINISH.md`, which
  `content/references/finish.md` routes to rather than restating, and the core
  inlines those bytes. One source, two deliveries. `--done` therefore appears in
  exactly one kind's prompt, inside its ending, and nowhere else in any prompt.

- **New `prompt` module seam; `methodology` narrows to the embed and the
  identity.** `methodology::compose` and the per-kind composition golden are
  deleted with the mandate they built, as is `content/MANDATE.md` — the framing
  unit that told a session what a composed mandate was. `prompt` **depends on**
  `methodology` rather than absorbing it, so provisioning's supplier never sits
  behind a prompt-composition seam. The kind→reference-file map is an exhaustive
  `match` over the kind enum in the driver, so a twentieth kind fails to compile
  until someone classifies it, and every path it yields is asserted against the
  embed. The 4 KiB size alarm lives in the suite rather than the build, for the
  reason the old classification alarm did, and every generated claim carries a
  control.

- **The mandate machinery is deleted, and `content/` is plain markdown again.**
  Gone: the unit-marker grammar and its total partition, the 164 `<!-- unit: -->`
  markers and 27 `<!-- file: order= -->` directives throughout `content/`, the
  fence-state parser, the whole-embed check (id uniqueness, `defers=` resolution,
  procedural reachability, deferral termination), the `build.rs` gate and the
  `#[path]` sharing that fed it, and `grove-llm methodology` with its listing.
  Nothing read a marker after the cutover, so this is pure removal. `build.rs`
  keeps its per-file `rerun-if-changed` walk, which was always the other half of
  what it did and is what makes an edit to `content/` reach the binary at all —
  `tests/methodology.rs` now compares the linked embed against the directory so
  that walk failing is visible. What the gate genuinely bought moves to the suite
  at the grain a corpus of prose can be checked at: every kind's reference file
  exists and is reachable from `SKILL.md`'s routing table, and the body stays
  inside its progressive-disclosure budget. `docs/specs/mandate-delivered-methodology.md`
  goes with the last machinery it described — the timing its own rule set, so no
  live code was ever left citing a record that had stopped describing it.

- **The two checks that are claims about the *embed* rather than about the
  machinery survive, re-based.** The **instructed-verb scan** — the embedded
  methodology instructs no `grove-llm` verb the embedded CLI lacks — already read
  the corpus as markdown file by file rather than unit by unit, because a unit
  boundary could cut a wrapped invocation in half; it is *more* load-bearing now,
  since the skill is once again the only thing teaching a session which verbs
  exist. The **flat-verb-surface pin** that makes that comparison mean what it
  claims is untouched. The **ending-prose drift pin** moved from two unit ids to
  the two files' own bytes, which is the same subject named a coarser way. The
  **family-scope guard** had no such re-basing and is simply gone: a `kinds=`
  scope a human had to widen by hand is no longer a thing `content/` contains.

- **The glossary is reworked as a current-state set.** *Methodology unit* and
  *File directive* describe deleted mechanism and are removed. *Mandate slice*
  stays a retired entry carrying only the three arguments that outlived it — the
  reopen refusal, the byte-exact inline, and why the core names a reference file
  instead of printing the routing table. *Triggering unit / procedural unit*
  becomes **Condition / procedure**, the live `if`/`then` split the
  progressive-disclosure skill is cut along, recording that the 140 markers were
  scaffolding for that rewrite and were deleted once they had done their work.
  *Build pairing* needed nothing: its skew was never marker-shaped.

- **`one-build-owns-a-session` gets its original skew back.** The shared mutable
  directory returns with provisioning, so a mismatched pair is two copies of a
  whole methodology rather than a split-brain inside one rule — and it is **quiet**
  rather than loud, which is precisely what the pre-launch pairing report and the
  per-verb stamp warning exist for. The record's substance is untouched; the
  compile-time methodology-identity constant stays deleted, because `grove-llm`
  links the embed for a second surviving reason.

## v18.3.1

- **`leaf-retire` and `leaf-prune` name the session's two remaining steps.**
  Sessions were doing the work correctly — artifact, `DONE` rename, commit — and
  then never running `grove-llm complete`. Under the interactive harnesses the
  configured templates launch (no `-p`, no `exec`), a turn that ends without the
  signal does not end the *session*: it returns to its prompt, the driver's
  watcher waits on a signal file that never appears, and the loop **stalls**
  rather than stopping. The instruction already existed — `skill-signal` — but it
  arrived in the mandate at session start, a whole session's worth of context
  before the moment it applies. So the terminal-marking pair now emits it at the
  moment of decision instead: `leaf-retire` is the last grove verb a session runs
  (Retire precedes Commit, and the commit is jj/git), so its output lands exactly
  where the reminder is useful, under every harness and with no personal
  configuration involved. It goes to **stderr** — stdout is data, and callers
  parse the printed paths — and `leaf-prune` emits it only when it actually
  marked something, once for a bulk node mark. The grow verbs stay quiet: they do
  not end a session's work. `src/complete.rs`'s doc comment, which read the
  no-signal case as "the loop stops", is corrected to say what it is — the
  driver only ever *observes* that when the session process itself ended, and an
  agent that forgets the verb never reaches it at all. That misreading is what
  made this failure mode hard to see.

- **The Signal step composes last, from `content/SIGNAL.md`.** The other half of
  the same failure: `skill-signal` lived in `content/SKILL.md` at position 2, so
  the sentence telling a session to run `grove-llm complete` as its **last
  action** was followed by seven whole files — the majority of the mandate —
  before the session had done anything at all. Composition order is a file's
  position then a unit's offset within it, so which file carries a unit is its
  only lever on where it lands; the unit therefore moves, byte for byte, into a
  new tenth `content/` file at the corpus's final position. Appending is the one
  insertion that renumbers nothing. `content/SKILL.md`'s loop narrative now reads
  Retire → Commit → Finish with nothing left in the gap: the unit self-locates
  ("once the task is retired and committed"), and a pointer left behind would
  either paraphrase a rule that has one home or duplicate the eighteen-label
  scope to keep itself out of a `finish` mandate. The runtime facts the driver
  authors still follow it — two lines, where seven files used to be. Pinned by
  `the_relaunch_ending_composes_last`, which asks the mandates that *carry* the
  unit rather than a spelled kind list, and by the composition golden, whose
  eighteen moved rows are the whole diff.

## v18.3.0

### Added

- **`<!-- file: order=<n> -->` — an embedded file declares its own mandate
  position.** The first line of every `content/` markdown file's body, and the
  key a composed mandate is ordered by: the file's position first, then each
  unit's own offset within that file. It is the same device and the same
  recogniser as a unit marker — an unindented whole line at neutral fence state —
  so `content/` gains no metadata language for ordering. Its two rules land on
  opposite sides of the build gate's existing split, because they need different
  amounts of the corpus: that a file *carries* a position is decided by that
  file's own text, and that positions *differ* only by the assembled set. Both
  fail `cargo build`, naming the file and offset. The directive is the **second
  and last** region no unit covers, and it is bounded by position rather than by
  judgement — one line, first — so total partition survives as a claim by being
  restated over **units**: every unit is in a mandate or reachable from one, and
  units cover every body byte past the directive, with the unread preamble and
  this one line named as the two exempt regions, neither of which can hide a
  unit. A **gap is legal**, because the composer sorts by the key rather than
  indexing on it — but it is not insertion slack: `content/`'s own run of 1–9 is
  a readability convention about the shipped corpus, pinned by a test rather than
  by the gate, so inserting a file between two others *does* renumber every later
  one, in one-character edits the gate localises by name.
- **A `kinds=` scope that names a family is held to the whole family.** Three
  markers spell a family of session kinds out label by label — the five
  `review-*` kinds, the five `integrate-review-*` kinds, and the vendor pair's
  two — because the grammar admits no family shorthand, and a kind added to one of
  those families had to be hand-added to the marker with nothing complaining. Two
  reds already existed for a twentieth kind, at `Kind::is_producer` and at the
  session-ending guard, and **neither pointed here**; the composition golden does
  not either, since a new kind produces a *new* golden section rather than failing
  an existing one. Each of the three is exactly a **family partition** of the kind
  set, so the guard derives what the scope should be instead of restating it: an
  exhaustive `match` classifies every kind into a family — the same device, and
  the same reason, as the producer classifier it sits beside — and the assertion
  is that the marker's scope reaches exactly that family's members, read through
  the composer's own `admits` so the claim is about reach rather than about
  spelling or order. A twentieth kind now fails to *compile* until it is
  classified, then fails the assertion naming the marker to widen and the label it
  lacks. The marker stays the single statement of scope; all that is written
  beside it is which family it claims to be. The check runs in **both
  directions** — a triggering unit scoped to exactly a multi-member family and
  named by no registration is reported too, because the registry would otherwise
  be the same silent omission one level up. That report takes an **authored
  answer either way**, since equal reach is evidence of family intent and not
  proof of it: registration says a kind joining the family joins the marker, an
  exemption beside it says it does not and carries the reason, and an exemption
  that stops excusing anything is reported like a registration that stops matching
  anything. A third check holds the classifier *itself* to the label
  taxonomy — the producer classifier's second half, which this one had claimed
  rather than carried — because a match forces a decision and not a correct one,
  and a kind filed in the wrong family passes both membership checks while leaving
  its own family's marker narrow. Every classifier carries both controls, on the
  rule that a sweep which cannot fail is worth nothing. `skill-signal`'s eighteen
  labels are a complement rather than a family and keep the ending guard that
  already covers them; the sixteen single-kind scopes are self-correcting and get
  nothing, since a kind added later arrives with its own units.
- **`Kind::ALL` is held to the enum it enumerates.** The array was hand-written
  and nothing forced a new variant into it: `label`, the producer classifier and
  the family classifier all make an author classify a twentieth kind, and every
  one of them still compiled with `ALL` left at nineteen — while every sweep in
  the crate and the suite reads the kind set *through* `ALL`, the count test
  included, which counts what `ALL` holds rather than what the enum declares. So
  an omitted kind was invisible to all of them at once. Each variant now claims
  its position in `ALL` by an exhaustive `match`, which leaves a twentieth with
  only a constant index past the end to claim — rejected at **compile time** under
  rustc's deny-by-default `unconditional_panic` lint, naming the length `ALL`
  actually has.

### Changed

- **A session's mandate is the methodology composed for the kind it was launched
  as.** `${prompt}` used to be one embedded launcher file — the same eleven lines
  for every session, with the methodology itself reaching the session only as a
  provisioned skill it was pointed at. The driver now passes the kind it already
  resolved into `methodology::compose`, which selects every triggering unit whose
  scope admits that kind, orders them by `(file position, offset within the
  file)`, and joins their **source bytes** with a single blank line. Nothing is
  paraphrased: driver-composed prose would make `content/` non-canonical, and a
  slice cannot contradict what it copies. Onto that the driver appends the only
  two facts nothing in `content/` can express — the selected leaf's stable handle
  and the stated VCS — which is also why the handle paragraph is now a statement
  and not an errand: it reads *the leaf selected for this session is `<handle>`*,
  where it used to tell the session to resolve and execute it and not to call
  `grove-llm pick`. Both of those instructions are composed methodology
  (`skill-bootstrap`, `skill-do-not-pick-again`), so saying them again in Rust put
  one rule in two places with nothing keeping them in step. Composed mandates run
  around 47–50 kB per kind, against a 64 KiB alarm that exists to catch a
  procedural body marked triggering rather than to bound argv. Composition drift
  is pinned by a golden of each kind's **ordered unit ids** rather than its bytes:
  nineteen ~48 kB byte goldens would move on every prose edit under `content/`,
  and a golden regenerated every session is a golden nobody reads, while ids carry
  all four drift shapes — a unit gained or lost, a scope widened or narrowed, a
  file reordered, a unit moved within its file — and none of the churn.
- **`content/prompts/continue.md` is now `content/MANDATE.md`, and carries no
  instructions.** One change to what a session receives rather than two: every
  instruction the launcher held duplicated a `kinds=*` unit that composition now
  delivers into the same prompt — Bootstrap, Decompose, Retire, Commit, Signal
  and Finish — so the duplicate was removable exactly when composition began
  delivering them, and the one clause that stated something composition does not
  ("use the grove skill") is the clause that goes false when provisioning
  retires. What is left is framing: what the mandate *is*, and what its
  completeness guarantees, which is **structural** — every unit marked triggering
  for this kind is present byte-exact, and what is withheld is marked either
  procedural or for another kind, so no *condition* was held back for the session
  to discover the existence of. Recognising that a delivered condition **is met**
  is stated as still the session's own, because the invariant settles inclusion
  and never detection. `grove-llm methodology <id>` is named there as the way to
  fetch a body a marker defers, so the id to ask for is always in the text the
  session is already holding. `provision::continue_prompt()` lost its only caller
  and is gone; the framing unit keeps its class, scope and file position and moves
  its file, id (`continue-launcher-framing` → `mandate-framing`) and body.
- **v18.2.0's "nothing consumes a unit yet" no longer holds.** That release
  shipped `grove-llm methodology` as an inspection tool over an embed nothing
  read, and said so twice. The composer is now that reader, and the verb has a
  second, non-inspection job: serving a session the procedural body its own
  mandate deferred. The entry it corrects is left as it was written, because the
  release it describes shipped that way.
- **Both delivery paths are live, so this lands structurally now and
  behaviourally next.** Global skill provisioning is **untouched**: every session
  still also receives the whole unsliced `content/` as a harness skill, so nothing
  is yet *withheld* from any kind: the specialisation changes what a prompt
  **carries**, not yet what a session can reach, and this increment is therefore
  verified by the composed mandates themselves rather than by watching a session
  behave differently. Retiring provisioning is the next increment, and the
  mandate-delivers-the-methodology decision sequenced it that way. (That record
  has since been reworked in place into *the skill delivers the methodology*,
  which reverses it — provisioning stays and the mandate goes — so this entry is
  left describing what the release it names shipped against, and is not a
  citation of current design.)
- **Each kind's mandate states exactly one session ending, and eighteen of the
  nineteen never see `--done`.** The methodology told every session both endings
  and the condition between them — run `grove-llm complete`, unless this is a
  `finish` leaf, in which case `--done` — which is a branch on a fact the driver
  resolved before the session existed. `skill-signal` is therefore **narrowed**
  from `kinds=*` to the explicit eighteen non-`finish` kinds and its branch
  deleted rather than relocated: every sentence it held is the relaunch ending or
  the mechanism that ending runs on, so there was no universal remainder to keep.
  `skill-finish` genuinely spanned both and splits three ways — a `kinds=*`
  remainder holding the negative trigger (*you do not discover that a grove is
  finished; the driver tells you by launching a `finish` session*) together with
  the clause telling every session that its own escalations are discretionary,
  then `skill-finish-cycle` (sentinel mechanics and the human gate) and
  `skill-finish-endings` (the outcomes, carrying the deferred teardown steps),
  both `kinds=finish`. The negative trigger stays universal deliberately:
  withholding it would attach an unasked question to a destructive action. Two
  units that restated the ending lose it — `skill-self-driving-loop` keeps what it
  genuinely owns (one fresh foreground session per task, zero engine state,
  restart as continuation, no daemon) and `skill-finish-cycle` keeps its
  instruction without its ending, with the driver's own contract documented in
  `docs/ARCHITECTURE.md` rather than kept as a second copy in `content/` that
  nothing holds in step. `grove-llm methodology` accordingly lists 140 units
  rather than 138.
- **A `finish` session that externalised work signals a plain relaunch** — newly
  stated, not newly legal. `pick` already passes the sentinel over while ordinary
  work is live and `grove-llm complete` was never gated by kind, so the path
  worked and nothing said so, while the same methodology tells a `finish`
  session — like every session — to externalise surfaced work rather than absorb
  it, which leaves it unable to tear down. The three endings are now stated as
  **outcomes of what the session did**: teardown completed → `grove-llm complete
  --done` and the loop stops; work externalised instead → plain `grove-llm
  complete`, the loop relaunches and picks the new leaf, and the sentinel waits
  without banking the confirmation; declined or no human present → no signal, the
  loop stops and the leaf stays live and resumable. This is a branch on what
  happened *during* a session, which no driver can resolve — which is exactly why
  it stays where a branch on kind does not.
- **The ending is guarded across all nineteen composed mandates.** Generated from
  `Kind::ALL`, so a twentieth kind fails loudly instead of launching sessions that
  never signal the loop: exactly one declared ending unit per kind, `--done` in
  the `finish` mandate and nowhere else, the completion verb named only by that
  mandate's own declared ending — so an ending unit introduced without being
  declared cannot hide behind the declared one's membership count — and the
  negative trigger present in all nineteen. Both classifiers carry positive
  controls, on this repository's own rule that a sweep which cannot fail is worth
  nothing. The explicit eighteen is spelled out rather than negated because a
  shorthand would silently absorb a kind added later and move the classification
  into a test, away from the marker it belongs beside; its mirror hazard — a kind
  silently *omitted* — is what these checks close. What no test can reach is
  pinned instead by a **hand-edited** byte constant over the two ending units'
  source, and hand-edited is the point: a `GROVE_TEST_UPDATE_GOLDENS=1` pin can be
  cleared without reading the new prose, and reading the new prose is the whole of
  what it is for. Its claim is bounded to what a two-unit constant can actually
  see — that the `finish` endings read as outcomes. The wider limb, that no unit
  anywhere restates an ending phrased around the verb, is carried in three parts
  instead: a new unit moves the composition golden, one naming the verb fails the
  complement sweep, and one phrased around the verb is caught by nothing
  mechanical and belongs to the classification review — recorded as an asserted
  non-detection in the sweep's own control, in the function a later reader would
  edit, so that turning the sweep into a phrase heuristic has to be deliberate.

## v18.2.0

### Added

- **`grove-llm methodology` — the embedded methodology, addressable by unit.**
  `content/` now carries an HTML-comment **unit marker** per marked span, and
  the new verb serves those spans out of the binary's own embed: given ids it
  writes their source bytes verbatim in the order given, and given no argument
  it lists every unit as five tab-separated fields (`<id>`, `<class>`,
  `<scope>`, `<defers>`, `<file>`, with `-` in either optional field). Every id
  a row carries is a fetch argument unchanged. It mutates nothing and resolves
  no working tree, so it answers from anywhere — including the environments a
  tree verb is refused in. This is an **inspection tool**: under provisioning a
  session still receives whole documents, and nothing consumes a unit yet (the
  mandate-delivers-the-methodology decision, since reworked in place into *the
  skill delivers the methodology*, which reverses its delivery half).
- **A malformed embed fails `cargo build`.** Every embedded markdown file must
  be fully classified — units partition its body, the marker grammar is fixed,
  and a runaway fence or an unclosed leading `---` block is an error — reported
  with the file and offset. Fences are recognised by CommonMark's own rule rather
  than a trimmed approximation of it, because a close accepted too freely returns
  the reader to neutral inside a code block and promotes an example marker to a
  real unit. A leading `---` is **reserved** for the opaque preamble, since
  nothing distinguishes it from a thematic break (write a leading rule as `***`),
  and a unit marker inside the block is an error rather than bytes silently
  skipped. Two file-level rules join them, each so a stated contract stops being
  an assumption about today's tree: a file ends in a newline, so concatenated
  fetch output keeps every marker on its own line, and an embedded path carries no
  control character, so the listing's five tab-separated fields need no escaping.
  The gate reads through the crate's own parser rather than a second
  implementation. Grove's compile-time artifact is not the human's task tree, and
  the build that produced it can see the whole of it.
- **...and so does a malformed embed no single file could have caught.** Four
  more classes fail `cargo build`, each needing the whole assembled unit set: a
  **duplicate id** anywhere in the embed, which would make one of the two units
  unaddressable by the verb that addresses only by id; a **`defers=` naming no
  declared unit**, or naming a `class=triggering` one, which would hand a session
  a condition where a procedure was promised; a **procedural unit no chain of
  deferrals reaches**, which is partition seen from the other end — an
  undiscoverable procedure is deleted from the methodology as surely as prose no
  parser can see; and a **chain of deferrals that returns to a unit it has
  already passed through**, whether after one hop or several. Reachability does
  not subsume that last one, and treating it as though it did was the hole: a
  ring of procedures deferring only to each other is entered by no triggering
  unit and fails as unreachable, but a ring a trigger *does* enter is reached
  like any other chain, and a session walking `defers=` out of its mandate is
  sent round it forever. The two run in that order, so an unrooted ring is still
  reported as the orphan it also is. Each failure is reported with the file and
  offset, and with the second site named where there is one — for a ring, the
  deferral that closes it and the ids in the order a session would walk them. The
  unknown id a *caller* passes to `grove-llm methodology` is unchanged and stays a
  runtime error — it is a caller's mistake, visible only when the call is made.

### Changed

- **A composed shape's steps drop the step suffix from their slugs.** A review
  chain is now cut as `<stem>` / `<stem>` / `<stem>` — three leaves differing
  only by kind and key — and `grove-llm leaf-add-pair <parent> <stem>` emits
  `research-a-<stem>`, `research-b-<stem>` and `combine-research-<stem>` rather
  than appending `-a`, `-b` and `-combine`. **The kind field is the canonical
  statement of a leaf's role, and the slug names the artifact rather than
  restating it.** All five markers were a 1:1 restatement of the kind sitting
  beside them, so each was a second and *unvalidated* source of truth for a fact
  grove already parses and routes on: nothing rejects `leaf-add <parent>
  foo-review --kind impl`, and when the two disagree the slug lies while the
  filename tells the truth. **Nothing was migrated and no format changed** — the
  suffix was always convention rather than grammar, so both spellings remain
  legal filenames and every existing leaf keeps the slug it was created with. The
  one cost is that a bare stem stops naming one leaf: `grove-llm resolve <stem>`
  on a chain reports the ambiguity and lists all three matches with their
  kind-bearing paths, which is usually the answer wanted. `resolve` keeps its
  **pick-style contract unchanged** — empty stdout, the diagnostic on stderr and
  exit zero — so a command substitution around a chained stem succeeds with an
  empty value rather than failing. Every *recommended* reference is unaffected,
  because the driver's mandate, the `**Reviews:**` / `**Integrates:**` lines,
  commit messages and grow-verb targets all name a `<slug>-k<key>` handle, a bare
  key or a path, and keys are unique tree-wide; the bare slug that `leaf-add` and
  `leaf-insert` *also* accept as a target convenience is the one reference that
  loses its step, and there ambiguity is a refusal naming the matching keys. One
  consequence belonged to the pair alone and is settled from the same principle:
  `research-a` and `research-b` would otherwise write the same
  `docs/research/<slug>.md`, so the kind supplies the discriminator — `-a.md`,
  `-b.md`, and the unadorned union for `combine-research`. No ADR: the decision is
  cheap to reverse, so the reasoning is durable in `content/TASK-FORMAT.md`
  instead.
- **`linkuistics` / `doubt-driven-development`: the escalation example drops the
  step suffix too.** The skill's *Composition with Grove* rule tells a picked
  producer to escalate a second review need with `grove-llm leaf-add <parent>
  <stem> --kind review-<producer>`, where it previously spelled the slug
  `<stem>-review`. Same rule, same budget, same predicate — the example now emits
  the slug the methodology teaches, so a session following the skill and a session
  following `content/TASK-FORMAT.md` cut the same filename. Logged separately
  because the plugin is installed on its own and is delivered by commit rather
  than by a grove release.
- **Both binaries now link `content/`**, `grove` to extract it and `grove-llm`
  to serve units, so both compute their **methodology identity** from the linked
  embed directly. The compile-time constant existed precisely so that naming the
  identity did not link the embed; that reason ended here, and the build-script
  hash traversal, `GROVE_CONTENT_HASH` and the equality test that kept two
  traversals in step went with it. `grove-llm --content-hash`, the pre-launch
  pairing report and provisioning's stamp are unchanged in behaviour and value.
  The release path's binary scan inverts to match: it now requires the embed in
  **both** shipped binaries.
- **`content/` is really classified now — 138 units, not nine placeholders.** The
  marking that landed with the build gate covered each embedded file with a
  single whole-file unit, which was enough to exercise the parser and nothing
  else. Every file is now subdivided into the actual triggering/procedural split,
  with `kinds=` scopes and `defers=` targets, across the whole nine-file embed. So
  `grove-llm methodology` lists 138 rows whose class, scope and deferral are
  real, and fetching a triggering unit shows the id of the procedure that
  completes it. **Nothing consumes a unit yet** — under provisioning a session
  still receives whole documents — so what changed for a caller is the inventory
  the verb serves, not what any session reads.
- **`content/` loses the prose only a reader of a *file* could use.** The loop's
  mermaid overview, `SKILL.md`'s `## Reference files` index, and `driving.md`'s
  title and `## In this guide` anchor index are gone, along with several
  near-verbatim restatements of rules their owning documents already state. Their
  rows and anchors name **files**, and `grove-llm methodology` addresses
  **units**, so each promised navigation the delivery path cannot honour; every
  rule they carried is stated by the unit that owns its step, and what is lost is
  a picture and a table of contents. One claim went rather than moved — the
  CLI-design wizard anti-pattern, prior-art evidence cited to a survey this
  repository does not contain, for a rule `leaf-add`'s flag-driven interface
  already embodies. `tests/reference_navigation.rs` lost with it the ~140 lines
  of heading-anchor machinery that existed only to keep the index honest.

## v18.1.0

### Added

- **The mandate states the working tree's VCS**, so no session detects it. Every
  session bare `grove` launches is now told, beside its handle, whether its tree
  is jj-enabled or plain Git, which root the driver resolved, and not to probe
  for it or trust a harness banner that disagrees. The driver already resolved
  the fact before the session existed and every tree verb already branches on
  it; only the session was re-deriving it, from a banner computed off `.git`
  alone that reads a native jj workspace as no repository at all
  ([claude-code#41435](https://github.com/anthropics/claude-code/issues/41435)),
  or from skill instructions a session can skip. The line carries identity and
  root only — each lane's commit commands stay in the embedded methodology's
  Commit step, so there is one source of truth across the build boundary. Nothing
  else about a launch changes: it is not a template word, not a verb, and no
  existing configuration has to be edited to receive it.

## v18.0.0

**Breaking.** Review composition is now **flat and lazy**. Existing task trees
keep working untouched — a chain node was only ever an ordinary node directory
whose slug ended in `-chain`, and nothing keyed on that token — but the two
verbs that built one are gone, so a session or script that called either must
change. There is no migration, and none is needed.

### Removed

- **`grove-llm leaf-add-chain` and `grove-llm leaf-promote-chain`.** A chain's
  steps are now cut one at a time with plain `leaf-add`, by the session that
  needs the next one, so the first verb would emit only a producer and the
  second — which existed solely to retrofit a chain node around a picked
  producer without changing its handle — collapses to one append.
- **The `PROMOTING-*` fail-closed transaction**, with its witness, recovery
  path, and every reserved-prefix refusal that named it. Escalating review no
  longer moves a producer, so nothing needs preserving across an interruption.
  The `FINISHING-*` finish transaction and the session-kind migration witness are
  untouched. `PROMOTING-*` is no longer reserved: a directory left by an
  interrupted promotion under an older binary is now an ordinary foreign entry
  every reader skips. **Recover it with the old binary before upgrading** — it
  has the recovery path that knows which phase it died in. Afterwards the repair
  is by hand and depends on that phase: the witness may hold only generated steps
  with the producer still in place (nothing to move), or the producer itself
  (move it back to its original position and name), or the producer plus a Git
  index entry for a child that never landed (`git rm --cached` that path too).
- **The chain node**, and with it the second node species. Both composition
  shapes are flat siblings named off a shared stem, so **every node carries a
  `BRIEF.md`** again and the Retire cascade's `BRIEF.md`-presence discriminator
  is gone — a node close now has the same work to do everywhere.
- **Generated `**Reviews:**` / `**Integrates:**` lines.** The markers survive as
  a documented convention in `content/TASK-FORMAT.md`, written by hand by the
  session authoring the body. Nothing writes them and nothing parses them, which
  is constraint 3 (task files are freeform markdown; nothing validates them).

### Changed

- **One build owns a session: the pairing is repaired where Grove owns it and
  reported everywhere else.** The methodology a session reads and the `grove-llm`
  it invokes must come from one `grove` build, and the global skill directories
  are where that can be broken from outside — they are shared by every build on
  the machine while the driver lease is scoped to one working tree. The old guard
  could not see it for two independent reasons, both gone: it compared crate
  **versions**, which do not move between a released binary and an edited
  checkout at that same version, and it resolved the **sibling** of its own
  executable — a binary that agrees with the driver by construction, and the very
  thing that hides the case `cargo run --bin grove` creates. In their place:
  `build.rs` emits the content hash of the embedded `content/` file payload as a
  compile-time constant, so a binary can name its **methodology identity**
  without linking the embed (only `grove` extracts content, so only `grove`
  carries it, and an in-crate test pins the build script's traversal against the
  runtime hash of the embed); `grove-llm --content-hash` prints it — a flag, not
  a verb, since no session calls it — exempt from the session-epoch guard exactly
  as `--version` is; each loop iteration re-verifies every installed skill
  directory's stamp and restores this driver's embed when another build has taken
  one, naming the directory; and each iteration resolves `grove-llm` through
  `PATH` — resolving a relative or empty entry, and running the probe, from the
  **worktree root** the session is spawned in rather than from wherever bare
  `grove` was typed — compares identities, and **reports** a missing,
  unidentifiable or mismatched binary while launching anyway. Inside a session,
  `grove-llm` warns on any verb when a skill directory's stamp is not its own,
  and never refuses.
  **The one property deliberately given up**: a `grove-llm` the driver cannot run
  used to be a hard stop before `.grove/` was created, and is now a printed line.
  That is intended — the driver never invokes `grove-llm`, a wrapper or `ssh` hop
  or container that re-derives `PATH` is supported policy, and behind one of those
  the probe is a proxy rather than an observation; a missed mismatch misleads one
  session, while a false refusal launches nothing at all on a machine that may be
  configured correctly. Each diagnostic names what its branch can know — a
  mismatch names the resolved path and both identities, an unidentifiable binary
  names the resolved path, this build's identity and why it could not answer, and
  a missing one names this build's identity and the search performed, there being
  no path or peer to name — and all three state the requirement: the build being
  driven must be the one a session's `PATH` resolves first, rather than
  prescribing `cargo install --path .`, which achieves that only where
  `~/.cargo/bin` outranks every other prefix holding a `grove-llm`. Concurrent
  groves at different builds remain unsupported, now announced rather than
  silent. See
  `docs/adr/one-build-owns-a-session.md`.
- **A producer's last act is to decide whether review is required** and, if so,
  `leaf-add` the `review-<producer>` leaf itself; a review's last act is to
  `leaf-add` the `integrate-review-<producer>` leaf **only if it has findings
  worth acting on**. A review that finds nothing creates nothing and simply
  retires, which removes the empty triage session. The larger payoff is that the
  creating session writes the new leaf's body, so it can carry the exact
  uncovered case or the findings verbatim — strictly more than a constructor
  rendering a goal sentence from a handle could supply.
- **`grove-llm leaf-add-pair` emits three flat siblings** at consecutive
  positions instead of a `<stem>-pair/` node, and prints three paths rather than
  four. It stays a single all-or-nothing call: **the research pair remains eager
  on purpose**, because a `research-b` cut by `research-a`'s own session would
  inherit that session's framing and corpus and destroy the independence the pair
  is run for.
- **A chain is no longer contiguous by construction**, and grove enforces
  nothing: steps land at the parent's next free position, so one cut after
  unrelated work lands after it, and a sibling `leaf-insert` can split a chain.
  Grove validates no cross-leaf grammar and `pick` is a walk, not a scheduler.
  **The cost of a gap is not the same at both hops**, though, and the methodology
  now says so instead of leaving the call to unaided judgement. A `review-*` step
  re-derives — its body names the producer's stable handle, task commits name
  their work item by that handle, so it locates the producer's commit and reads
  that diff against the current source — and nothing it consumes was written down
  for intervening work to stale, so `leaf-add` is right for it wherever it lands.
  An `integrate-review-*` step consumes citations its review already froze into
  prose, against a working tree that has since moved, and the drift is
  **silent**. So an integration is cut where `pick` reaches it next, by a
  condition that is mechanical and directory-local: **`leaf-insert` at the first
  sibling entry after the review whose subtree still holds live work**. An
  *entry*, because the walk descends a node directory in place, so a later
  sibling node with a live descendant blocks and is itself the target; terminal
  leaves, wholly terminal nodes and the driver's `finish` sentinel do not block;
  and when nothing blocks, `leaf-add` is right, because pre-order finishes the
  review's own directory before any later sibling of an ancestor. There is no
  exception to check — the blocking work has not run when the leaf is cut, and no
  leaf's eventual file set is part of its contract. Nothing enforces any of this;
  it is a rule for the session cutting the leaf.
- **A freshly created leaf's body is the bare template** — the stable handle and
  empty sections — with no rendered goal and no relationship line, so the
  creating session has nothing to edit around.
- ADRs *grove-owns-escalated-review* (escalation resolves to `leaf-add`) and
  *task-tree-transactions-fail-closed* (loses `PROMOTING-`, keeps `FINISHING-`)
  are reworked in place, and `docs/specs/doubt-grove-review-mechanics.md` with
  them.
- **Provisioning is documented as a build boundary, not a commit boundary.** The
  methodology is embedded with `include_dir!`, so a session reads the
  `content/` its own binary was *built* with; the idempotence stamp hashes that
  embed rather than any working tree, and a warm no-op against a checkout whose
  `content/` has moved ahead is correct. The docs previously said only that the
  binary "provisions the embedded methodology on every bare `grove`" — true, and
  read by a meta-grove as a promise that the next session in the loop consumes
  just-committed content. It does not, and should not: **any** skew between the
  skill and the CLI it instructs is unsafe, in *both* directions — a newer skill
  names verbs added since the binary, an older one names verbs removed since the
  skill. The second is the case that surprises, and this release supplies it: the
  `v17.0.0` skill instructs `leaf-add-chain`, removed here, so pairing that skill
  with any binary built from now on hands a session a call that cannot succeed.
  There is no safe direction to drift in, so the only safe skew is none, and one
  embed per build is what delivers it. `content/SKILL.md`, `docs/USAGE.md`,
  `docs/ARCHITECTURE.md` §Embedded methodology and `CONTEXT.md` (a new
  **Embedded methodology** entry) now state the boundary and this reason. The
  half that *is* checkable is enforced: `tests/provision.rs` asserts the embedded
  methodology instructs no `grove-llm` verb the embedded CLI lacks, so a shipped
  binary cannot hand a session a call that cannot succeed.

### Fixed

- **The guidance assertions no longer read `CHANGELOG.md`, which a release cut
  made unreadable.** `tests/composition_guidance.rs` had scoped its current-state
  claims to the live `## Unreleased` section, on the reasoning that a released
  `## v<N>.<m>.<p>` heading is frozen and must not be swept. That scope does not
  survive its own release: `release.toml` renames the live heading to the version
  and re-seeds an empty one, so every entry the cycle logged leaves the scope at
  the moment it ships. The positive assertions then failed — and the bans failed
  worse, passing vacuously over an empty slice while proving nothing. Cutting
  this release is what surfaced it; the assertions arrived mid-cycle and had
  never met a cut. Widening to the whole file was rejected for the reason the
  original scope existed: a current-state positive would pass on superseded
  prose, and a ban would forbid the history that records it. Both failures have
  one cause — the changelog is a record of what changed and when, not a
  description of the current design, so a current-state claim has no
  well-defined scope in it. The claims stay pinned to the surfaces that do
  describe the current design; the one surviving changelog assertion reads the
  whole file on purpose, because a permanent upgrade note is what a frozen record
  is for.
- **`docs/CONFIGURATION.md`: the Codex sandbox cannot write a colocated `.git`,
  and `--add-dir` does not fix it.** Grove's own launch policy leans on a Codex
  template granting the store with `--add-dir ${repo}` — which does register as a
  session workspace root — but the sandbox protects a `.git` path component more
  specifically than whatever root encloses it, so a colocated store's
  `.git/objects` stayed unwritable and jj could not snapshot, let alone commit.
  Every unattended Codex review in a jj grove died at its retire-and-commit
  boundary. *Adjacent settings Grove does not own* now records the profile rule
  that lifts it, kept relative so it holds for every root, together with why
  nothing narrower than the whole gitdir is reliable (`jj git export` writes
  reflogs under `.git/logs/`) and why `.git/hooks` and `.git/config` are pulled
  back to `read` rather than left inside the grant — both are code that later
  runs *outside* the sandbox. Grove ships no configuration, so this is guidance,
  not a code change. Verified on codex-cli 0.147.0.
- **A grow verb now takes each destination atomically instead of checking it
  first.** `Path::exists()` was the wrong question twice: it follows symlinks,
  so a dangling one at a planned destination read as *absent* and the write
  followed it, creating or truncating a target that could be anywhere on disk —
  and a rollback would then remove the link, leaving that target standing. It
  also reported `false` for every other error, turning "I could not tell" into
  "go ahead". The up-front sweep now uses `symlink_metadata`, where only
  `NotFound` means free, and every write claims its destination with an atomic
  non-clobbering create, which closes the gap between the sweep and the write
  that no check can. The sweep survives as the diagnostic that makes a realistic
  collision a refusal naming what stands in the way rather than a rollback.
- **`leaf-add-pair` unwinds the leaf whose creation succeeded and whose write
  did not.** Creating and filling are two syscalls, and a failure between them
  left an empty but well-formed leaf outside the rollback — the residue that
  reads exactly like a deliberately hand-cut partial pair. Ownership is now
  recorded the instant the create succeeds, before any byte is written.
- **The grow verbs' interruption promise is stated accurately.** They are
  all-or-nothing on a *reported error*; rollback runs only when control returns
  through the error path, so process death mid-run can still leave a partial
  shape. `docs/ARCHITECTURE.md`, `CONTEXT.md` and
  `docs/specs/doubt-grove-review-mechanics.md` said otherwise, and contradicted
  their own statement that finish teardown and the session-kind migration are
  the only operations promising interruption recovery.
- **Every handoff document retires before it commits.** The review-mechanics
  spec, the doubt skill and `docs/USAGE.md` had the sequence inverted against
  `content/SKILL.md`'s task boundary; under jj that seals the artifact and pushes
  the producer's `DONE` rename into the *next* task's change.
- **The `cargo clippy --all-targets` baseline is back to zero**, and a
  `[lints.clippy] all = "deny"` table in `Cargo.toml` now holds it there. Eight
  warnings had accumulated unnoticed, which is the expected outcome when nothing
  reports them: an ordinary `cargo build` or `cargo test` never evaluates a
  clippy lint, and this repo has no CI, so the manifest is the only place a gate
  can live. Denying the group costs ordinary builds nothing — rustc does not
  evaluate `clippy::` tool lints — so it bites only under `cargo clippy`. Two of
  the eight were answered with a reasoned suppression rather than a change,
  because the lint was wrong on the merits:
  - The `libc::stat` widening casts in `src/finish_cleanup.rs` are `#[allow]`ed:
    field widths are target-dependent (`dev_t` is `i32` on Apple and `u64` on
    linux-gnu; `ino_t` narrows to `u32` on some targets), so *every* spelling is
    redundant on one supported target and required on another. Clippy analyses
    one target and its "unnecessary" verdict does not travel.
  - The three finish-transaction enums in `src/repo/finish_commit.rs` are
    `#[expect]`ed rather than boxed: each is built at most once per teardown,
    consumed by a single `match`, and never collected, so boxing would add an
    allocation and indirection to the fail-closed path for no measurable gain.
    `expect` over `allow` deliberately — a size ratio can stop holding as the
    types evolve, and the suppression should expire loudly when it does.
    `[lints.rust] unfulfilled_lint_expectations = "deny"` is what makes that
    expiry an error instead of another unread warning.

- **The driver lease and session epoch files state `truncate(false)` explicitly.**
  Behaviour is unchanged — `create` alone never truncated — but the intent is now
  on the page, and it is load-bearing: both opens happen *before* the lock is
  acquired, so truncating there would destroy an incumbent holder's record
  before knowing whether the lock could even be taken, and on the failing path
  would corrupt a lease still owned by someone else. Truncation belongs to
  `write_record` / `write_epoch_contents`, which do it under the lock. Only the
  lease open tripped `suspicious_open_options`; the epoch open escapes the lint
  solely because clippy cannot follow a builder split across statements, and is
  stated explicitly so a future refactor into a chained call stays clean.

## v17.0.0

**Breaking.** Launch policy moves entirely into a new personal configuration
file, and the human command surface collapses to bare `grove`. Existing
installations must write `~/.config/grove/config.kdl` before Grove will start;
existing task trees are migrated automatically on first run.

### Added

- **`~/.config/grove/config.kdl` is the entirety of user launch policy.** It
  assigns each of the nineteen session kinds one complete command-template
  string, which chooses the executable, harness, model, reasoning effort, and
  approval/sandbox policy. Grove shell-splits the template for quoting but runs
  no shell: it expands `${prompt}`, `${session_name}`, `${worktree}`, and
  `${repo}` as whole words and executes the argv directly, adding no
  harness-specific arguments of its own. All nineteen kinds must be present
  exactly once; there are no defaults, families, or inheritance, so a target is
  complete when read on its own. Diagnostics are aggregate rather than
  first-error, naming every missing, duplicate, unknown, and malformed entry with
  its source location. See [`docs/CONFIGURATION.md`](docs/CONFIGURATION.md) and
  ADR *complete-session-configuration*.
- **Session kinds live in leaf filenames**, as
  `NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md`, with the current
  grammar recorded by a positive `.grove/FORMAT` witness. The closed set grows to
  nineteen: the previous seventeen, minus plain `research`, plus `research-a`,
  `research-b`, and the driver-reserved `finish`. A research vendor pair now
  reaches two independently configured commands without per-leaf metadata.
- **Automatic legacy migration.** Bare `grove` converts the original `NNN-slug/`
  layout, the v1 flat dotted-decimal layout, and v2 trees whose leaves lack
  filename kinds, in one fail-closed transaction and one `.grove/`-scoped commit.
  An unknown kind or a structurally ambiguous research pair stops before mutation
  and names the exact paths rather than guessing a target. In plain Git that
  commit runs with user Git hooks disabled, as the teardown commit does: both are
  unattended and path-scoped, and a hook can mutate unrelated working-tree files
  even while rejecting the commit, which no index rollback restores. Signing and
  repository failures stay visible. See ADR
  *task-tree-transactions-fail-closed*.
- **One live driver per working tree**, held by a kernel lock on a control file
  in the workspace's own VCS administration directory. A second `grove` in the
  same tree is refused immediately instead of queueing, while different Git
  worktrees and jj workspaces stay independent. A per-launch session epoch binds
  each agent's `grove-llm` operations to the driver that launched it, so a stale
  session cannot act on the next one. See ADR
  *one-live-driver-per-working-tree*.
- **An unfinishable workspace is refused at start-up, not at teardown.** Grove's
  teardown ends in one atomic same-filesystem rename of `.grove/` into the
  workspace's VCS administration directory, with no copy fallback, so a working
  tree on a different filesystem from that directory could never finish. Lease
  acquisition now compares the two, before configuration validation and before
  any `.grove/` is observed or created: the refusal names both paths, both
  filesystems, the marker that produced the resolution — including a `.git`
  file's gitdir target — and the two remedies, and it mutates nothing, so
  repairing the layout and rerunning simply continues. Only a linked Git worktree
  or a submodule can fail this, but every layout is measured rather than
  classified, because a symlinked `.git` or `.jj` leaves the working tree without
  changing the marker's kind. The teardown gate still performs its own
  comparison against its own exact operands. See ADR
  *supported-workspace-layouts*.
- **Fail-closed teardown.** The finish cycle now runs as a transaction that keeps
  `.grove/` present and unwalkable until the repository has proven the exact
  `.grove/`-scoped deletion commit. Interruption yields either the live tree back
  or a named `Recovery pending` diagnostic — never a half-deleted tree, and never
  an absent `.grove/` mistaken for success. Unrelated staged, working-tree, and
  Jujutsu working-copy changes stay out of the commit. See ADR
  *task-tree-transactions-fail-closed*.

### Changed

- **Bare `grove` is the only human lifecycle command.** It provisions the
  embedded methodology, takes the working tree's driver lease, validates
  configuration, brings the tree to a runnable shape, performs one authoritative
  pick, and launches that kind's configured command — then relaunches on a
  completion signal and stops on anything else. A fresh tree initializes a real
  `requirements` leaf and an exhausted tree materializes a resumable `finish`
  leaf, so every session owns a real selected leaf.
- **The driver picks once, and the session obeys a mandate.** The selected leaf's
  stable handle is embedded in `${prompt}`; the launched session resolves that
  handle and never picks again. A leaf inserted during the launch window becomes
  the next iteration's work instead of preempting the running session.
- Configuration is fully validated before *every* task-tree mutation and again
  immediately before every launch, and is never cached across iterations. A
  failed pre-mutation read leaves the tree byte-identical.
- Each iteration resolves `grove-llm` beside the running `grove` executable and
  rejects version skew as a resumable no-mutation stop.
- Global skill provisioning now sweeps the embedded methodology to *every*
  installed harness's personal skill directory, independently of launch policy,
  before the driver owns a working tree. `content/prompts/continue.md` is the
  single surviving launcher.
- `grove --help` and `grove --version` are metadata-only: they provision nothing,
  discover no repository, and acquire no lease.
- **The declared MSRV is now true.** `rust-version` moves from `1.74` to `1.85`,
  the floor the locked dependency graph actually builds on: edition-2024 crates
  (`clap` 4.6, `clap_lex` 1.1, `assert_cmd` 2.2) make any cargo below 1.85 fail
  during dependency resolution, so the previous claim was unbuildable rather than
  merely untested. Established by running the toolchains — 1.85 passes `cargo
  check --locked --all-targets` and 1.84 fails — with that command pair recorded
  beside the claim in `Cargo.toml`. Installation is unaffected: the crate is not
  published to crates.io and Homebrew ships prebuilt archives, so no consumer
  compiles Grove.

### Removed

- **All legacy launch routing.** Harness detection, `.grove-stamps/`, the
  `--harness` and `--no-launch` flags, every `GROVE_<KIND|FAMILY>_HARNESS` and
  `GROVE_*_MODEL` variable, leaf `**Harness:**` metadata, grow-verb harness
  flags, task-body `**Kind:**` markers and their read-side `impl` degradation,
  and Grove's hidden model, session-name, and Codex-sandbox argument injection.
- **The `do`, `migrate`, and `retire` subcommands.** Migration is automatic and
  node-brief promotion is ordinary session work.
- **Review target receipts and diversity warnings**, along with the structured
  `kind --with-harness --json` routing peek and `GROVE_SESSION_TARGET`. Whether a
  review's command differs from its producer's is now visible configuration
  policy that Grove cannot and does not infer from opaque command strings.
- **User-settable `GROVE_*` configuration**, including `GROVE_HARNESS_BIN*`,
  `GROVE_LLM_BIN`, `GROVE_SKILL_DIR`, and the kill-grace overrides. Test suites
  inject tools, clocks, and grace durations through internal module seams.
  `GROVE_SIGNAL_FILE` remains an internal loop-control channel, not a setting.
- Remove Grove's Herdr-specific status reporting, hook injection, agent hint,
  configuration splice, and bundled task-tree viewer. Grove's completion-signal
  lifecycle remains unchanged.

## v16.5.0

### Added

- Add `grove-llm leaf-promote-chain` to atomically promote a picked producer leaf into a brief-less review chain while preserving its stable handle, with fail-closed locking and recovery.
- Record producer receipts for review targets, including source session, generation, and effective target metadata, with support for decomposed producers.

### Changed

- Coordinate doubt-driven review with Grove so each picked leaf gets at most one in-session reviewer, substantial findings become Grove-managed review work, and same-harness or same-model review launches produce advisory warnings.

## v16.4.0

### Added

- **`linkuistics` / `simplify-project`: reduce a mature repository's maintenance
  surface without hiding compatibility changes.** The skill starts from a broad
  preservation contract — runtime, CLI help, configuration, persisted formats,
  install paths, package contents, and release mechanics — then applies
  evidence-backed keep/fold/move/delete decisions, assigns each retained fact a
  canonical owner, and verifies public surfaces before and after the reduction.

## v16.3.1

### Changed

- **The repository is smaller and its human-facing documentation has clear
  ownership.** The root README now describes the products and their installation
  paths, while focused usage, configuration, architecture, and release guides
  carry the operational detail. The portable plugin installer and its tests now
  live with the plugins they install, Herdr fork maintenance lives beside that
  plugin, and the retained Linkuistics source provenance is consolidated into a
  single record.

### Removed

- **Obsolete bridge code and historical planning artifacts no longer obscure
  the maintained system.** The unused `codex-bridge` crate and stale generated
  stamps are gone, and superseded ADR, research, specification, and workflow
  documents have been folded into the maintained guides or removed. Grove's
  live machine-local `.grove-stamps/` harness binding remains documented and
  unchanged.

## v16.3.0

### Added

- **`linkuistics` / `using-codebase-memory`: query a codebase knowledge graph
  from any shell.** `codebase-memory-mcp` serves the same fourteen tools over MCP
  and over a CLI (`codebase-memory-mcp cli <tool> '<json>'`); the skill documents
  the CLI surface, so the capability reaches every harness that has a shell rather
  than the three that speak MCP. Pi is the one that forces the question — it
  refuses MCP by design — and an MCP-only answer would have stranded it while
  costing three config dialects for the rest. The second payoff is composition: a
  question that spans several graph queries becomes one script instead of a
  round-trip per call.

  **What makes it a skill rather than a link to a README is the silent failure
  modes**, each re-derived against `codebase-memory-mcp 0.8.1` by running every
  command it documents. On failure stdout is *empty* and the error goes to stderr,
  so the obvious `| jq` idiom shows nothing and masks the exit status behind
  `jq`'s own 0. Malformed JSON arguments are discarded rather than reported — you
  are told "project not found", the same message an unindexed project gives.
  `search_graph` truncates at a default `limit` of 200 and flags it only in
  `has_more`/`total`; `trace_path` caps callers at 100 and flags nothing at all.
  `min_degree` gates on **total** degree, so neither `relationship` nor
  `direction` makes it directional and a "high fan-in" recipe built on it is
  wrong. And `trace_path` on a bare `function_name` that several symbols share
  resolves to **none** of them — zero callers, exit 0 — where the
  `qualified_name` answers. Each of these reads as a valid empty or short answer,
  which is what makes them worth a document.

  It also says when *not* to use the CLI: where the MCP tools are available they
  are better for single queries — typed arguments, no shell quoting — and in
  Claude Code an absent `mcp__codebase-memory-mcp__*` tool means "deferred, not
  yet loaded", not "server not running".

  Shipping it needed no distribution machinery: `install.sh` globs the skills
  directory, so codex, gemini and pi pick it up on the next run, and the plugin
  manifest lists no skills. What the manifest did need was to *describe* it —
  `plugins/linkuistics/.claude-plugin/plugin.json` gains the capability in its
  description and three keywords, which is the whole of what marketplace
  discovery has to go on.

### Changed

- **The provisioned methodology gains a section on verifying a claim about the
  repo itself** (`content/driving.md`), the counterpart to its existing section on
  verifying framework decisions against the source. Sessions assert things about
  their own codebase constantly and reach for a repo-wide grep as the evidence;
  the section's point is that **every one of that instrument's failure modes
  produces a clean-looking result**, so a clean sweep is not yet evidence.

  It names the four modes — a flag that is not the flag you meant (ripgrep's `-E`
  is `--encoding`, `-r` is `--replace`, and `rg -rn` therefore *succeeds* while
  printing fabricated contents), a search space that excludes the target (`.grove/`
  is a dotdir skipped without `--hidden`), rendered `--help` text as the wrong
  surface entirely, and the residual case of an instrument that is well-formed but
  blind. Only the last needs new machinery: a **positive control** plus a
  **cross-tree control**, on the reasoning that a broken instrument reads clean
  *everywhere*, so clean-here-plus-dirty-there cannot be faked.

  The rest is about sweeps that get narrowed without anyone deciding to narrow
  them: grep the **claim**, never a file list written before the work; a claim's
  **scope** is part of the claim and goes stale exactly as a file list does, and a
  scope stated as a set of trees can never reach a file that is in no tree; and a
  finding against a *section* does not reach the document's **summary layer**.
  Plus two smaller rules — never document a claim with a count of itself, since
  the sentence stating the count invalidates it, and a clause that only reads as
  true beside a false neighbour is load-bearing on the sentence you are deleting.
  Enumerate-then-classify replaces pattern-list sweeping throughout: a pattern list
  is complete only as far as the list, so a longer one moves the leak rather than
  closing it.

- **ADR *task-tree-scheme* §5 now covers an index *into* a work item**, not only
  how an item is named — a review finding (`B5`), a numbered decision, a bare key
  (`k29`). The rule as written governed naming, so prose could satisfy it in full
  and still carry a reference resolving nowhere; the addition requires the pair
  (`branch-review-k14 B5`). The load-bearing half is that **a bare index is worse
  than a bare position**: a position at least fails to resolve, while an unscoped
  number can resolve *incorrectly* against an unrelated live series that shares
  its shape. `src/` and `tests/` are swept to match — 48 such citations across 11
  files, comments only, no behaviour change. `CONTEXT.md`'s *Work-item handle*
  carries the matching `_Avoid_`.

- **A review chain and a vendor pair are now a *node directory*** — reversing the
  "a chain gets no node directory of its own" line v16.2.0 shipped. `leaf-add-chain`
  and `leaf-add-pair` write `NN-<stem>-chain-k<key>/` (or `-pair-`) holding their
  three steps at `01`–`03`; four keys per shape, not three, and stdout is **four**
  absolute paths with the node's leading, so a caller can `leaf-add <node>` a late
  step straight into it.

  **The directory is what makes the group structural.** A flat run of stem-sharing
  leaves is contiguous and similarly prefixed, and nothing in a listing tells it
  apart from three adjacent unrelated leaves; a directory is a first-class object
  in `yazi`, Finder and `ls -R` alike, collapsible and countable without any of
  them being taught the convention. That is the argument the earlier rejection got
  wrong — its legibility case rested on `find` and on the one viewer grove ships.

  **The node carries no `BRIEF.md`, by rule**, and that is what killed the two
  costs the earlier answer was built on: it buys no charter written because a step
  demanded it (constraint 4), and the Retire cascade's close finds **nothing to do**
  at a chain node — no `Done when` to check, no brief to promote — where a
  decomposition node's close has both. The discriminator is a
  **file's presence, never a name pattern** — the
  `-chain` / `-pair` token is ordinary slug text nothing keys on, present only so
  the node's slug does not collide with its first child's under `resolve`.

  **Existing flat chains are not migrated, and that is a decision.** Detecting one
  needs the `-review` / `-integrate` suffix parsing the design forbids, and would
  misfire on a leaf legitimately named `foo-review`. A flat chain is a valid tree
  that `pick` walks correctly. No format bump; `grove do`'s migration is untouched.

  Falling out of the same change: `leaf-add` / `leaf-add-chain` / `leaf-add-pair`
  now accept **any node directory** as `<parent>`, where they had required one
  holding a `BRIEF.md`. That guard predated the two node species and refused the
  shape the design names explicitly — `leaf-add <chain-node> <stem>-late-step`,
  which lands a step decided on afterwards inside the chain rather than behind
  every unrelated live leaf. `pick`, `brief-chain`, `resolve`, `leaf-insert`,
  `leaf-decompose`, `leaf-retire` and `leaf-prune` needed no change at all, and
  neither did the tree viewer. `docs/specs/task-kind-taxonomy.md` carries the
  design and the lapsed arguments; ADRs *task-tree-scheme* and *task-kind-taxonomy*
  carry the two node species and the charter discriminator.

- **The Retire cascade no longer asks the human to confirm a node's close — it
  checks, promotes and reports instead.** For *every* node species, where the
  methodology previously said *"ask the user before treating a brief-carrying node
  as done"* and re-asked at each ancestor up the chain. In its place a
  brief-carrying node's close runs four steps the session discharges itself: check
  the node's brief `Done when` against what the subtree delivered; `leaf-add` the
  missing work if the check fails and the gap can be named; escalate — stop and say
  so — if it cannot, because the residue is a scope judgement rather than work; then
  promote what survives from the brief upward and name the closed node by its
  `<slug>-k<key>` handle in the commit message. A brief-less chain node's close
  still has nothing to do. Leaf retirement is unchanged (it was never confirmed —
  mechanical bookkeeping), `leaf-prune` is unchanged (still HITL), and the finish
  cycle's single gate is unchanged and is now the loop's **only routine human gate**.

  **The generating rule is two ordered tests**, and it is what the change is
  actually for: does the answer change what is written, and if so, is the fact the
  session's to establish or the human's to decide? A node close fails the first —
  a node is **never marked**, so whatever the human answered the tree was
  byte-identical afterwards, and a node closed in error is reopened by one
  `leaf-add` with nothing to undo. The question was also in the Retire step, which
  *every* kind runs, so an AFK leaf that happened to be its subtree's last was
  guaranteed to stall at a moment nothing in the tree predicts — the one thing that
  makes the HITL/AFK mark wrong by construction. The trade is that the question was
  synchronous and the report is not: a human who would have said "no" now says it
  one session later, against a node nothing has marked.

  **Zero code changed** — the cascade was always prose. What changed is
  `content/SKILL.md`'s Retire step and every surface that restated it:
  `content/BRIEF-FORMAT.md`, `content/TASK-FORMAT.md` and `content/driving.md`
  (each justified the `BRIEF.md` discriminator by the confirmation — its job is now
  to select which closes have *work*, not which get *asked*), `docs/grove.md`,
  `docs/workflows/multi-step.md` (whose walkthrough beat was *"the user said not
  yet"* and is now check → `leaf-add`), and doc comments in `src/llm_cli.rs` and
  `src/tree_grow.rs`. The mutation test asserting a chain node is brief-less is
  **kept** — under the new rule a stray charter silently gives the cascade
  close-time work over a rollup nobody wrote. New ADR
  `docs/adr/confirmation-boundary.md` carries the tests and the four rejected
  options; *pruning*, *task-tree-scheme*, *task-kind-taxonomy*,
  *in-session-finish-cycle*, `docs/specs/task-kind-taxonomy.md` and `CONTEXT.md`
  were reworked in place to cite it.

- **`release.toml`: the cut now closes this file's `## Unreleased` heading itself.**
  `pre-release-replacements` renames it to the version being released and re-seeds
  an empty one, replacing the `BEFORE the cut:` note that asked the releaser to do
  it by hand. The standing heading was ratified one entry above this one on the
  grounds that a session must be able to log its change when it makes it; a manual
  rename made the other half of that convention depend on remembering a step, in
  the one file whose whole value is an accurate record of what changed and when. A
  forgotten rename is invisible except on inspection — the release's entries sit
  under "Unreleased", above its own empty version heading.

  **One replacement does both halves**, because the match does not consume the
  blank lines around the heading: writing `## Unreleased` back *plus* the version
  heading below it leaves the accumulated entries where they are, now under the
  version. So no `<!-- next-header -->` marker was added — cargo-release's
  documented idiom needs two replacements and puts an anchor comment in the file,
  and the heading is already its own anchor.

  **`exactly = 1` is the guard, and this file's own prose is what it guards
  against.** `search` is a regex, and the string `## Unreleased` appears many times
  here — in the preamble's explanation of the convention, and now in this entry
  about it — a count that rises every time someone documents the thing. Only the
  heading is ever a *whole line*, which is why the search is anchored
  `^## Unreleased$` (cargo-release compiles it with multiline mode on, so the
  anchors bind per line). An unanchored search aborts the cut — "at most 1
  replacements expected, found N" — rather than corrupting anything, and a releaser
  who renames by hand first, or drops the standing heading, fails the same loud way.
  The preamble now states the one constraint an editor of this file is under.

- **`leaf-add-chain` / `leaf-add-pair` could leave a partial chain node behind
  when the permanent-key space ran out.** The node directory was created before
  each child's key was derived, and the derivation was unchecked `node_key + 1 + i`
  arithmetic — so a tree whose highest key sat near the `u32` ceiling panicked
  mid-write (debug) or *wrapped* to `k0` (release), leaving a live two-step node
  that reads exactly like a deliberately cut partial chain. Both verbs now
  allocate the whole four-key run **before the first write** and refuse an
  exhausted keyspace with the tree untouched, which is where every other
  resolution failure already lands. `leaf-add` and `leaf-insert` share the same
  fallible allocator; neither could produce a partial tree, but both had the same
  panic.

- **`install.sh` silently re-pointed every installed skill when run from a side
  working tree.** It derives its link source from `${BASH_SOURCE[0]}` and
  unconditionally re-links, so a run from a linked git worktree or a secondary jj
  workspace — the normal place to develop an unmerged skill — repointed all 48
  links (16 skills × 3 harnesses) at a tree that disappears when that tree is
  removed. It printed `linked` 48 times and looked like a success; the damage
  surfaced later, as skills that had simply stopped existing. It now probes
  whether the tree it lives in is the repo's main checkout and **refuses** if not,
  because a warning is the wrong shape for damage that is silent and delayed.
  `--force` opts into the one legitimate case, testing an unmerged skill live, and
  says on stderr what it is doing.

  **The probe is jj-first, mirroring the binary's `repo::vcs_of`**
  (*symmetric-vcs-rule*, which now names three enforcers rather than two). That
  ordering is load-bearing rather than merely consistent: a secondary jj workspace
  of a *colocated* repo is not a git worktree, so a git-first probe answers "not a
  repository" and misses precisely the case the guard exists for. It reads with
  `--ignore-working-copy`, since every other jj invocation snapshots as a side
  effect and an install-time audit has no business mutating the tree it is
  auditing.

  **No probeable VCS disables the guard rather than blocking**, and the marker has
  to sit at the script's own directory: probing without that check would let
  `git rev-parse` walk *up* out of an unpacked or vendored copy of this repo and
  judge it by the enclosing repository, refusing an install that is fine.

  `install.test.sh` covers it — dependency-free (bats is not assumed), eleven
  cases built as real working trees in a scratch directory and run against an
  isolated `HOME`: nine tree shapes across git, jj, colocated jj and no-VCS, plus
  two argument checks. The isolation is not tidiness — the defect under test *is*
  an unwanted write to `$HOME`, so reproducing it there would cost a manual repair
  of every installed link.

- **`grove retire --no-launch` now checks the readiness it reports — and, for the
  first time, says what it does.** The option shipped with no description at all,
  rendering as a padded blank row beside two described ones; it was the only
  undescribed option in either binary. Writing the description was not cosmetic,
  because the two dry runs were not the same thing: `grove do --no-launch`
  resolves everything a launch would fail on (*model-per-task-kind*: "`--no-launch`
  resolves the launch it declines to perform"), while retire's resolved the
  harness, printed `would exec claude`, and returned before loading the prompt or
  pre-flighting anything.

  **The asymmetry is fixed rather than documented**, because that rule was never
  `do`-specific — a flag that reports *readiness* on both verbs cannot be a
  checked claim on one of them. Retire's dry run now runs the launch's own code
  path up to the exec: it loads and substitutes the `retire` prompt, and assembles
  the invocation, which is what runs the codex sandbox pre-flight and derives the
  VCS-store grants. **The prompt is the sharp case and it is unique to this verb** —
  `grove retire` never provisions, so it reads a global skill dir some *earlier*
  `grove do` had to have written, which is the one launch dependency a user cannot
  see and precisely the one the old dry run sat on top of.

  The exec itself is the one step a dry run cannot share, so it stands in the
  strongest predicate on it: the binary is on `PATH`. That is checked against
  `harness.exec_bin` and not the loop's overridable bin name — `grove retire` has
  no bin seam, and checking the name it does not use would report on a binary this
  verb never runs.

  **What stays asymmetric, and why it is not a defect:** the report still names no
  leaf, kind or model, because `grove retire` resolves none. It peeks no leaf and
  passes the harness no model, so `grove do --no-launch`'s per-kind routing report
  has no counterpart here.

  A new guard walks both binaries' clap models and fails on anything a help
  surface *lists* without describing — arguments and subcommands alike. Asserted
  against the model rather than the rendered text: scraping `--help` for a token
  followed by nothing has to reproduce clap's two layouts and its wrapping, and
  that parser is likelier to be wrong than the thing it checks — wrong in the
  direction that manufactures a false clean.

## v16.2.0

### Added

- **`grove-llm leaf-add-chain` and `leaf-add-pair` — one call per composition
  shape.** The entry below made the review chain and the vendor pair the shape a
  session is *told* to reach for; these make reaching for one no more work than
  cutting the first leaf of it.

  ```
  grove-llm leaf-add-chain <parent> <stem> --kind <producer>
     <stem>            <producer>              <stem>-a        research, **Harness:** <a>
     <stem>-review     review-<producer>       <stem>-b        research, **Harness:** <b>
     <stem>-integrate  integrate-review-<p>    <stem>-combine  combine-research
  ```

  **What the verbs buy is a derivation, not keystrokes.** `<producer>` ⇒
  `review-<producer>` ⇒ `integrate-review-<producer>` is precisely the
  parameterisation the seventeen-kind set bought in v16.0.0, and until now
  nothing had spent it — every chain was transcribed from a seventeen-row table
  by hand. A typo there is already caught (`--kind` gates on write); a
  **well-formed wrong** kind is not. `--kind review-impl` beside a `design`
  producer is a perfectly valid invocation, and what it costs is a reviewer
  reading for correctness, security and tests where it should be asking whether
  the ADRs are a minimum coherent set — a *discipline* misroute nothing
  downstream detects.

  **The pair declares both vendors, and refuses two that are the same.** A single
  `--harness` naming only the second producer would leave the first resolving
  through kind → family → stamp at launch time — so "two corpora" would be a
  forecast about routing policy rather than a fact in the tree, unverifiable when
  the leaves are cut and silently false if the policy changes before they run.
  Both producers therefore carry a `**Harness:**` line. Declaring the first costs
  nothing it did not already cost: a leaf naming the *stamped* harness is not a
  reroute, so the unscoped model keys still apply.

  **One call is one mutation.** Slugs, kind and vendors are validated before the
  first write; positions and keys come from one snapshot; every destination is
  checked free before any is written; anything that still fails mid-write rolls
  the run back, and a rollback that cannot complete names the residue by path.
  Stdout prints the three paths *after* the run succeeded and prints nothing at
  all otherwise — three separate `leaf-add` calls would leave a live prefix of a
  chain that reads exactly like a deliberately hand-cut partial one, which is the
  wrong-but-well-formed residue the verbs exist to prevent.

  **Nothing is enforced and nothing is parsed.** `leaf-add` is untouched, no tree
  is inspected, no chain is linted for completeness, and skipping a chain — or
  cutting a partial one — stays a normal choice. The verbs *write* the naming
  convention; nothing ever reads it back. The refusals (`--harness` on a chain,
  `--kind` on a pair, `--kind research` on a chain, one vendor named twice) are
  authoring-time argument validation with a human present, and each names the
  mechanism that *does* express what was asked: the chain's refusal points at
  `GROVE_REVIEW_HARNESS`, the pair's at `leaf-add-chain`, and back.

  A generated shape is **byte-identical to the same leaves cut by hand**. There
  is deliberately no retrofit verb for adding review steps to a producer that has
  already run — that is two `leaf-insert` calls, which is exactly the work such a
  verb would do. `docs/specs/task-kind-taxonomy.md` carries the design, the
  all-or-nothing contract, and the honest note that the mechanism was adopted on
  reasoning about the error class rather than on a measured incident; ADR
  *cli-binary-split* carries the three-leg bar a `grove-llm` verb clears and now
  lists both verbs in its normative enumeration.

### Changed

- **The review chain and the vendor pair are now the shape a session reaches for
  by default** — and a cut chain is legible from `find .grove` alone. v16.0.0
  tripled the kind set and documented both composition patterns
  (`X` → `review-X` → `integrate-review-X`; `research` → `research` →
  `combine-research`), but only in the reference material: nothing in the
  guidance a session reads *while it is cutting leaves* told it to reach for
  them, so in practice it did not. The evidence was this repo's own task tree —
  32 permanent keys, 26 leaves, zero chains and zero pairs.

  Three surfaces now carry it, in the order a session meets them: the
  **bootstrap prompt** (whose one decomposition shapes every later session),
  `SKILL.md`'s **Decompose** step, and `TASK-FORMAT.md`. Each states the
  escalation call rather than just the shape — chain an artifact that is
  load-bearing (a spec, a decomposition you will build on for months, a
  subsystem); a one-file change wants a mid-session subagent instead.

  **A chain is named off a shared stem with a terminal step suffix**: `<stem>` /
  `<stem>-review` / `<stem>-integrate`, and `<stem>-a` / `<stem>-b` /
  `<stem>-combine`. The suffix is terminal because that keeps a chain contiguous
  under its stem — a leading token (`review-<stem>`) sorts every review beside
  every other review and scatters exactly the chains the naming exists to
  reveal. A chain deliberately gets **no node directory of its own**: a node
  already means *this work proved bigger than one session*, and one per chain
  would overload that signal, buy a `BRIEF.md` no step earned, and apply
  `leaf-decompose` — a reactive verb — speculatively.

  **No behaviour changed and nothing new is enforced.** grove still validates no
  ordering between leaves, does not parse the suffix, and will not warn when a
  chain is absent; `leaf-add` still makes exactly one leaf, because minting three
  where one was asked for would grow the tree speculatively (constraint 4).
  Skipping a chain stays a normal choice. `docs/specs/task-kind-taxonomy.md`
  carries the reasoning, including the node-per-chain option and why it lost.
  (This release also ships two verbs that cut a whole shape in one call — see
  *Added*. They do not reopen the clause above: `leaf-add` is untouched, and a
  caller naming a *shape* is not a verb inferring one from an argument that meant
  one leaf.)

- **A chain stays a convention, not a construct** — the follow-on question to the
  entry above, asked and closed. Should a chain be *first-class*: a unit `pick`
  will not walk out of, whose close needs no confirmation? **No** — and two of the
  three costs that would motivate it turn out not to exist. A chain's steps sit at
  adjacent positions, so `pick`, which returns the first live leaf in pre-order,
  already runs them in sequence; that is the very ordering a decomposition node's
  children get, not a weaker one. And the Retire cascade's confirmation is asked
  **per node** — so a chain, deliberately not a node, is never asked one. That
  makes it a cost node-per-chain would *introduce*, which is a second and
  independent reason that option lost, on a different axis from the one that
  decided it.

  The one real gap is that a sibling-level `leaf-insert` can split a chain, where
  a node's children are protected by containment. Closing it would cost `pick` its
  defining property: answering *is a group in flight?* needs either state outside
  the tree (constraint 1 — and `pick`'s statelessness is what makes restart ≡
  continuation) or a rule that skips live work and ranks groups, which is a
  scheduler no reader of `find .grove` can predict. ADR *task-tree-scheme* now
  states the invariant — **`pick` is a walk, not a scheduler** — which also covers
  why an outcome taxonomy cannot carry a `blocked` mark, and glossary term
  **Pick** carries it into every session's bootstrap.

  What changed in practice is guidance, not behaviour: **cut a chain's steps
  together**, and reach for `leaf-insert` rather than `leaf-add` for a step decided
  on *after* its producer already ran — `leaf-add` appends at the end, behind every
  unrelated live leaf, which is how a chain actually comes apart. Both candidate
  marks (a brief field; inference from the children's kinds) and the tempting
  middle option (`leaf-add` inferring placement from a shared stem) are costed in
  `docs/specs/task-kind-taxonomy.md`, along with what would reopen the question.

- **The skill's Pick step now says where the pick actually happens.** A session
  reading `SKILL.md` met Pick as its first act, when under `grove do` it is the
  *second*: the driver already ran `grove-llm kind --with-harness` to bind this
  session's harness and model, necessarily **before the session existed**. The
  step, glossary term **Kind routing**, and ADR *model-per-task-kind* now state
  the invariant that follows — **the driver's peek is a forecast, not a
  reservation, and the tree wins**. grove hands a session no leaf identity, so
  work the leaf `pick` returns even if the launch was routed for a different one.

  Whether to *bind* the two was the open question, and the answer is **no**. The
  divergence it targets is real and was constructed rather than assumed: with a
  `leaf-insert` landing inside the launch window — measured at **≥8s**, which is
  essentially all harness boot, against a ~0–30ms peek — the driver launched
  `claude/opus` for a `design` leaf while the session worked a `review-impl` leaf
  that `GROVE_REVIEW_HARNESS=codex` routes to codex. A **cross-vendor** misroute,
  and the sharper half, since a wrong model is a preference axis `/model` can fix
  in-session and a wrong harness is a correctness axis that cannot be.

  Binding lost anyway, on four counts: it inverts the authority (a pre-session
  forecast beating the tree, which is state outside the tree — constraint 1); it
  discards a `leaf-insert`, the human's one means of preempting the loop; it
  **cannot be mandatory**, because the skill also drives sessions with no driver
  at all, so an unset variable is indistinguishable from a driver that forgot;
  and refusing to start on a mismatch is a gate (constraint 5). Against that, the
  loop already self-heals in **one** iteration off the same zero-state
  re-derivation that gives restart ≡ continuation — so the residual is one leaf
  executed on the wrong vendor, once. No behaviour changed.

- **The launch line now names the leaf the launch routed on.** Every `grove do`
  iteration printed `grove: launching claude (model: opus)` — true about the
  launch and silent about the work. It now prints:

  ```
  grove: launching claude (model: opus) — session-leaf-binding-k28 (design)
  ```

  This is the one part of the complaint the entry above leaves standing. The
  peek is a forecast the session may disagree with, and until now **nothing on
  screen said so**: a session running a leaf routed for a different kind looked
  identical to one that was not. The diagnostic closes that with no gate, no
  environment export and no state outside the tree — the three grounds binding
  was rejected on. It stands on its own too, since *what a grove is working on*
  was absent from the driver's own output, and the scrollback is the only record
  of what each session in a loop was on.

  Named by the **stable `<slug>-k<key>` handle**, not the path
  (*task-tree-scheme*): a position moves under `leaf-insert` and the scrollback
  outlives it. Deliberately unlike `grove do --no-launch`, which keeps printing
  the path because it names something the operator opens next. The tail is
  omitted whenever there is no leaf to name — the bootstrap launch (no `.grove/`
  to walk), the finish-cycle launch, and any walk that cannot resolve one — so a
  line that reports a leaf is always reporting a real one. The kind shown is the
  **peek's**, so the line says what the launch actually resolved on rather than
  re-reading the file and reporting something the launch never saw.

### Fixed

- **A hand-marked node directory no longer hides its whole live subtree.**
  `.grove/01-DONE-node-k1/` holding a live leaf made `pick` print *no live leaves;
  this grove is done*, and the driver's next move was to allocate a finish leaf
  and propose teardown. Only the human gate stood between a silent tree and
  deletion. The task-shaped strictness rule that already covered Markdown names
  now covers **every** positioned, keyed name at either species: the `.md` suffix
  declares whether a name is a leaf or a node directory, and such a name must
  parse completely as that species *and* name an entry that is that species on
  disk. A node directory wearing an outcome infix, a directory at a leaf's name, a
  file or symlink at a node's name — each is now a malformed tree that stops reads
  and mutations, naming the path and what the name declared. Grove writes none of
  them (`leaf-retire` and `leaf-prune` refuse a node operand), so each is reachable
  only by hand, which is exactly where "a node is never marked done" — a rule a
  human has to know rather than one the grammar makes unstateable — gets broken.
  Entries outside the grammar stay foreign at either species, `BRIEF.md` stays
  outside the rule, and the unpositioned `PROMOTING-` / `FINISHING-` /
  `PREPARING-FINISH-` / `MIGRATING-` witnesses keep their own earlier refusals.

  One reader now owns the species check for the whole tree interface, where three
  copies had each decided independently to skip a mismatch. That silently reached
  further than selection: the copy feeding key allocation made a dropped subtree's
  keys invisible, lowering the visible maximum permanent key so the next
  `leaf-add` would re-issue a key still live inside it. Symlinks are covered by
  the same test rather than a new one — a symlink is neither a regular file nor a
  directory, where the old check would have handed the driver a mandate resolving
  outside `.grove/` entirely.

- **A codex grove no longer dies at startup in an untrusted working tree.** A
  `grove do` on codex printed one line — `Error adding directories: Ignoring
  --add-dir (…) because the effective permissions do not allow additional
  writable roots` — and the loop stopped on what looked like a mute non-signal
  exit. The cause: codex's effective sandbox is `read-only` for any project the
  user has not **trusted**, trust is per-directory with **no inheritance from
  parent directories**, and under `read-only` the VCS-store grants grove passes
  so a session can commit (*codex-gitdir-grant*) are refused **fatally** —
  despite the word "Ignoring", codex exits 1 before drawing any TUI. A
  brand-new working tree, which is exactly what `grove do` bootstraps into, is
  untrusted by construction; the ADR's "harmless when the sandbox is off" had
  only ever covered `danger-full-access`, and this third mode was the default.

  Every codex launch — the loop's, `grove retire`'s, and the `--no-launch` dry
  run's — is now pre-flighted: grove asks codex what sandbox *this* launch would
  get, by running `codex exec` with the same model flags and the same grants and
  reading the one `sandbox:` line of its header, then **refuses before spawning**
  with a message naming both ways out (trust the project, or set `sandbox_mode =
  "workspace-write"`). Per launch rather than once per `grove do`, because codex
  models route through `--profile`, which is a config layer that can itself set
  `sandbox_mode`. ~0.15s, killed the instant the header arrives, so it spends no
  tokens, writes no trust entry and leaves no rollout. It is codex's own verdict,
  not a reimplementation of its trust rules, so there are no false refusals — and
  anything unanswerable (codex unspawnable, no header, an unknown mode) proceeds,
  because a probe that cannot answer must never be what stops a loop.

  The probe passes **`--skip-git-repo-check`**, which is what makes it the TUI
  rather than a stricter cousin of it. `codex exec` refuses to start when the cwd
  is neither trusted nor inside a git repo — one line to stderr, exit 1, no
  header — while the TUI grove actually launches has no such gate. Untrusted is
  the very condition that makes the sandbox `read-only`, so in a **jj-native**
  working tree the two arrive together and an unflagged probe went mute in
  precisely the case it exists for: the verdict degraded to `Unknown`, the loop
  launched, and codex died on `--add-dir` with the cryptic one-liner the
  pre-flight is here to replace. The flag moves no policy — the same tree reports
  the same `read-only` with or without a `.git` beside it.

  grove **refuses rather than elevating**: passing `--sandbox workspace-write`
  would make every launch succeed, but the sandbox posture is the user's and
  codex's trust prompt exists so a human answers it once. It refuses rather than
  *degrading*, too — `-c sandbox_workspace_write.writable_roots=[…]` is silently
  ignored under `read-only` rather than fatal, so that flag form would buy a
  session that comes up and then cannot commit, and grove's Commit and Retire
  steps are mandatory.

- **A harness spawn that is not the session no longer inherits authority to end
  one.** `GROVE_SIGNAL_FILE` is the loop driver's kill channel — it SIGTERMs its
  child the moment that path appears — and an environment is inherited, not
  addressed, so any spawn that merely declined to *set* it still handed its child
  whatever the driver carried. All three harness spawns now scrub the loop's
  control environment through one helper, and exactly one of them (the session's
  own) grants a path back afterwards. Latent in production, where no real harness
  writes that file; not latent in a meta-grove, where the pre-flight above spawned
  the harness binary unscrubbed and this repo's own `cargo test` killed the live
  session it was typed into.

- **herdr no longer labels a grove pane with the wrong agent.** herdr identifies
  a pane's agent from its foreground process group, preferring the group
  *leader*; in a grove pane the leader is `grove` itself, which herdr cannot
  identify, so it fell back to scoring every member — where a `codex mcp-server`
  helper could outrank the harness grove had actually launched. The pane then read
  `codex` whatever it was running, and herdr evaluated the wrong agent's screen
  manifest against the TUI.

  Every harness grove spawns now carries `HERDR_AGENT=<harness name>` —
  herdr's own documented hint for a **host-visible wrapper that hides the real
  agent**, which is exactly what `grove do` is. It goes on the child rather than
  on `grove` (a process cannot rewrite its own exec-time environment, which is
  what herdr reads), and at **both** launch sites: a `grove retire` pane is
  mis-detected the same way. No fork hunk and no process-group surgery — both
  were considered and rejected, the latter because it buys the same outcome by
  rewriting the driver's signal topology, which *self-driving-loop* and
  *herdr-optional-ui*'s release table both rest on.

  Unlike the turn hooks, the hint is **not gated on running under herdr**: it
  changes no argv and spawns nothing, nothing but herdr reads it, and the gate's
  own three variables are not what herdr's detection depends on — so gating could
  only lose the fix. The value is the harness name verbatim, because grove's three
  names are already herdr's three canonical labels; a name herdr does not know
  parses to nothing and degrades to the previous behaviour.

  Visible only where grove is not holding hook authority, since a landed report
  already takes precedence over detection — and in practice that means *while a
  harness is running and grove is not reporting*, **not** after the release at
  `complete --done`: the hint rides the harness's exec-time environment, so it
  dies with the harness and a finished grove's pane has nothing left to detect.

  **Measured on real panes** against the installed herdr `0.7.5-linkuistics.1`.
  Two `grove retire` panes with byte-identical process shapes — leader `grove`, a
  live `claude`, a `codex mcp-server` helper — read `agent: codex` with
  `agent_status: done` on the hint-less v16.1.0 build (a live session reported as
  finished: the headline complaint in miniature) and `agent: claude` on this one.
  On a `grove do` pane with grove's authority released to expose detection, the
  pane re-acquired as `claude` after one detection sweep and herdr then read a
  stalled grilling session as **`blocked`** off claude's own screen manifest —
  so the fix also makes `fallback_state`, which *herdr-optional-ui* previously
  disclaimed for grove panes, work underneath grove's own reports. The hint turns
  out to be inherited by the harness's whole process subtree, so even the `codex`
  helper carries `HERDR_AGENT=claude` and cannot win the hint path either.

## v16.1.0

### Added

- **A claude-hosted grove now reports `blocked` the moment it stops to ask, not
  when the session ends** (*herdr-turn-boundary-hooks*). v16.0.0 shipped
  driver-level reporting and named its own gap: the driver is the harness's
  *parent*, so it sees a session start and a session end and nothing in between,
  and a session that stalls **mid-session** on a question ends no session — it
  read `working` until a human noticed. That gap is now closed on claude.

  Every claude launch under herdr carries an inline `--settings` hook block
  wiring claude's turn events to a new `grove-llm report-turn`:
  `UserPromptSubmit` ⇒ `working`, `Stop` ⇒ **`blocked` unless
  `$GROVE_SIGNAL_FILE` says the task completed on purpose**. That discriminator
  is the whole design, and it needs **no new model contract**: `grove-llm
  complete` is already mandatory as every task's last action, so the fact the
  hook reads is already being deposited. Hence no flapping — a task that
  finishes normally reports `working`, and the grove-finished case says nothing
  at all and leaves `idle`-then-release to the driver. It is also why herdr
  cannot fix this upstream and grove can: herdr sees a turn end and cannot tell
  "finished" from "asking"; grove knows, because grove is the thing that
  relaunches.

  **Injected per launch, persisting nothing.** claude merges hooks across
  settings sources, so grove contends with neither herdr's own installed
  `SessionStart` hook nor yours, writes to no file you own, and leaves nothing
  behind. **Nothing is injected outside a herdr pane at all** — the argv is then
  byte-identical to a grove without turn hooks. Under herdr the cost is one
  ~3ms process spawn per boundary, socket or no socket.

  **claude only, and the other two are blocked on facts rather than effort.**
  codex has no turn-end hook event (`session_end` is the boundary the driver
  already sees), and persists hook trust per source-and-content-hash so an
  injected hook has no trust record — the only escape disables trust for every
  hook in the invocation. pi's herdr extension already reports full lifecycle,
  but reports `idle` at turn end, which is the same conflation.

- **A claude-hosted grove parked on a permission prompt now reads `blocked` too**
  (*herdr-turn-boundary-hooks*). A permission prompt stalls an unattended loop
  exactly as badly as a question does, and it ends no turn — so the boundary
  hooks above never fired for it, and grove's own pane authority had taken away
  the screen detection that used to catch it by accident.

  The injected block gains two more rows, and they are a **pair**: `Notification`
  ⇒ `blocked`, `PostToolUse` ⇒ `working`. The restore is what makes it a pair
  rather than one more event — granting a permission fires no event of its own,
  so without it the pane would stay `blocked` from the first prompt of a session
  until its turn finally ended. `PostToolUse` is the only thing claude fires in
  between, which is why the restore is per-tool-call; the reports are not
  deduplicated, and that is deliberate, since a report on every tool call is also
  what re-asserts grove's authority if herdr restarts mid-session.

  The `Notification` row is matched to `permission_prompt`,
  `elicitation_dialog` and `elicitation_url_dialog` — exactly the dialogs claude
  raises a notification for only after **six seconds of human silence**, so a
  prompt you answer straight away never flaps the pane, and one that reaches the
  hook really is unattended. Two cases stay uncovered and are named in the ADR: a
  parallel batch whose sibling outlives the prompt can lift the block early, and
  a tool that renders its own dialog (`AskUserQuestion`) raises no notification
  at all.

- **`herdr-plugin`: a herdr plugin that renders the live `.grove/` tree** — the
  other half of *herdr-optional-ui*'s split, and the half that needs nothing from
  grove at all. A pane shows the tree with the live leaf marked and its kind
  beside it, finished node directories collapsed to their counts, live ones
  expanded, and it follows the loop as it advances. Install it with
  `herdr plugin install Linkuistics/grove/herdr-plugin` and bind
  `linkuistics.grove.open-tree` to a key.

  It reads the `.grove/` directory scheme (*task-tree-scheme*) and nothing else:
  no socket, no state file, no call into `grove` or `grove-llm`. That is what the
  ADR promised and it is now literal — the plugin and the binary version
  independently, deleting the plugin changes nothing about grove, and deleting
  grove leaves the plugin with nothing to render but breaks nothing. It reports
  no state either: `idle`/`working`/`blocked` stays with the binary, because
  herdr's lifecycle authority is a compiled-in allowlist a plugin cannot join.

  Because the scheme puts position, outcome, slug and key all in the *filename*,
  the whole shape costs one `scandir` per directory and the only file read is a
  live leaf's `**Kind:**` line — so refreshing is a 1 s poll rather than a
  filesystem watcher. One zero-dependency Python file plus a manifest; `python3`
  is the only requirement, so there is no build step. macOS and Linux.

  Consequence worth naming: **changing the `.grove/` directory scheme is now a
  plugin-compatibility question**, as *herdr-optional-ui* anticipated.

## v16.0.0

The loop starts talking to herdr, and the task-kind set triples. A `grove do`
pane now reports `working` / `blocked` / `idle` over herdr's own socket, so a
grove parked on a question stops reading as *done*; and the five task kinds
become seventeen, parameterised rather than flat, with the two routing
mechanisms they need. **Two breaking changes**: `work` is renamed `impl`, and a
kind that resolves no model variable now fails the launch instead of silently
inheriting your default — the migration snippet is under the taxonomy entry.

### Added

- **A `grove do` pane now tells herdr what it is doing** (*herdr-optional-ui*).
  The loop driver reports `working` while a session runs, `blocked` when the loop
  parks needing a human, and `idle` when the grove finishes — over herdr's own
  unix socket, addressed by the `HERDR_*` variables herdr puts in the pane
  environment. This fixes the complaint the feature exists for: a grove that
  stopped overnight used to read as **done**, because herdr derives `done` from
  `idle && !seen` and nothing ever reported `blocked`.

  **Entirely optional, and never load-bearing.** With no herdr present — or with
  stock, unpatched herdr, which drops the reports — every grove behaviour is
  unchanged, minus the status surface. A refused or wedged socket is a no-op
  bounded at 500ms, never a failed launch and never a stalled loop.

  Two details worth knowing. **Release is not "on every exit":** grove hands the
  pane back on `complete --done` and on SIGTERM/SIGHUP, but a loop parked without
  a completion signal keeps its `blocked` report — releasing would return the
  pane to screen detection, which reads a parked grove as `idle`, restoring the
  very bug. And **the driver sees session boundaries, not turn boundaries**: a
  session stalled *mid-turn* on a question reads `working`, not `blocked`. That
  is a strict improvement on `done`, not the whole fix; intra-turn state needs
  per-harness hooks. Uncovered by design, since herdr never expires an authority:
  SIGKILL, panic, OOM and power loss pin the pane at grove's last state until the
  next `grove do` or a `herdr pane release-agent`.

- **`install.sh` links into Pi as well as codex and gemini.** Pi reads personal
  skills from `~/.pi/agent/skills`, one level deeper than the others; the
  existing "install only if the parent directory exists" guard needs no
  special-casing for it, since `dirname` yields `~/.pi/agent` — present only
  when Pi is set up. Pi had been symlinked by hand against `Linkuistics/skills`
  and so was left behind by the graft, frozen at the pre-archive skill set;
  re-running `install.sh` re-points it here and picks up `using-jujutsu` and
  `git-to-jj-mapping`, which no hand-linked harness had.

### Changed

- **Five task kinds become seventeen, and routing gains the two mechanisms they
  need** (*task-kind-taxonomy*, *model-per-task-kind*; membership and each kind's
  discipline in `docs/specs/task-kind-taxonomy.md`). The set is now
  **parameterised rather than flat**: five producers — `requirements`, `design`,
  `planning`, `prototype`, `impl` — each with its own `review-<producer>` and
  `integrate-review-<producer>` step, plus `research` and `combine-research`. The
  old set of five carried two loads badly once a workstream ran across more than
  one vendor: one `review` label meant one discipline and one model bucket for
  five genuinely different reads (judging whether a decomposition is made of
  vertical slices is not judging whether code is correct), a review's findings had
  no named successor session and borrowed `work`, and `planning` fused eliciting
  *what* to build with deciding *how*. The set stays **closed** and still
  gate-on-write / degrade-on-read: a grow verb rejects an unknown `--kind` (a
  human is present to fix it), while an unrecognised `**Kind:**` line warns and
  reads as `impl`, so a hand-edited file can never jam an unattended relaunch.

  **`work` is renamed `impl`** — it named both a member of the set and the
  category containing it. Existing trees need no edits: `**Kind:** work` reads as
  `impl` *silently* (it is the previous spelling, not a typo), while `--kind work`
  on a grow verb is refused with an error naming the replacement.

  **Grilling moves off `planning` and onto `requirements`.** `planning` keeps its
  methodological force — still the sole branch in the loop's Execute step, still
  the only kind that grows the tree generatively — but no longer interrogates, and
  flips from HITL to AFK. Only `requirements` and `prototype` are now HITL, by a
  rule rather than a list: a kind is HITL when *a human's own words are the
  session's input or its deliverable*. The mark **predicts, it does not permit** —
  any kind may stop and ask a human, and doing so is always legitimate.

  **A fresh grove's bootstrap leaf follows the grilling: `root-init` now mints
  `requirements`, not `planning`** (*fresh-grove-start-contract*, reworked in
  place). On a brand-new working tree the session's only input is the human's own
  words — nothing else is on disk — which is the HITL rule itself, so keeping the
  `planning` label would have meant re-marking `planning` HITL or carving a
  bootstrap-shaped exception into a mark that is deliberately rule-generated. The
  leaf is labelled for the discipline that *always* applies; a small workstream's
  bootstrap session may still cut its own leaves (the permitted requirements /
  design / planning fusion), and a larger one adds a `planning` leaf for a fresh
  session. `root-init` takes no `--kind`, because the driver launches the `start`
  session *before* that verb has run and can only route the bootstrap by
  construction. **Config consequence:** `GROVE_REQUIREMENTS_MODEL` (or its
  harness-scoped spelling) is now the one variable a brand-new grove cannot start
  without — it was `GROVE_PLANNING_MODEL`.

  **Routing keys on a family, not only the full kind.** Two families exist,
  `REVIEW` and `INTEGRATE_REVIEW`, so `GROVE_REVIEW_HARNESS=codex` is one line
  covering all five `review-*` kinds instead of five lines hand-kept in sync.
  Without it a seventeen-kind set would not have paid for itself. Model resolution
  is now four keys, **harness-major** — `GROVE_<HARNESS>_<KIND>_MODEL`,
  `GROVE_<HARNESS>_<FAMILY>_MODEL`, `GROVE_<KIND>_MODEL`,
  `GROVE_<FAMILY>_MODEL` — because the harness axis is a *correctness* axis (a
  codex profile name is garbage to pi) while the kind axis is only a *preference*
  axis. A **rerouted** launch still consults no unscoped var, so the lattice
  truncates to the first two.

  **A leaf may name its own harness**, with a `**Harness:** <name>` line beside
  `**Kind:**` (`leaf-add --harness`, `leaf-insert --harness`, and inherited by
  `leaf-decompose` like the kind). Precedence is **leaf, kind, family, stamp**.
  Almost no leaf carries one: it exists for the research **vendor pair** — two
  `research` leaves differing *only* by which vendor runs them, then a
  `combine-research` step — which is the one shape a kind→harness *function*
  cannot express. Unlike `**Kind:**`, the line is read **strictly**: an
  unrecognised name, or an empty `**Harness:**`, refuses to launch rather than
  degrading, because a wrong harness would run the leaf on a vendor the tree
  explicitly said not to.

  **Breaking: a kind that resolves no model variable now fails the launch.** This
  *inverts* the previous rule ("unset ⇒ no `--model`; the session inherits your
  own default"). The old rule never clobbered a default you already had — true,
  and beside the point: falling through to the harness's own default is still
  grove deciding which model runs a `review-impl` leaf, only less visibly, and it
  makes **partial configuration** indistinguishable from complete configuration.
  Three exemptions, each an absence of the question rather than a default: no live
  leaf (the finish-cycle iteration), a harness with no model flag at all, and an
  unset `GROVE_<KIND>_HARNESS` (which means the *stamped* harness — an explicit
  on-disk binding). A degraded kind peek now bails in every case, not only when a
  harness override is configured, so the zero-subprocess launch is gone: the kind
  peek runs on every iteration.

  **If you drove groves with model variables set, migrate before your next
  `grove do`** — a stale config now stops the loop the first time it reaches a
  kind it does not cover. Rename `GROVE_*_WORK_MODEL` to `*_IMPL_MODEL`, and add a
  variable for every kind your groves actually reach. Full coverage is about nine
  variables against a ceiling of 95, because the stamped harness absorbs every
  kind that is not rerouted:

  ```
  export GROVE_REVIEW_HARNESS=codex          # one line, all five review-* kinds
  export GROVE_CODEX_REVIEW_MODEL=sol-xhigh  # rerouted ⇒ needs the scoped spelling
  export GROVE_REQUIREMENTS_MODEL=opus GROVE_DESIGN_MODEL=opus
  export GROVE_PLANNING_MODEL=opus GROVE_PROTOTYPE_MODEL=sonnet
  export GROVE_IMPL_MODEL=sonnet GROVE_RESEARCH_MODEL=opus
  export GROVE_COMBINE_RESEARCH_MODEL=opus GROVE_INTEGRATE_REVIEW_MODEL=opus
  ```

  **Two composition patterns run over the set, and grove enforces neither.** The
  **review chain** (`X` → `review-X` → `integrate-review-X`) is sequential and
  adversarial, and each step is a different kind — so per-kind routing expresses
  it. The **vendor pair** (`research` → `research` → `combine-research`) is
  breadth-and-confirmation, two independent surveys unioned, and is why the
  per-leaf declaration exists. grove does not validate that a `review-X` leaf
  follows an `X` leaf and will not warn when one does not, because a grammar is a
  relation *between* leaves and grove expresses no relation between leaves — the
  same principle that keeps "the reviewer must not be the author" out of grove. A
  non-blocking lint was costed and rejected: it would fire on a tree the human
  deliberately shaped, demand no action, and re-trigger on every `leaf-insert`.
  `combine-research`, not `research`, carries the adversarial move — two vendors
  on overlapping corpora can agree on something false, so **agreement without
  independent primary sourcing is a red flag, not a confirmation**.

  Documented across `grove do --help`, README (`## Configuration`),
  `content/TASK-FORMAT.md`, `content/SKILL.md`, `content/driving.md`,
  `content/grilling.md`, `content/SPEC-FORMAT.md`, `content/BRIEF-FORMAT.md`,
  `docs/grove.md`, `docs/driving-a-grove.md`, `docs/workflows/multi-step.md` and
  `CONTEXT.md`. The field guide's worked example (`refactor-to-archon`) predates
  the taxonomy, so it now says so once and keeps its quotations verbatim rather
  than back-dating the old labels out of the record. Two artifacts were also
  re-attributed, which is a correction and not a relabel: a **spec** is written by
  a `design` task, not a `planning` one (`planning` consumes it and cuts the
  leaves), and a **`BRIEF.md`** is written by whichever session creates its
  node — `planning` cutting the tree *generatively*, or a leaf of any kind that
  proved bigger and decomposed itself, since `leaf-decompose` is kind-agnostic and
  inherits the parent's kind.

- **The `linkuistics` and `testanyware` plugins now ship from this repo;
  `Linkuistics/skills` is archived** (*skills-monorepo*). That repo's history is
  grafted in here, so `git blame` on `plugins/linkuistics/skills/*` still traces
  past the merge. The two components live together because they change in
  lockstep — most grove changes need a matching skill change, and two repos made
  every such change a cross-repo pair no single commit could carry.

  **If you installed the marketplace before this change, run both of these:**

  ```
  /plugin marketplace remove linkuistics
  /plugin marketplace add Linkuistics/grove
  ```

  Nothing else changes: the marketplace keeps the name `linkuistics` (its
  identity is the `name` field in `marketplace.json`, never the repo URL), so
  `linkuistics@linkuistics`, `testanyware@linkuistics` and every
  `linkuistics:<skill>` reference keep working untouched. **Re-pointing is not
  optional even though nothing will break loudly:** an archived GitHub repo stays
  readable, so `autoUpdate: true` keeps *succeeding* against `Linkuistics/skills`
  — the skills simply freeze at the last commit before the archive, with no error
  surfaced. `install.sh` users re-clone from `Linkuistics/grove` instead.

### Fixed

- **`grove do --no-launch` now checks the readiness it reports — and says what it
  found** (*model-per-task-kind*). The flag returned *above* both config checks,
  so on a half-configured environment it printed `grove: ready in <path>` and
  exited 0 while the very next real `grove do` died on a missing model variable.
  That is the same partial-configuration invisibility the required-model rule
  exists to eliminate, arriving through the dry-run door. It now runs the same
  pre-flight and the same per-kind routing resolution a launch does — the
  identical code path, not a parallel config check — so it fails on exactly what
  a launch fails on and names exactly the variables a launch names.

  The report is informative rather than merely non-committal:

  ```
  $ grove do --no-launch
  grove: ready in ~/code/acme/rate-limiting — next leaf .grove/02-design-k2.md (design) on claude, model opus (no-launch)
  ```

  A brand-new tree reports the bootstrap session instead of a leaf, and a grove
  with no live leaves left still reports ready — the finish-cycle session has no
  task to require a model for. **Two new failure modes, both deliberate:** the dry
  run now needs `grove-llm` resolvable and the harness binary on `PATH`, because
  both are conditions of the launch it is reporting on. It still writes no stamp —
  a documented dry run must never permanently rebind the grove.

## v15.0.0

grove goes dual-VCS: jj-enabled working trees — native, colocated, and
secondary jj workspaces — drive first-class alongside git, from detection
through the codex sandbox grants to the prose.

### Added

- **jj-enabled working trees drive first-class.** A thin jj-first probe, no
  VCS trait: the nearest ancestor's `.jj/` wins over a `.git` beside it, so a
  colocated repo drives through jj while a git repo nested under a jj tree
  stays git — the repo's state alone picks the interface, and git remains it,
  silently, in not-jj-enabled trees. In a jj-enabled tree the workspace root
  comes straight from the marker walk (no jj binary needed); main-repo
  resolution from a secondary workspace runs `jj workspace root --name
  default --ignore-working-copy` (never snapshotting); renames go plain (jj
  has no index to keep in step — a `git mv` in a colocated tree would stage
  into an index jj ignores); and the adoption migration commits via
  fileset-scoped `jj commit .grove` (jj-authored: change-id, op-undoable).
  Tests cover all three repo shapes.
- **codex launches grant the jj store** (*codex-gitdir-grant*). The
  `--add-dir` grant goes per-VCS: git trees keep the absolutized common dir
  exactly as before; jj-enabled trees grant the main workspace's `.jj` — a
  secondary workspace's every op lands in the *main* workspace's `.jj/repo`,
  outside the sandbox cwd — plus the main `.git` when it exists (colocated),
  where jj's git backend writes commit objects and exported refs into the
  carved-out gitdir. Each grant proven load-bearing by live `codex exec`
  probes across every jj shape. Previously the grant derivation shelled
  `git rev-parse --git-common-dir` unconditionally, so a codex launch in a
  jj-native tree died at spawn.

### Changed

- **The prose speaks dual-VCS, git default** (*user-owned-worktrees*,
  reworked in place to user-owned *working trees*). Conceptual lines go
  VCS-neutral ("the VCS holds the history"); wherever a concrete command
  appears, both interfaces are named (`git init` / `jj git init --colocate`,
  `git rev-parse --show-toplevel` / `jj workspace root`); verb-behaviour
  notes carry the jj plain-rename reality; CLI help drops its git-only
  precondition claim. grove reads no branch — and now, no bookmark.

## v14.0.0

The harness trial's remaining launch-time gaps close: codex sessions can
commit, pi sessions carry names, and a `brew upgrade` mid-loop stops the loop
cleanly at the next session boundary instead of hanging it at the next
completion signal.

### Added

- **Version-skew guard** (*self-driving-loop*). Before each session launch
  the driver confirms that `grove-llm --version` — resolved as the *agent*
  resolves it, through `GROVE_LLM_BIN`/PATH, deliberately not the driver's
  prefer-the-sibling rule (the stale sibling would match the stale driver and
  hide exactly the skew being checked for) — still reports the driver's own
  compiled-in version. A `brew upgrade` mid-loop replaces the binary on disk
  while the running driver keeps executing the text segment it started with,
  silently splitting the signal protocol's two halves: every session hangs at
  its completion signal, nothing relaunches, no diagnostic. On a confirmed
  mismatch the loop now stops before the next session, naming both versions
  and the restart instruction (restart ≡ continuation). An *unreadable*
  version (missing binary, failed or unparseable `--version`) only warns and
  continues — the guard guides, it does not gate. Checked per session, not
  per driver start, because a mid-loop upgrade is precisely what a start-time
  check misses.
- **`## Requirements` in the spec format.** SPEC-FORMAT.md gains an optional
  requirements section — one `### Requirement:` per behaviour with a SHALL
  statement specific enough to test, each `#### Scenario:` a WHEN/THEN
  acceptance case; scenarios say *what must pass*, `## Test seams` says
  *where it is tested*. The requirement/scenario language is adapted from
  OpenSpec (MIT, `LICENSES/openspec.LICENSE`) — the spec language only, none
  of its delta/validation machinery.

### Fixed

- **codex sessions can commit** (*codex-gitdir-grant*). codex's
  `workspace-write` sandbox carves the repository gitdir out read-only, so
  `git commit` — and with it grove's mandatory Commit and Retire steps —
  failed inside every codex session. Every codex launch now appends
  `--add-dir <absolutized git-common-dir>`: one path covers both repo shapes
  (a linked worktree's gitdir is a subpath of the common dir; a plain
  checkout's common dir *is* `.git`), grants are additive so the default
  writable roots stay intact, and the flag is harmless when the sandbox is
  off. No other harness is touched.
- **pi sessions are pre-named at launch.** pi does have a launch-time
  session-name flag — `--name/-n` (verified live on pi 0.80.10) — so pi
  launches now set the standard `<repo>: <name> grove` display name; v12
  shipped without pre-naming on the mistaken reading that no such flag
  existed.

## v13.0.0

The self-driving loop's session-end kill moves from the agent to the loop
driver: an in-agent self-kill cannot be trusted under every harness sandbox.

### Breaking

- **`GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID` are gone.** The loop driver no
  longer exports either — it spawns the harness session directly (no `sh -c
  export…exec` wrapper) and kills its own child itself, so the agent never
  needs its own PID. `grove-llm complete`'s `--pid`, `--grace`, and
  `--kill-grace` flags are gone with them; `GROVE_KILL_GRACE` and
  `GROVE_KILL_GRACE_KILL` are now read by the driver instead of by `complete`.

### Fixed

- **`grove-llm complete` now actually ends a codex session.** codex's
  Seatbelt sandbox denies a same-sandbox process signalling its own session
  (`(allow signal (target same-sandbox))`), so the previous self-spawned
  delayed killer's `kill -TERM`/`kill -KILL` silently failed under codex (the
  `EPERM` was hidden by `2>/dev/null`) — a codex-driven loop never relaunched
  on its own. The loop driver, running outside any harness sandbox and always
  able to signal its own child, now watches for the completion signal itself
  and applies the same grace → SIGTERM → kill-grace → SIGKILL sequence to the
  session it spawned.
- **An out-of-range `GROVE_KILL_GRACE` no longer panics the driver.** The
  watcher clamped negatives but passed non-finite and absurdly large values
  straight to `Duration::from_secs_f64`, which panics on them: `inf` or `1e300`
  took the whole loop down. Non-finite values now fall back to the default and
  finite ones clamp into `[0, 3600]`.
- **`grove do`'s pre-flight now checks every harness a per-kind override could
  route to, not just the stamped one.** Dropping the `sh -c` wrapper (above)
  changed a missing harness binary's failure mode: a genuinely unspawnable
  binary now aborts `grove do` with a loud `ENOENT`, rather than the old
  `sh`-absorbed exit 126 that looked like a friendly "the human exited" stop.
  Loud-over-silent is right, but pre-flight validated only
  `resolve_for_launch`'s stamped harness — so `GROVE_REVIEW_HARNESS=pi`
  against a grove stamped to `codex` passed pre-flight with `pi` not
  installed, ran for however long, and only aborted once a review leaf was
  finally picked. Pre-flight now resolves and checks every configured
  `GROVE_<KIND>_HARNESS` override too (through the same
  `GROVE_HARNESS_BIN`/`GROVE_HARNESS_BIN_<NAME>` resolution the real launch
  uses), naming the offending override var and binary in the diagnostic.

## v12.0.0

grove learns to route: a `pi` harness joins claude and codex, leaves can be
routed per **kind** to a different harness than the grove's own, and model
selection becomes harness-scoped — the shape needed to drive two harnesses
(two subscriptions) concurrently and still send every review to one reviewer.

### Breaking

- **codex launches use `--profile`, not `--model`.** A codex profile binds
  model + reasoning effort, which a bare model flag cannot; model-per-task-kind
  values for codex now name profiles defined as `$CODEX_HOME/<name>.config.toml`
  files (not a `[profiles.<name>]` table in `config.toml`).
- **`GROVE_HARNESS_PID` replaces `GROVE_CLAUDE_PID`.** The loop wrapper still
  co-exports the old name and `grove-llm complete` still reads it as a
  fallback — for this release only.
- **Skill provisioning is multi-harness.** `grove do` extracts the embedded
  methodology into every installed harness's skills dir (`~/.claude/skills/grove`,
  `~/.codex/skills/grove`, `~/.pi/agent/skills/grove`), replacing symlinked
  `grove` entries with real dirs (links are removed as links, never followed).
  A `grove` entry that is neither a symlink nor grove-provisioned is refused.

### Added

- **`pi` harness** (`--harness pi`): launches `pi` with `--model` (pi accepts
  `provider/id` patterns), no session pre-naming (pi has no launch-time name
  flag), skills under `~/.pi/agent/skills`.
- **`GROVE_<KIND>_HARNESS`** — route leaves of one kind to another harness at
  launch (e.g. `GROVE_REVIEW_HARNESS=pi`). Model resolution follows the
  post-override harness. Unknown names fail loudly.
- **`GROVE_<HARNESS>_<KIND>_MODEL`** — harness-scoped model vars
  (e.g. `GROVE_PI_REVIEW_MODEL`) that beat the base `GROVE_<KIND>_MODEL`.

### Fixed

- **Explicit `--harness` now always persists** to `.grove-stamps/<name>`.
  Previously only multi-harness repos stamped, so an explicit choice in a
  repo with a single (different) harness dir silently reverted on the next
  plain `grove do`.

## v11.0.0

grove exits the git-topology business. Through v10.0.3, `grove do <name>` owned a canonical layout — it created `<repo>/.grove-worktrees/<name>/` on a same-named branch, re-attached it if orphaned, and the finish cycle merged that branch to the default, removed the worktree, and deleted the branch. That layout fights any tool that wants to own worktree placement itself (e.g. [worktrunk](https://github.com/max-sixty/worktrunk)): grove and the tool each assume they control creation, naming, and teardown. grove now owns none of it — the workflow's one precondition is *a git working tree*, user-provided, on any branch, anywhere on disk; grove reads no branch, ever. (ADR `user-owned-worktrees`)

### Breaking

- **`grove do` loses its `<name>` argument and `--start-point`.** Run it argument-less, from inside the working tree you've already created (`git init`, `git clone`, a plain checkout, or a linked worktree from your own tooling). It still inspects state on disk and dispatches exactly as before — no `.grove/` yet → bootstrap; a live tree → continue; no live leaves → propose the finish cycle — the state-dispatch logic is unchanged, only where the worktree itself comes from (ADR `do-is-sole-lifecycle-verb`, reworked in place: the creation and orphan-reattach dispatch arms are gone, since there is no longer a canonical location to create or re-attach).
- **The grove's name is now the working tree's own directory basename**, read via `git rev-parse --show-toplevel` — never derived from a branch or a canonical path. It names the root brief, the harness session (`<repo-basename>: <name> grove`), and the harness stamp (`<repo>/.grove-stamps/<name>`), exactly as before; only the source of the name changed.
- **`grove retire` addresses a node as `grove retire <node-path>`, in-worktree** — the old two-part `<name>/<node-path>` addressing is gone with the canonical layout, since `<name>` is no longer a lookup key, just the label the current working tree happens to carry.
- **The complete finish cycle shrinks from six steps to three.** It now ends at *promote → delete `.grove/` in one commit → signal `complete --done`*; the old merge / worktree-remove / branch-delete steps are gone (ADR `in-session-finish-cycle`, reworked in place). Integrating the branch and tearing down the working tree are the user's own git/gh (or worktree tool) from here — grove creates no topology, so symmetrically it merges and deletes none. Resume logic shrinks to match: a half-finished grove resumes at "promote" if `.grove/` still exists, or reports "already finished" if it's already gone — the old merge-base / worktree / branch resume checks no longer apply.
- **No topology convenience verbs.** An earlier design for this release sketched `grove create <name>` / `grove remove <name>` as opt-in utilities outside the main workflow; both were eliminated in the same grilling that settled the inversion; nothing in the CLI surface replaces the worktree/branch handling that's gone. The surface is now exactly `do` / `migrate` / `retire`.

### Migrating an existing grove

Nothing needs to move. A worktree already sitting at `<repo>/.grove-worktrees/<name>/` on branch `<name>` is just an ordinary git working tree now — `cd` into it and run argument-less `grove do`; `.grove-worktrees/` carries no special meaning to grove any more, so leaving it in place or relocating it with plain `git worktree move` are equally fine. A new grove needs a working tree you make yourself before the first `grove do`.

## v10.0.3

Two bug fixes, each filed against a precondition that was false or never examined: grove could not rename a task-tree entry it had not yet committed, and the codex harness declared a command-line flag codex does not have.

### Fixed

- **A task-tree entry grown this session can now be renamed.** grove renames an entry with `git mv` in five places — `leaf-insert`'s sibling renumber, `leaf-decompose`'s leaf→`BRIEF.md` promotion, `leaf-retire`'s `DONE` infix, `leaf-prune`'s `ABANDONED` infix, and the v1→v2 migration. But `git mv`'s job is to move an entry in git's *index*, and grove's grow verbs deliberately create entries **untracked** — working-tree changes only, folded in by the enclosing task's commit. So grove routinely produced files its own rename primitive rejected, and the ordinary rhythm of a planning session — grow a few leaves, then realise one must sequence earlier, or decompose one, or finish one — hit a raw `fatal: not under version control` with no hint at the cause or the fix. That was never git being obstructive; it was grove asking git to move something it had not been told about. The primitive is now fixed in one place (`src/tree_rename.rs`) and dispatches on the entry's state: **tracked ⇒ `git mv`**, so the index entry moves with the file and `git status` still shows a clean staged rename before you commit; **untracked ⇒ a plain filesystem rename**, because there is no index entry to move — the entry was untracked before and is untracked after, at a new name, and the same `git add` that was always going to fold it in still does. Neither branch is a fallback for the other; each is the correct operation for the entry's state. A commit records no rename information either way (git infers renames at diff time, by content similarity), so the two branches commit byte-identical trees. (Issue #3)

  **`leaf-prune` changes behaviour.** It had met this same defect and answered it differently — detecting an untracked leaf and refusing with "run `git add` first". Renaming one now simply works, so that check is gone. Its two-phase validate-before-mutate walk stays, because a leaf whose `ABANDONED` name is already taken still has to fail the whole call cleanly rather than half-mark a subtree.

- **The codex harness no longer passes a flag codex does not have.** `src/harness.rs` gave codex `name_args: &["--name"]`, under a comment claiming it had been verified against `codex --help`. codex has no `--name` — checked against codex-cli 0.144.1, zero matches. Session names do exist in codex, but are assigned *after* start (`codex resume` takes a "session id (UUID) or session name", and the name is set in-session via `/rename`); naming at launch is an open upstream request. A grove session on codex would therefore have died in codex's argument parser before any session began — latent until now only because harness selection runs in single-harness mode and no grove drives codex. Conversely, codex *does* accept `-m, --model <MODEL>`, so its `model_args: &[]` had opted it out of model-per-task-kind (v10.0.0) for no reason. Both are corrected: codex declares no launch-time name flag (the launch paths already skip pre-naming when the template is empty) and takes part in model-per-task-kind on the same terms as claude. A false verification claim is worse than no comment — it tells the next reader not to re-check — so the field's own doc comment, which carried the same claim one level up, is corrected too. (Issue #1)

## v10.0.2

A bug fix: the self-driving loop swallowed the one warning that explains a model downgrade, so an unrecognised task kind quietly launched its leaf on the cheapest model.

### Fixed

- **An unrecognised `**Kind:**` line no longer downgrades a leaf's model in silence.** Reading a kind *degrades* rather than errors — an unrecognised token (a typo, a hand-edited file, or a tree written by a newer grove) warns and is treated as `work` (ADR `task-kind-taxonomy`), so a typo can never jam an unattended relaunch. But the warning rides a **zero exit**: `grove-llm kind` prints it on stderr and exits 0. The loop driver ran that peek through `Command::output()`, which *captures* stderr, and only read it back on the failure branches — so on the degrade path the warning was discarded outright. The leaf then launched on `GROVE_WORK_MODEL`, which in a typical configuration is the *cheapest* model, with nothing on screen to say why. Because the degrade always lands on `work`, the failure was always in the cheap direction, and so never announced itself as a failure. `resolve_kind` now **inherits** the child's stderr instead of capturing it, so every `grove-llm kind` diagnostic — this warning and any future one — reaches the operator; the failure branch drops its now-redundant echo. The degrade itself is unchanged: the kind still resolves to `work` and the leaf still launches, because model selection must never be a reason to stop the loop. (Issue #2's sibling, issue #4)

  **On the symptom that prompted this.** Issue #4 reported research leaves launching on Sonnet. That observation was a **version skew, not a dispatch bug, and it needs no fix**: those leaves lived in the grove that was *building* the five-kind taxonomy, driven by the then-installed v9.1.0 — whose `Kind` was `{Work, Planning}` and which had no degrade-on-read at all, so `grove-llm kind` simply failed on `**Kind:** research`, `resolve_kind` degraded to `None`, no `--model` was passed, and the session inherited the harness's own default. Dispatch on v10 is correct and verified end-to-end. What the investigation *did* surface is the defect fixed above — and the reason it was hard to find in-session is precisely that the diagnostic was being thrown away.

## v10.0.1

A documentation fix. Two files still described grove's task kinds as a binary after v10.0.0 made them a closed set of five. The binary is unchanged apart from the embedded methodology it carries.

### Changed

- **`content/SKILL.md` and ADR `self-extension-core-and-methodology` point at the task-kind taxonomy instead of restating a count.** Both still read "the two task kinds" after v10.0.0 replaced the `planning`/`work` binary with a closed set of five — a stale count in the *embedded methodology*, which `grove do` extracts to `~/.claude/skills/grove/`, so a session was told there were two kinds while `leaf-add --kind` already accepted five. Both now name the taxonomy and cite where it is defined (`content/TASK-FORMAT.md`, ADR `task-kind-taxonomy`) rather than restating a number that a sixth kind would falsify again. The v10.0.0 entry below has also been completed: the five-kind taxonomy shipped in that release but was never written down here.

## v10.0.0

This release expands grove's task kinds from a `planning`/`work` binary to a closed set of five, replaces grove's two planning artifacts with one — the spec absorbs the PRD — and gives grove a representation for abandoned work: a leaf can now be pruned in place (`ABANDONED`) instead of hand-deleted, closing the defect where deletion silently re-issued a retired permanent key.

### Breaking

- **grove has one planning artifact: the spec. `docs/prd/` is gone.** Through v9.1.0 grove named two artifacts a planning increment could produce — a PRD (`docs/prd/`, "human-facing agreement checkpoints; committed, never retired") and a design spec (`docs/specs/*-design.md`, "workstream-level technical design"). No document ever honoured the distinction, in this repo or downstream, because no step ever tested it. They are now one artifact: **a spec at `docs/specs/<slug>.md`**, slug-named, described by a new bundled format guide `content/SPEC-FORMAT.md`. Two rules make the set checkable. **Membership:** *would a session on an unrelated future grove need to read this?* If not it is a `BRIEF.md`, and it dies with `.grove/` — work-orders, keep/delete tables, and "the input for the next three leaves" were always briefs. **Grain:** an ADR records one decision and its trade-off; a spec describes how an area works, and *cites* the ADRs in its area rather than restating them. Like `docs/adr/` since v9.0.0, **`docs/specs/` is now a minimum coherent set describing the current design** — edited, merged and split in place, and deleted once a spec describes nothing (constraint 1: the artifacts hold the present, git holds the past). It does *not* inherit the PRD row's "committed, never retired". Specs also drop the `## Decomposition` section, which is brief material.

  **Migrating a repo that has `docs/prd/`.** grove ships no migration for this — nothing in the binary reads `docs/prd/` or `docs/specs/`, so a stale directory breaks nothing and can be converted whenever a grove next drives the repo. Per file: (1) `git mv docs/prd/<file>.md docs/specs/<slug>.md`, stripping any `NNNN-` or `YYYY-MM-DD-` prefix and any `-design` suffix — the slug is the identity; (2) apply the **membership test** — a document that only ever served one grove's leaves was a brief, so delete it rather than move it, and a document that no longer describes anything is deleted too (git holds both); (3) reshape what survives to `SPEC-FORMAT.md`'s sections, moving any `## Decomposition` into the relevant `BRIEF.md`; (4) remove `docs/prd/` once empty. Existing `docs/specs/*-design.md` files rename to `docs/specs/<slug>.md` by the same rule.

### Added

- **A closed set of five task kinds — `planning`, `research`, `prototype`, `work`, `review` — each with its own session discipline and model bucket.** Through v9.1.0 every leaf was either `planning` or `work`, which forced three genuinely different sessions — a citation-disciplined literature survey, a deliberately throwaway spike, and a fresh-context adversarial read — to share one label, one discipline and one model. grove now names all five, and each earns its place by carrying behaviour beyond a name. The set is **closed**: a sixth kind is a change to grove's code and docs, not a free-text label a leaf may coin — because grove *reasons* about the kind rather than merely reporting it (`leaf-decompose` gives a new node's first child its parent leaf's kind, so a research leaf that proves bigger becomes a research node, and a label grove cannot enumerate is one it cannot give defaults for or key a model bucket on). Enforcement is deliberately **asymmetric — gate on write, degrade on read**: `leaf-add` / `leaf-insert` / `leaf-decompose` reject an unknown `--kind` with an error listing the five (a human is present at authoring time, so catching `reserch` there costs one retry), while an unrecognised `**Kind:**` line — hand-edited, or written by a future grove version — emits a warning and is treated as `work`, because the self-driving loop relaunches unattended and a typo must not jam it (constraint 5: grove guides rather than gates). Only **`planning`** carries methodological force — it remains the loop's sole branch and the only kind that grows the tree; the other four are work-shaped sessions producing an artifact, differing in discipline, not in what the loop does with them. Each kind is marked HITL (`planning`, `prototype`) or AFK (`research`, `work`, `review`) in `content/TASK-FORMAT.md` — documented guidance with no machinery behind it. Existing trees are unaffected: `planning` and `work` keep their labels and meanings, and no leaf changes kind on its own. (ADR `task-kind-taxonomy`)
- **Five model variables, one per kind**, extending v9.1.0's two: `GROVE_PLANNING_MODEL`, `GROVE_RESEARCH_MODEL`, `GROVE_PROTOTYPE_MODEL`, `GROVE_WORK_MODEL`, `GROVE_REVIEW_MODEL`. The v9.1.0 rules are unchanged — an **unset variable means no `--model`** for that kind (the session inherits your own default, so grove stays a no-op until you opt in), and the launch model is a default, not a lock. Documented in `grove do --help`, README (`## Configuration`), and CONTEXT.md. (ADR `model-per-task-kind`)
- **`content/SPEC-FORMAT.md` — the spec's shape, the fifth bundled format guide.** Five sections that earn their place (`## Problem`, `## Solution`, `## Decisions`, `## Test seams`, `## Out of scope`) plus three rules: **synthesise, never re-interview** (the grilling *is* the interview and it already happened — a session that writes a spec by re-asking the questions is running grilling twice); **behavioural, not procedural** (interfaces, types and contracts — no file paths, no line numbers, no code, with one exception for a prototype snippet that encodes a decision more precisely than prose can: a state machine, a schema, a type shape); and **speak the project's language** (`CONTEXT.md`'s vocabulary; respect the ADRs in the area). Upstream `to-spec`'s mandated "LONG, extremely extensive" user-story list is optional here. `docs/adr/self-extension-core-and-methodology` has claimed grove ships a "CONTEXT / ADR / PRD format guide" set since it was written; the guide it named never existed, and that absence is the direct cause of five downstream documents carrying three naming conventions.
- **Test-seam sketching as a grilling move.** `content/grilling.md` gains *Agree the test seams*: when the increment covers code that will be tested, sketch the seams the work will be tested through and put them to the human before the design is committed. Prefer existing seams to new ones, propose new ones at the highest point you can, and drive the count down — the ideal number is one. The agreement is recorded in the spec's `## Test seams`, or, when the increment writes no spec (the common case), in the node's `BRIEF.md` — the brief chain is how a node's settled design reaches its child work leaves. `content/BRIEF-FORMAT.md`'s Pointers gains the corresponding optional item. What a seam *is* and how to judge one is owned by the **`linkuistics:codebase-design`** skill, which grove now names as a second reason the `linkuistics` plugin is a prerequisite (the first being `linkuistics:decision-records` for ADRs). Adapted from `mattpocock/skills@d574778` `to-spec` and `to-tickets`.
- **`grove-llm leaf-prune` — abandon a leaf, marked `ABANDONED` in place**, the pruning counterpart to `leaf-retire`'s `DONE`. Until now the only way to drop planned work that was decided against was a bare `git rm`, which lowers the permanent-key counter (`next_key` is `max key in tree + 1`) and lets a later `leaf-add` re-issue an already-spoken-for key — silently repointing every durable reference to it (issue #2). Marking in place instead keeps the key visible forever, exactly as `DONE` already does. `pick`, `resolve`, and `next_key` treat `ABANDONED` as non-live alongside `DONE`; `resolve` on a pruned entry now reports it as abandoned rather than looking identical to a live leaf. Given a **node**, `leaf-prune` bulk-marks every live leaf in its subtree in one call — an atomic two-phase walk that validates the whole subtree before mutating any of it, so a mid-walk failure is a clean no-op rather than a half-pruned tree — leaves `DONE` leaves untouched, and refuses the grove root (abandoning a whole workstream is a branch-delete, not a tree mark). Pruning is HITL only: the methodology (`content/SKILL.md`, `TASK-FORMAT.md`, `driving.md`) states plainly that an agent never prunes on its own, and that the durable *why* a path was rejected belongs in the ADR set, not in `.grove/`, since the task tree is deleted at the finish cycle. (ADR `pruning`)

### Removed

- **grove's two dead specs.** `docs/specs/2026-07-04-adr-disposition.md` (an executed keep/delete/merge table, self-described as "the executable input for `corpus-rework-k6`" — a brief's job) and `docs/specs/2026-07-04-adr-minimum-coherent-set-design.md` (a design seed whose content is now in `linkuistics:decision-records`, `content/ADR-FORMAT.md`, `content/SKILL.md` and `content/driving.md`). Neither was cited by any file; both are in git. This is the new rule applied to the repo that wrote it.

## v9.1.0

This release adds per-kind model selection to the self-driving loop: `grove do` launches planning and work sessions on different models via Claude Code's native `--model`. Additive and backward compatible — with neither new env var set, grove launches exactly as in v9.0.0 (no `--model`).

### Added

- **Per-kind model selection — planning and work leaves launch on different models.** Before each task launch the self-driving loop peeks the next live leaf's kind (`planning` vs `work`) and starts its `claude` session on the model named by `GROVE_PLANNING_MODEL` or `GROVE_WORK_MODEL` — so a grove can run planning (grilling, design) on a stronger reasoning model and mechanical work (code, docs, tests) on a cheaper/faster one. Selection uses Claude Code's **native `--model` flag** on the same subscription — no router, no proxy (a multi-provider proxy was rejected: it needs an API key and breaks or drains Max billing, whereas native `--model` does Opus↔Sonnet routing on the subscription for free). The kind is re-derived from the filesystem every iteration, so the loop stays stateless and restart ≡ continuation is preserved (`self-driving-loop`). Two load-bearing rules: an **unset variable ⇒ no `--model`** for that kind — the session inherits the user's own `ANTHROPIC_MODEL`/settings default, so grove is a no-op until you opt in, never clobbers a default you already have, and setting only one variable is fine (the other kind still inherits); and **the launch model is a default, not a lock** — an in-session `/model` switch overrides it but does *not* persist across relaunch, since the next task is a fresh session re-keyed on its own kind. The brand-new-grove `start` path is planning by construction (its first leaf is always planning, `fresh-grove-start-contract`), so it uses `GROVE_PLANNING_MODEL` unconditionally. Configured via `grove do --help`, README (`## Configuration`), and CONTEXT.md. (ADR `model-per-task-kind`)
- **`grove-llm kind [<leaf>]` — resolve a leaf's task kind from its task file.** A new verb on the `grove-llm` surface that reads the `**Kind:**` line via `leaf::Kind::parse` and prints a single lowercase token (`planning` or `work`). With no argument it resolves `pick`'s next live leaf; on a done grove it prints the standard "no live leaves" diagnostic on stderr and exits 0 (mirroring `brief-chain`). This is the peek the loop driver uses to select each launch model; `pick`'s bare-path output is unchanged.

## v9.0.0

This release reworks grove's decision-record methodology and prunes grove's own ADR corpus to a minimum coherent set. The CLI surface and task-tree behaviour are unchanged; the entire delta is in the embedded methodology (`content/`, extracted to `~/.claude/skills/grove/`) and grove's own `docs/adr/`.

### Breaking

- **The ADR philosophy is no longer bundled — grove now names the `linkuistics` plugin as a prerequisite.** Through v8 grove shipped a self-contained ADR guide (`content/ADR-FORMAT.md`, bundled from `mattpocock/skills`) carrying the format, the template, and the when-to-write test. That guidance now lives in the **`linkuistics:decision-records`** skill, and `ADR-FORMAT.md` shrinks to a grove-specific **placement** note (where ADRs live — slug-named `docs/adr/<slug>.md` — and multi-context placement under a root `CONTEXT-MAP.md`). `content/SKILL.md` now declares the `linkuistics` plugin a **documented prerequisite** for raising or reworking ADRs. The dependency is documentation-level, not install-enforced, and everything else grove needs stays self-contained (constraint 6) — but the long-standing "ADR guidance travels inside grove" guarantee is deliberately dropped. (`self-extension-core-and-methodology`)

### Changed

- **ADRs are a slug-named minimum coherent set reworked in place — not an append-only numbered log.** Records move from sequential `docs/adr/NNNN-slug.md` to **`docs/adr/<slug>.md`**; the slug *is* the record's identity and is cited by slug/title, never by number. The methodology now treats `docs/adr/` as a **minimum coherent set describing the current design**: when a decision changes, the set is reworked *in place* — merge / split / delete — and the briefs and ADRs that cite it are reconciled, rather than a superseding ADR being appended (git holds the history). Both the planning **Discover** step (grilling) and the **Retire** step of the loop gain an explicit ADR-set-reconciliation moment. (`task-tree-scheme`, deferring to `linkuistics:decision-records`)
- **grove's own `docs/adr/` reduced from 35 numbered records to 7 slug-named survivors.** The obsolete TUI/multiplexer/inbox tower — the `0013`–`0030` trellis/rmux ADRs and the `grove-meta` inbox-model records — described machinery already shed across the v6→v8 refactors and no longer reflected the current design; it is deleted. The surviving coherent set is `cli-binary-split`, `do-is-sole-lifecycle-verb`, `fresh-grove-start-contract`, `in-session-finish-cycle`, `self-driving-loop`, `self-extension-core-and-methodology`, and `task-tree-scheme`.
- **All numeric ADR citations reconciled to stable slugs across code, tests, and content.** References such as `ADR-0032`, `ADR-0011`, and `ADR-0035` in `src/`, `tests/`, and `content/` are rewritten to their survivor slugs (`self-driving-loop`, `fresh-grove-start-contract`, `task-tree-scheme`), so citations remain valid under the number-free slug scheme and survive future reorders.

## v6.2.1

### Fixed

- **`grove tui`'s native nav and whichkey surfaces no longer render blank.** The left `grove-nav` fleet list and the bottom `grove-whichkey` hint bar are `PaneId::Host` panes — trellis's third pane kind, which supplies its cells by drawing into an off-screen ratatui `Buffer` that the server converts to `CharacterChunk`s in `Pane::render` (exactly like a WASM plugin pane; terminal panes use a separate grid path). But both of trellis's pane-render loops gated the call that actually invokes `Pane::render` and feeds its chunks to the client — `render_pane_contents_for_client` — to `PaneId::Plugin` alone. Host panes therefore had their frame drawn but `render` never called, so every host surface composited to nothing. The companion `add_pane_contents` dispatch had already been widened to `Plugin | Host` when the host pane kind was added; the parallel render gate was the one site missed. Both gates (tiled and floating) now match `Plugin | Host`, and a new end-to-end regression test drives a host surface through `inject_host_pane → Tab::render → Output::serialize` and asserts its content reaches the composited output — the guard whose absence let the surfaces ship blank.

## v6.2.0

### Breaking

- **`grove tui` is resolved purely from config — the cwd git-repo anchor is gone.** The dashboard is now driven entirely by `fleet.toml` (`repos` + `scan_roots`) plus additive, repeatable `--repo <path>` flags; no cwd git root is detected or auto-included. `grove tui` runs from **any** directory — including a non-git one — and the pre-launch `not in a git repo (cwd: …)` error is gone (previously the gate fired before the fleet was even built, so a `scan_roots` manifest couldn't be reached from outside a repo). The cost is that the zero-config "stand in a repo → see its groves" convenience is removed: standing in a repo with no manifest now shows the empty-state, not that repo's groves. The deliberate replacements are a one-line manifest (`repos = ["."]` or a `scan_roots` entry) or **`grove tui --repo .`** to pin the current directory. An empty fleet (no manifest, no scan hits, no `--repo`) still launches the TUI, to an in-nav empty-state pointing at `~/.config/grove/fleet.toml` and `--repo` — no precondition branch is reintroduced. The fleet TUI is now a **singleton `grove-fleet` session**: a second `grove tui` re-attaches it rather than spawning one session per launch directory. The `grove tui` argument changes from a single positional `[<repo>]` to repeatable `--repo` flags. (ADR-0027)

## v6.1.0

### Breaking

- **trellis is the only TUI — the legacy in-terminal dashboard and the `--local` flag are gone.** v6.0.0 shipped the native trellis dashboard but left the pre-v6 in-terminal ratatui dashboard behind a hidden `grove tui --local` escape hatch, gated by an *off-by-default* `trellis-seam` build feature. Because nothing in the release path turned that feature on, the **shipped binary actually built without trellis and always fell back to the local, single-repo dashboard that ignores `~/.config/grove/fleet.toml`** — the fleet view never reached users. This release removes the `trellis-seam` feature (the `trellis` crates are now unconditional dependencies, so a default `cargo build` is trellis-capable), deletes the in-terminal `tui::run` event loop, and removes the `--local` flag (`grove tui --local` is now an unknown-flag error). `grove tui` launches the native trellis dashboard, unconditionally. (ADR-0026)

### Changed

- **The trellis TUI no longer reads the user's zellij configuration.** grove embeds a vendored zellij fork (trellis), not zellij, but the trellis client still resolved its config *base* from the user's zellij locations — `$ZELLIJ_CONFIG_DIR`, `$XDG_CONFIG_HOME/zellij`, `~/.config/zellij/config.kdl`, plus the user theme/layout dirs — and merged grove's config on top, so a user's stray `~/.config/zellij` could perturb grove's dashboard (and grove would even `mkdir` that directory). The fork's `find_default_config_dir` — the single chokepoint all of those sources funnel through — now returns `None` unconditionally: grove's config is trellis's built-in defaults plus grove's in-process `GROVE_TUI_CONFIG` only. A user with a populated `~/.config/zellij` now sees identical grove behaviour to one with none, and grove never creates that directory.

### Removed

- **The obsolete `grove-nav` WASM plugin.** The nav became a native in-process surface during the v6 fork (ADR-0020); the WASM plugin's embed site was already gone (nothing did `include_bytes!` on it), leaving the `crates/grove-nav` crate and its `build.rs` compile step as dead weight that also broke `cargo build` inside nested worktrees. Both are deleted; the `"grove-nav"` name now denotes only the native nav *pane* in the layout. As a result, **no `wasm32-wasip1` target is needed to build grove**.

## v6.0.0

This release rebuilds grove's TUI on its own multiplexer. The CLI and methodology surface (`grove`, `grove-llm`, the `.grove/` task tree, durable markdown artifacts) is unchanged; the entire delta is the dashboard substrate and the multi-repo/embedding capabilities it unlocks.

### Breaking

- **grove's TUI is now a native, in-process multiplexer; it no longer drives an installed zellij from outside.** Through v5 the dashboard shelled out to a stock zellij and compensated for living *outside* it with a dumb-terminal proxy seam (ADR-0016), a WASM nav plugin (ADR-0018), and a reply-only back-channel (ADR-0019). v6 hard-forks zellij 0.44.3 into **`trellis`** — grove's own TUI framework, vendored under `crates/trellis` — and compiles grove's dashboard in **natively**: grove owns `main`, links the forked `zellij-*` crates, and starts the trellis client in-process (ADR-0020/0021). All three indirection layers evaporate. Consequences for users: **the TUI is no longer walk-away-able** — deleting grove no longer leaves a stock zellij behind (the CLI/methodology core *remains* walk-away, since durable artifacts are still standard markdown), there is **no dependency on a separately-installed zellij**, and the binary is larger. `trellis` is a hard fork with **no upstream-rebase cadence** — upstream zellij is watched only as a CVE/advisory feed and relevant fixes are hand-patched. zellij's MIT license and copyright are preserved under `content/LICENSES/zellij.LICENSE` and `crates/trellis/`.
- **`grove tui` renders the native dashboard by default; the legacy in-terminal dashboard moves behind `grove tui --local`.** The pre-v6 in-terminal ratatui dashboard (no trellis, no embedding) is retained only as a hidden dev/debug escape hatch (`--local`). The supported path is the native trellis pane. Migration: nothing for normal use — `grove tui` still launches the dashboard; scripts that depended on the old in-terminal rendering should pass `--local`.

### Added

- **Fleet view — one grove process surfacing groves across many repositories.** The dashboard is no longer single-repo. Fleet membership is resolved from a hand-editable XDG manifest at `$XDG_CONFIG_HOME/grove/fleet.toml` (falling back to `~/.config/grove/fleet.toml`) with two keys: `repos` (explicit repo roots, always included) and `scan_roots` (directories grove walks to discover repos containing a `.grove-worktrees/`). `--repo <path>` flags layer additively, the current repo (cwd's git root) is always included, and repos reached by more than one route dedup by canonical path (ADR-0025). With no manifest and no flags the fleet is just `[current repo]`, so existing single-repo behaviour is preserved with zero config. Navigation is grouped two-level (repo → grove) with cross-repo, repo-qualified working-set keys, fleet-scale fs-watch, and interactive filtering.
- **The working set — a grove's harness plus its embedded tools, switched as a unit.** Selecting a grove in the nav swaps that grove's *working set* into a constant content region beside an always-on-screen nav (ADR-0022): the agent harness terminal alongside auxiliary tool panes (a shell, `yazi`, a VCS/lazygit view), each toggleable, with a responsive layout that picks a default-visible set by breakpoint. Non-selected harness ptys keep running and capturing scrollback — switching groves parks the current panes (alive, off-screen) and mounts the selected grove's, never suspending a harness. `trellis`'s headline capability is seamlessly embedding *other* fully-emulated TUI apps as first-class regions, with first-class observability of those wrapped tools.
- **Native navigation, per-grove detail, and whichkey hint bar.** A constant full-height nav surface focused by a `Ctrl-o` leader (no "home tab" to travel to); per-grove detail mounted beside the harness via an in-place content-swap substrate (ADR-0023); a native `$EDITOR` drop driven by embedded-tool exit observability (ADR-0024); and a single grove-owned full-width whichkey hint bar. The pre-v6 tab-per-grove model (`GoToTab`/`Alt-1..9` switching) is retired in favour of nav-driven content swapping.

## v5.1.0

### Added

- **`grove-llm root-init [<slug>]` — scaffold a brand-new grove's tree.** A fresh grove (worktree + branch exist, no `.grove/` yet) had no bootstrap path: `grove-llm pick` errored `grove root not found` and no verb could create the root. `root-init` creates `.grove/`, the root `BRIEF.md` stub, and a first **planning** leaf `010-<slug>.md` (default slug `plan`), so `pick` immediately returns work and the grove drops into the steady-state loop. Working-tree change only, no commit; refuses to clobber an existing `.grove/`. Creating the first leaf — not just the brief — is load-bearing: a brief-only `.grove/` reports "no live leaves; this grove is done" and would mis-trigger the finish cycle, leaving a fresh grove indistinguishable from a finished one (ADR-0011). The `start.md` launcher prompt and `content/SKILL.md` now name `root-init` as the first step of a fresh grove.
- **`grove-llm inbox-remove --for=<name>` — finish-cycle inbox cleanup.** The complete finish cycle tore down a grove's worktree and branch but orphaned its `grove-meta` inbox, so a *finished* grove kept showing up as a **Seed** in `grove status` / the TUI. The finish cycle gains a step that removes `inboxes/<name>/` via this verb. It refuses-and-instructs while observations are still pending (drain first) rather than silently discarding work another grove may have captured since the session's bootstrap drain, and is an idempotent no-op when the inbox is absent — so the state-checked finish resume needs no marker file. `CONTEXT.md`'s **Seed** definition no longer counts a finished grove's inbox; a still-orphaned inbox now signals an *incomplete* finish (ADR-0012).

## v5.0.0

### Breaking

- **`grove start`, `grove continue`, and `grove finish` removed; `grove do` is the sole lifecycle entry verb.** `grove do` already subsumed start/continue (no grove by that name → create the worktree and open a bootstrap session; live worktree → continue; branch present but worktree gone → re-attach and continue), so both were strictly redundant (ADR-0009). `grove finish` is removed too: finishing a grove is now an **in-session** step — when the grove has no live leaves left, the running loop proposes the complete finish cycle (promote durable artifacts → delete `.grove/` in a focused commit → `git -C <repo> merge <name>` → remove the worktree → delete the branch; single confirmation gate, propose-and-wait so headless runs report rather than act, and state-checked resume with no marker file — step-level design in ADR-0010). Migration: replace `grove start <name>` / `grove continue <name>` / `grove finish <name>` with `grove do <name>`. The `--start-point <ref>` flag, formerly on `grove start`, now lives on `grove do` and applies on the new-grove path. Trade-off: there is no longer a way to force-finish a grove that still has live leaves — retire or clear the leaves first.

## v4.0.0

### Breaking

- **`grove list` removed.** Its output (grove names, one per line) is a subset of `grove status`, now the canonical visibility surface (ADR-0007). Migration: parse `grove status` instead of `grove list`.
- **`grove version` removed.** Its output (CLI version + per-harness installed version) is subsumed by `grove status`. Migration: use `grove --version` for the CLI version alone, or `grove status` for the full cli/repo/worktree picture.
- **`grove update` removed; `grove install` is now idempotent** (ADR-0008). One verb converges on the bundled version from any starting state: not installed → install; same version → no-op (no empty commit); different version → update. It always prints a per-harness outcome line — `installed @ X`, `already at X, no change`, or `updated X → Y` — making the result explicit and safe to rely on in CI/setup scripts. There is no `--update` / `--force` flag and no deprecated `grove update` alias. Migration: replace `grove update` with `grove install` (add `--version <tag>` to pin). The default commit subject is still `Install grove v<ver>` for a fresh install and `Update grove to v<ver>` when refreshing an existing one. The stored `VERSION.md` stamp is now canonical (no leading `v`); the git fetch ref is unchanged.

## v2.2.0

- `**Retire.**` doctrine in `content/SKILL.md` is now imperative and procedural: after committing a task, the session mvs the just-finished leaf into `.grove/done/` (mechanical, no ask), then walks the parent chain. If a node has no live leaves left, the session **asks the user** before retiring it — the confirmation gives them a moment to add a follow-up leaf — then promotes any still-relevant brief content upward and `mv`s the node into `.grove/done/`. The cascade recurses through ancestors until a node still has live leaves or the grove root is reached. The inner-loop mermaid graph and the `multi-step.md` walkthrough are updated to match.

## v2.1.0

- `grove install` and `grove update` now produce a single path-scoped git commit covering every targeted harness path (per ADR-0001). `--no-commit` opts out and prints the staging command; `-m`/`--message` overrides the default message. Pre-flight refuses if install-scope paths already have staged hunks; unrelated dirty state elsewhere is left alone. Hook failures leave the materialisation in place and print a follow-up `git commit -- <paths>`. Multi-harness invocations produce one combined commit; no-op materialisations skip the commit.
- New per-flow walkthroughs under `docs/workflows/` (install, update, start, multi-step, finish) with an index; README and `docs/grove.md` cross-link to them.
- Documentation clarifies that `grove continue` is a session launcher and notes that up-arrow history recall surfaces the last continue prompt.

## v2.0.0

Breaking on-disk layout change. Every storage location is now dot-prefixed and the per-grove namespace is gone where it was redundant.

- Task tree: `groves/<name>/` → `.grove/` (inside the grove's worktree). One worktree = one grove, so the name no longer needs to namespace the task tree.
- Worktree: `worktrees/<name>-grove/` on branch `<name>-grove` → `.grove-worktrees/<name>/` on branch `<name>`.
- Harness stamp: `groves/<name>/.harness` → `.grove-stamps/<name>`.
- `grove finish` now explicitly deletes `.grove/` in a focused commit before merging, so the default branch never carries any grove's local state. The history of completed groves lives in git's commit graph, not in retained `done/` directories.
- `grove uninstall`'s "live groves" check is now "any worktree exists in `.grove-worktrees/`" — simpler and authoritative.

Migration: existing groves on v1.x layout need manual relocation (`mv groves/<name> .grove-worktrees/<name>-grove/.grove`, then rebranch, then refresh content with `grove update`). New repos pick up the new layout automatically.

## v1.0.1

- Relicense from MIT to Apache-2.0 (matches sibling Linkuistics projects); add the missing LICENSE file at repo root.
- Add `docs/grove.md` — project-level intro covering the methodology rationale and the CLI's workstream verbs.

## v1.0.0

- Initial public release of the grove CLI.
- Lifecycle verbs: `install`, `update`, `uninstall`, `version`, `status`, `list`.
- Launcher verbs: `start`, `continue`, `takeover`, `retire`, `finish`.
- Multi-harness support with auto-detection of `.claude/` and `.codex/`; `.harness` stamp used as a per-grove disambiguator.
- Release pipeline producing macOS arm64 and Linux x86_64/arm64 binaries.

## The `Linkuistics/skills` changelog, carried in verbatim

The skill plugins under `plugins/` were developed in a separate repo,
`Linkuistics/skills`, until its history was grafted in here (*skills-monorepo*).
That repo's changelog is preserved below exactly as it stood at the graft: one
never-versioned `Unreleased` section, newest entry first. It is a closed record —
nothing new is appended to it.

- Added the `using-jujutsu` and `git-to-jj-mapping` skills. `using-jujutsu`
  auto-fires on version-control work: in a jj-enabled repo (a `.jj/` directory
  exists) it drives everything through Jujutsu's native model
  (working-copy-as-commit, `jj new`/`jj describe`, bookmarks, op-log undo);
  otherwise git remains the interface, silently — the skills never convert a
  repo or offer to. `git-to-jj-mapping` is the on-demand git→jj command and
  concept reference, loaded only when a translation is needed. Reconciled the
  existing skills with the jj design: `guardrail` now also gates `jj abandon`
  and `jj op restore` (hook pattern list, SKILL.md table, and test suite; jj
  0.43 has no force-push flag to gate — pushes are lease-checked by default,
  pinned by a defer test), and `decision-records` generalises its "git holds
  the past/history" phrasing to "the VCS holds …" at all six sites. Updated
  the `linkuistics` manifest description/keywords and the README skills table.
- `authoring-conventions`: added **Negation** (steering by prohibition drags the
  forbidden behaviour into context and makes it more available, not less; state the
  positive target, reserve `never`/`don't` for guardrails that can't be phrased
  positively), the **context load / cognitive load** vocabulary for the user-invoked vs
  model-invoked lever plus the **router skill** cure for cognitive-load pile-up, and a
  **sentence-level no-op hunt** (test each sentence against the no-skill default; delete
  failing sentences outright rather than trim words). All three are drawn from
  `mattpocock/skills`' `writing-great-skills` skill (MIT), which postdates this repo's
  prior-art survey. `codebase-design`: added the concrete parallel-sub-agent "design it
  twice" procedure (divergent per-agent briefs: minimize-interface / maximize-flexibility
  / optimize-common-caller / ports-and-adapters), from the same upstream's
  `DESIGN-IT-TWICE.md`. Refreshed the prior-art survey's mattpocock citations with a dated
  note pointing at `writing-great-skills/{SKILL,GLOSSARY}.md` @ `d574778` as the current
  canonical source.
- Added the `decision-records` skill — ADRs as a **minimum coherent set**
  describing the design's current state (current-state over changelog,
  edit/merge/split/delete in place, identity by slug not number, the
  when-to-write test, a minimal template). The minimal template, the three-part
  when-to-write test, and the qualifying examples are distilled from
  `mattpocock/skills`' ADR-format material (MIT); the coherent-set framing is
  original. Updated the `linkuistics` manifest description/keywords and the
  README skills table.
- grove moved to its own repo (`Linkuistics/grove`) and is now distributed via
  `brew tap Linkuistics/taps && brew install grove`. The `grove/` directory and
  `scripts/materialise-grove.sh` were removed from this repo; `docs/grove.md`
  now points readers to the new repo.
- Added `grove-start` and `grove-next` shell launchers, bundled with the
  grove skill and materialised alongside it. They collapse the per-session
  restart ritual (`/clear` → `/rename` → kickoff prompt) into a single
  command: `grove-start <name>` creates the worktree at
  `<repo>/worktrees/<name>-grove/` on branch `<name>-grove` and launches a
  pre-named bootstrap session; `grove-next <name>` cd's into the worktree
  and launches a pre-named continuation session. Both work from anywhere
  inside the repo (any worktree) via `git rev-parse --git-common-dir`.
  Updated SKILL.md, docs/grove.md, and the materialise test to match.
- Moved the grove skill out of the `linkuistics` plugin tree
  (`plugins/linkuistics/skills/grove/` → `grove/`) so installing the plugin
  no longer ships a global grove. Grove is materialisation-only — a global
  installed copy would conflict with the per-project pinned copy and
  re-introduce the version-drift problem grove exists to prevent. Updated
  `scripts/materialise-grove.sh`, its test, `README.md`, and `docs/grove.md`.
- Renamed the repo to `Linkuistics/skills` and the plugin to `linkuistics`
  (namespace `linkuistics:`); added an Apache-2.0 licence.
- Added the `grove` skill — a methodology for hierarchical, self-extending,
  git-tracked task-tree workstreams. Bundles three convention files from
  `mattpocock/skills` (MIT); upstream licence in `grove/LICENSES/`.
- Added `scripts/materialise-grove.sh` — copies grove into a consuming repo
  and stamps `VERSION.md`.
- Added `docs/grove.md` — problem, solution, install/update guidance, and example
  prompts. Restructured `README.md` to lead with grove as a top-level section
  alongside the coding-style skills.
- Made grove's heritage explicit up front in `docs/grove.md` and the README's
  `## grove` section: bundling of Matt Pocock's `grill-with-docs` conventions
  and DDD's Ubiquitous Language and bounded-context concepts.
- Made grove's single-worktree-per-grove convention explicit in `SKILL.md`
  (loop preamble) and `docs/grove.md` (the "Git worktrees" subsection now
  reads as directive rather than descriptive).
- Made grove name the session: SKILL.md now instructs the LLM to suggest
  `/rename <project>: <grove-name> grove` on the first turn of grove
  activity, once per session.
- Elevated grilling in `SKILL.md`: a planning task now **opens** with a
  grilling session via `grilling.md`, rather than listing "grills" as one of
  several planning-task activities. Matched in `docs/grove.md` section 2 and
  the "Run a planning task explicitly" prompt.
- Initial release. Coding standards packaged as agent skills:
  - `coding-style` — universal principles (auto-loads on any file).
  - `coding-style-{rust,python,elixir,bash,swift,typescript}` — per-language
    style guides, auto-loading by file extension.
  - `cli-tool-design` — LLM-friendly CLI design guidance, with the audit
    checklist and refactoring sequence split into `references/`.
- Claude Code marketplace manifest (`.claude-plugin/marketplace.json`).
- `install.sh` for symlinking skills into Codex / Gemini CLI.

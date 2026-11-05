# Pack membership

This specification settles six questions of pack membership that the remaining
repository moves need. It decides where each asset goes, and it moves no file.
Each later move under `repository-moves-k5` applies the decisions that name it.

`pack-membership-k171` wrote the decisions. The human approved the two decisions
that change the destination table of the root brief. A `review-design` leaf
reads this specification before the TestAnyware move.

Grove owns this specification, for the reason that it owns the
[external pack contract](external-pack-contract.md). That contract owns the pack
layout, the family rule, the reference rules and conformance. The
[owner delivery design](owner-skill-delivery.md) owns the ten Ravel sources and
the native residue. This specification adds membership to those designs and
changes none of their decisions.

## Problem

The root brief gives each destination repository its domain. Six classes of
asset do not follow from a domain alone:

1. **The ADR and spec formats.** The root table gave them to DesignSkills, but
   every kind reaches them through the spine.
2. **The always-on hooks.** The root table gave them to PersonalSkills, but they
   are the hooks of a writing craft skill.
3. **Grove's assets of moving craft skills.** Grove holds documents, scripts and
   tests for craft skills that leave Grove.
4. **Grove's tests that name moving kinds.** Some tests use a moving kind as a
   fixture name, and others read a moving kind's file.
5. **The trial cases that name moving skills.** They are the evidence of the
   common-loading acceptance.
6. **The acceptance of each move.** The research move proved one pattern, and
   later moves add new boundaries.

Without one decision for each class, each move would answer the question again.
Two moves could then give different answers.

## Solution

- The two format files stay in the spine. DesignSkills receives the craft
  skills that deepen them.
- WritingSkills owns the always-on hooks and both hooks plugins. Its ordinary
  writing plugin gets the hooks through a dependency on the Claude Code hooks
  plugin. PersonalSkills owns the native-only `guardrail` plugin.
- An asset goes with the owner of the claim that it serves. Grove keeps its own
  claims and the evidence that Grove cites.
- A test keeps a fixture label. A test that reads a moved file reads a core kind
  or a fixture instead.
- The trial cases stay as records of the builds that they measured. The runner
  refuses a case when the supplier of a required path does not hold it at the
  declared revision.
- Each move owes the base proofs of the research move. A move that adds a new
  boundary also owes the proofs of that boundary.

## Decisions

### The ADR and spec formats

**Decision.** `ADR-FORMAT.md` and `SPEC-FORMAT.md` stay in the spine, beside the
other three format files. No move changes them. DesignSkills receives
`decision-records` and `codebase-design`, the craft skills that deepen the two
formats.

These facts decide it:

- **Every kind meets these rules.** Four trigger sentences of the spine's
  `SKILL.md` name the two files, and every kind loads the spine. Each row that
  core's manifest gives the two files therefore binds every kind.
- **The placement function puts the rules there.** Rule 3 of
  [every normative rule has one owner](../adr/corpus-rules-have-one-owner.md)
  sends a rule with an artifact occasion to the format file of that artifact.
- **The files are Grove's fallback.** Sometimes the absence of a craft skill
  changes what a session writes. Grove then states the rule locally, as
  [a skill states what binds without its dependencies](../adr/a-skill-states-what-binds-without-its-dependencies.md)
  requires. The two files are that statement for `decision-records` and
  `codebase-design`.

The design rejected these options:

- **Move both files to DesignSkills, as a skill that the spine loads by name.**
  Every kind would then depend on DesignSkills, also `planning` and `finish` in
  core. A core-only installation would lose its ADR test. Core alone would fail
  conformance, because a directed load in core would name no core skill.
- **Split the files.** The spine would keep the when-to-write test, the placement
  and the set rules. DesignSkills would keep the template and the rules for
  writing a spec. A session of any kind that passes the ADR test then writes the
  record. The split puts that next step in a pack that the session can lack.
  The craft skills already hold the deeper authoring detail, so this split
  exists already.

The root table now gives core "the tree and artifact formats (task, brief,
glossary, ADR and spec)". It gives DesignSkills design craft and states that
"the craft skills deepen the ADR and spec formats, which stay in the spine".

**Applied by:**

- `pack-membership-k171` changes the root table.
- `requirements-pack-k173` keeps the names of the two files in its moved text.
  The pack alone lists them as external resources, and the aggregate resolves
  them in the spine.
- `design-pack-k175` states in the DesignSkills README that the two formats stay
  in core.

### The always-on hooks

**Decision.** WritingSkills owns the always-on layer of
`technical-communication`, with these parts:

- The hook scripts and the always-on texts. They stay inside the skill
  directory.
- The skill-free Claude Code hooks plugin of the
  [native residue](owner-skill-delivery.md#native-residue).
- The Codex hooks plugin, whose manifest is `.codex-plugin/plugin.json` in the
  skill directory.
- The Pi extension in the skill's `harnesses/pi/`.
- A Codex marketplace file at the WritingSkills root. It lists the Codex plugin
  with a `local` source.

PersonalSkills owns the Claude Code plugin that holds only `guardrail`. The hook
of `guardrail` is user-invoked, not always-on. The owner's choice to turn on the
always-on layer is a setting, not a source. It stays in the owner's mise-tracked
harness settings, which enable the hooks plugin.

These facts decide it:

- Each hook runs a script of the skill and sends a text of the skill. A plugin
  in another repository needs a path into a WritingSkills checkout or a copy of
  the scripts.
- The owner delivery design rejected hooks that name a checkout path. A layout
  change at a pin advance breaks such a hook without a report. A copy gives the
  scripts two owners.
- Codex adds a plugin only from a marketplace. The current listing names the
  plugin by a path inside its own repository. The listing therefore stays beside
  the plugin until a remote source is proven.

The design rejected these options:

- **PersonalSkills owns the hooks, as the earlier root wording stated.** Its
  plugin has the path problem or the copy problem above.
- **PersonalSkills owns the Claude Code plugin, and WritingSkills owns the rest.**
  The Claude Code plugin has the same problem, and the layer gets two homes.

The root table now gives WritingSkills "the technical-communication always-on
hooks plugins for Claude Code and Codex". It gives PersonalSkills the personal
skills and "the native-only `guardrail` plugin with its user-invoked hook".

**The ordinary writing plugin.** Native users install the ordinary WritingSkills
plugin for its craft skills. That plugin declares the Claude Code hooks plugin
as a dependency, and it declares no hook. Neither its manifest nor a
`hooks/hooks.json` at its root names a hook. Each hook therefore has one
declaration, in the hooks plugin. Three installations follow:

| Installation | Hooks | Native `technical-communication` skills |
| --- | --- | --- |
| The ordinary writing plugin | The dependency installs the hooks plugin. One declaration for each hook | One, from `craft/` |
| The hooks plugin alone, which a Ravel user enables | One declaration for each hook | None |
| Both plugins | One declaration for each hook | One, from `craft/` |

These facts decide it:

- [The off file is the only switch](https://github.com/Linkuistics/WritingSkills/blob/main/docs/adr/the-off-file-is-the-only-switch.md)
  turns the layers on at installation. The `linkuistics` plugin met that
  through the `hooks/hooks.json` of its root, which Claude Code discovers by
  default.
- The hook scripts move with the skill to `craft/technical-communication/`,
  which is the root of the hooks plugin. Nothing then gives the ordinary plugin
  a hooks file.
- The research plugin declares `"dependencies": ["grove"]`. Its native
  installation also installs `grove`, as
  [Distribution](external-pack-contract.md#distribution) records. A dependency
  resolves by its bare name in the marketplace that lists the plugin.

The design rejected these options:

- **The ordinary plugin also declares the hooks.** An installation that enables
  both plugins then runs each hook twice. The hook declarations also get two
  owners.
- **The ordinary plugin has no hooks and no dependency.** A native user who
  installs only that plugin then gets the skill without the always-on layer.
  Each plugin still passes validation, so no check reports the loss.

**Applied by:**

- `writing-craft-pack-k181` moves the layer, both hooks plugins and the Codex
  marketplace file. It gives the ordinary writing plugin its dependency on the
  hooks plugin.
- `personal-pack-k183` moves `guardrail` and its plugin.
- `skill-marketplace-k185` lists both Claude Code plugins. The hooks plugin
  must be in the marketplace that lists the ordinary writing plugin, because
  the dependency names it by its bare name. The move lists the Codex plugin in
  the marketplace repository only if Codex resolves a plugin source outside
  that repository. It records the result in the moves record.

### The Grove assets of moving craft skills

**Decision.** An asset goes with the owner of the claim that it serves. Apply
these tests in order. The first match decides.

1. **Grove's own claim.** The asset states, checks or records Grove's
   behaviour, Grove's design or this workstream. It stays in Grove. When it
   reads a moving skill's bytes, it stops reading them from Grove's checkout.
2. **The skill's own claim.** The asset states, decides, explains or checks the
   behaviour of the craft skill. It moves to the skill's pack in the skill's
   move. Such assets are the skill's specification, guide, design views,
   decision records, self-tests and evaluations.
3. **Evidence.** The asset records a finished measurement. It stays with its
   citers, because [evidence outlives the instrument](../adr/evidence-outlives-the-instrument.md)
   draws the line at citation. When a Grove file that stays cites the record,
   the record stays in Grove. When only moving files cite it, it moves with
   them.

A moved asset keeps no Git history in the pack, as
[provenance and licences](external-pack-contract.md#provenance-and-licences)
states for moved skills. The pack's first commit names the Grove commit and the
source paths. A link between a Grove file and a moved file becomes a public URL.

The move of a skill finds every Grove file that names that skill. It classifies
each file by this rule, and it records the result in the
[moves record](../repository-moves-record.md). The table decides the classes
that this leaf names. A move that finds another class applies the same rule.

| Asset | Test | Disposition | Applied by |
| --- | --- | --- | --- |
| The `technical-communication` specification, guide and design views, and the two `design:technical-communication` tasks of the Taskfile | 2 | Move to WritingSkills | `writing-craft-pack-k181` |
| The decision records `the-off-file-is-the-only-switch`, `documents-are-gated-replies-are-measured` and `technical-communication-carries-no-standard-text` | 2 | Move to WritingSkills | `writing-craft-pack-k181` |
| `docs/technical-communication-publication-review.md` | 2 | Moves to WritingSkills. It checks the skill's text against the notice of the standard | `writing-craft-pack-k181` |
| `docs/research/harness-hooks-a.md` | 3 | Moves to WritingSkills. Its citers all move: the hook script, the specification, two decision records and the provenance section | `writing-craft-pack-k181` |
| `docs/evaluations/writing-code-walkthroughs/` and the decision record `behavioral-acceptance-is-closed-unmet` | 2 | Move to WritingSkills | `writing-craft-pack-k181` |
| The formal-methods records: `results-of-formal-methods-trial.md`, `formalism-findings.md`, `candidate-lessons.md` and `review-yield.md` | 1 | Stay in Grove. They record Grove's own campaign, and a Grove decision record and `reference_navigation.rs` cite them. The moved skills cite them by public URL | `design-pack-k175` and `engineering-pack-k177` |
| `scripts/pi-extension-load.test.sh`, the checker self-test and the `tc-hook` lint of `scripts/check.sh` | 2 | Move to the `check` task of WritingSkills. `scripts/check.sh` drops them in the Grove removal change | `writing-craft-pack-k181` |
| The assertions over `doubt-driven-development/SKILL.md` in `composition_guidance.rs` and `session_kind_guidance.rs` | 2 | Move to a check of EngineeringSkills. The assertions over Grove's own files stay | `engineering-pack-k177` |
| `plugins/linkuistics/PROVENANCE.md` | 2 | Splits by skill. Each section moves to `plugins/<plugin>/PROVENANCE.md` in its pack. The table of authoring sources moves with `authoring-conventions` | Each craft move. `personal-pack-k183` removes the empty file |

**The doubt assertions.** They check that the skill's section "Composition with
Grove" yields to Grove's review allowance. They also check the order of the
skill's handoff passage. Those are claims of the skill's text, so the check goes
to the skill's repository. That check reads Grove's matching phrases from the
pinned `GROVE_COMMIT` checkout that the pack's conformance task already makes.
A change of Grove's wording then reaches the pack by the expand and contract
steps of [the checker without a copy](external-pack-contract.md#the-checker-without-a-copy).

That is a visible trade-off. Grove's suite no longer reads the skill's text, so a
drift appears when the pack moves its record forward. The design rejected these
options:

- **Grove keeps the assertions and reads the skill from a pinned pack checkout.**
  Its unit tests would then need the network cache and a pin. The check of the
  skill's text would also be in another repository than the text.
- **Grove keeps the assertions as history.** The composition is a live contract,
  and a record that nothing runs leaves it unchecked.

**The provenance file.** Each moving skill links to `../../PROVENANCE.md`. From
`craft/<skill>/SKILL.md`, that path is `plugins/<plugin>/PROVENANCE.md`. A file
at that path in each pack keeps every link, and Ravel captures it with the source
directory. Only `authoring-conventions` cites the keys of the authoring table.
The design rejected these options:

- **Keep the file in Grove.** Each moved link would then name a file outside its
  source directory, which no delivery route makes readable.
- **Copy the whole file into each pack.** Each section would then have several
  owners.

### The Grove tests that name moving kinds

**Decision.** Each test that names a moving kind is in one of three classes, and
each class has one rule.

| Class | Examples | Rule |
| --- | --- | --- |
| **A fixture label.** A test writes a synthetic skill with a moving kind's name into a temporary tree. Or it matches that name in a prompt or a log | `directory_route.rs`, `native_skill_delivery.rs`, `lifecycle_cutover.rs`, `prompt.rs`, `grove-floor-directory-case.sh` and the fixtures of `test_harness_trial.py` | The label stays. No move changes it |
| **A walk of core's shipped skills.** A test reads `plugins/grove/skills/` and checks what it finds | `shipped_kinds` and `shipped_markdown` in `session_kind_guidance.rs`, the corpus of `plugin_fallback.rs` and the check of the kind table | The walk shrinks with each move and needs no change |
| **A named read of a moving file.** A test reads a file of a moving kind from Grove's checkout, or it lists such a file | `add_kind` in `conformance.test.sh`, and the citation registry and `MOVING_KIND_OPENINGS` in `plugin_fallback.rs` | The move that removes the file changes the test in the same Grove change |

A fixture label stays because a kind is an open token, as
[a kind is an open token](../adr/a-kind-is-an-open-token.md) states. The
fixture's bytes are its own, so the test checks label handling whatever pack
ships the kind. A renamed label would make the test depend on the destination
table and would prove nothing more.

A walk keeps its floor. `shipped_kinds` requires more than three kinds. After
every move, core ships four: `finish`, `planning`, `review-planning` and
`integrate-review-planning`.

A named read follows these rules:

- A control that needs a property of a real kind uses a core kind with that
  property. When no core kind has the property, the test writes a fixture. No
  core kind has a supporting file today.
- A list of moving files drops each moved entry.
- A registry row whose rule moves becomes a pinned row of the pack's manifest.
  The pack alone then checks that the owner states the phrase and that no other
  pack file states it.
- No Grove unit test reads the bytes of a pack. `check:packs` reads the pinned
  pack checkouts, and the unit tests stay offline.

The pinned-row case occurs once. `grove-design/SKILL.md` cites `codebase-design`
and states what binds without it. After that row moves, the sweep of
`plugin_fallback.rs` for an unregistered citation covers core only. The design
accepts that gap. Reopen it when a pack adds a citation of a craft skill.

**Applied by:**

- `requirements-pack-k173`: the `inherited-load` control reads
  `grove-requirements/grilling.md`. It takes a fixture kind with a supporting
  file. The requirements kinds leave `MOVING_KIND_OPENINGS`.
- `design-pack-k175`: the `two-kinds-one-rule` control stops adding the design
  kind. The design kinds leave `MOVING_KIND_OPENINGS`. The registry row for
  `codebase-design` moves to the DesignSkills manifest. The last paragraph of
  the dependency decision record then states that the registry covers core and
  that each pack pins its own citations.
- `engineering-pack-k177`: the controls that add the impl kind take core kinds
  or fixtures. The `two-kinds-one-rule` control takes a rule that core owns.
  The impl and prototype kinds leave `MOVING_KIND_OPENINGS`.
- `editorial-pack-k179`: the editorial kinds leave `MOVING_KIND_OPENINGS`. The
  list is then empty, and the test drops it.

### The trial cases that name moving skills

**Decision.** The trial cases stay unchanged, as records of the builds that they
measured. No move retargets them.

These facts decide it:

- The runner prepares a Grove checkout at a named commit. A case names its
  local sources, its mandatory files and its planted files inside that
  checkout. An external source comes from its own checkout at a declared
  commit. A native copy can come from an older commit of the Grove checkout.
- Every case except the research cases launches the `impl` kind. Most cases name
  `plugins/linkuistics` as a source.
- The [acceptance record](../common-loading-acceptance-record.md) names the case,
  the runner commit and the Grove build of each run. A case therefore still runs
  against the Grove commit that its record names.
- The staging trials of cutover order 5 run every source on both harnesses, from
  the real pack remotes. They accept the moved state.
- The boundary trials of the later moves add new cases with `external` sources,
  on the model of the research cases.

**The refusal.** Before any launch, the runner refuses a case when the supplier
of a required path does not hold it at the declared revision. Each required path
has one supplier:

| Required path | Supplier |
| --- | --- |
| A local source directory and its mandatory files, a planted file, a native copy without a `commit`, and a file that a continuation edits | The prepared Grove commit |
| A native copy with a `commit` | That commit, in the history of the prepared checkout. The current directory copy beside it is a separate path of its own source |
| An external source directory and its mandatory files | The checkout of the external source at its declared commit, or the working copy that `--working-copy` names |
| The kind skill of each launched kind | The source that supplies the kind. That source is a local or external source, a native copy, or the plugin checkout that a marketplace entry of the prepared commit pins. That plugin is a native plugin of the case or a dependency that the plugin's manifest declares |
| A skill that the task relies on without naming it | The source or the native plugin that supplies the skill |

A case whose task relies on a skill that it does not name, such as implicit
discovery, adds that skill's path to the case. A case that measures the absence
of a skill declares that skill absent, and the refusal does not require it.
`missing-kind`, for example, launches `impl` and measures the absence of
`grove-impl`. The runner reads these declarations only for the refusal, so no
earlier reading changes.

The check runs before a `moveAway` or `moveRoot` fault. A case that moves a
source away therefore passes the check, and Ravel's resolution then observes
the fault. The refusal names the case, the path, the supplier and the revision.
It states that the case measures a build whose supplier held the path.

`trial:controls` proves the refusal with these readings:

- A case against a commit that holds its local paths passes the check.
- The same case against a commit without one of them refuses with the
  diagnostic and a nonzero status.
- A case with an external source passes the check, although the prepared commit
  holds neither the external source directory nor its kind skill.
- A case with a native copy from an older commit passes the check, although the
  prepared commit no longer holds the copied directory.

The design rejected these options:

- **Retarget each case to external sources.** A retargeted case is a different
  instrument under the same name, and its record would then describe a case file
  that no longer exists.
- **Remove the cases.** The acceptance record cites each case by name.
- **Let Ravel's resolution refuse.** A case that expects `skill_source_missing`
  would then pass for the wrong reason.

**Applied by:** `design-pack-k175`. Its Grove removal is the first that removes
a path that a case requires from the prepared commit: `decision-records` and
`cli-tool-design`. The runner change lands before that removal.

### The acceptance of each move

**Decision.** Each pack move owes these base proofs:

1. The pack's `task check` passes from a clean checkout of its published commit,
   with the public `GROVE_REPOSITORY` and no override.
2. Grove's `task check:packs` passes with the new pin, and
   `bash scripts/check.sh` passes on the Grove line.
3. The move follows the order of
   [the checker without a copy](external-pack-contract.md#the-checker-without-a-copy).
   That order is a first pack commit, the Grove removal, and a second pack
   commit whose aggregate passes.
4. The record names the Git tree ID and the `resourceDigest` of each source
   directory. Both come from a clean checkout of the published commit.
5. Each moved skill matches its baseline entry or a recorded change, as order 2
   of the [cutover contract](owner-skill-delivery.md#cutover-contract) requires.

Proofs 2 and 3 apply to a pack that ships kind skills. TestAnyware and
PersonalSkills ship craft skills only, and Grove's conformance does not cover
craft skills. For such a source, `check:packs` checks only that the pinned
commit holds the source directory, and it reports that directory-only check by
source name. The source's own `task check` is its check.
`testanyware-skill-move-k172` makes that change, because it adds the first
pinned source without kind skills. The hooks plugin
`technical-communication-hooks` ships no skill, and it takes the same check.

The branch applies only to a source that holds neither a kind skill nor a rules
manifest. It does not depend on whether the source holds `skills/`. TestAnyware
holds `skills/` without a kind skill, and PersonalSkills holds no `skills/`. A
source with kind skills and no manifest therefore still fails conformance.

A move owes no directory trial, native trial or refusal of its own unless it
adds a boundary. The research move proved those mechanisms. The staging trials
run every source on both harnesses later.

A move that adds a boundary owes the proofs in its row:

| Boundary | First move | Static proof | Trial |
| --- | --- | --- | --- |
| A source that is not a Linkuistics pack | `testanyware-skill-move-k172` | The source directory holds the skill and each resource that it names. `ravel skills resolve` gives the same `resourceDigest` before and after a build writes `target/` | A directory trial in each harness: a task activates `using-testanyware` without naming it, and the agent reads its `SKILL.md`. The skill has no supporting file. Its command reference is output of the installed `testanyware` command, and the staging trials cover that output. No VM runs, and the trial adds no file to the skill |
| An external member of a cross-pack family | `requirements-pack-k173` | The pack alone lists the spine's `references/review.md` and `references/integrate-review.md` as external resources. The aggregate resolves them, and the core rows of `review-*` and `integrate-review-*` bind the pack's members | A directory trial in each harness: `review-requirements` reads `references/review.md` from the core source before it acts. A native Claude Code trial reads the same file under the plugins' namespaces |
| The first `craft/` root | `design-pack-k175` | `ravel skills resolve` lists the craft skills under the roots `["skills", "craft"]`. `claude plugin validate` accepts `"skills": ["./craft/"]`. `install.test.sh` installs a fixture pack with `craft/` beside `skills/`, and a fixture pack with `craft/` and no `skills/`. For each fixture, a later run removes the link of a removed craft skill | A directory trial in each harness: a task activates a craft skill from the craft root without naming it. A native Claude Code trial lists that skill under the plugin's namespace |
| A family skill outside research | `editorial-pack-k179` | The named set `editorial` becomes a list of kinds, and its rows move to the WritingSkills manifest. The control in which a member stops loading its family skill fails | A directory trial in each harness: an editorial kind reads the spine and `editorial-family` before it acts |
| A second plugin in one repository | `writing-craft-pack-k181` | Each plugin passes `claude plugin validate`. The ordinary writing plugin declares the hooks plugin as a dependency and declares no hook. The count includes dependencies and each hooks file that Claude Code discovers by default. It gives one declaration for each hook to the ordinary plugin alone, to the hooks plugin alone and to both | The trial of the native residue: Claude Code runs the hooks from the skill-free plugin and lists no native `technical-communication` skill. A native installation of the ordinary writing plugin alone, in an isolated Claude Code home, installs the hooks plugin and runs the hooks |
| A source whose only root is `craft` | `personal-pack-k183` | `ravel skills resolve` lists the craft skills under the root `["craft"]`. `install.sh --pack` links the craft skills of a plain checkout of the published commit in an isolated home, by the craft-only fixture of `design-pack-k175`. `check:packs` reports its directory-only check for the source | None. The installer and the pack check fail without a model, and the craft-root trial of `design-pack-k175` covers discovery from a craft root |

The other moves owe these proofs:

- `engineering-pack-k177` owes the base proofs only. Its `craft/` root and its
  family members repeat boundaries that earlier moves prove.
- `personal-pack-k183` owes the proofs of the craft-only row. It also repeats
  the boundary of a second plugin. For that boundary, it owes
  `claude plugin validate` for the `guardrail` plugin and a native listing in an
  isolated home that shows `guardrail` alone. The `personal`
  source resolves without `guardrail`. The staging trials cover `/guardrail`
  and its hook.
- `skill-marketplace-k185` moves no pack, but it changes the route of every
  native user. It owes a native installation of each listed plugin from the
  relocated marketplace, in an isolated Claude Code home. The installation of
  the ordinary writing plugin also installs the hooks plugin.
- `moves-handoff-k187` owes no trial.

The design rejected these options:

- **Every move repeats the whole research proof.** The staging trials run each
  source on both harnesses, so a repeated trial measures a mechanism twice and
  adds no boundary.
- **No move runs a trial before the staging trials.** A broken boundary would
  then appear only after every later move built on it.

## Root table changes

The human approved two changes to the destination table of the root brief:

- Core holds the tree and artifact formats. DesignSkills holds design craft,
  whose skills deepen the ADR and spec formats in the spine.
- WritingSkills holds the technical-communication always-on hooks plugins.
  PersonalSkills holds personal skills and the native-only `guardrail` plugin.

`pack-membership-k171` made both changes in `.grove/_BRIEF.md`.

## Test seams

- **Conformance.** Each pack move runs its pack alone and in the aggregate, as
  the base proofs state. The requirements move shows the format files as
  external resources.
- **The trial runner.** `trial:controls` shows the refusal of a case whose
  local path the prepared commit lacks, and the clean reading beside it. It also
  admits a valid external case and a valid case with a historical native copy.
- **The installer.** `install.test.sh` installs and reconciles a craft root
  beside `skills/` and a craft root alone.
- **The boundary trials.** Each boundary row runs its trials from a plain checkout
  of the published pack commit.
- **Grove's suite.** Each move runs `bash scripts/check.sh` after it changes the
  tests of the named-read class.

## Out of scope

- **Moving any file.** Each move applies these decisions in its own leaves.
- **Changes to kind guidance.** No decision here changes a charter or a
  procedure.
- **The checks of craft skills.** Each pack keeps them, as the external pack
  contract states.
- **A citation sweep over external packs.** The design accepts the gap and states
  when to reopen it.

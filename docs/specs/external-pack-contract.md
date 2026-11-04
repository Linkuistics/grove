# External pack contract

This specification settles how a skill pack outside Grove's repository joins
Grove's methodology. ResearchSkills is the first such pack, and its move proves
the contract. Each later pack move reads this specification before
repository-moves-k5 plans its work.

`external-pack-contract-k155` wrote the decisions. `external-pack-contract-k163`
reviewed them, and `external-pack-contract-k164` applied the review's findings.
`external-pack-proof-k162` added the [proven facts](#proven-facts) after the
research move.

Grove owns this specification, because Grove owns core, the conformance runner
and the rules for references between packs. The
[Grove delivery design](codex-skill-provisioning.md) owns how a session receives
skills on each route. The [owner delivery design](owner-skill-delivery.md) owns
the owner's source set, the pins and the cutover on the owner's machine.
Ravel's skill-discovery specification owns source capture and the loading rules
of its preamble.

## Problem

Grove's methodology ships as one plugin in one repository. The spine, every
kind skill and every family file are under `plugins/grove/skills/`. Several
mechanisms assume that one location:

- The conformance runner reads one skills root. It expects the spine at
  `<root>/grove`. It resolves a reference only in the referring skill or in
  the spine.
- A research kind reaches its family file as a resource of the spine.
- `TASK-FORMAT.md` counts every kind. A test compares that count with the
  directories of one plugin.
- The marketplace lists plugin roots inside Grove. The manual installer links
  skills from Grove's checkout only.
- The trial runner builds each source path from the prepared Grove checkout.

The root brief moves the research kinds, the editorial kinds and others into
repositories of their own. Each move must keep one owner for each rule. It must
keep each required edge reachable on each delivery route, and it must keep
conformance checkable. Without one contract, each move would answer these
questions again, and the answers could differ.

## Solution

A pack is a repository that ships methodology skills in Grove's plugin layout.
Core is the pack in Grove's repository, and an external pack is any other. The
contract has six parts:

1. **Layout.** Each pack keeps its skills in `plugins/<plugin>/skills/`. That
   plugin directory is the Ravel source directory and the Claude Code plugin
   root.
2. **Family skills.** A family whose kinds all ship in one external pack keeps
   its file in a family skill's `SKILL.md`. Each member loads it by name. A
   family whose kinds ship in several packs keeps its file in the spine.
3. **References.** Pack text names another skill by its bare name, in a
   directed load. It never writes a plugin namespace.
4. **Conformance.** Each pack keeps the manifest of the rules that its own files
   own. Grove's runner checks a pack alone and checks the aggregate of core with
   the supplied packs.
5. **Distribution.** The marketplace lists an external plugin with a pinned
   `git-subdir` source. The manual installer takes the plugin directory of a
   pack.
6. **Identity.** A checkout of a pack is accepted by the `resourceDigest` of its
   source directory.

The [visual views](../design/external-pack-contract/README.md) show the layout,
the edges between skills and the conformance runs.

## Terms

- **Pack**: a repository whose plugin directories ship methodology skills or
  craft skills.
- **Core**: the pack in Grove's repository. Its plugin `grove` holds the spine,
  `configure-grove`, the core kind skills and the family files of core's
  families.
- **External pack**: a pack in another repository. ResearchSkills is the first.
- **Source directory**: the plugin directory `plugins/<plugin>/` inside a
  checkout of a pack. Ravel captures it as a `local` source, and Claude Code
  installs it as the plugin root.
- **Kind directory**: `skills/` in a source directory. It holds kind skills and
  any family skill of the pack.
- **Craft directory**: `craft/` in a source directory. It holds standalone craft
  skills, such as `decision-records`.
- **Family skill**: the skill that carries the family file of a family whose
  kinds all ship in one external pack. Its name is `<family>-family`.
- **External resource**: a supporting resource that a pack's text names and
  that another pack's skill holds, such as the spine's `references/review.md`.
- **Directed load**: the sentence form "Load the `<name>` skill". It names a
  skill that the reading session must load.
- **Aggregate**: core together with one or more external packs, each at an exact
  revision.

When grove-research-removal-k158 lands the move, the skills glossary gains
**Pack**, **Source directory** and **Directed load**. The grove glossary gains
**Family skill**, as [Rule placement](#rule-placement) states.

## Decisions

### Layout

ResearchSkills keeps the layout of Grove's own plugins:

```text
ResearchSkills/                     repository root, outside the source
├── LICENSE                         Apache-2.0
├── README.md                       source directory, roots, kinds, provenance
├── Taskfile.yml                    the pack's checks
├── .gitignore                      ignores .cache/
└── plugins/
    └── research/                   source directory and plugin root
        ├── .claude-plugin/
        │   └── plugin.json         manifest name "research"
        ├── conformance/
        │   └── rules.tsv           the rules that this pack's files own
        └── skills/                 kind directory, the only Ravel root
            ├── grove-research-a/SKILL.md
            ├── grove-research-b/SKILL.md
            ├── grove-combine-research/SKILL.md
            └── research-family/SKILL.md
```

These facts bind the layout:

- The repository is `~/Development/AgenticOrchestration/ResearchSkills`, with
  colocated Git and Jujutsu. Its public remote is
  `https://github.com/Linkuistics/ResearchSkills`.
- The source directory is `plugins/research`. It is also the Claude Code plugin
  root.
- The manifest is `plugins/research/.claude-plugin/plugin.json`. Its `name` is
  `research`, so the native namespace is `research:`. A moved kind is
  `research:grove-research-a` under a native Claude Code installation.
- The Ravel roots are `["skills"]`. ResearchSkills has no craft skills, so it
  has no craft directory.
- A later pack with craft skills adds `craft/` beside `skills/`. Its manifest
  lists `"skills": ["./craft/"]`, and its Ravel roots are `["skills", "craft"]`.
  Claude Code scans `skills/` by default and adds each listed directory.
- The repository root holds what the pack does not deliver: the licence text,
  the README, the Taskfile and tool configuration.
- Nothing writes build output or a cache inside the source directory. The
  pack's checks write their cache to `.cache/` at the repository root, which
  Git ignores.
- Each relative symlink stays inside the source directory. Ravel refuses an
  absolute or escaping link.
- No Git attribute changes the bytes of a checkout, such as a filter or an
  end-of-line conversion.

Core already has this shape. `plugins/grove/` is its source directory and its
plugin root, with the root `skills`. One shape gives one form of Ravel record,
one form of marketplace entry and one form of runner input.

The source directory is a subdirectory for three reasons:

- Ravel captures the whole source directory except `.git` and `.jj`, whatever
  an ignore file says. A repository-root source would put the README, the
  Taskfile and any cache into each capture. Its `resourceDigest` would then
  change with edits that deliver nothing.
- The installer reconciles links by the path segment `plugins/<plugin>/skills/`.
  The trial runner names a source by its `plugin` path. Both work for an
  external pack without a change.
- One repository can ship a second plugin. WritingSkills and PersonalSkills
  each need one for their native residue, as the
  [owner delivery design](owner-skill-delivery.md#native-residue) states.

The design rejected these alternatives:

- **The repository root as the source directory.** It allows the simpler
  `github` marketplace source. It also captures tooling and caches, and an
  edit of the README changes the identity of the pack.
- **`kinds/` instead of `skills/`.** It needs a manifest path, and it differs
  from core for no gain.
- **The plugin name `grove-research`.** It reads as the name of a kind skill.
  The name `research` equals the Ravel source ID of the owner design, so one
  token names the pack on both routes.

The plugins reference states no rule for two installed plugins with one
manifest name. A user who also installs another plugin named `research` can
meet that conflict.

### The family procedure

`references/research.md` moves from the spine to
`plugins/research/skills/research-family/SKILL.md`. Its body stays byte for
byte. The file gains this frontmatter and nothing else:

```yaml
---
name: research-family
description: The procedure that the research-a and research-b session kinds share — what a survey owes and what the pair owes each other. Use when the grove-research-a or grove-research-b skill directs you here.
harnesses: [any]
---
```

These rules bind every family skill:

- A family skill is named `<family>-family`. It never has the `grove-` prefix.
  A skill named `grove-<name>` is a kind by definition, so a prefixed name
  would add a kind with no charter that a leaf could launch.
- A family skill sets no `disable-model-invocation`. On Claude Code that key
  stops the model from loading the skill, which would block the member's
  directed load.
- The family skill is the only copy of the family file. No member holds a copy
  or a link.

`research-a` and `research-b` load the family skill after the spine.
`combine-research` does not load it, because it does not load
`references/research.md` today. Its own `SKILL.md` carries its rules. The move
changes no charter of any kind.

The family file becomes a skill, and not a resource inside a skill, for this
reason. A load by name works the same way under a native namespace and under
directory delivery. It also gives the agent the directory of the loaded skill.

A resource needs a directory that the agent can find on every route. Claude
Code substitutes `${CLAUDE_PLUGIN_ROOT}` in the text of a plugin skill, so a
native plugin can name a file that its skills share. Directory delivery and the
manual installer substitute nothing. On those routes, the agent finds a skill's
directory only by loading that skill.

The design rejected these alternatives:

- **The file inside one member's skill.** The other member would have to load
  that member's `SKILL.md` to find the directory, and that file states the
  wrong identity.
- **A file outside every skill, reached by `../` or by
  `${CLAUDE_PLUGIN_ROOT}`.** Ravel permits a `../` path inside one source, but
  a symlink install resolves it only by its physical path. Only a native plugin
  substitutes the variable. Each form would need a second rule for the other
  routes.
- **A symlink in each member.** A checkout with `core.symlinks=false` writes a
  link as a regular file that holds the target.
- **A family skill whose `SKILL.md` points at `references/research.md`.** It
  adds a read and gains nothing.
- **The file in the spine.** The root brief moves the research family out of
  core.

### References between skills

These rules bind the text of every pack:

1. A pack names another skill by its bare name, in a directed load: "Load the
   `<name>` skill". The runner reads that form as an edge.
2. Pack text never writes a plugin-qualified alias, for core or for its own
   pack. After the directed loads, the text states the namespace rule in the
   words of the mandate.
3. A skill-relative path names a supporting resource. It resolves in the
   referring file's own skill first. Where that skill has no such file, the
   path resolves in a skill that the same file loads. Try the loaded skills in
   the order of the loads. The skill in which the path resolves owns the
   resource. A supporting file of a skill also uses the loads of that skill's
   `SKILL.md`, after its own. A session reads such a file only after that
   `SKILL.md`, so those loads are in force.
4. The path of a supporting resource stays inside the skill that it resolves
   in. It never climbs out with `../`.
5. A directed load that crosses a pack boundary has one more sentence. It sends
   the session to the launch prompt's rule for a missing kind skill.

The runner reads a directed load by these rules:

- The phrase is the words "load the", one code span that holds the skill name,
  and the word "skill", in that order.
- The first letter can be either case, so "Then load the `research-family`
  skill" is a directed load.
- Emphasis markers do not change the match. The phrase can wrap across a line
  break inside one paragraph.
- An HTML comment, a fenced code block or a block quote holds no directed load.
  An example or a quotation of the phrase is not an edge.

A link from a skill to a document outside its source directory is a repository
link. No delivery route makes it readable, and the runner does not check it.
Normative text never depends on such a link.

A pack move changes the namespace of a skill, which is why the mandate removed
its fixed alias. An alias in pack text would be wrong after a later move, a
rename or an installation under another name. Under directory delivery a
qualified name is still only a name request, so the alias gains nothing there.

Core's kind skills still write the alias `grove:grove` in their openings,
because they are older than this rule. Each kind that moves takes the bare-name
opening in its own move. repository-moves-k5 also changes the openings of the
kinds that stay in core. No check reads an alias, so the older openings break
no conformance run in the meantime.

Rule 3 keeps the meaning of core's text. Each core kind loads the `grove`
skill and then names a spine resource, such as `references/review.md`. Under
rule 3, that path resolves in the spine, as it does today. A kind that moves to
another pack still loads `grove` and names the spine resource. A moved
`review-design` names `references/review.md`, which is then an external
resource of its pack.

Rule 5 follows from the reworked
[dependency decision](../adr/a-skill-states-what-binds-without-its-dependencies.md).
Inside one pack, the loaded skill ships with the loading skill and cannot be
absent. Across packs, an installation can omit the other pack. The launch
prompt already defines the outcome of missing guidance for the kind skill. The
sentence extends that outcome to the skills that the kind loads.

The moved kinds change only their opening imperative. `research-a` and
`research-b` open as follows. Each keeps its own last paragraph, which names
its half:

```markdown
**Load the `grove` skill now.** Then load the `research-family` skill. These
are bare skill names. If your harness requires a plugin namespace, use the
namespace under which each skill is actually installed. Do not assume one. If
you cannot find or read a skill named here, follow the launch prompt's rule for
a missing kind skill.

`research-a` and `research-b` are one family with one procedure, and the
`research-family` skill is it: the goal, what the survey owes, what the pair
owes each other, and which output path each half writes. This skill states none
of it a second time. Everything else — the constraints, the bootstrap, and the
execute, decompose, retire and commit procedures — is the spine's.
```

`combine-research` opens as follows. The rest of its body stays:

```markdown
**Load the `grove` skill now.** It is the shared spine and holds everything
this kind does not own: the constraints, the bootstrap, and the execute,
decompose, retire and commit procedures. `grove` is the bare skill name. If your
harness requires a plugin namespace, use the namespace under which `grove` is
actually installed. Do not assume one. If you cannot find or read the `grove`
skill, follow the launch prompt's rule for a missing kind skill. What follows is
`combine-research`'s, and is stated nowhere else.
```

### Rule placement

This session reworks three decision records:

- [Every normative rule has one owner](../adr/corpus-rules-have-one-owner.md).
  Rule 2 now files a family's rule in the family file of the pack that ships
  every kind of the family. In core that file is in the spine's `references/`.
  In an external pack it is the pack's family skill. When the family's kinds
  ship in several packs, the file is in the spine's `references/`. An edge to a
  family skill is a directed load by name.
- [A skill states what binds when its dependencies are absent](../adr/a-skill-states-what-binds-without-its-dependencies.md).
  A directed load is a required dependency, not a citation. Across packs, the
  pack declares that dependency to each route, and the loading text states the
  outcome of its absence.
- [A behavioural rule is covered by asserting its delivery](../adr/behavioural-coverage-asserts-delivery.md).
  The composed loaded path follows directed loads across the supplied packs.

The new rule is true before the move and after it. Before the move, the
research kinds ship in core, so their family file is in the spine.

The review and integration families will ship in several packs. Core keeps
`review-planning` and `integrate-review-planning`. Later moves put the other
members in RequirementsSkills, DesignSkills and EngineeringSkills. Their family
files, `references/review.md` and `references/integrate-review.md`, therefore
stay in the spine. The spine is the one skill that every member loads already,
so the branch adds no dependency.

Such a family has these consequences:

- Its rows stay in core's manifest. Their load sets `static(review-*)` and
  `static(integrate-review-*)` bind the matching kinds of the run. Core alone
  checks core's members, and the aggregate checks each supplied member.
- A change of its procedure is a change of core, and a pack cannot check those
  rows alone. The placement record accepts that cost for such a family.
- A member in another pack reaches the file as an
  [external resource](#what-a-pack-checks-alone).

A move changes the moved files, their rows in the manifests and every reference
to them in one change. It copies no family procedure and changes no charter.

grove-research-removal-k158 makes these changes, because they describe the
moved state:

- **The spine's family-file list.** The spine's `SKILL.md` lists the family
  files that the spine carries: `review.md`, `integrate-review.md` and
  `editorial.md`. It drops `research.md`. It names no family skill, because a
  rule of one family has the class `none` in the condition register. The list
  states no count of its files, so a later move changes only the list.
- **The fatness rule of `plugins/grove/README.md`.** A rule that one family
  owns is one file. When one pack ships every kind of the family, the file is
  in that pack. In core it is a file in the spine's `references/`. In an
  external pack it is the pack's family skill. Otherwise the file is in the
  spine's `references/`. Each member's skill directs a load of the skill that
  holds it. The lists of fat and thin kinds name only core's kinds, and the
  inventory names the family files that the spine carries.
- **The glossary entry "Family reference file".** The entry states that the
  file lives in the pack that ships every kind of the family. A family whose
  kinds ship in several packs keeps its file in the spine. In core it names the
  spine's `review.md`, `integrate-review.md` and `editorial.md`. In
  ResearchSkills it names the family skill `research-family`. The spine's
  `SKILL.md` lists only the spine's own family files.
- **The glossary entry "Spine skill / kind skill".** It gains the family skill
  as a third species. A family skill is not a kind, because its name has no
  `grove-` prefix.
- **The `grove` plugin descriptions.** The manifest and the marketplace entry
  of the `grove` plugin drop "research" from their list of family files.

The rules that name research labels in core stay in core.
`references/execute.md` states the review allowance of each kind, and
`references/decompose.md` states when a design or planning session cuts a
research pair. Neither rule binds only the research family, so the placement
function keeps both in the spine.

### The kind table

`TASK-FORMAT.md` stops counting the whole set of kinds. Kinds ship in several
packs, and a kind exists if and only if its skill is installed. A count in core
would claim facts about other repositories that core's tests cannot read.

grove-research-removal-k158 makes these changes:

- The heading "The twenty-three kinds" becomes "The kinds".
- The table keeps every row and gains a last column, "pack". Core's rows name
  `grove`. The research row names `ResearchSkills`.
- The sentence that adds the rows up to "twenty-three" goes. A sentence states
  that the methodology's kinds ship in several packs, so the page keeps no
  count.
- A kind skill reaches its family's file where rule 2 of the placement record
  puts it. That is the family's pack, or the spine for a family across packs.

`the_guidance_counts_the_kind_set_correctly` becomes a check in two parts. Each
core `grove-<kind>` skill has a table row whose pack is `grove`. Each label in
such a row has a core skill. The check also keeps its sweep for a phrase that
counts the whole set, now for every cardinal.

No mechanical check covers the external rows. A pack that adds or removes a
kind must change core's table by hand. The research row names
`ResearchSkills`, and the pack's README lists its kinds. The design accepts
that gap.

A bounded check would keep kinds open. For each supplied pack that the table
names, the aggregate could compare the pack's kinds with its rows. It could
accept each kind of a pack that the table does not name. That check needs a map
from the table's pack names to the supplied source directories, and the runner
takes no such input. The design does not add that input for one row. Reopen
this decision when a session finds that a pack's kinds and its table rows
disagree.

### Conformance

#### Inputs

The runner `plugins/grove/conformance.sh` keeps its current arguments and adds
two:

| Argument | Meaning |
| --- | --- |
| `--pack <dir>` | Adds an external pack. `<dir>` is its source directory, which holds `skills/` and `conformance/rules.tsv`. Repeat it for each pack |
| `--alone` | Checks exactly one `--pack` without core. The runner then reads neither core's skills nor core's manifest |

Without `--pack`, the runner checks core alone, as it does today. Core alone
is the aggregate with no external pack, so a directed load in core that no core
skill answers fails. With `--pack` and without `--alone`, it checks the
aggregate of core and each named pack. `--skills` and `--rules` keep their
meaning for core, and the runner refuses them with `--alone`.

An optional `conformance/removed-paraphrases.tsv` beside a pack's manifest
lists the retired paraphrases of that pack's rules.

#### What a pack checks alone

A run with `--alone` checks only what the pack's own files decide:

- Each row of the pack's manifest names an owner inside the pack, and the owner
  file exists.
- Each pinned phrase of the pack's rows has its owner as its only site among
  the pack's files.
- Each behavioural row of the pack is on the loaded path of each pack kind that
  it binds. The path follows directed loads inside the pack only.
- Each skill-relative path in the pack resolves among the pack's skills, or the
  runner lists it as an external resource, as below.
- Each directed load names a skill of the pack, or the runner lists it as an
  external edge. An external edge is not a failure in this mode.

The runner resolves a skill-relative path in the order of rule 3. It tries the
referring skill, and then each loaded skill in the order of the loads. When it
reaches a loaded skill that the pack does not ship, it stops. It then lists the
path as an external resource, with the name of that skill. An external resource
is not a failure in this mode. The aggregate resolves it.

A path that resolves in no tried skill is a missing file, and it fails. That is
the result for a pack-local resource that is absent, in a file that loads only
skills of its own pack.

A pack alone therefore passes without core's content. It needs core only for
the runner.

#### What the aggregate checks

The aggregate is the union of the skills of core and of each supplied pack:

- No two packs ship skills of the same name.
- Each directed load names a skill of some supplied pack.
- Each skill-relative path resolves by the rules above, across packs. Each
  external resource of an alone run therefore resolves, or it fails here as a
  missing file.
- A loaded path holds skill-qualified files, such as `grove/SKILL.md` and
  `research-family/SKILL.md`. Its first file is the kind's own `SKILL.md`.
  It follows directed loads and skill-relative paths across packs. The spine is on
  a kind's path only because the kind loads it.
- Each row of each manifest names an owner in its own pack, and the owner file
  exists. No two manifests hold a row with one rule ID.
- Each pinned phrase has its owner as its only site among the files of all
  supplied packs. The spine's `SKILL.md` states no row's phrase.
- Each behavioural row is on the path of each kind that it binds, in any pack.
  `static(all)` binds every kind of the aggregate.
- The retired paraphrases of every pack are absent from every pack.

The runner reads directories and runs no VCS command. The task that supplies a
pack checks its revision, as [The checker without a copy](#the-checker-without-a-copy)
states.

#### Manifest changes

pack-conformance-runner-k156 and grove-research-removal-k158 make these
changes:

- A row whose owner file is absent fails. The class "pending" was an aid for
  the migration, and its count is zero. Without this rule, a row left in core
  after its owner moved would pass in silence.
- `static(23)` becomes `static(all)`. The runner drops the unused `static(22)`.
- `static({a,b})` accepts a list of kinds, separated by commas.
- The runner keeps no named set for the family of an external pack. The named
  set `research` goes with its rows. `editorial` becomes an explicit list when
  its family moves.
- The prefix sets `review-*` and `integrate-review-*` stay. Each binds the
  kinds of the run whose labels match it, in any supplied pack.
- The eight rows that the moving files own move to
  `plugins/research/conformance/rules.tsv`. The six rows of `research.md` take
  the owner `research-family/SKILL.md` and the load `static({research-a,research-b})`.
  The two rows of `combine-research` keep their owner and their load.

#### The checker without a copy

No pack copies Grove's runner. A pack's `Taskfile.yml` records two variables:

| Variable | Value |
| --- | --- |
| `GROVE_REPOSITORY` | The Git URL of Grove. The default is `https://github.com/Linkuistics/grove.git` |
| `GROVE_COMMIT` | The full commit ID of the Grove revision that the pack checks against |

The reusable tasks of a pack are these:

| Task | Effect |
| --- | --- |
| `grove:checkout` | Makes a plain Git checkout of `GROVE_COMMIT` under `.cache/grove/<commit>`, with Git's global and system configuration off. It reuses that checkout only when its `HEAD` is the commit and `git status --porcelain --ignored` prints nothing |
| `check:conformance` | Runs the checkout's `plugins/grove/conformance.sh --pack plugins/research --alone`, then the same runner with `--pack plugins/research` for the aggregate |
| `check:plugin` | Runs `claude plugin validate plugins/research`. It skips with a message when Claude Code is not installed |
| `check` | Runs every check of the pack |

One record gives both the checker and core's content at one revision. A
change of the runner reaches the pack only when the pack changes its record.

Grove checks the other direction. While Grove hosts the marketplace, its
marketplace entry for a pack is Grove's record of that pack's revision. A Grove
task `check:packs` reads `url`, `path` and `sha` from each `git-subdir` entry.
It makes a plain checkout of `sha` under `target/external-packs/<plugin>/<sha>`,
with the same isolation and reuse rule. It then runs the pack alone and the
aggregate with Grove's working tree.

`scripts/check.sh` runs `task check:packs` as a principal check. A `git-subdir`
entry without `sha` fails that task, because Grove checks only exact revisions.
The first run needs the network, and later runs reuse the cache.

When repository-moves-k5 moves the marketplace out of Grove, Grove keeps a
record of its own with the same fields. The marketplace repository then checks
its own pins.

A change that breaks a pack follows the expand and contract steps. The pack
publishes a commit that works with the old core and with the new core. Each
record then moves to the new revision.

A move has the same order. The new pack's first commit records a Grove commit
that still ships the moving files. That pack passes alone, and its aggregate
reports each moving skill and each moved row as a duplicate. Grove then removes
its copies and its rows, and it pins that first pack commit. A later pack commit records the removal commit, and its
aggregate passes. The two pack commits have the same `plugins/research`, so
both deliver the same bytes.

The committed `GROVE_REPOSITORY` stays the public URL. A published pack commit
records only a `GROVE_COMMIT` that Grove's public remote holds. A clean checkout
of that pack commit then runs `task check` with no override. A run during
development can pass the local repository as a task variable. Such a run is
never the evidence for a published pack commit.

This workstream reaches `main` only in finish. Its Grove line therefore goes to
Grove's public remote as the bookmark `gh-issue-25`. Order 2 of the
[owner cutover contract](owner-skill-delivery.md#cutover-contract) publishes
the same line for the `grove-core` candidate, and the research move publishes
it earlier. The session that records a Grove commit in a pack pushes the line
through jj first. Then it publishes the pack commit. Each push moves the
bookmark forward to a descendant.

Finish rebases the line onto `main`. When `main` has moved, the rebase gives the
line new commit IDs. The remote bookmark then holds the recorded commits alone.
Finish pushes only `main` and the release tag, so the remote bookmark stays
where the last push put it. Nobody moves or removes the remote `gh-issue-25`
while a published pack commit records a commit that only that bookmark holds. A
pack moves its record to a released Grove commit with its next change, by the
expand and contract steps above.

#### Controls that must fail

`conformance.test.sh` proves each control with a fixture external pack, not
with the real research pack. Each control must be seen to fail:

| Control | Mode | Expected failure |
| --- | --- | --- |
| A pack skill that loads only skills of its own pack names a resource that the pack does not ship | Alone and aggregate | A missing file |
| A pack skill loads `grove` and names a resource that core does not ship | Aggregate | A missing file. The same pack alone lists an external resource |
| A family member no longer loads its family skill | Alone and aggregate | The family's rows are off that member's path |
| A pack kind no longer loads `grove` | Aggregate | The core rows of `static(all)` are off that kind's path |
| A directed load names a skill that no supplied pack ships | Aggregate | An unresolved load. The same pack alone lists an external edge |
| The fixture's fenced example of a directed load loses its fence | Aggregate | An unresolved load |
| A pack file states a pinned phrase of core | Aggregate | Two owners |
| A core file states a pinned phrase of the pack | Aggregate | Two owners |
| Two packs ship skills of one name | Aggregate | A duplicate skill |
| Two manifests hold one rule ID | Aggregate | A duplicate row |
| A manifest row names an absent owner, or an owner in another pack | Alone and aggregate | An unshipped owner |

Two clean readings rule out a runner that always fails. The fixture aggregate
passes, and core alone gives its earlier result.

The fixture pack also proves the edge form and the external resource. These
facts hold in its clean readings:

- One member opens with the research kinds' exact text, so "Then load the
  `research-family` skill" gives an edge.
- One member loads `grove` and names `references/review.md`, a resource of
  core. Alone, the run lists that path as an external resource and passes. The
  aggregate resolves it in the spine.
- A fenced example names a skill that no supplied pack ships, and it gives no
  edge.

These rules bind every fixture of `conformance.test.sh`:

- A fixture ships the owner of each row in each manifest that its run reads. It
  copies all of core, or it reads a manifest of only the rows whose owners it
  ships. The current fixtures copy the spine and some kinds, and they read
  core's whole manifest. pack-conformance-runner-k156 changes them before an
  absent owner fails.
- Each control first reads its fixture clean. Then it makes its one mutation.
- A clean reading asserts the exit status 0. A dirty reading asserts a nonzero
  exit status and the intended diagnostic. A runner that prints the diagnostic
  and exits 0 therefore fails the control.
- The current behavioural cases stay, on the changed fixtures.

### Provenance and licences

- **Licence.** ResearchSkills uses Apache-2.0, the licence of Grove. The moved
  text is Grove's, and the author keeps one licence for it. The repository root
  holds the full `LICENSE`. The plugin manifest sets `"license": "Apache-2.0"`,
  so the delivered source directory names its licence.
- **`LICENSES/`.** ResearchSkills has none. The moved files hold no passage
  adapted from a third party. A later adapted passage adds `LICENSES/` inside
  its own skill, as `grove-impl` does in core.
- **History.** The moved files keep no Git history in the pack. Grove keeps
  their whole history, across their earlier paths under `content/`. A filtered
  copy would rewrite every commit ID and carry commits that describe unrelated
  Grove work. The pack's first commit and its README name the Grove commit and
  the four source paths.

### Distribution

- **The Claude Code marketplace.** Until repository-moves-k5, Grove's
  `.claude-plugin/marketplace.json` lists the plugin with this entry:

  ```json
  {
    "name": "research",
    "source": {
      "source": "git-subdir",
      "url": "https://github.com/Linkuistics/ResearchSkills.git",
      "path": "plugins/research",
      "sha": "<published ResearchSkills commit>"
    },
    "description": "Grove's research kinds: the research-a and research-b survey pair, their shared procedure, and the combine step."
  }
  ```

  A `git-subdir` source names one subdirectory of another repository. A
  `github` source takes no path, so it cannot name `plugins/research`. The
  entry name equals the manifest name, as the marketplace documentation
  directs.
  The `sha` gives native users the commit that Grove's aggregate check
  accepted.
- **The dependency on core.** The research manifest declares
  `"dependencies": ["grove"]`. A bare name resolves in the marketplace that
  lists the plugin, and an installation also installs a missing dependency. A
  native user therefore receives the spine with the research kinds. A user who
  takes core from Ravel and research natively also receives a native spine.
- **The research manifest.** It sets `name`, `description`, `author`,
  `repository`, `license`, `dependencies` and `keywords`. It sets no `version`.
  Claude Code then derives the version from the commit, as the commit-SHA
  version rule of the skills glossary states.
- **The manual installer.** `plugins/install.sh` gains `--pack <plugin-dir>`,
  which can repeat. It scans a named plugin directory as it scans a bundled
  plugin, with the same eligibility and the same workspace guard. Link
  reconciliation covers each plugin that the run scans. A run without
  `--pack plugins/research` leaves the research links unchanged. The pack ships
  no installer. Its README gives the command
  `<grove checkout>/plugins/install.sh --pack <research checkout>/plugins/research`.
- **The Codex marketplace.** It stays unchanged. It lists only the
  technical-communication hooks plugin, and the research pack has no hooks.
- **Ravel.** A directory route names the pack with a record beside core's:

  ```json
  { "id": "research", "type": "local", "path": "<checkout>/plugins/research", "roots": ["skills"] }
  ```

  The pack adds no mandatory file. Grove's mandate names the kind skill, and
  the kind skill loads the rest.

The marketplace that the owner's Claude Code follows tracks Grove's `main`.
This workstream reaches `main` only in finish, so the new entry reaches the
owner then. A native user also needs `/plugin install research@linkuistics`.
discovery-rollout-k6 records that step.

### Checkout boundary

In any checkout of ResearchSkills, the source directory is `plugins/research`
and the roots are `["skills"]`. The path does not depend on the tool that made
the checkout.

A checkout is accepted by its `resourceDigest`, as the
[owner delivery design](owner-skill-delivery.md#pack-checkouts) states for each
source. `ravel skills resolve --json` gives it for a clean checkout of the
accepted commit. The digest covers `plugins/research` only. An edit of the
README, the Taskfile or the licence text therefore leaves it unchanged, and an
edit inside the source directory changes it.

Each accepted checkout also records the Git tree ID of the source directory,
from `git rev-parse <commit>:plugins/research`. The tree ID identifies the
committed files. The digest identifies the captured files. An equal tree ID
never admits a different digest.

### Proof shape

A source record of a trial case keeps `id`, `plugin` and `roots`. It gains an
optional `external` object:

```json
{
  "id": "research",
  "plugin": "plugins/research",
  "roots": ["skills"],
  "external": {
    "repository": "https://github.com/Linkuistics/ResearchSkills.git",
    "commit": "<40 hexadecimal digits>"
  }
}
```

With `external`, `plugin` names the source directory inside that repository's
checkout. The runner makes a plain Git checkout of `commit` under the trial's
`sources/`, with the isolated Git configuration of the research recipes. It
refuses before any launch when the commit is unknown, when `HEAD` differs or
when the checkout is not clean. `moveAway` works on an external source.

The record of each run names these facts for an external source:

- The repository and the commit.
- The checkout path.
- The Git tree ID of the source directory.
- The `resourceDigest` of the source.

A run option `--working-copy <id>=<dir>` replaces the checkout of one source
with a working copy. The record then names the working copy's path, its Jujutsu
change and commit, and whether the working-copy change is empty. One case
therefore serves the checkout proof and the working-copy proof.

## Test seams

- **The runner.** `conformance.test.sh` proves each
  [control](#controls-that-must-fail) with a fixture pack, and core alone keeps
  its result.
- **The pack.** `task check` passes from a clean checkout of the published
  ResearchSkills commit, with its committed public `GROVE_REPOSITORY`.
- **Grove.** `bash scripts/check.sh` passes, with `task check:packs` over the
  pinned ResearchSkills commit.
- **The kind table.** The renamed test passes, and a deliberate count of the
  whole set fails it.
- **The installer.** `install.test.sh` installs a fixture pack with `--pack`,
  and reconciles its links.
- **Directory delivery.** research-directory-trials-k160 runs the research
  cases in both harnesses, from a plain checkout and from the working copy.
- **Native namespace.** research-native-namespace-k161 loads
  `research:grove-research-a` in Claude Code from Grove's bare-name mandate.

## Proven facts

external-pack-proof-k162 recorded these facts after the research move. Later
pack moves read them before the work of repository-moves-k5. The
[Research pack audit](../common-loading-acceptance-record.md#research-pack-audit)
holds the verdict of each criterion of external-research-pack-k4 and the
check runs of this section.

### Subjects

| Fact | Value |
| --- | --- |
| The published ResearchSkills commit | `1c18a90945a0b2ac4086bfe6f2ba90066fcf4bcc`, the public `main` of `https://github.com/Linkuistics/ResearchSkills.git`. It records the removal commit. Its parent `dc3a4812db3b0dbfef42c8cd6a0aa291e43dd545` is the first pack commit, and Grove's marketplace pins it. Both commits are on the public remote |
| The tree ID and `resourceDigest` of `plugins/research` at that commit | Tree `79dc2a530cd2d30cd088073423437bb18d61b401`, the same at both pack commits. `resourceDigest` `7fb3d473f66feafb49afd8846fd94cf3e1fb7b01c584d4ad5f0e54814a8b3fdb`, from a clean checkout and from the working copy |
| The Grove commit with no research files | `57414e00cd876a9c0e1f8021786384ba7e368580`, the removal commit of grove-research-removal-k158. Its `plugins/grove` has the `resourceDigest` `ea34f8afb116560d853f7fdf6caf2c8becaff40a3b196c2e5d7af3843a374f91` |
| The Grove commit in ResearchSkills' `GROVE_COMMIT`, and the remote bookmark that holds it | `57414e00cd876a9c0e1f8021786384ba7e368580`. Grove's public remote holds it under the bookmark `gh-issue-25`, which is at its descendant `2328c7f0d45a` |
| The Ravel 2.0 build of the trials and of the check | The line `skill-directory-delivery` at the output commit `71f70fcd3f92`, build commit `d324668b86ca`. The build identity is `a4f55d04b2d19a9948f2bcdcb3f305ee20ae80fae7f9011ec878f117d98e6519`. The `bin/ravel` SHA-256 is `26b487afb957c233848e8e360e29c73539a33ba89fc3444b26ad13a2eb4bf0a3`, the value of `scripts/harness-trials/builds.json`. The build reports the version string 1.1.0, so its digest identifies it |
| The native spelling observed in Claude Code | `research:grove-research-a` for the kind, `research:research-family` for the family skill and `grove:grove` for the spine. Claude Code 2.1.318 listed each research skill as `research:<name>` and no bare `grove-research-a` |
| The trial records of both harnesses | [Research directory trials](../common-loading-acceptance-record.md#research-directory-trials), in Claude Code and Codex, from a plain checkout and from the working copy. [Research native namespace](../common-loading-acceptance-record.md#research-native-namespace), in Claude Code |
| The reusable tooling | [Reusable pack tooling](#reusable-pack-tooling) |
| Each difference between this contract and the proven state | [Differences from the decisions](#differences-from-the-decisions) |

### The proven boundaries

- **Layout.** The pack's tracked files are `.gitignore`, `LICENSE`,
  `README.md`, `Taskfile.yml` and six files under `plugins/research`: the
  manifest, `conformance/rules.tsv` and the four `SKILL.md` files of
  [Layout](#layout). The checks wrote only `.cache/`, which Git ignores, and
  each checkout was clean after its run.
- **Checkout boundary.** In any checkout of ResearchSkills, the source
  directory is `plugins/research` and the Ravel roots are `["skills"]`. The
  plain Git checkouts of the trial runner and of external-pack-proof-k162,
  and the jj working copy, gave the same `resourceDigest`. An edit outside `plugins/research` left the tree ID
  unchanged: the second pack commit changed only the Taskfile and the README.
- **Resource references.** The kinds name `grove` and `research-family` by
  bare names. The pack alone lists 3 external edges, the loads of `grove`,
  and 0 external resources. In both harnesses the agent read the spine, the
  kind skill and `research-family` before it acted. A delegate read
  `grove-research-a` from the retained research directory in each harness.
- **Conformance boundary.** The pack alone checks 14 loaded paths, 0
  single-source rows and 8 rows with no pinned wording. Core alone at the
  removal commit checks 116 single-source rows, 2424 loaded paths and 64
  unpinned rows. The aggregate checks 116, 2783 and 72, the counts of core
  before the move.
- **Refusals.** A moved checkout and a missing root each refused as
  `skill_source_missing` before any harness launched, and the restored
  resolution resolved. A launch without a network resolved the same
  directories.

### Reusable pack tooling

| Tool | Where | Use |
| --- | --- | --- |
| The conformance runner | `plugins/grove/conformance.sh --pack <source directory> [--alone]` in a Grove checkout | `--alone` checks one pack. Without it, the run checks core with each named pack. `--pack` repeats |
| The pack's tasks | `grove:checkout`, `check:conformance`, `check:plugin` and `check` in ResearchSkills' `Taskfile.yml`, with the variables `GROVE_REPOSITORY` and `GROVE_COMMIT` | A later pack copies these tasks and changes the source directory |
| Grove's pack tasks | `check:packs`, which runs `scripts/check-packs.sh`, and its controls `scripts/check-packs.test.sh`. `check:skills` and `scripts/check.sh` run both | The script reads each `git-subdir` entry of the marketplace and checks it alone and in the aggregate. `--marketplace` and `--cache` let a test supply fixtures |
| The plugin manifest | `plugins/research/.claude-plugin/plugin.json`: `name`, `description`, `author`, `repository`, `license` `Apache-2.0`, `dependencies` `["grove"]` and `keywords`, with no `version` | `claude plugin validate` passes it with two warnings, no `version` and no icon |
| The marketplace entry | The `git-subdir` entry of [Distribution](#distribution) in `.claude-plugin/marketplace.json`, with `sha` `dc3a4812db3b` | `/plugin install research@linkuistics` also installed `grove` as a dependency |
| The manual installer | `plugins/install.sh --pack <checkout>/plugins/research` | The source directory must have the shape `plugins/<plugin>/skills/` |
| The trial-runner fields | In `scripts/harness_trial.py`: a source's `external` object with `repository` and `commit`, the run option `--working-copy <id>=<dir>` (`WORKING_COPY` of `trial:claude` and `trial:codex`), `moveAway`, `moveRoot` and the Claude Code field `nativePlugins` | [The runner](../common-loading-acceptance-record.md#the-runner) owns each field. The cases `research-discovery`, `research-delegate`, `research-source-missing`, `research-root-missing` and `research-native-namespace` are the models for a later pack |

### Differences from the decisions

Each difference below adds to a decision or limits it. None of them changes a
decision of this contract.

- **Two more runner fields.** [Proof shape](#proof-shape) names `external` and
  `--working-copy`. The proof also needed `moveRoot`, which moves one root of
  a source for the missing-root refusal. It also needed `nativePlugins`, which
  installs plugins from the marketplace for the native proof. The runner
  also refuses a commit with no directory at `plugin`, and a continuation case
  refuses an external source.
- **The installer refuses a shared skill name.** `install.sh --pack` refuses a
  skill name that two scanned plugins ship, before any link changes. The
  workspace guard covers each pack's checkout, two levels above the plugin
  directory.
- **The kind table feeds the label tests.** No check compares a pack's kinds
  with its rows in the table, as [The kind table](#the-kind-table) accepts.
  Grove's tests take the labels of the external rows for the environment
  scrub and the label round trips. A malformed label in the research row
  therefore fails a test.
- **A local marketplace loads core in place.** In the native proof, the
  marketplace came from a Grove checkout. Claude Code loaded `grove` from that
  checkout and `research` from its cache, through the `git-subdir` source. A
  marketplace from GitHub puts both in the cache. The owner's marketplace
  follows `main`, which does not list `research` before finish.
- **The marketplace pins the first pack commit.** Grove pins `dc3a4812db3b`,
  as [The checker without a copy](#the-checker-without-a-copy) directs, and
  the public `main` is `1c18a90945a0`. Both commits have the tree
  `79dc2a530cd2`, so a native user and a directory user receive the same
  bytes. The installed version string is `dc3a4812db3b-07869d54`.
- **The delegate read had a name.** The Claude Code parent named both research
  skills in its delegate prompt. The run proves reachability from a delegate,
  not discovery by a delegate. Codex encrypts that prompt.

### Evidence before repository-moves-k5

repository-moves-k5 requires this evidence. Each item names its commit or
build:

| Evidence | Commit or build | Record |
| --- | --- | --- |
| A published ResearchSkills commit whose checks pass from a clean checkout, with the public `GROVE_REPOSITORY` and no override | `1c18a90945a0`, with `GROVE_COMMIT` `57414e00cd87` under the remote bookmark `gh-issue-25` | The [audit](../common-loading-acceptance-record.md#research-pack-audit) |
| A Grove commit with no research files, on which core and aggregate conformance and `bash scripts/check.sh` pass with the Ravel 2.0 build | `57414e00cd87`, and the line head `f26e93064c73` | grove-research-removal-k158 and the [audit](../common-loading-acceptance-record.md#research-pack-audit) |
| Directory delivery in Claude Code and Codex from a plain checkout and from the working copy, with a delegate read and both refusals | Runner `8a93002fe9f1`, Grove build `57414e00cd87`, Ravel `a4f55d04b2d1…` | [Research directory trials](../common-loading-acceptance-record.md#research-directory-trials) |
| A native Claude Code record under the plugin's namespace | Marketplace `57414e00cd87`, `research` `dc3a4812db3b`, Ravel `a4f55d04b2d1…` | [Research native namespace](../common-loading-acceptance-record.md#research-native-namespace) |
| This contract with its proven facts and its reusable tooling | The commit that adds this section | [Reusable pack tooling](#reusable-pack-tooling) |

## Out of scope

- **The other pack moves.** repository-moves-k5 plans them from this contract
  and its proven facts.
- **The relocation of the marketplace.** repository-moves-k5 moves it to
  `~/Development/Delivery/skill-marketplace`.
- **Checks of craft skills.** The methodology's conformance covers kind
  directories. A pack's craft skills keep the checks of their own repository.
- **The editorial family.** It stays in the spine until WritingSkills moves.
- **A kind registry.** Neither executable reads a pack to admit a kind. The
  conformance checks read content only.
- **Simpler review and integration.** That work is
  [issue #27](https://github.com/Linkuistics/grove/issues/27).

# `grove` — the methodology, as skills

<!-- tc: pending because the conversion has not reached this part of the document -->

Homebrew installs Grove's methodology with Grove, and Ravel delivers it to each
session ([`../README.md`](../README.md)). The `grove` binary installs no skills. It is a shared
**spine** skill and one `grove-<kind>` skill per session kind beside it, over
that spine. **A kind exists iff a skill of that name exists**, so core's set is
whatever `skills/` holds, and external packs such as RequirementsSkills,
DesignSkills and ResearchSkills ship more: no code carries a list of kinds and no assertion counts them. Prose here describes the set as it stands, which is a different thing from
machinery holding it.

The separate [`configure-grove`](skills/configure-grove/SKILL.md) operator skill
maintains the owner's Ravel policy, its settings and a checkout's
choice file. Its name
does not use the `grove-<kind>` prefix because it is not a lifecycle task kind.

| | |
|---|---|
| skills | `grove` — the shared spine — and one `grove-<kind>` per kind |
| installation | Grove's formula, as the source `share/ravel/skills/grove-core/` of the keg |
| conformance | [`conformance.sh`](conformance.sh) and its engine [`conformance.awk`](conformance.awk), with the controls in [`conformance.test.sh`](conformance.test.sh) |
| design | [`../../docs/specs/module-decomposition.md`](../../docs/specs/module-decomposition.md), decision 11 and test seam 4 |

## What the spine holds, and what it does not

<!-- tc: on -->

**The fatness rule.** A rule that several families share is in the spine. A
rule that one kind owns is inline in that kind's own `grove-<kind>` skill. A
rule that one *family* owns is **one file**, and each member's skill directs a
load of the skill that holds it, by name. The family's packs decide where that
file is:

- When one external pack ships every kind of the family, the file is that
  pack's family skill. The research pair's file is the `research-family` skill
  of ResearchSkills. The four editorial stages' file is the `editorial-family`
  skill of WritingSkills.
- Otherwise the file is in the spine's `references/`. That covers a family that
  core ships alone. It also covers a family whose kinds ship in several packs,
  such as the five reviews and the five integrations. Their requirements
  members ship in RequirementsSkills, and their design members ship in
  DesignSkills. Their impl and prototype members ship in EngineeringSkills.

<!-- tc: pending because the conversion has not reached this part of the document -->

**Nowhere twice**, which is what
[`corpus-rules-have-one-owner`](../../docs/adr/corpus-rules-have-one-owner.md)
and
[`restatement-declares-its-class`](../../docs/adr/restatement-declares-its-class.md)
make checkable.

So the spine carries `SKILL.md`, the five format documents, the seven shared
loop-step references, and the family files of `review` and `integrate-review`.
It carries no kind reference and no list of kinds: **a kind exists iff a skill
of that name exists.**

**What "inline" means, concretely.** A single-kind rule is in the body of that
kind's own `SKILL.md` — not in a `references/` file the skill points at. The
distinction is the whole of the difference between the rule's two halves: what is
inline arrives *with* the skill in one hop, and what is in the spine is reached
by a second one. In core, two kinds are fat this way (`planning`, `finish`)
and two are thin: core's `review-planning` and `integrate-review-planning` each
state their identity and
direct one load, because everything else they obey belongs to their family and
the family's text is the spine's. The READMEs of RequirementsSkills,
DesignSkills, EngineeringSkills, ResearchSkills and WritingSkills describe their
kinds. The four editorial stages (`draft`, `copy-edit`, `art`, `proof`) of
WritingSkills are the one hybrid: each
directs a load of its family's file *and* carries its own charter inline, because
what the four share is the pipeline's mechanics and what divides them is the
whole point of having four.

No kind in core carries a file beside its `SKILL.md`. The one such file was
`grove-impl/LICENSES/addyosmani-agent-skills.LICENSE`, which only that skill's
provenance comment cites, and it moved with `grove-impl` to EngineeringSkills.
The interview procedure `grilling.md` moved with
`grove-requirements` to RequirementsSkills. The visual design procedure and its
diagram viewer moved with `grove-design` to DesignSkills.

## The second hop is not measured, and that is the open gap

[`../../docs/research/wording-micro-test.md`](../../docs/research/wording-micro-test.md)
measured **one** hop: a prompt naming one target, and whether the session loaded
it. Nothing measures the second hop — from a `grove-<kind>` skill to the spine it
directs a load of.

- What is **inline** in a kind's `SKILL.md` is unaffected: there is no second hop
  to lose.
- What is **in the spine** loses the guarantee. The runner below asserts the
  spine's rules are *on* every bound kind's composed loaded path; it cannot
  assert a session walks that path.

**The reopen condition is a session observed acting without a spine rule** — an
observation, not a suspicion. Recorded here rather than only in a commit message
because this is where a future session meets it. If you are that session and you
have just found the case, this paragraph is what you are reopening.

## One copy, and where it lives

`plugins/grove/skills/` is the authoritative methodology source. There is no
separately authored copy, and no binary embeds one. Grove releases up to 22.8
embedded a build-time snapshot for Codex provisioning; later releases removed it
and leave its installed output unchanged.

There was one until `delete-provisioning-k19`: the `grove` binary compiled its
own `content/` tree into itself and swept it into `~/.claude/skills/grove` and
two siblings on every invocation, so the shared files existed twice and a
`conformance.sh` assertion held them byte-identical. That leaf deleted
`content/`, the embed, the sweep and the harness registry together, and the
temporary agreement check with them.

Two files of that corpus have no counterpart here rather than a moved one.
`content/SKILL.md` was a *kind router* — it sent a session to one of ten
reference files — where the spine's `SKILL.md` is a *condition register*: it
states that the driver already named your `grove-<kind>` skill, and names only
files the spine carries. And `content/SIGNAL.md` and `content/SIGNAL-FINISH.md`
became driver prose at `prompt-names-the-kind-k18`: `${prompt}` states grove's
exit contract once for every kind, and `finish`'s endings are inline in
`grove-finish/SKILL.md`: record a completed teardown with
`grove-llm record-teardown`, then make the launch prompt's exact ending command the final action; signal
externalised work with the launch prompt's exact ending command; keep an
unanswered confirmation in the same interactive session. An explicit human
request to stop uses the launch prompt's stop command. Ravel ends/reaps the
managed process before returning.
Grove reads Ravel's public stdout result and its own launch teardown record to decide
whether to relaunch, finish or stop. Grove passes no session-name or root
parameter to policy; naming the session belongs to the methodology.

## Installing

Homebrew installs this plugin directory with Grove, at
`share/ravel/skills/grove-core/` of the keg. Homebrew links it into Ravel's
default skill directory, `$(brew --prefix)/share/ravel/skills/`. Ravel 2.0
reads it there with no settings and delivers it to each session. `brew install`
and `brew upgrade` are the whole setup. Grove ships no manual installer, and no
marketplace lists this plugin.

Claude Code namespaces a plugin's components — *"in the UI, the agent
`agent-creator` for the plugin with name `plugin-dev` will appear as
`plugin-dev:agent-creator`"*
([plugins reference](https://code.claude.com/docs/en/plugins-reference), *plugin
name*) — so the native spelling of the spine there is `grove:grove`, and a
kind's skill is `grove:grove-<kind>`. These are examples of that native
spelling. The skills here never write it; see
[the names a skill writes](#the-names-a-skill-writes).
See [Codex skills that earlier Grove releases installed](../README.md#install--codex)
for the links that Grove 22.8 and earlier wrote.

Every skill here declares `harnesses: [any]`. No installer reads the key now,
so it stays as metadata.

**That declaration was `[claude-code]` for three leaves, and the reason it was
is worth keeping.** The `grove` binary owned `~/.claude/skills/grove`,
`~/.codex/skills/grove` and `~/.pi/agent/skills/grove`, and the spine is also
called `grove`: linking it would have written a path the binary rewrote on its
next invocation. The kind skills declared it for a second reason — a
`grove-<kind>` name collided with nothing, but each directs a load of the spine,
and installing a member where the spine could not go would have landed that load
on the binary's provisioned copy instead. `delete-provisioning-k19` stopped the
binary writing those paths, and the declarations flipped with it.

**One consequence of that history is a debt, not a state.** The three
binary-written directories are ordinary directories, not symlinks, and
the removed manual installer refused to install over one at a path it did not
own. They must be removed by hand once, on any machine that ran a
pre-`delete-provisioning-k19` build.

## The token a prompt names

**The prompt names the bare `grove-<kind>` skill and no plugin namespace.** On
Claude Code a plugin skill is invoked as `<plugin>:<skill>`
([plugins reference](https://code.claude.com/docs/en/plugins-reference), *plugin
name*). Through a native skill folder or a directory that Ravel supplies, it is the bare
directory name. The plugin that actually supplies a skill decides its
namespace. A pack move can change that plugin without changing the kind. The
prompt therefore names one bare target in one imperative:

> **Load the `grove-<kind>` skill now.** [...] This is the bare skill name. If
> your harness requires a plugin namespace, use the namespace under which
> `grove-<kind>` is actually installed. Do not assume one.

That keeps the element
[`../../docs/research/wording-micro-test.md`](../../docs/research/wording-micro-test.md)
measured as load-bearing: one imperative names one target, so the session
performs no selection and has nothing to defer. Grove has no branch on the
harness and no kind-to-plugin map. Before the first pack move, the prompt also
spelled `grove:grove-<kind>`. That fixed alias would name nothing after a move,
so the [delivery contract](../../docs/specs/codex-skill-provisioning.md#launch-and-compatibility)
removed it.

The same instruction loads the skill first after any mandatory guidance. It
also states what to do when the skill cannot be found or read, because a
missing skill cannot state that itself. `crates/grove-loop/src/prompt.rs`
holds the wording.

## The names a skill writes

<!-- tc: on -->

The skill text follows the same rule as the prompt. A skill names every other
skill by its bare name. It writes "Load the `grove` skill" for a directed load,
and "the `codebase-design` skill" for a citation of a craft skill. It writes no
plugin-qualified name, as rule 2 of "References between skills" in
[the external pack contract](../../docs/specs/external-pack-contract.md)
requires. A later pack move changes the plugin of a skill, and a qualified name
then addresses nothing.

The kinds that stay in core open with the contract's bare-name text. After the
load, that text tells the session to use the namespace under which `grove` is
actually installed. The kinds that leave core in a later move keep
`grove:grove` in their openings, and each move changes those bytes.

A citation of a craft skill states what binds without that skill. The spine's
`references/grove.md` holds that statement for each cited craft skill, and
names no plugin or pack. `crates/grove/tests/plugin_fallback.rs` enumerates
every `<word>:<word>` token in the shipped markdown, and fails on an
unclassified one.

## Conformance

`conformance.sh` is the methodology's delivery assertion
([`behavioural-coverage-asserts-delivery`](../../docs/adr/behavioural-coverage-asserts-delivery.md)).
It is the only standing instrument for that rule. `plugin-kind-skills-k17` and
`delete-provisioning-k19` deleted the Rust suites that measured the old embedded
corpus. The runner has no dependencies. Its bash front end reads the arguments
and lists the packs, the skills and the files. The awk program
`conformance.awk` beside it does the checks.

```
./plugins/grove/conformance.sh          # core alone
./plugins/grove/conformance.test.sh     # the controls over the runner
```

A pack is a source directory that holds `skills/` and `conformance/rules.tsv`.
Core is this plugin. The
[external pack contract](../../docs/specs/external-pack-contract.md#conformance)
owns the inputs and the checks of each mode. The runner reads directories and
runs no VCS command.

| Run | Arguments | What it checks |
| --- | --- | --- |
| Core alone | none | Core's skills against core's manifest. Core alone is the aggregate with no external pack |
| Pack alone | `--pack <dir> --alone` | One pack against its own manifest, without core |
| Aggregate | `--pack <dir>`, once for each pack | Core and each supplied pack together |

Three assertions hold in each run:

1. **Each behavioural rule is on the composed loaded path of each kind that
   binds it.** The first file of the path is the kind's own `SKILL.md`. The
   path follows each directed load, "Load the `<name>` skill", and each
   skill-relative path. The spine is on a kind's path only because the kind
   loads it.
2. **No rule has two owners.** A phrase sweep covers each file of the run. It
   sweeps the spine's `SKILL.md` from the other direction, because a condition
   register states situations and paths, never procedures. Each row names an
   owner that its own pack ships. No two packs ship one skill name, and no two
   rows have one rule ID.
3. **Each skill-relative path resolves, and each directed load names a shipped
   skill.** The runner extracts each backticked `.md` token and then classifies
   it. A token that it cannot classify fails, because a pattern list is
   complete only as far as the list.

A pack alone has no core. A directed load or a path that reaches a skill of
another pack is then an external edge or an external resource. The runner
lists each one, and the aggregate resolves it. The run does not fail on it.

### The pack-facing invocation

A pack runs Grove's runner from a checkout of Grove at the commit that the
pack records. It does not copy the runner. The contract's
[checker without a copy](../../docs/specs/external-pack-contract.md#the-checker-without-a-copy)
names the Taskfile variables and tasks. From the root of the pack's repository,
the two runs are:

```
"<grove checkout>/plugins/grove/conformance.sh" --pack plugins/research --alone
"<grove checkout>/plugins/grove/conformance.sh" --pack plugins/research
```

The second run reads core's skills and manifest from the same checkout, so one
record gives the checker and core's content at one revision. The pack's
optional `conformance/removed-paraphrases.tsv` lists the retired paraphrases of
its rules. Exit status 0 means that each assertion holds, 1 means that an
assertion failed, and 2 means bad usage.

### The manifests

Core's inventory is [`conformance/rules.tsv`](conformance/rules.tsv). It was
transcribed from `docs/specs/corpus-rule-ownership.md`, which went with
`content/` at `delete-provisioning-k19`. The file is now the inventory itself,
so a row changes here alone. The header of the file describes each column and
each load predicate.

**A row names its owner skill-qualified**, such as `grove/references/execute.md`
or `grove-planning/SKILL.md`. Each kind owns a file called `SKILL.md`. A sweep
that reported sites skill-relative would therefore collapse two kinds that state
one rule into one site. It would then read clean over the duplicate. The control
`one kind-owned rule stated by a second kind is caught` credits the grain.

A row whose owner file its pack does not ship fails. A row left in a manifest
after its owner moved to another pack would otherwise pass in silence.

### What it does not prove

- The single-source sweep is **phrase-scoped**. A paraphrase, a second
  statement of the same rule in other words, reads clean. Core's three known
  paraphrases are in
  [`conformance/removed-paraphrases.tsv`](conformance/removed-paraphrases.tsv).
  Nothing claims that no fourth exists.
- **One** row is delivered by the **driver**, not by a skill: the final ending
  command of the launch prompt, which `${prompt}` states for each kind. Its
  owner cell is `${prompt}`. A runner over an installed skill set cannot read a
  prompt, so the runner reports that row and asserts nothing about it.

  It was two rows until `prompt-names-the-kind-k18`. A `finish` session's three
  endings rode the prompt, because the driver inlined a signal file for each
  kind. They are now inline in `grove-finish/SKILL.md`, so that row came back
  inside this runner's reach. Its completed-teardown ending records the teardown
  before it publishes Grove's result through Ravel. The record and the result
  have different owners and purposes.
- Rows with no pinned wording carry only the loaded-path assertion.
- The engine keeps to POSIX awk. Its controls ran with the awk of macOS only.

The controls in `conformance.test.sh` make the readings evidence. An instrument
that has only ever read clean is indistinguishable from a broken one. Each
control therefore reads its fixture clean, makes one mutation, and requires the
runner to fail with the intended diagnostic and exit status 1. The pack
controls use a fixture pack, never the real research pack.

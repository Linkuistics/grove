# `grove` — the methodology, as skills

Grove's methodology ships as a Claude Code plugin, installed the way this repo's
other two plugins already are ([`../README.md`](../README.md)). It is a shared
**spine** skill and one `grove-<kind>` skill per session kind beside it, over
that spine. **A kind exists iff a skill of that name exists**, so the set is
whatever `skills/` holds: no code carries a list of kinds and no assertion counts
them. Prose here describes the set as it stands, which is a different thing from
machinery holding it.

| | |
|---|---|
| skills | `grove` — the shared spine — and one `grove-<kind>` per kind |
| marketplace | declared in [`../../.claude-plugin/marketplace.json`](../../.claude-plugin/marketplace.json) |
| conformance | [`conformance.sh`](conformance.sh), with its controls in [`conformance.test.sh`](conformance.test.sh) |
| design | [`../../docs/specs/module-decomposition.md`](../../docs/specs/module-decomposition.md), decision 11 and test seam 4 |

## What the spine holds, and what it does not

**The fatness rule.** A rule shared across families is in the spine. A rule owned
by one kind is inline in that kind's own `grove-<kind>` skill. A rule owned by a
*family* — the five reviews, the five integrations, the two research halves — is
**one file in the spine**, and each member's skill directs a load of it by name.
**Nowhere twice**, which is what
[`corpus-rules-have-one-owner`](../../docs/adr/corpus-rules-have-one-owner.md)
and
[`restatement-declares-its-class`](../../docs/adr/restatement-declares-its-class.md)
make checkable.

So the spine carries `SKILL.md`, the five format documents, the seven shared
loop-step references, and the three family files. It carries no kind reference
and no list of kinds: **a kind exists iff a skill of that name exists.**

**What "inline" means, concretely.** A single-kind rule is in the body of that
kind's own `SKILL.md` — not in a `references/` file the skill points at. The
distinction is the whole of the difference between the rule's two halves: what is
inline arrives *with* the skill in one hop, and what is in the spine is reached
by a second one. Seven kinds are fat this way (`requirements`, `design`,
`planning`, `prototype`, `impl`, `combine-research`, `finish`) and twelve are
thin: the five `review-*`, the five `integrate-review-*` and the two research
halves each state their identity and direct one load, because everything else
they obey belongs to their family and the family's text is the spine's.

Two files ride inside a kind's skill rather than in its `SKILL.md` body, and both
are that kind's alone: `grove-requirements/grilling.md`, a procedure reached on a
condition rather than read every session, and
`grove-impl/LICENSES/addyosmani-agent-skills.LICENSE`, which only that skill's
provenance comment cites.

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

`plugins/grove/skills/` is the methodology. There is no second copy.

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
signalling contract once for every kind, and `finish`'s three endings are inline
in `grove-finish/SKILL.md`.

## Installing

**Claude Code**, through the marketplace — this is the supported path today:

```
/plugin marketplace add Linkuistics/grove
/plugin install grove@linkuistics
```

Claude Code namespaces a plugin's components — *"in the UI, the agent
`agent-creator` for the plugin with name `plugin-dev` will appear as
`plugin-dev:agent-creator`"*
([plugins reference](https://code.claude.com/docs/en/plugins-reference), *plugin
name*) — so the spine is `grove:grove` there, and a kind's skill is
`grove:grove-<kind>`.
**Other harnesses**, through [`../install.sh`](../install.sh)'s symlink farm:

```sh
./plugins/install.sh
```

Every skill here declares `harnesses: [any]`, so the script links all twenty
into `~/.codex/skills/`, `~/.gemini/skills/` and `~/.pi/agent/skills/` — one
directory per skill, named exactly as the skill is.

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
`install.sh` refuses to install over one at a path it does not own. They must be
removed by hand once, on any machine that ran a pre-`delete-provisioning-k19`
build.

## The token a prompt names

**Settled here, and `prompt-names-the-kind-k18` uses it rather than re-deriving
it.** On Claude Code a plugin skill is invoked as `<plugin>:<skill>`
([plugins reference](https://code.claude.com/docs/en/plugins-reference), *plugin
name* — namespacing is stated there for every plugin component); through the
symlink farm it is the bare directory name. The prompt therefore names both
spellings of the **one** target, in one imperative:

> **Load the `grove-<kind>` skill now** — on Claude Code, where plugin skills are
> namespaced, that is `grove:grove-<kind>`.

That keeps the element
[`../../docs/research/wording-micro-test.md`](../../docs/research/wording-micro-test.md)
measured as load-bearing — one imperative naming one target, so the session
performs no selection and has nothing to defer — while leaving grove with no
branch on the harness and no registry to hold one. A session reads one line and
uses whichever spelling its harness offers.

## Conformance

`conformance.sh` is the methodology's delivery assertion
([`behavioural-coverage-asserts-delivery`](../../docs/adr/behavioural-coverage-asserts-delivery.md)),
moved out of the Rust suite because two of the four things that walk covered no
longer exist once the methodology is a plugin. `plugin-kind-skills-k17` deleted
`tests/rule_ownership.rs`, whose whole subject is assertion 2 below, and
`delete-provisioning-k19` deleted the last two — the coverage walk in
`tests/lifecycle_invariants.rs` and the per-kind word budgets in
`tests/loaded_path_budgets.rs` — whose subject was the corpus the binary embedded
and the path it composed. This runner is the only standing instrument now.
Dependency-free bash over the shipped skill set:

```
./plugins/grove/conformance.sh          # the assertions
./plugins/grove/conformance.test.sh     # the controls over them
```

Three assertions, and it asserts nothing about how many kinds there are:

1. **Every behavioural rule is present on the composed loaded path of every kind
   that binds it.** The path is the closure of what the shipped files actually
   name, read off the bytes rather than replayed from a table.
2. **No rule has two owners.** A phrase sweep over every shipped file, with the
   spine's `SKILL.md` swept from the other direction — a condition register
   states situations and paths, never procedures.
3. **Every file a skill names by path exists.** Every backticked `.md` token is
   enumerated and then classified; a token the runner cannot classify fails,
   because a pattern list is complete only as far as the list.

Its inventory is [`conformance/rules.tsv`](conformance/rules.tsv). It was
transcribed from `docs/specs/corpus-rule-ownership.md`, which went with
`content/` at `delete-provisioning-k19`; the file is the inventory now rather
than a copy of one, so a row changes here alone.

**A row's owner is written shipped-set-relative** — `grove/references/execute.md`,
`grove-impl/SKILL.md` — and not skill-relative. The grain matters exactly once
there is more than one skill: every kind owns a file called `SKILL.md`, so a
sweep reporting sites skill-relative would collapse two kinds stating one rule
into a single site and read clean over the duplicate it exists to find. The control
`one kind-owned rule stated by a second kind is caught` is what credits it.

**What it does not prove**, stated rather than papered over:

- The single-source sweep is **phrase-scoped**. A paraphrase — a second
  current-state statement of the same rule in other words — reads clean. The
  three known ones are pinned in
  [`conformance/removed-paraphrases.tsv`](conformance/removed-paraphrases.tsv);
  nothing claims a fourth does not exist.
- **One** row is delivered by the **driver**, not by any skill: the signal step,
  which `${prompt}` states as a mechanism for every kind. Its owner cell is
  `${prompt}` and neither assertion touches it — a runner over an installed skill
  set cannot read a prompt. It is reported as prompt-delivered rather than
  *pending*, because pending promises a later leaf that ships the file and no
  leaf will.

  It was two until `prompt-names-the-kind-k18`. A `finish` session's three
  endings rode the prompt because the driver inlined a per-kind signal file; they
  are now inline in `grove-finish/SKILL.md`, so that row came back inside this
  runner's reach. The corpus copy that briefly restated it alongside,
  `content/SIGNAL-FINISH.md`, went with `content/` at
  `delete-provisioning-k19`, so the pair is one statement again.
- A row whose owner file is not shipped is **pending**, not asserted. The count
  reached zero at `plugin-kind-skills-k17`; a run reporting a pending row again
  is reporting a skill that did not ship.
- Rows with no pinned wording carry the loaded-path assertion only.

The controls in `conformance.test.sh` are the reason any of that is evidence: an
instrument that has only ever read clean is indistinguishable from a broken one,
so each assertion is watched failing against a skill set wrong in one named way,
and one case requires a clean reading so *always fails* is ruled out too.

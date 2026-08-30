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

## The source of truth, while there are two copies

The `grove` binary still compiles its own `content/` into itself and provisions
it to `~/.claude/skills/grove` and two siblings. So the same bytes exist twice
right now, and which copy is authoritative is not a matter of taste:

- **`plugins/grove/skills/grove/` is the source.** Edit the methodology here.
- **`content/`'s shared files are the copy, and they are what gets deleted** at
  `delete-provisioning-k19`, along with provisioning itself. Nothing in
  `content/` is edited on its own.

`conformance.sh` holds the two byte-identical for every file they share, and
says so in its report. That check exists only while both do, and it goes with
`content/`.

**One file is split rather than shared, and is expected to differ.**
`content/SKILL.md` is a *kind router*: it sends a session to one of ten reference
files, and the live binary still provisions it. The spine's `SKILL.md` is a
*condition register* — it states that the driver already named your
`grove-<kind>` skill, and names only files the spine carries.

**Where the rest of `content/` goes.**

| still in `content/` | lands where | at |
|---|---|---|
| the seven single-kind references, and `grilling.md` | inline in `grove-<kind>` — **done** at `plugin-kind-skills-k17` | landed |
| `SKILL.md`'s kind-routing table | deleted; the prompt names the kind's skill | `prompt-names-the-kind-k18` |
| `SIGNAL.md`, `SIGNAL-FINISH.md` | driver-authored prose in the prompt | `prompt-names-the-kind-k18` |
| everything else | already here | `plugin-spine-k16` |

The two signal files never become skill files: the driver already inlines their
bytes into `${prompt}`, and `grove-finish/SKILL.md` names the prompt rather than
a file for the three endings. `k18` moves who authors them, not where a session
reads them.

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
That is why it does not contend with the `grove` skill the binary provisions into
`~/.claude/skills/`: the two are addressed differently. **The reference states
the namespacing and does not state the collision rule**, so the second half rests
on the namespacing plus the observed case — a session listing
`linkuistics:cli-tool-design` and a bare `grove` side by side — rather than on a
documented guarantee. If the two ever do contend, `delete-provisioning-k19`
removes the provisioned copy and the question with it.

**Other harnesses**, through [`../install.sh`](../install.sh)'s symlink farm —
**not yet**. Every skill here declares `harnesses: [claude-code]`, so the script
skips them and says why. The kind skills declare it for a second reason of their
own: a `grove-<kind>` name collides with nothing the binary writes, but each one
directs a load of the spine, and installing a member where the spine cannot go
would land its directed load on the binary's provisioned `content/` instead —
a worse outcome than not installing it.

That is not a workaround for a collision; it is the declaration being true. The
binary owns `~/.codex/skills/grove` and `~/.pi/agent/skills/grove` until
`delete-provisioning-k19`, and `install.sh` refuses to install over a real
directory at a path it does not own. Every declaration here flips to `[any]` when
the binary stops writing those paths.

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
`tests/rule_ownership.rs`, whose whole subject is assertion 2 below; the Rust
suites that remain — the coverage walk in `tests/lifecycle_invariants.rs` and the
per-kind word budgets in `tests/loaded_path_budgets.rs` — have subjects this
runner does not carry, and go with `content/` at `delete-provisioning-k19`.
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

Its inventory is [`conformance/rules.tsv`](conformance/rules.tsv), transcribed
from [`../../docs/specs/corpus-rule-ownership.md`](../../docs/specs/corpus-rule-ownership.md).
Adding, removing or repointing a row edits both.

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
  runner's reach. **What it does not reach is the corpus copy.** `content/`
  still ships `SIGNAL-FINISH.md`, carrying the same table in nearly the same
  words, until `delete-provisioning-k19` deletes it — and the temporary
  spine-and-`content/` byte check below cannot see it, because
  `SIGNAL-FINISH.md` has no spine counterpart to be compared against. That pair
  is a restatement with no declared class for one more leaf, recorded here
  rather than machinery written for a thing whose lifetime is one leaf. The
  corpus copy is the one that is unreachable: nothing routes a session into
  `content/` once the prompt names a plugin skill.
- A row whose owner file is not shipped is **pending**, not asserted. The count
  reached zero at `plugin-kind-skills-k17`; a run reporting a pending row again
  is reporting a skill that did not ship.
- Rows with no pinned wording carry the loaded-path assertion only.

The controls in `conformance.test.sh` are the reason any of that is evidence: an
instrument that has only ever read clean is indistinguishable from a broken one,
so each assertion is watched failing against a skill set wrong in one named way,
and one case requires a clean reading so *always fails* is ruled out too.

# chain-node-k9

**Kind:** impl

## Goal

Make `leaf-add-chain` and `leaf-add-pair` write a **chain node** — one directory
holding the three steps — and reconcile every description of the tool to it.

## Context

The decision is already made and recorded (`chain-as-node-k7`). This leaf executes
it; it does not re-litigate it. Read the design record first:

- `docs/specs/task-kind-taxonomy.md` § *A chain is a node directory, named by a
  shared stem plus a step suffix* — the shape, the reasoning, the naming rules,
  and the no-migration decision.
- `docs/adr/task-tree-scheme.md` — the two node species and the charter
  discriminator; § *Reference a work item by its stable handle* for why children
  keep the stem.
- `CONTEXT.md` §§ *Node directory*, *Review chain / vendor pair*.

The shape both verbs must write:

```
grove-llm leaf-add-chain <parent> <stem> --kind <producer>
   NN    <stem>-chain-k<a>/                   chain node — no BRIEF.md
     01  <stem>-k<a+1>            **Kind:** <producer>
     02  <stem>-review-k<a+2>     **Kind:** review-<producer>
     03  <stem>-integrate-k<a+3>  **Kind:** integrate-review-<producer>

grove-llm leaf-add-pair <parent> <stem> --harness-a <name> --harness-b <other>
   NN    <stem>-pair-k<a>/                    chain node — no BRIEF.md
     01  <stem>-a-k<a+1>          **Kind:** research, **Harness:** <name>
     02  <stem>-b-k<a+2>          **Kind:** research, **Harness:** <other>
     03  <stem>-combine-k<a+3>    **Kind:** combine-research
```

Four keys per shape, not three — the node holds the first. Stdout becomes **four**
absolute paths, node directory first, so a caller can `leaf-add <node>` straight
into it.

## Done when

**Code** (`src/tree_grow.rs`, and wherever the two verbs' stdout contract lives):

- Both verbs create the node directory and write their three leaves inside it.
- **No `BRIEF.md` is written.** This is the discriminator the Retire cascade uses;
  a stray charter silently restores the confirmation the design removed.
- Whole-shape-or-nothing survives: validate and resolve before the first write,
  positions and keys from one snapshot, destinations checked free up front, and a
  mid-write failure rolls back — now one recursive remove of a directory that did
  not exist beforehand, which is strictly easier than the flat shape's three
  unlinks. A rollback that cannot complete still names the residue by path.
- Stdout is four paths after success, or nothing.
- `leaf-add`, `leaf-insert`, `leaf-decompose`, `leaf-retire`, `leaf-prune`,
  `pick`, `resolve` and `brief-chain` are **untouched**. If any needs a change to
  cope with a brief-less node, that is a finding worth surfacing — the design
  claims they all already cope (`brief-chain` skips a missing level, `pick` reads
  no briefs, the plugin never opens one).

**Tests** — the three properties the spec marks as worth *falsifying by mutation*
rather than merely asserting: the kind derivation, the absence of `BRIEF.md`, and
the rollback leaving no directory. A happy path that made four things does not
test the reason the verb exists.

**Prose** — replace the flat-shape guidance, never sit beside it (parallel
old-and-new guidance is how *compose-task-chains-k29* failed the first time):

- `content/SKILL.md` — the Decompose step's "a chain gets **no node directory**"
  paragraph, and the node definition (a node no longer always holds a `BRIEF.md`).
- `content/SKILL.md` Retire step — **"ask before treating a node as done" becomes
  "ask before treating a *brief-carrying* node as done"**, with one line on why.
- `content/TASK-FORMAT.md` — the node definition at ~L53 and the chain argument at
  ~L161.
- `content/BRIEF-FORMAT.md` — asserts at ~L6 that *every* node carries a charter.
- `content/driving.md`, `content/prompts/*.md` — wherever the hand-cut or flat
  chain shape appears.
- `docs/grove.md` — carries "A chain still gets no node directory of its own"
  verbatim.
- `CHANGELOG.md`.

## Notes

**No migration, and it is a decision rather than an omission.** Detecting a flat
chain in an existing tree needs the `-review`/`-integrate` suffix parsing the
design forbids, and would misfire on a leaf legitimately named `foo-review`. A flat
chain is a valid tree that `pick` walks correctly. No format bump; `grove do`'s
migration step is untouched. **This grove's own tree proves the case** — the leaf
you are reading is one of a *flat* chain (`chain-node-k9` / `chain-node-review-k10`
/ `chain-node-integrate-k11`), cut before the verb changed. It must keep working to
the end.

**Nothing may key on the `-chain` / `-pair` token.** It is ordinary slug text,
present only so the node's slug does not collide with its first child's under
`resolve`. The chain-vs-decomposition discriminator is the **presence of
`BRIEF.md`** — a file test, not a name parse. Getting this wrong reintroduces the
convention-parsing the whole design exists to avoid.

**The tree viewer needs no change** — verified while deciding: `herdr-plugin/`
matches any `NN-<slug>-k<key>/` as a node, never opens a `BRIEF.md`, and collapses
a finished node to one counted line. Do not "fix" it. Re-render to confirm:
`NO_COLOR=1 python3 herdr-plugin/grove_tree.py .`

**Interaction with `retire-confirmation-k12`**, which sits behind this chain: it
revisits whether *any* retirement needs confirmation. If it lands first the cascade
prose here may need one more pass. The brief-carrying rule is independent of its
outcome — it narrows which nodes are asked about, and the broader leaf asks whether
to ask at all.

**Consult `linkuistics:coding-style-rust`** and `tree_grow.rs`'s existing
conventions; this is an edit to code that already has a house style.

# The CLI is two binaries split by audience: `grove` and `grove-llm`

grove ships two binaries produced from a single Cargo crate (one `[lib]`, two
`[[bin]]` targets) and installed together by the Homebrew formula. **`grove`**
holds the verbs a human at a terminal invokes — the lifecycle set `do` /
`migrate` / `retire`. **`grove-llm`** holds the verbs the LLM driving a grove
session invokes mid-session to perform deterministic mechanics — `root-init`,
`pick`, `brief-chain`, `kind`, `resolve`, `leaf-add`, `leaf-add-chain`,
`leaf-add-pair`, `leaf-insert`, `leaf-decompose`, `leaf-retire`, `leaf-prune`,
`complete` — plus `report-turn`,
which sits on `grove-llm` because it is loop machinery rather than a human verb,
but which the model never calls (grove injects it as a harness hook,
*herdr-turn-boundary-hooks*).

This record is the **one place the verb surface is enumerated**. Other ADRs that
touch a verb cite it rather than re-listing the set — two lists drift apart, and a
set that disagrees with itself binds nothing.

## Why split the surface at all

Two distinct audiences exercise the CLI, and they barely overlap. A human types
`grove do` once in the worktree and forgets about it. The LLM inside
that grove's session, by contrast, calls verbs every loop to *drive* the process —
pick the next leaf, walk the brief chain, grow the tree, signal completion. Mixing
both sets on one binary produces a `--help` that confuses both audiences: the human
sees mid-session verbs they should never type, and the LLM has to filter out
lifecycle verbs that would re-enter a session it is already inside. Splitting by
audience keeps each binary's `--help` a clean, complete list for exactly one reader.

The gain is strongest for the LLM: a parser-checked verb surface gives a **stronger
correctness guarantee** than prose instructions to a language model. Every
deterministic step in the loop with a stable input/output shape is a `grove-llm`
verb rather than an instruction the model might paraphrase wrong.

## Why flat, hyphenated verb names on `grove-llm`

`grove-llm`'s verbs are single hyphenated tokens (`leaf-add`, `brief-chain`), not
nested clusters (`leaf add`), for LLM-specific reasons:

- A single `grove-llm --help` lists every verb the LLM might call — the key
  **bootstrap-recovery** move if a session drops context and has to re-orient on
  what verbs exist.
- Hyphenated single tokens give clap a global suggestion pool on typo:
  `grove-llm leaff-add` yields "did you mean `leaf-add`?"; under nested clusters a
  mistyped noun would only see that noun's subcommand pool.
- To an LLM reading or generating an invocation, `leaf add` and `leaf-add` render
  to the same tokens — the visual-hierarchy argument for nested clusters buys
  nothing here because no human reads this `--help` by design.

## What earns a `grove-llm` verb

"Every deterministic step with a stable input/output shape" is the rule above, and
on its own it would admit almost anything. A step earns a verb when **all three**
hold:

1. **Deterministic** — the output is a function of the arguments. No judgement
   runs inside the verb.
2. **Derived from something grove already owns** — the closed kind set, the
   tree's positions and keys, a naming convention grove authored. This is the leg
   that matters: it means a session doing the step by hand can get it *wrong*,
   not merely do it slowly, and a wrong-but-well-formed result is the error class
   a parser cannot catch but a derivation can.
3. **The judgement stays outside** — the caller decides *whether*; the verb
   decides *what*. A verb that must infer why it was called fails this leg.

Leg 3 is what separates a convenience from a policy, and it is where the
one-leaf-per-call instinct actually comes from. It is **not** a rule that a verb
writes one file: `root-init` writes a root brief *and* a first leaf,
`leaf-decompose` a brief *and* a first child, because in each case the caller
named a *shape*. What leg 3 forbids is a verb inferring the shape from an
argument that meant something narrower.

The bar also excludes, by leg 3 alone, any verb that inspects the tree to decide
whether something *ought* to exist — a chain linter, a completeness check. Those
would gate besides (constraint 5), but they fail this bar first.

## Considered options

- **`leaf-add --chain`, minting a review chain from the verb that means one
  leaf** (rejected) — versus `leaf-add-chain` / `leaf-add-pair` as distinctly
  named verbs (adopted; `docs/specs/task-kind-taxonomy.md`, *Constructing a chain
  is one call*). Both write three leaves, so the difference is entirely leg 3: on
  a flag, `leaf-add` would be making the escalation call (chain, or a mid-session
  subagent?) that is the only judgement in the pattern, on the strength of an
  argument the caller used to mean *one leaf*. Under its own name, the caller has
  already made that call and the verb only executes it. Nothing would reopen the
  flag — a named verb dominates it on every axis.

  A closing generalisation an earlier draft of that rejection carried — *"the
  verbs stay one leaf per call"* — does **not** survive, and was already false
  when written: `root-init` writes a root brief and a first leaf, and
  `leaf-decompose` a brief and a first child. The rule is leg 3, not a file
  count, as *What earns a `grove-llm` verb* now says outright.
- **Single binary, one flat namespace** — all verbs on `grove`. Rejected: the
  audience-mixed `--help` grows without bound and the human's discovery surface
  degrades with every LLM-only verb.
- **Single binary with a nominal `grove llm <verb>` cluster.** Rejected in favour
  of a *structural* split: the git porcelain/plumbing and kubectl/kubeadm precedents
  show a separate binary communicates intent more cleanly than a subcommand the
  human is told to ignore, and the LLM benefits from a `--help` with nothing else
  mixed in.
- **`argv[0]` dispatch (busybox-style)** — one executable symlinked as both names.
  Rejected as more complex to package and reason about than two `[[bin]]` targets
  sharing one `[lib]`; for a tool whose whole point is being legible by default, an
  explicit second binary beats a clever single one.

## Consequences

- Both binaries share all internal logic via the `[lib]`; only the clap dispatchers
  differ. The Homebrew formula installs both from one release tarball and its
  `test do` block exercises both.
- Internal Rust callers invoke library functions directly — they never shell out to
  either binary. The split is structural at the CLI layer only.

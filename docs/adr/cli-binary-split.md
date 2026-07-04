# The CLI is two binaries split by audience: `grove` and `grove-llm`

grove ships two binaries produced from a single Cargo crate (one `[lib]`, two
`[[bin]]` targets) and installed together by the Homebrew formula. **`grove`**
holds the verbs a human at a terminal invokes — the lifecycle set `do` /
`migrate` / `retire`. **`grove-llm`** holds the verbs the LLM driving a grove
session invokes mid-session to perform deterministic mechanics — `root-init`,
`pick`, `brief-chain`, `resolve`, `leaf-add`, `leaf-insert`, `leaf-decompose`,
`leaf-retire`, `complete`.

## Why split the surface at all

Two distinct audiences exercise the CLI, and they barely overlap. A human types
`grove do auth-redesign` once at the terminal and forgets about it. The LLM inside
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

## Considered options

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

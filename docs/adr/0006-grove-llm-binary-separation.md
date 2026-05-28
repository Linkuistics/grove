# LLM-driven verbs live on a separate `grove-llm` binary, not on `grove`

The CLI surface is audience-split into two binaries shipped together. **`grove`** holds the verbs a human at a terminal invokes — launchers (`start`, `continue`, `takeover`, `retire`, `finish`), repo admin (`install`, `update`, `uninstall`, `version`, `status`, `list`), the `grove-meta` branch admin cluster (`meta init|remote|sync`), and the diagnostic `inbox-show`. **`grove-llm`** holds the verbs the LLM driving a grove session invokes mid-session to perform deterministic mechanics — currently `inbox-add` and `inbox-drain`, with leaves 020–050 of the `080-audit-llm-cli-boundaries` subtree adding `pick`, `brief-chain`, `leaf-add`, `leaf-insert`, `leaf-decompose`, and `leaf-retire`. Both binaries are produced by a single Cargo crate (one `[lib]`, two `[[bin]]` targets) and installed together by the Homebrew formula.

## Status
accepted

## Why split the surface at all
Two distinct audiences exercise the CLI. A human types `grove start auth-redesign` once at the terminal and forgets about it. The LLM inside that grove's session, by contrast, calls verbs every loop to *drive* the process — capture an observation, drain the inbox, pick the next leaf, walk the brief chain. The audit at `.grove/020-design-seed-convention/080-audit-llm-cli-boundaries/BRIEF.md` (decisions Q1–Q4 of the running log) identified that every deterministic step in `content/SKILL.md`'s loop with a stable input/output shape is a candidate for promotion to a CLI verb, motivated not by observed failures but by the **stronger guarantee of correctness** that a parser-checked surface provides over prose instructions to a language model.

Once the LLM-driven set was enumerated it became clear that the two audiences barely overlap. The human cares which verbs they need to know to drive grove from outside a session; the LLM cares which verbs are safe to invoke inside one. Mixing them on one binary produces a `--help` that confuses both audiences — the human sees mid-session verbs they should never type, and the LLM has to filter out launcher verbs that would re-enter a session it is already inside.

## Considered options
Three shapes were considered:

1. **Single binary, one large flat namespace** (status quo before this ADR for the migrated verbs). `grove inbox add`, `grove inbox drain`, plus the future `grove pick`, `grove brief-chain`, `grove leaf-add`, etc. on the same binary as the launchers. Rejected because the audience-mixed `--help` keeps growing and the human's discovery surface degrades with every LLM-only verb added.
2. **Single binary with a subcommand cluster `grove llm <verb>`**. The audience split becomes nominal (a subcommand the human is told to ignore) but stays on one binary. Considered seriously during the audit grilling and initially adopted as Q2. Rejected when Q4 surfaced the precedent of git porcelain/plumbing and kubectl/kubeadm — both projects discovered that a *structural* split (separate binary) communicates intent more cleanly than a nominal split, and that an LLM-driven set in particular benefits from a `--help` that lists every verb the LLM should call with nothing else mixed in (parent BRIEF Q3 — bootstrap-recovery property if a session drops context).
3. **Single binary with `argv[0]` dispatch** (busybox-style). One executable, symlinked as `grove` and `grove-llm`; behaviour selected by the invocation name. Rejected as more complex to package and reason about than two `[[bin]]` targets sharing one `[lib]` — for a tool whose whole point is being legible-by-default, an explicit second binary beats a clever single one.

## Why flat verb names inside `grove-llm` (not nested clusters)
The verbs are hyphenated single tokens — `inbox-add`, `inbox-drain` — not nested `inbox add`, `inbox drain`. The rationale is LLM-only:

- A single `grove-llm --help` invocation lists every verb the LLM might call. This matters most for **bootstrap-recovery**: if a session drops context and the LLM has to re-orient on what verbs exist, one `--help` is the recovery move.
- Hyphenated single tokens give clap a global suggestion pool on typo. `grove-llm inboxx-add` produces a "did you mean `inbox-add`?" pointing at the right verb; under nested clusters, `grove-llm inbox addd` would only see `inbox`'s subcommand pool.
- For an LLM reading the verb in prose or generating an invocation, the difference between `inbox add` and `inbox-add` is purely surface-form — both render to the same tokens at the model's level. The argument for nested clusters (visual hierarchy in human `--help` output) does not apply on this binary because no humans read this `--help` by design.

This is in deliberate contrast to the **human** binary, where verbs are nested under noun clusters — `grove meta init|remote|sync`, `grove inbox show` — because a human user benefits from the hierarchy ("anything to do with the meta branch is under `grove meta`"). The two binaries pick opposite verb shapes for audience-driven reasons, not from inconsistency.

## Why two migrated verbs, not more
Only `grove inbox add` and `grove inbox drain` migrated in this ADR's introducing leaf (`010-scaffold-grove-llm-and-migrate-inbox`). They are the verbs the LLM was already invoking mid-session before the binary split existed. Other LLM-driven verbs (`pick`, `brief-chain`, `leaf-add`, `leaf-insert`, `leaf-decompose`, `leaf-retire`) ship as new verbs in subsequent leaves of the same subtree — they did not need migration because they were prose-mechanics before. Borderline verb `grove inbox show` stayed on the human binary as a diagnostic, in nested noun-verb form. Flattening it to `grove inbox-show` was considered (single-subcommand cluster reads worse locally) and rejected: the rest of the human surface uses nested noun-verb clusters (`meta init`, `meta remote add`, `meta sync`), and consistency across the surface matters more than the marginal local win. The contrast with `grove-llm`'s flat verbs is deliberate and audience-driven.

## Hard cutover, not deprecation aliases
`grove inbox add` and `grove inbox drain` were removed entirely from the `grove` binary; no shim, no deprecation warning. The only known caller surface is the LLM driving sessions via `content/SKILL.md`, which this leaf updates atomically. The Homebrew formula installs both binaries in lockstep — a user updating `grove` necessarily picks up `grove-llm`. The grove project is pre-v1 in spirit and the inbox feature shipped within the same iteration that contained this audit; carrying deprecation shims would have been all cost and no benefit.

## Consequences
- The Cargo crate now declares a second `[[bin]]` target. Both binaries share all internal logic via the `[lib]` — the `grove::inboxes` and `grove::meta` modules are unchanged, only the clap dispatchers (`crate::cli` for `grove`, `crate::llm_cli` for `grove-llm`) are new or shrunken.
- The Homebrew formula's `bin.install` line installs both binaries from a single release tarball, and the `test do` block exercises both.
- Future leaves (020–050 of the `080-audit-llm-cli-boundaries` subtree) extend `grove-llm`'s verb set without further restructuring; each leaf adds verbs and updates `content/SKILL.md`'s loop prose to reference the new verb names.
- Internal Rust callers (`install::run` triggering `inboxes::materialise`, for instance) continue to call the library functions directly — they never shell out to either binary. The split is structural at the CLI layer only.
- The `grove-meta` branch and its sync semantics (ADRs 0002–0005) are unchanged. Only the verb names referencing those mechanics moved; the on-disk shape, the fetch/push policy, and the cross-repo rule all stand as-is.

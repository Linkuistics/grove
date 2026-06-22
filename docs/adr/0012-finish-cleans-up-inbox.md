# The complete finish cycle removes the grove's inbox; pending observations refuse-and-instruct rather than being deleted

The complete finish cycle (ADR-0010) tore down a grove's worktree and branch
but left its `grove-meta` inbox at `inboxes/<name>/` untouched. By the glossary's
own definition a *Seed* was "an inbox whose addressed grove does not currently
exist as a worktree — whether yet to start **or already finished**", so the
orphaned inbox (kept alive by its `.gitkeep`) rendered in `grove status` / the
TUI as a seed, indistinguishable from a not-yet-started workstream. We add a
**new finish step** — a thin `grove-llm inbox-remove --for=<name>` verb the
finish prose calls — that deletes `inboxes/<name>/` entirely. When observations
are still pending it **refuses and instructs** (drain first) rather than deleting
them. This is the finish-side counterpart of ADR-0011's start-side fix: both make
a grove's lifecycle state legible on the seed/done axis.

## Status
superseded by ADR-0031 `0031-shed-machinery-keep-self-extension-core-and-methodology.md` — the inbox is deleted, so the finish cycle no longer has an inbox to clean up (it returns to the five-step cycle of ADR-0010).

## Decision 1 — refuse-and-instruct on a non-empty inbox, do not silently delete
[[Drain]] runs only at `grove do`, so by finish time another grove may have
captured observations into this inbox since the session's bootstrap drain.
Blindly `rm -rf inboxes/<name>/` would destroy un-triaged work. The verb instead
enumerates pending `.md` observations (after a fetch, so a remote-pushed
observation counts too) and, if any remain, **bails** with an instruction to
drain first — directly mirroring ADR-0005's non-ff *refuse-and-instruct* stance
for the same reason: a genuine "there is unreconciled state here" condition is
the one case where proceeding silently is wrong. Once the inbox holds only its
`.gitkeep`, the verb removes the whole directory and commits
`inbox: remove <name> (grove finished)`, pushing best-effort like every other
`grove-meta` write (ADR-0005).

At finish the triage dispositions narrow: an observation cannot be *incorporated*
(the work is done) or *deferred to a later leaf* (there are none), so the live
options are **re-seed elsewhere** (`grove-llm inbox-add --to=<other>`) or
**reject**. The finish prose says so.

- *Rejected — delete only when already empty (Option B):* simpler, but a finished
  grove that happens to have a late observation keeps showing as a seed — the
  exact symptom, merely narrowed.
- *Rejected — tombstone instead of delete (Option C):* keep the directory, write
  a marker, and teach `repo_view`/TUI a third `Finished` lifecycle. Bigger blast
  radius (seed classification + a new badge) to *preserve* data the finish cycle
  is meant to retire; the grove's history already lives in git
  (`git log inboxes/<name>/`), so nothing is lost by deletion.

## Decision 2 — a thin `grove-llm` verb, called by prose, not Rust in the cycle
The finish cycle is in-session LLM prose, not Rust automation (ADR-0010), and the
LLM must never touch `grove-meta` git plumbing directly (ADR-0006). So the
mechanical removal is a verb — symmetric with `leaf-retire` and the other
working-tree tree verbs — that the finish *prose* invokes as step 4. The
judgement (triage the pending) stays in prose; only the mechanics (refuse-check,
remove, commit, push) are in the verb.

The verb is **idempotent**: a no-op when `inboxes/<name>/` is absent (never
seeded, or already removed by a prior — resumed — finish run). This is what lets
the state-checked finish *resume* (ADR-0010, constraint 1 — no marker file) cover
the new step for free: "if `inboxes/<name>/` is gone, skip 4" is satisfied either
way, and re-running after a partial finish is harmless.

## Step placement and ordering
The new step is **4** of six: promote (1), delete `.grove/` (2), merge (3),
**clean up inbox (4)**, remove worktree (5), delete branch (6). It must precede
worktree removal because the verb resolves the repo from cwd, and the session's
cwd sits inside the worktree step 5 deletes — running inbox cleanup after that
would leave the verb unable to find the repo. Grouping it with the teardown steps
(remove inbox, remove worktree, remove branch) reads as "remove the grove's three
on-disk footprints". The single pre-teardown confirmation gate (ADR-0010) now
covers steps 2–6.

## Why this is recorded
The finish contract binds every future grove retirement (hard to reverse); that
finish now writes to `grove-meta` at all is *surprising* without this context;
and the disposition of pending observations is a *genuine trade-off* with two
rejected alternatives (B, C). All three ADR tests pass. Evidence for the symptom
is the `grove-startup-confuses-the-LLM` grove's root `BRIEF.md` (primary evidence
item 5).

## Consequences
- `content/SKILL.md`'s Finish step gains step 4 and the matching resume guard;
  the headless-gate sentence now says steps 2–6.
- `CONTEXT.md`: the **Seed** entry no longer calls a *finished* grove's inbox a
  seed (a still-orphaned inbox now signals an *incomplete* finish); the
  **Complete finish cycle** entry gains the cleanup step.
- A new `grove-llm inbox-remove --for=<name>` verb (`src/inboxes.rs::remove`,
  wired in `src/llm_cli.rs` / `src/cli.rs`). Integration tests cover empty-inbox
  removal, refusal on pending, idempotent no-op, and the end-to-end
  "a finished grove does not appear as a seed after remove" assertion that ties
  the verb to `repo_view`'s seed classification.
- Detecting "finished" *without* the cleanup (e.g. reading `grove-meta` history)
  stays out of scope, as `repo_view`'s `Lifecycle` comment already notes; this
  ADR removes the need for it by removing the inbox instead.

# probe-carve-out-k3

## Goal

Reconcile `using-jujutsu`'s "First action: probe the repo" with a grove mandate
that says *do not probe*. Skills context only.

## Context

`mandate-states-vcs-k2` makes the loop driver state the VCS in every session's
mandate. `plugins/linkuistics/skills/using-jujutsu/SKILL.md` opens with:

> ## First action: probe the repo
> Before the first VCS command of a session, detect which interface the repo
> picks — in this order: 1. `jj root` … 2. `git rev-parse --show-toplevel` …

A grove session in a jj tree loads that skill and reads an instruction it has
been told to ignore. That contradiction is on the page today.

**Agreed resolution: a generic carve-out that never names grove.** The skill
ships to users who have never heard of grove, and `CONTEXT-MAP.md` currently has
grove→skills as the only cross-context dependency; naming grove here would add
one pointing back. Phrase it as the general rule instead:

> an authoritative statement of the repo's VCS in your prompt outranks both the
> harness banner and a probe.

That is a **generalisation of a rule the section already carries** — it already
says to trust the probe over harness metadata, citing
[claude-code#41435](https://github.com/anthropics/claude-code/issues/41435).
The new rule extends that ordering upward: prompt statement > probe > banner.
Write it as one ordering, not two competing rules.

## Done when

- `using-jujutsu`'s probe section states the precedence, without naming grove,
  and without contradicting its own harness-banner rule.
- The edit is short. This is a precedence clause on an existing section, not a
  new one.
- Consider whether `git-to-jj-mapping` needs the same clause. Probably not — it
  translates commands rather than selecting an interface — but check rather than
  assume.

## Notes

`plugins/CONTEXT.md` is this context's glossary. Add a term only if the writing
resolves one; a precedence clause probably resolves none.

Separate commit from `mandate-states-vcs-k2` by design — one commit, one owning
context (`CONTEXT-MAP.md`, *A durable record has one owner*).

`plugins/install.sh` symlinks these skills, so unlike the driver change this one
is live for sessions on the next skill load — but it describes behaviour the
not-yet-rebuilt binary does not produce. Landing the contradiction in that
direction is the harmless one: a session told to probe still probes correctly.

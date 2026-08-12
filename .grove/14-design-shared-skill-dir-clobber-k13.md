# shared-skill-dir-clobber-k13

## Goal

Decide what Grove does when **more than one `grove` build writes the same global
skill directory**. `provisioned-skill-refresh-k9` established that a session must
read the methodology its own binary was built with — skew in *either* direction
hands it a verb the CLI lacks. The global skill dir is the one place that
invariant can be broken from outside, and nothing currently notices.

## Context

Provisioning is per **invocation**, not per loop iteration
(`launch::bare_grove` calls `provision_installed()` once, before lease
acquisition). The destination is global and shared:
`~/.claude/skills/grove`, `~/.codex/skills/grove`, `~/.pi/agent/skills/grove`.
The driver lease is per *working tree*, so it does not serialise this at all.

Two concrete ways a live loop's skill dir changes underneath it:

1. **A second grove.** Driver A (build X) provisions embed X and starts looping.
   Driver B (build Y) starts in another working tree and provisions embed Y —
   the stamp differs, so it rewrites. Every later session A launches reads
   embed Y while A's own `grove-llm` is build X.
2. **Dogfooding, which is the likely one here.** `cargo run --bin grove` from
   this repo provisions the *repo's* `content/` over the installed copy. Someone
   reading `provisioned-skill-refresh-k9` and wanting the fresh skill would
   reach for exactly that — and produce the unsafe pairing the k9 contract
   exists to forbid: current methodology, `v17.0.0` `grove-llm` on `PATH`.

Case 2 is not hypothetical for a meta-grove; it is the obvious next move for
anyone who has just read why the skill looked stale.

## Done when

The decision is recorded (ADR-shaped — it has a real trade-off) and reconciled
with `docs/ARCHITECTURE.md` §Embedded methodology and `CONTEXT.md`'s
**Embedded methodology** entry, both of which currently state the
same-build invariant without naming what can violate it.

Options to weigh, not a prescription:

- **Detect, don't defend.** A session's `grove-llm` could compare its own
  embedded content hash against the stamp in the skill dir it was told to use,
  and refuse or warn on mismatch. Cheap, and it catches both cases — but it
  needs the session to know which skill dir it read, which an opaque configured
  command does not report.
- **Re-provision per iteration.** Restores this driver's embed each launch.
  Fixes case 1 for the *winner* of a race and does nothing for case 2; two
  concurrent drivers at different builds would simply alternate. Note it is
  otherwise a no-op — a driver never re-execs, so absent a clobber every
  iteration extracts identical bytes.
- **Declare it unsupported and say so.** Concurrent groves at different `grove`
  builds share one global skill dir and cannot both be served; document the
  constraint and the `cargo run` hazard, and spend nothing on mechanism.
- **Stamp the launch.** Have the driver pass the embed hash it provisioned into
  the session environment, so a mismatch is detectable without guessing the
  directory.

## Notes

Filed by `provisioned-skill-refresh-k9`, which deliberately did not decide it:
that leaf was scoped to the refresh boundary and to narrowing the self-hosting
claims, and this is a distinct defect with more than one defensible answer.

`tests/provision.rs` already enforces the half that lives inside one build (the
embed instructs no verb its own CLI lacks). This leaf is about the half that
lives *between* builds, which no test can currently see.

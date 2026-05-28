# 020-cli-drain-as-verb-and-bootstrap-fetch

**Kind:** work

## Goal

Promote drain from launcher-prompt prose to a CLI verb so the workflow
is enforced by the CLI's contract, not by LLM discipline (per the
binding principle from leaf 060's grilling). Wire fetch-before-drain
into `grove start` / `grove continue` so a session triages the
latest-known state when a remote is configured.

## Scope

- **New CLI verb: `grove inbox drain --for=<name>`.** Encapsulates:
  - Fetch (best-effort) the `grove-meta` branch if a remote is
    configured; on fetch failure, warn loudly to stderr and continue
    with local state; on non-ff against unpushed local commits,
    refuse-and-instruct (do not auto-merge).
  - Enumerate the per-observation files in `inboxes/<name>/`
    (excluding `.gitkeep`).
  - Surface the enumerated files to the calling session as the drain
    payload (the LLM triages each — incorporate / defer / reject —
    inside the session it was launched in, not inside the CLI).
  - After the LLM signals triage completion, delete the triaged files
    in one git operation, commit with message
    `drain <name>: N incorporated, M deferred, K rejected`, push
    best-effort with one auto-retry on non-ff. Skip the push if no
    remote configured.
  - Sub-decision: where in the `meta`/`inbox` namespace does this
    verb live? Recommendation: `grove inbox drain` because drain is
    fundamentally an inbox operation and the verb shape parallels
    `grove inbox add`. Document the decision in the leaf's commit
    message.
- **Bootstrap-side wiring.** `content/prompts/start.md` and
  `content/prompts/continue.md` invoke `grove inbox drain --for=<name>`
  rather than re-explaining the drain workflow inline. The launcher
  prompts shrink correspondingly. (See user memory: "Launcher prompts
  stay small.")
- **Triage protocol.** The CLI emits the drain payload in a stable
  machine-readable form (e.g. JSON or newline-delimited paths +
  body-on-stdin) so the LLM's triage step is well-bounded. The triage
  outcome is communicated back via `grove inbox drain --commit
  --incorporated=<paths> --deferred=<paths> --rejected=<paths>` or a
  similar follow-up call — the exact shape is the leaf's decision.

## Out of scope

- Capture (covered by leaf 010, lands first).
- `grove meta remote`/`grove meta sync` (covered by leaf 030).
- Branch rename (covered by leaf 070).
- TUI integration (covered by leaf 080).

## Done when

- `grove inbox drain` exists; invoking it on an inbox directory
  surfaces the payload and (after triage signal) deletes the triaged
  files with the disposition-named commit.
- Launcher prompts (`content/prompts/start.md` and
  `content/prompts/continue.md`) call the verb, do not re-explain the
  drain workflow inline, and are shorter on net.
- Fetch-before-drain works for both the remote-configured and
  no-remote cases; the no-remote case is a no-op fetch.
- Fetch failure produces a clear stderr warning and the session
  continues. Non-ff against unpushed local commits produces a refusal
  with remediation text.
- ADR-0005 referenced from one pointer comment in the drain CLI's
  implementation site.

## Pointers

- ADR-0005: `docs/adr/0005-grove-meta-sync-semantics.md`
- Existing inbox CLI: `src/inboxes.rs`.
- Launcher prompts: `content/prompts/start.md`,
  `content/prompts/continue.md`.
- Parent BRIEF: `.grove/020-design-seed-convention/060-sync-semantics-and-inbox-shape/BRIEF.md`.
- User memory steer (referenced from MEMORY.md):
  `feedback_launcher_prompts.md` — keep `content/prompts/*.md` small.

## Notes

- The CLI's drain payload shape is a load-bearing decision: too rigid
  and the LLM can't apply judgment; too freeform and the contract is
  fictional. Lean toward a structured payload (per-observation
  metadata + body) and let the triage signal be a simple list-of-paths
  plus disposition.
- If the leaf 010 idempotency check (content-hash) is implemented
  cleanly, drain can deduplicate the payload too — but that's a
  nice-to-have, not a requirement.

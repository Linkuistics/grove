# skill-design-k5

## Goal

Decide how the always-on layer, the checker and the skill's text fit together,
so that every implementation leaf after this one builds against a written
design instead of choosing its own. Deliver one spec and the ADR set the root
brief requires.

## Context

- Read `plan-k1`'s running log in full. Its decisions are the human's: do not
  reopen one without the human. Read `harness-hooks-a.md` from
  `harness-hooks-k4`, the vendored checker from `vendored-checker-k3` and its
  running log, the `guardrail` skill (the one existing harness-specific hook),
  `docs/specs/codex-skill-provisioning.md`, `plugins/CONTEXT.md` and
  `CONTEXT-MAP.md`. The existing ADR `restatement-declares-its-class` may bear
  on how rules are restated; read it.
- The spec lands in `docs/specs/technical-communication.md`. ADRs follow the
  `decision-records` skill and its test; `docs/adr/` is flat and repo-wide, and
  `CONTEXT-MAP.md` records which context owns each record.
- Do not copy or quote the standard into the spec or any ADR. Both are public.

## Done when

The spec and ADRs are committed, and each of these has a recorded answer.

1. **Layers and layout.** What each layer is: the skill file, the always-on
   instruction and the reminder (their wording is a harness-neutral artifact),
   the shared hook scripts, the per-harness adapters. Where each one lives under
   `plugins/linkuistics/skills/technical-communication/`. How the rules are laid
   out for progressive disclosure, and a size budget for the always-on text,
   which costs context on every turn.
2. **The switch.** One mechanism every harness reads, how it is flipped, and what
   "off" does to each layer, including text already primed into a global
   instruction file that a hook cannot unprime.
3. **The reply log.** Location, one record's fields, how harness, session and
   position within a session are recorded so the experiment's questions (does
   compliance fade, do harnesses differ) can be answered, how an
   explicitly exempted output is marked, and whether it stores reply text. The
   recommendation to weigh is scores and per-rule counts only, which keeps
   private text out of a file on disk.
4. **The checker's contract.** Limits per text type (instructions and
   descriptions), the paragraph check, British spelling without a dictionary,
   whole-file mode, the justified-departure marker (its syntax and how a reason
   is carried), output formats and exit codes for the hook and for the log, and
   what counts as prose in Markdown, code comments and commit messages.
5. **The document gate and conversion.** Which files count as prose documents and
   which classes never convert (licences, vendored text, quotations, generated
   files, historical records). The fallback for a document too large to convert
   in one session. How a conversion lands as its own change before the edit that
   triggered it, under jj.
6. **Precedence in grove's own text.** What grove's "Human-facing communication"
   section says once the precedence order sits above it. `grove-communication-reword-k15`
   makes the edit; this leaf decides what it must say.
7. **Harness mapping.** From the survey: which layers each harness gets, and what
   Codex, Pi and Gemini get where they have no hook. Whether `plugins/install.sh`
   or Grove's startup provisioning touches anything beyond skill symlinks. The
   global priming sections are the human's files and are maintained outside this
   repository, and `docs/specs/codex-skill-provisioning.md` says Grove does not
   write Claude's configuration. **Ask the human before deciding whether this
   repository starts managing those files.** Items 2 and 3 also deserve a live
   check with the human.
8. **Restatement discipline.** How the rules are restated without the standard's
   text, and a procedure `publication-k16` can follow to sweep the repository
   for overlap. The standard is page images only, so the procedure cannot assume
   machine-readable text.
9. **The ADR set.** The licence posture is required: it passes the test (a
   one-way door once text is public, surprising without context, chosen over
   rejected alternatives). Decide which of the other decisions above also pass,
   and write only those.
10. **The leaves after this one.** Re-read `document-checks-k6` through
    `publication-k16` against the design. Where the design changes a leaf's
    scope, edit its body, and `leaf-insert` or `leaf-add` what is missing.

## Notes

- The leaves below build on this design for the rest of the grove, which is the
  condition `references/decompose.md` gives for a `review-design` chain. Weigh
  one at the end, and cut it lazily as the last act if you judge it needed.
- The checker's passive-voice check matches word shapes, not grammar. Treat it as
  an advisory in the design, never as a gate.

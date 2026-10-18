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

## Decisions (running log)

**The human's global instruction files stay the human's.** This repository
never writes the priming sections of `~/.codex/AGENTS.md` or
`~/.pi/agent/AGENTS.md`. The user document offers one optional line the human
may paste; that line names the off file, so the switch still reaches it. The
human chose this over a block that `install.sh` manages (outside the switch,
left behind on uninstall, in files maintained elsewhere) and over no line at
all.

**Codex: a small hooks plugin in the skill directory, trusted once by hand.** It
carries all four layers. Its entries never change shape, so the trust (a hash
of each entry) survives script updates. Until the human trusts it, Codex has the
skill and the model runs the checker itself. Chosen over per-launch `-c` flags
through Ravel (Grove launches only, unproven in a real turn) and over skill and
fallback only.

**Gemini CLI: the skill only; `gemini-always-on-k14` is abandoned with the
human's confirmation.** Gemini receives the skill through `install.sh` when it
is installed, and nothing more. The survey keeps the extension layout for a
later grove. Chosen over building and live-testing an extension for a harness
the human does not use now.

**`plugins/install.sh` also links the Pi extension, and runs no Codex
command.** It links the extension into `~/.pi/agent/extensions/`, the same kind
of link it already makes for skills, so Pi has every layer from installation.
The human adds and trusts the Codex plugin by hand, as the user document says.
Grove's startup provisioning is unchanged: it places skill links only. Chosen
over also running `codex plugin add` (trust stays manual anyway) and over no
code delivery.

**The switch: one off file; off acts at each layer's next event, with no
retraction.** The file is `~/.local/state/technical-communication/off` (under
`$XDG_STATE_HOME` when set); absent means on. Every hook reads it first on every
event. Text already in a running session stays until that session ends; new
sessions get no instruction, and the reminder, gate and log stop at once. Live
check with the human: chosen over also sending a one-line "now off" notice,
which needs per-session marker state.

**The reply log: scores and counts only, one file per session.** A record
stores counts, rates, per-rule counts and the exemption mark, and no reply text
or matched fragments. Each session has its own JSON Lines file holding start
records and reply records, so the writer counts position cheaply under a file
lock and records replies since the last start (which a compaction resets). Live
check with the human: chosen over storing flagged fragments or full text, and
over one file for every session.

**No review chain.** The human agreed the main decisions live, and the detailed
contracts meet real code first in `document-checks-k6` and `reply-scoring-k7`,
which repair the spec in place when they find a defect. Chosen with the human
over a `review-design` and `integrate-review-design` pair, to keep the plan lean.

**The ADR set is three records; the spec carries the rest.** Each was tested
against all three conditions. `technical-communication-carries-no-standard-text`
is the required licence posture. `documents-are-gated-replies-are-measured`
passes: a reply gate added later would break the log's series, the choice
surprises a reader who expects consistency, and the reply gate was rejected on
evidence. `the-off-file-is-the-only-switch` passes: moving the file silently
turns the skill on again for anyone who turned it off, a file looks odd beside
an environment variable, and three alternatives were rejected. These fail
"hard to reverse" and live in the spec with their reasons: the precedence
order, British spelling, lazy conversion and its thresholds, the log's fields,
the markers, the layout, and the boundary that this repository never writes the
human's instruction files or grants trust. The agreed diagram sources are kept
beside the design in `docs/design/technical-communication/`, after the
precedent of `docs/design/harness-selection-and-execution/`, with two Taskfile
tasks that serve and export them.

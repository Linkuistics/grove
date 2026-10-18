# vendored-checker-k3

## Goal

Put `danyuchn/asd-ste100-skill`'s checker into this repository as our own code,
under its MIT notice, with its own self-test running in `scripts/check.sh`.
Change none of its behaviour. This leaf only moves the code in; the leaves that
extend it come after the design (`document-checks-k6`, `reply-scoring-k7`).

## Context

- Upstream is `danyuchn/asd-ste100-skill` on GitHub at commit `3944283` (MIT,
  Dustin Yuchen Teng). The file is `scripts/ste-lint.py`: stdlib-only,
  self-tested, Markdown-aware. The root brief and `plan-k1` say what it checks
  and what it lacks.
- It is downloaded code, and the auto-mode classifier refused to run it in the
  requirements session. Clone it into a new, empty directory in the scratchpad.
  Read the whole script before running anything, including its self-test. Once
  it is vendored it is this repository's code.
- Destination: `plugins/linkuistics/skills/technical-communication/scripts/`.
  The licence goes in `LICENSES/<source>.LICENSE` inside the skill directory,
  copied verbatim from upstream at that commit, with an attribution comment in
  the script that names the upstream commit. `plugins/grove/skills/grove/LICENSES/`
  shows the convention.
- The skill directory has no `SKILL.md` until `skill-sentences-k8`. Check what
  `plugins/install.sh`, `plugins/install.test.sh`, `plugins/grove/conformance.sh`
  and the build-time skill discovery that embeds `plugins/` in the Grove binary
  do with a directory that has no `SKILL.md`, and keep `bash scripts/check.sh`
  green either way. If a tool needs a `SKILL.md`, say so in the running log
  rather than adding a stub.

## Done when

- The checker, the licence file and the attribution comment are in place. Apart
  from the attribution comment the script is byte-identical to upstream, so
  every later change shows up as ours.
- The self-test runs from `scripts/check.sh` and passes.
- `bash scripts/check.sh` is green.
- The running log records anything the read-through found that the later leaves
  need: how the checker takes input, its output format, its exit codes, how it
  decides what is prose, and any behaviour that surprised you.

## Notes

- Do not add tests beyond the self-test it ships with. The root brief decides
  that effectiveness is judged in the field.
- The `PROVENANCE.md` entry for this vendoring belongs to `publication-k16`.


## Decisions (running log)

**Vendored as `scripts/ste-lint.py`, same name, mode 644 as upstream.** The
script is `danyuchn/asd-ste100-skill@3944283c183eea13fe5bd902b572da3b4de8bf44`,
`scripts/ste-lint.py` (681 lines, sha256 `32263de1…886d1`). Six comment lines
follow the shebang and name the commit; removing lines 2 to 7 gives a file that
`cmp` finds identical to upstream (a one-byte change fails the same `cmp`, so
the comparison is not blind). The licence is copied verbatim to
`LICENSES/danyuchn-asd-ste100-skill.LICENSE` (`cmp` identical to upstream's
`LICENSE`; MIT, "Copyright (c) 2026 Dustin Yuchen Teng"). It is not executable,
as upstream; callers run `python3 …/ste-lint.py`.

**Read before run.** I read all 681 lines first. It only reads the named files
or stdin and writes to stdout; the self-test writes two tiny files in a
`TemporaryDirectory`. No network, subprocess, `eval` or `exec`. I ran it from
the scratchpad with `python3 -I`.

**Self-test wired into `scripts/check.sh`** as "technical-communication checker
self-test": `python3 -I plugins/linkuistics/skills/technical-communication/scripts/ste-lint.py --selftest`.
It prints `selftest OK` and exits 0.

**What the tooling does with a skill directory that has no `SKILL.md`: nothing,
and nothing breaks.**
- `plugins/install.sh` finds skills with `find … -mindepth 4 -maxdepth 4 -name SKILL.md`,
  so the directory is simply not linked anywhere yet. `install.test.sh` checks
  `harnesses:` only on directories found the same way.
- `plugins/grove/conformance.sh` reads only `plugins/grove/skills`.
- `crates/grove/build.rs` embeds **every file** under `plugins/`
  (`include_bytes!`, with the execute bits), so the script and licence reach the
  Grove binary. Only a directory with an eligible `SKILL.md` becomes a Codex
  skill. When `skill-sentences-k8` adds the `SKILL.md` it needs a
  `harnesses:` line, because `install.test.sh` fails a skill without one and
  `build.rs` treats a skill without one as not eligible for Codex.
- No stub `SKILL.md` was needed or added.

## What the later leaves need to know about the checker

**Input.** `ste-lint.py [FILE …]` reads the files in order, or stdin when none
is named. Flags: `--json`, `--baseline N` (hard findings allowed, default 0),
`--disable RULE,RULE`, `--selftest`. There is no `--enable`, no profile or
mode, and no way to name a text type. The only API besides the command line is
`lint(text, filename) -> (findings, words_total)`; the file has a hyphen in its
name, so another script must load it by path (`importlib`) rather than
`import`. Each file gets its own `lint` call, so the synonym-rotation memory
does not carry from file to file. A file that cannot be opened is a usage error.

**Output.** Text mode prints one line per finding,
`FILE:LINE:COL rule: message [match]`, then a blank line, a summary line
(`N violations (H hard, baseline B), W words, R per 100 words`) and a fixed
hedges note, all on stdout. `--json` prints one object: `violations` (list of
`{file, line, col, rule, level, match, message}`), `count`, `hard_count`,
`baseline`, `words`, `per_100_words`. `count` and `per_100_words` include the
advisory findings. The level word for a hard rule is the odd string
`"advisory-free"`; the other level is `"advisory"`. Messages for the semicolon
rule cite "Rule 8.1" of the standard (a number, not text from it; the
`publication-k16` sweep may still want to decide whether to keep it).

**Exit codes.** 0: hard findings at or under the baseline (advisories never
fail). 1: hard findings over the baseline. 2: usage error, including an
unreadable file or an unknown rule or flag. **Surprise:** a file that is not
UTF-8 raises an uncaught `UnicodeDecodeError`, so Python exits 1 with a
traceback. That is the same code as "hard findings", and a gate that reads only
the exit code cannot tell them apart. Only `OSError` is caught.

**Rules.** Hard: `semicolon`, `phrasal-verb` (five soft verbs only),
`marketing-adjective`, `nominalization` (perform/conduct/carry out + a noun in
-tion/-sion/-ment/-ance/-ence/-ysis), `long-sentence` (one `MAX_WORDS = 25`
module constant for every text), `synonym-rotation` (ten fixed verb groups, per
file: flags the later members at their first occurrence), `dangling-conjunction`
(a list item that ends with and/or). Advisory: `passive-voice`,
`present-perfect` (modal + have is protected as a hedge). Nothing checks
spelling, paragraph length, instruction versus description length, or
vocabulary. `--disable` is the only way to silence a rule; there is no
in-document pragma, so the "justified departure" mechanism is new work for
`document-checks-k6`.

**How it decides what is prose.** Line by line, regex only. Skipped: fenced code
(a line starting with three backticks or three tildes toggles), inline code
(masked with spaces, so columns hold). Not skipped, and counted as prose:
headings, front matter, link targets and URLs, emphasis markers, HTML lines (they
only end a sentence block), table cells (each cell is its own block), blockquote
text (the `>` prefix is dropped), list items. Sentences are split on
terminal punctuation with a short abbreviation list; a wrapped sentence counts
once.

**Surprises, each seen by running it.**
- A fence does not remember its own kind or length. A stray fence line of the
  other kind inside a fenced block (a "```" inside "~~~") flips the state, so
  everything after it is judged as the wrong kind (prose as code or code as prose).
- A semicolon inside a link target is flagged (`[x](https://e.com/a;b)`), and
  front-matter values (`title: Use utilize; verify`) are linted as prose.
- `**bold; text**` is flagged (emphasis is prose), `` `a; b` `` is not.
- The word count includes every token that is not in a code span or fence,
  including `---`, `#` and list markers, so `per_100_words` is a little low for
  heavily formatted files. `per_100_words` counts advisories as findings.
- `--baseline` is for hard findings only; nothing offers a score of "hard only".

**Python.** Stdlib only. It uses `parse_intermixed_args` (3.7+) and f-strings
only; `python3` here is 3.15.0.

**Not done, by design.** No tests beyond the self-test. No `PROVENANCE.md` entry
(`publication-k16`). No `SKILL.md`.

## Verification

`bash scripts/check.sh` on the final tree: 18 of 19 checks pass, including the
new "technical-communication checker self-test". The one failure is
"Ravel caller assets and provenance", and it is not from this leaf:
`scripts/ravel-policy-typecheck.sh` line 12 pins Bun 1.4.2 and this machine has
Bun 1.4.3 (`policy typecheck: Bun 1.4.2 is pinned; install it and retry`). The
script is untouched here, and running it alone fails the same way. So
`check.sh` is **not green on this machine** until Bun is pinned back to 1.4.2
or the pin moves. I did not change the pin: that is a repository decision. The
finish cycle runs `check.sh` again and will meet the same failure.

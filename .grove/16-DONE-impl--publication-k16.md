# publication-k16

## Goal

Make the grove ready for the finish session's integration and release: record
the provenance, document the skill for its user, and sweep the repository for
anything the licence posture forbids. This leaf releases nothing. Integration
and release belong to the finish cycle (`CLAUDE.md`, "Finish, integrate, and
release").

## Context

- The licence posture is in the root brief, in `plan-k1`'s log and in the ADR
  from `skill-design-k5`. The sweep procedure is in the spec
  `docs/specs/technical-communication.md`, section *Restatement discipline*. The standard is `~/Downloads/ASD-STE100_ISSUE9.pdf`, page images
  only; scratch notes stay in the scratchpad.
- `plugins/linkuistics/PROVENANCE.md` holds the sources of the skills in this
  plugin. The vendored checker needs an entry: upstream, commit `3944283`, MIT,
  and what we changed.
- `plugins/README.md` and `plugins/CONTEXT.md` list the skills and their terms.
  `docs/USAGE.md` and similar user documents may need a short account of the
  skill. Check what the repository lists skills in, by enumerating the places that
  name an existing linkuistics skill, not by searching a fixed list.

## Done when

- `PROVENANCE.md` carries the entry, and the plugin and context documents list
  the skill. `plugins/CONTEXT.md` already defines **Always-on skill** and
  **Harness entry**; name `technical-communication` as their instance, and bring
  **Symlink install** up to date with the Pi extension link.
- A user document says in plain words what the skill does, that it is on from
  installation, how to turn it off, where the reply log is and how to read it.
  It also gives the commands that add and trust the Codex plugin (from
  `codex-always-on-k12`), says that Pi gets the extension through
  `plugins/install.sh` and that Gemini CLI gets the skill only, and offers the
  optional line for a global `AGENTS.md`. That line names the off file.
- The licence sweep is done as the spec's procedure says, with positive controls
  that were seen to fail first. It finds no text, rule wording or example from the
  standard anywhere in the repository or in `.grove/`, and no use of the
  trademarked name beyond describing the skill as modelled on ASD-STE100.
- A dry run of `plugins/install.sh` shows the skill eligible on all four
  harnesses, and the always-on pieces are in the places the design says.
- The ADR set matches what was built: reconcile it as `ADR-FORMAT.md` directs.
- `bash scripts/check.sh` is green on the whole grove.

## Notes

- The user document also says where to report a failure seen in use: as an issue
  on `Linkuistics/grove` on GitHub, which is where later groves pick it up.

## Decisions (running log)

- **Where the skill is listed.** I enumerated the places that name an existing
  linkuistics skill, not a list I expected. They are `plugins/README.md` (table
  and a new always-on paragraph), `plugins/CONTEXT.md`, `PROVENANCE.md`, the two
  manifest descriptions (`.claude-plugin/marketplace.json` and
  `plugins/linkuistics/.claude-plugin/plugin.json`, plus one keyword), and the
  root `README.md` documentation list. `CHANGELOG.md` is the release's.
  `docs/USAGE.md` is Grove's own guide and does not list skills.
- **`PROVENANCE.md`** has an entry for the skill: the licence posture, and a
  table for the vendored checker (upstream, commit `3944283c…`, MIT, notice
  file, entered unchanged in `vendored-checker-k3`). It says what we changed
  (whole-document checks, reply scoring) and that only the checker came from
  upstream. It links the licence ADR.
- **`plugins/CONTEXT.md`** names `technical-communication` as the instance of
  **Always-on skill** and lists its three **Harness entries**. **Symlink
  install** now says it also links a Pi-eligible skill's `harnesses/pi/*.ts`
  into `~/.pi/agent/extensions/`.
- **The user document is `docs/TECHNICAL-COMMUNICATION.md`**, linked from the
  root `README.md` and `plugins/README.md`. It says what the skill does, what
  each harness gets, the install commands (Claude Code plugin, the three Codex
  commands and the trust step, `./plugins/install.sh` for Pi and Gemini CLI),
  the off file with its path and both commands, the optional `AGENTS.md` line
  (it names the off file), where the reply log is, three `jq` commands that I
  ran on a generated log, and where to report a failure (an issue on
  `Linkuistics/grove`). It is written under the skill's own rules: 0 hard
  findings.
- **Existing documents were not converted.** The listing edits to
  `README.md`, `plugins/README.md`, `plugins/CONTEXT.md` and `PROVENANCE.md`
  add short text only. Converting those whole files is the lazy-conversion
  procedure's job, and it is a separate change for the human to ask for.
- **The sweep surface** was `jj diff --from main --to @ --summary`, every file in
  `.grove/`, and the
  ignored files under `plugins/`, `docs/` and `.grove/` (only compiled `.pyc`
  files, which I deleted at the end because they are build leftovers).
- **Read in pairs.** I read all of Part 1 as page images beside the nine
  reference files, `SKILL.md` and the instruction. Verdict for each of the
  rules of sections 1 to 9, and for the eight general recommendations: no
  sentence, no rule title, no word list and no physical-domain example is
  reproduced. Close calls, all fixed: a sentence in `noun-clusters.md` that
  followed a standard sentence; a linking-verb list in `verbs.md` in the
  standard's order; the marking rules of a vertical list in the standard's
  order; the comma example and two sample notes in `procedures.md`; the list of
  description types; the parentheses list (regrouped from seven kinds to four);
  one hyphenated verb in `punctuation.md`; the order of the eight practices in
  `practices.md`; and three "with" examples that mapped one to one.
- **The reviewer allowance is spent.** Three leaves asked this leaf for a
  fresh-context read of the nine reference files. One reviewer read all of
  Part 1 against them and found more than I had: adapted examples (the
  standard's skeleton with software nouns), near-paraphrase of some consecutive
  sentences, and rule-title echoes in the rule tables. I classified each
  finding four ways.
  - *Valid and fixed:* the three "with" examples and their three rewrites; the
    notes examples and the notes test in `procedures.md`; the unknown-actor
    passive example in `verbs.md`; the articles examples and the pipeline list in
    `sentences.md`; the pronoun list and example, the swap-failure triple and
    the possessive sentences in `practices.md`; the short-form example in
    `noun-clusters.md`; the short-terms example in `words.md`; the caution
    example and two sentences in `safety.md`; the table rows for
    `one-action-per-step`, `notes-inform-only`, `full-words` and
    `command-first`; "a note in a procedure"; and the five-word "a comma or a
    semicolon".
  - *Visible trade-offs, left as they are:* an example that demonstrates a
    rule shares the rule's skeleton by necessity (a condition moved to the
    front, an article added, "that" added, an English word for a Latin
    abbreviation); the coverage tables cite rule numbers, which the spec
    allows; each file follows the order of one section's rules; single generic
    terms used as identifiers (`word-count`, `vertical-list`, `false-friend`,
    `no-latin-abbreviation`).
  - *Noise:* the reviewer counted the rule index in `SKILL.md` as mirroring
    structure. It lists identifiers per file, which the design requires.
- **A second review is cut: `publication-k17`**, a `review-impl` leaf. The
  rewrites are substantive and the allowance is gone, so the doubt is the tree's
  (`references/execute.md`). Its body states the doubt and my three trade-off
  calls for it to test. It names no provider evidence, because this launch has
  none (`GROVE_PROVIDER_EVIDENCE` is unset).
- **Probe sweep.** 69 phrases of eight words at most, typed from the pages in the
  scratchpad (rule titles and distinctive sentences of all nine sections), matched
  case-insensitively with any whitespace between words, over the whole surface.
  Controls: a phrase planted across two lines in mixed case in scratch copies of
  one file from `plugins/`, `docs/` and `.grove/` was found in all three. The
  same phrases with one word changed were not found. A first mutation (`item` to
  `items`) was found by the substring match, which showed the mutation was weak;
  I replaced it with a word change. I ran the controls again with the final
  tool and again after the last edit. Result on the final tree: one hit,
  "topic sentence", in the log of `skill-sentences-k8`, which reports an earlier
  sweep. It is a general writing term. The digest of the whole surface except
  this file was `a5b834d0…` before and after the final `check.sh`. The phrase list
  stays in the scratchpad.
- **Trademark sweep.** The full title, the short code with digits, and the two
  sets of initials as whole words, matching case. In the shipped tree only the
  allowed phrase remains. I fixed: the ADR (the full title twice, and the owner's
  initials several times), the spec's own list of patterns (it now describes them,
  so it holds none), one comment in `scripts/check.sh`, and a test string in the
  checker (built in two parts, with an `ours:` comment). The ADR now says
  "four routes" and names the Codex hooks plugin.
- **For the human: the finished leaves' logs name the standard.** The root brief
  and the logs of `plan-k1`, `skill-sentences-k8`, `skill-words-k9` and
  `skill-safety-punctuation-practices-k10` hold 100 or more hits of the
  trademark patterns. They record your own decisions about the licence, so I did
  not reword them. `finish-commit` removes `.grove/` from the tip, and the
  history keeps the words whatever I do. Choose before the finish: accept the
  hits as descriptive working notes, or squash the history of those files. I
  recommend accepting them, because a name in a decision log is not use of the
  standard's text. The spec's step 5 now records that a hit in a finished log
  is a finding for the human.
- **`plugins/install.sh` has no dry-run mode.** I ran it with `HOME` set to a
  scratch directory that held `.codex`, `.gemini` and `.pi/agent`, and with
  `--force` (this is a jj workspace). It linked the skill into all three skill
  directories (guardrail skipped, as designed) and linked
  `technical-communication.ts` into `.pi/agent/extensions/`. Claude Code gets
  the skill and `hooks/hooks.json` from the `linkuistics` plugin. The Codex
  plugin comes from `.agents/plugins/marketplace.json`. The always-on pieces
  are where the spec's layout puts them.
- **ADR set reconciled** as `ADR-FORMAT.md` directs: the three ADRs match the
  build, the licence ADR is corrected (trademark wording, owner wording, four
  routes), and the harness survey has a *Where the findings landed* section
  that names the two ADRs and the spec, so the research bridge runs both ways.
  No new ADR is warranted: the decisions this leaf made are procedure (the
  spec) and a pointer to the human (above).
- **`bash scripts/check.sh` is 19 of 20.** The failure is the Ravel policy
  typecheck, which pins Bun 1.4.2 while this machine has 1.4.3, as in
  `codex-always-on-k12` and `pi-always-on-k13`. It reads none of my files.
- **Open for the finish.** The `.grove/` trademark hits (above); the review
  leaf `publication-k17`; and the Codex marketplace route needs the release,
  because `codex plugin marketplace add Linkuistics/grove` reads the public
  repository.

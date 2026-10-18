# plan-k1

## Goal

Establish what the STE skill must do, in the human's own words, and settle the
decisions that shape it before any design starts.

The human's request, verbatim:

> I have downloaded the ade-ste100 issue 9 document into ~/Downloads/. I want to
> create a skill that ensures that all communications with an LLM (such as we
> are having now), and all documentation produced by an LLM, conforms to this
> standard.

## Context

- The standard is ASD-STE100 Simplified Technical English, Issue 9
  (2025-01-15), 434 pages, at `~/Downloads/ASD-STE100_ISSUE9.pdf`; the grove
  name's "ade" means "asd". The PDF permits printing and disallows text copying.
- Its copyright notice: possession grants no licence, and no reproduction or
  publication "in whole or in part" may be made without the written authority of
  an officer of ASD. Free use, reproduction and publication is granted only to
  eight enumerated categories (ASD national-association members and their
  companies, AIA/AIAC members, ICCAIA members, customers of those, member-country
  Ministries of Defence, A4A, Airworthiness Authorities, and universities and
  research institutes for educational purposes). "ASD-STE100 Simplified
  Technical English" is an EU registered trademark. The standard defines no
  conformance.
- This repository is public and Apache-2.0. A skill under `plugins/` is
  redistributed three ways: the public Claude marketplace, the snapshot each
  released Grove binary embeds for Codex provisioning, and `plugins/install.sh`.
- Prior art: Writegood (`~/Development/Writegood`) surveyed STE —
  `docs/research/prior-art.md`, "Whether STE can ship inside a compiled skill" —
  and its ADR `optimisation-loop` names an independently authored
  controlled-language skill "in the shape of ASD-STE100" as a candidate for its
  measurement loop. The MIT `danyuchn/asd-ste100-skill` re-expresses the
  principles and ships no dictionary.
- What the standard says of itself (general introduction, paraphrased): it was
  built for technical documentation only, though its rules can also help oral
  communication; it is meant to sit under a house style guide, not be used
  alone; it does not regulate formatting, abbreviations or units; its approved
  meanings and spelling follow American English (Merriam-Webster); it is free to
  every writer and user, while distributing it without the STE Maintenance
  Group's written permission is prohibited. Part 1 has nine sections: words,
  multi-word nouns, verbs, sentences, procedural writing, descriptive writing,
  safety instructions, punctuation and word count, writing practices. Rule 1.12
  admits technical verbs for computer processes.
- `danyuchn/asd-ste100-skill`, analysed at commit `3944283` (MIT, Dustin Yuchen
  Teng): an on-request rewriter for agent-facing text (tool descriptions, error
  messages, prompts) that returns only the rewritten text, with a strict mode
  and a looser mode for prose. It departs from STE by choice (keeps meaningful
  present perfect, drops vocabulary rules in prose mode, adds non-STE checks)
  and covers the sentence-level core, but is thin on technical nouns and verbs,
  connecting words, articles, procedures, warnings, punctuation beyond the
  semicolon, word counting, spelling and Section 9. It is named after the
  trademark and presents two short rule passages as quotations. Its
  `scripts/ste-lint.py` is stdlib-only, self-tested, Markdown-aware, and checks
  semicolons, long sentences (one 25-word cap), nominalisation, soft phrasal
  verbs, marketing adjectives, synonym rotation, dangling list conjunctions, and
  passive voice and compound tenses as advisories; it never flags hedges. It
  was assessed by reading only: running downloaded code was refused here.
- Always-on surfaces in the human's setup (2026-10-17). Skills reach Claude
  Code through the marketplace and Codex, Gemini and Pi through
  `plugins/install.sh`; linkuistics skills declare `harnesses: [any]` unless
  their mechanism is harness-specific (`guardrail` is `[claude-code]`). Claude
  Code has no global `CLAUDE.md`; its always-on context comes from plugins (the
  enabled `explanatory-output-style` plugin injects this session's explanatory
  mode at session start), and its user-level hooks are infrastructure only.
  Codex and Pi each have a global `AGENTS.md` with a "Skills" priming section;
  Gemini has none. Nothing in this repo's tree or history writes those
  priming sections (Codex's carries `skills-priming` markers), so they are
  maintained outside it. A skill alone loads only when a task matches it, so
  covering all output needs an always-on layer outside the skill.
- Style rules that the skill will meet: grove's own "Human-facing
  communication" section (`plugins/grove/skills/grove/SKILL.md`) asks for a
  guided, conversational, explanatory style; the explanatory output style adds
  insight blocks; this repo's prose leans British but is mixed, while STE's
  approved spelling is American.
- Vendored MIT material in this repo follows one convention:
  `LICENSES/<source>.LICENSE` inside the skill, plus an attribution comment
  naming the upstream commit (see `plugins/grove/skills/grove/LICENSES/`).

## Done when

- Every open decision below is settled with the human and recorded in the
  running log.
- The root brief states the grove's goal and done-criteria.
- The next leaf is cut, so a fresh session can continue.

## Notes

Open decisions, interdependent, grilled one at a time (foundational first):
distribution and licence; what "conforms" can mean; which outputs and harnesses
the skill reaches; how existing text converts; how it is enforced; how it ranks
against existing style rules, and with that which spelling it uses; the test
seams and how the experiment is measured; the skill's name; whether the
always-on part ships switched on.

## Decisions (running log)

**Distribution and licence — public and independently authored.** The skill is
published from this repository, in `plugins/linkuistics/skills/`, and installs
to every harness by the same routes as the other skills there. It contains no
ASD text: no rule wording, no explanatory text, no examples taken from the
standard, and no dictionary. It claims no STE conformance, and it is not named
or branded with the "ASD-STE100 Simplified Technical English" trademark;
describing it as modelled on ASD-STE100 is acceptable. The human chose this
over a private skill carrying ASD material and over waiting for ASD's written
authority. The human also accepts approximate vocabulary control — the cost of
shipping no dictionary — as minor.

**Prior art — build our own and vendor danyuchn's linter.** The skill is written
fresh rather than installed or forked from `danyuchn/asd-ste100-skill`. That
skill is an on-request rewriter for agent-facing text, follows its own profile
of STE, covers the standard only partly, and is named and quoted in ways the
licence decision above excludes. Ours is built around *composing* — rules the
model follows while it writes — and covers all nine sections of Part 1 in our
own words. Its `scripts/ste-lint.py` is vendored under its MIT notice, by the
repo's `LICENSES/` convention, and extended (separate limits for instructions
and descriptions, a paragraph check, and other gaps). Its sound ideas are
re-expressed, not copied: enforceable-without-dictionary versus
dictionary-dependent rules, a profile per text type, hedges as content, no added
facts, stopping at unambiguous rather than shortest. The human chose this over
installing it as-is and over forking it.

**Reach — all natural-language output, for any reader, on every harness.** The
skill governs every reply the model writes to the human, every document it
writes for people (docs, READMEs, ADRs, specs, changelogs, commit messages, PR
and issue text, code comments), and every text it writes for other models
(subagent prompts, skill files, instruction files such as `AGENTS.md`, grove
task files and briefs) — on every harness the shared skills reach: Claude Code,
Codex, Pi and Gemini. Outside it, because they are not prose: code, commands,
tool output, quoted material, file names and identifiers, Markdown formatting
(house style; STE governs only the words inside it), and the model's private
reasoning. The human wants the maximum benefit, framed "at least as an
experiment": maximum reach, but the experiment should be measurable and easy to
switch off.

**Existing text — lazy conversion of the whole document on touch.** When the
model edits an existing document that is not in STE, it converts the whole
document as part of that work. Untouched documents stay as they are; nothing is
bulk-converted. The human chose this over converting only new and changed text
and over converting everything now, accepting larger diffs and heavier reviews
for faster consistency. Because a touched document must then pass as a whole,
the mechanical check can lint whole files rather than changed lines. The human
accepted, without objection, three defaults that follow from it: text whose
wording is not ours to change never converts (licences and legal notices,
vendored or upstream text, quotations, generated files, historical records such
as past changelog entries); a conversion lands as its own change before the edit
that triggered it, so meaning preservation is reviewed separately; and the
design must give a fallback for documents too large to convert in one session,
such as converting only the section the edit touches.

**Enforcement — instruct everywhere, gate documents, measure replies.** An
always-on instruction on every harness directs the model to write by the skill,
and a short reminder before each prompt keeps the rules from fading in long
sessions. Every prose file the model writes or converts is checked
automatically, and the model fixes the findings before it finishes. Every reply
is checked as well and its score is logged (violations per 100 words, with
per-rule counts), but a reply is never held back or sent back for a rewrite.
One switch turns all of it off. Where a harness cannot run a check
automatically, the skill tells the model to run the checker itself; the design
verifies, harness by harness, which hook events exist. The human chose this
over instruction alone, which leaves no evidence and fades over long sessions,
and over gating replies too: in Claude Code a reply is on screen before any
check can run on it, so a reply gate only adds a corrected second reply, costs
a turn per failure, and acts on the checker's false alarms (its passive-voice
check matches word shapes, not grammar). The reply log is what makes this an
experiment: it shows whether replies improve, whether compliance fades within a
session, and whether harnesses differ. This also settles what "conforms" means
here: the model follows rules restated in our own words for all nine sections
of Part 1, the checker enforces the subset that needs no dictionary, vocabulary
is controlled only approximately, and no STE conformance is claimed.

**Precedence — the human's request, then meaning, then STE, then house style.**
When rules conflict, this order decides, highest first. (1) The human's explicit
request for a particular output wins, for that output only; the switch is for
ending the experiment, not for one-off outputs. (2) Meaning: the model never
drops information, a hedge or a caveat, and never adds a fact, to satisfy a
rule; it looks first for wording that keeps both the rule and the meaning, and
departs from the rule only when none exists. (3) STE, for words and sentences,
over any other style rule that conflicts with it on wording. (4) House style —
grove's "Human-facing communication" section, the explanatory output style,
document templates, commit conventions and project style guides — for
everything STE leaves alone: formatting, templates, what to explain,
abbreviations and units. This follows the standard's own design, which places
it under a house style guide. The order is written into the skill's always-on
text, so the model does not settle conflicts case by case. The human chose this
over putting STE above everything but the switch, and over letting house style
win. Consequences for the design: replies keep their explanatory content and
insight blocks but lose long and idiomatic sentences; grove's communication
section may be reworded so the two rules do not pull against each other; the
document checker needs a way to accept a justified departure; and the reply log
marks outputs the human explicitly exempted, so they do not skew the
measurement.

**Spelling — British.** The skill uses British spelling, a deliberate and stated
departure from the standard's American spelling, with the -ise forms grove's own
methodology files already use ("externalise", "parameterised", "summarises").
This repo's existing prose is mixed — in `plugins/`, "behaviour" appears 69
times against 18 for "behavior", while in `docs/` the American forms are as
common or more — so lazy conversion also normalises spelling.

**Name — `technical-communication`.** The skill lives at
`plugins/linkuistics/skills/technical-communication/`. The name avoids the
trademark, and the skill describes itself as modelled on ASD-STE100. No skill
of that name exists in this repo or in any installed harness's skill folders
(checked 2026-10-18).

**Testing — none built; effectiveness is judged in the field.** The human wants
to develop and publish the skill, use it for a while, and iterate on it in
future groves, logging failures as GitHub issues. No test suite, evaluation or
agreed test seams are built in this grove. The human's reason: for tools like
grove and ravel, correctness and effectiveness differ; tests capture
correctness efficiently, but effectiveness is best measured in the field over a
longer term. The vendored checker keeps the self-test it ships with, and nothing
else is tested. The reply log from the enforcement decision is the field
measurement, and GitHub issues on `Linkuistics/grove`, where the skill ships
from, carry failures to the groves that iterate on it. This grove ends when the
skill is published, which in this repo means integrated into `main` and
released.

**Default — always on.** The human is the only user of this skill, so it ships
switched on: the always-on instruction, the reminder, the document gate and the
reply log run from installation, and the switch exists only to turn them off.
The human chose this over shipping the always-on part switched off, knowing
that anyone else who installs the `linkuistics` plugin, or uses Grove with
Codex, would receive it too. The design needs no opt-in path and no per-user
settings.

With this answer the human confirmed the shared understanding summarised to
them: every decision above, with the always-on default as their one correction.

# Current and revised documentation guidance: a descriptive comparison

This directory compares two sets of Grove skills on three document cases and one design exchange.
Each case ran once per arm with the same prompt bytes. The comparison describes what the outputs
show. It makes no statistical claim, and one run per arm cannot separate the guidance from chance.

## What this directory does not hold

The case fixtures under `../cases/` are not on `main`. The exchange proxy prompt and its answer
sheet are with them. They stay in the unpublished history of the grove workspace
`grove.gh-issue-14-documentation-skills-for-editorial-pipeline`, with the rest of the evaluation
tree ([oversized evidence stays out of main](../../../adr/oversized-evidence-stays-out-of-main.md)).
To repeat a run, a reader therefore needs that workspace.

The revised core revision `72c2b41994b4` is not published either. Its content reached `main`
rebased onto release v22.13.0, in the change that carried this directory. Each stored output under
`outputs/<case>/<arm>/files/` keeps the links that resolve from its fixture's root. Those links do
not resolve here, so the repository's link test does not read those directories.

## Arms

`tools/arms.tsv` records the twelve revisions. `tools/build-arms.sh` exports each plugin with
`git archive` into a scratch directory.

| Plugin | Current arm | Revised arm |
| --- | --- | --- |
| grove (core) | `7a2a537c794c` (`main`) | `72c2b41994b4` (`documentation-guidance-spine`) |
| writing | `578f84d1f5a9` | `990a555b8276` |
| requirements | `c51ec8bebd81` | `636b36242410` |
| design | `aa088156d1a5` | `5c072b792429` |
| engineering | `8d1a2bc3e07b` | `55aff4acbd58` |
| research | `8f321f2e99e0` | `67cb6c856d93` |

The current arm is Grove `main` with its `plugins/packs.json` pins. The revised arm is the spine
bookmark with each pack at its local `main` after `proof-and-checkpoints-k651`. The engineering and
research heads differ from their pins only in `README.md`, so their skills are the same in both arms.

## Cases

All four cases reuse existing fixtures from `../cases/` without change. The other case directories
hold tool probes, so they do not fit this comparison.

| Case | Fixture | Stage | Leaf |
| --- | --- | --- | --- |
| D1 | `transfer/fixtures/B-draft` | `draft` of a three-page EditorConfig guide | `editorconfig-k2` |
| D2 | `ail/fixtures/A-copy-edit` | `copy-edit` of the AIL orientation pages | `orientation-k2` |
| D3 | `ail/fixtures/A-proof` | `proof` of the AIL orientation pages and their two figures | `orientation-k2` |
| E1 | `exchange/fixtures/E-design` | `design` of the Courier SMS retry policy, with a human exchange | `retry-policy-k2` |

The six increments come from `docs/specs/documentation-guidance.md`. The table shows which case
exercises each one.

| Increment | D1 | D2 | D3 | E1 |
| --- | --- | --- | --- | --- |
| 1. Document contract and draft | yes | | | |
| 2. Format selection and capability snapshot | weak | | weak | |
| 3. Copy-edit and house style | | yes | | |
| 4. Figure contract and art | | | yes | weak |
| 5. Proof and reader checkpoints | | | yes | |
| 6. Requirements and design exchanges | | | | yes |

"Weak" means that the case touches the increment only in part. D1 and D3 reach increment 2 through
the format and media part of the contract. No case asks a model to choose a format. E1 reaches
increment 4 only through the sketch rules for an exchange.

## Seam 1: repository checks

These checks ran on the revised arm. They show that the skills hold together. They say nothing
about the documents that a model writes with them. The logs are in `seam1/`.

| Check | Result |
| --- | --- |
| Spine `task check:skills` at `72c2b41` | Pass. 13 tests pass. |
| Spine `task check:packs` | Pass. It reads the old pack pins in `plugins/packs.json`, so it does not check the new pack heads. |
| Spine `task figures:check` | Pass. No committed figure has a `regenerate.sh` yet. |
| Requirements `task check` | Pass with its own pin and with the revised core. |
| Writing `task check` with its own Grove pin | Fail. Four FAIL lines name files that exist only in the revised core. |
| Design `task check` with its own Grove pin | Fail. Two FAIL lines name `FIGURE-FORMAT.md` and `references/documentation-toolchain.md`. |
| Writing and Design `task check` against the revised core | Pass. Every assertion holds. |

The two failures are integration order, not a guidance defect. Each pack names its Grove revision in
`GROVE_COMMIT`. That pin moves once the revised core lands on `main`, and `curated-integration-k647`
owns that step. Until then, a pack check run with its own pin reports these failures.

## Seam 2: the model comparison

Each run happened once. The differences below are what one pair of runs shows. A second pair could
differ. Each run's `meta.json` holds its turns, cost and gate result.

| Case | Arm | Turns | Cost (USD) | Seconds | Case gate |
| --- | --- | --- | --- | --- | --- |
| D1 | current | 21 | 0.91 | 208 | pass, 33 links |
| D1 | revised | 27 | 1.59 | 361 | pass, 22 links |
| D2 | current | 24 | 1.41 | 249 | pass, 262 links |
| D2 | revised | 27 | 1.65 | 201 | pass, 262 links |
| D3 | current | 30 | 1.42 | 398 | pass, 297 links |
| D3 | revised | 28 | 1.73 | 765 | pass, 299 links |
| E1 | current | 10 | 1.08 | 356 | pass, 6 links |
| E1 | revised | 4 | 2.06 | 897 | pass, 11 links |

### D1: draft

Both arms checked the guide against the EditorConfig specification source. Both marked the same gap
in the specification, the ends of a `{num1..num2}` range.

The revised reference page gives each property the same fields: values, case of values, default,
effect and source. Each entry has a stable anchor. Each example carries a label that says whether it
is complete or a fragment, and what a fragment leaves out. The current reference page is short prose
with one bare example. The revised `draft` skill adds these rules for entries and examples, so this
difference matches increment 1. The revised guide has 4843 words, and the current guide has 3018.

One result is inconclusive. The current run listed both planned figures under `## Handed forward`
for the art stage. The revised run drew no figure and handed nothing forward. The rule for handing
work forward has the same text in both arms. A guidance change therefore does not explain this
difference, and this report does not count it as a regression.

### D2: copy-edit

The current run changed "serialize" to "serialise" and "realize" to "realise". It kept "initialized"
and "uninitialized", so the pages now mix two spellings. It also added a fact that the draft did not
state, about storage that a binding reads before initialisation. It did not cut an art leaf for the
missing figures.

The revised run kept the US spelling throughout. Its reason was that the normative files use it and
that the contract sets no spelling. The revised `house-style.md` is a closed list with no spelling
point, so this behaviour matches increment 3. The revised run also made terms consistent across the
pages and removed bold emphasis that the contract forbids. It cut an art leaf for the two empty
figure slots.

### D3: proof

Both runs found the same gap. No orientation page said which kinds of API the language serves. Both
added that fact from the reader addendum of the contract.

The revised run did three further things that the revised `proof` skill and `FIGURE-FORMAT.md` ask
for:

- It compared the figure text with its PlantUML source, and it added one sentence so that the prose
  carries the fact that figure G4 shows.
- It found that the figure titles do not name the diagram type. It left that defect for a correction
  run through the art stage.
- It named the first reader checkpoint, and it said that a clean proof does not show that readers
  understand the pages. The checkpoint itself belongs after the tutorial, so the run did not hold it.

The current run made no figure check and named no checkpoint. It also reported that both SVG files
were empty. They hold 7923 and 9482 bytes. The run read the output of `wc -l` as a byte count. That
is a model error, not a guidance effect.

### E1: design exchange

The current arm needed a second attempt. `failed/e1-current-attempt-1.md` records the first one.
In both arms the turn counts include the proxy's turns.

Both sessions began from the evidence. Both found that most repeat texts paid for messages that were
still on their way, and both matched the human's "about a third" with 35 % extra segments. Both
explained delivery receipts and the receipt delay before the options used them. Both put the problem
before the proposal, and both gave the costs of the option they advised against.

The clearest difference is the restatement. The revised session asked the human to restate each
consequential decision in their own words. Its spec quotes both restatements beside the decision.
The current session asked yes-or-no questions, and neither its transcript nor its spec contains a
restatement. This difference matches increment 6.

The revised session also changed its own recommendation on new evidence. It had first recommended
an idempotency key. It then found 13 notices whose first text never got a receipt. The key would block
the retries of those notices. The session recorded this as a cost of the option that the human did
not choose.

Two results come from the harness, not the guidance:

- The current exchange ran to the ten-turn limit. From turn 3, the proxy picked the same two
  "unknown term" answers each turn, although the session had explained both terms. The stop rule
  ends an exchange only when the proxy picks no answer. Turns 3 to 10 cost about 0.19 US dollars
  in all and changed nothing.
- In turn 3 of the revised exchange, a background task woke the session after its report. The first
  runner lost that report, so the proxy saw only the follow-up question. `transcript.md` restores the
  lost message and labels the repair.

### Regressions and costs

No regression in these runs comes from a guidance rule. The report therefore cuts no correction
leaf.

The revised arm cost more in every case: 1.59 against 0.91, 1.65 against 1.41, 1.73 against 1.42,
and 2.06 against 1.08 US dollars. The revised D1 guide is also 60 % longer. The extra length comes from the
uniform reference entries. A reader may find the longer reference easier or harder to use. These
runs cannot tell which.

## What neither seam establishes

- **Human comprehension.** No human read any output. A model proxy answered the exchange, and no run
  held a reader checkpoint. Whether a reader understands the revised documents better is open. The
  sample read in `sample-reading-k11` is the first evidence on that question.
- **Chance.** Each case ran once per arm. One pair of runs cannot separate a guidance effect from the
  variation between two runs of the same model. The D1 figure handover shows such variation under one
  unchanged rule.
- **Increment 2.** No case asks a model to choose a format or to record a capability snapshot. These
  runs say little about that increment.
- **The tree machinery.** The fixtures have no Grove driver, so no run used `grove-llm`. The runs
  show what a stage writes, not how the tree verbs behave.
- **A shared harness confound.** Every run inherited `ANTHROPIC_BASE_URL` from this session. Its model
  calls therefore passed through a local context-management proxy. That proxy adds its own
  instructions and message reference tags to each request. The revised E1 spec cites two such tags,
  `m00036` and `m00043`, as the sources of the owner's words. The confound applies to both arms in the
  same way, so it does not favour one arm. It does mean that the runs are not plain model calls in
  the strict sense. The cited tags are a defect of the harness in that sample.
- **The fixtures.** A model wrote the fixtures, the contracts and the answer sheet. The cases test
  the guidance against those inputs, not against a real project.

## Samples for the human read

These six samples suit the human read in `sample-reading-k11`. Each pair shows one case in both
arms, so a reader can compare the arms on the same input. The paths are relative to `outputs/`.

| Sample | Case | Arm | File | What to look for |
| --- | --- | --- | --- | --- |
| S1 | D1 | current | `d1-draft/current/files/docs/editorconfig/guide/reference.md` | Whether a reader can use one property entry without reading the rest. |
| S2 | D1 | revised | `d1-draft/revised/files/docs/editorconfig/guide/reference.md` | The same question for the uniform entries. Is the extra length a help or a cost? |
| S3 | D2 | current | `d2-copy-edit/current/files/semantic/specification/interface-language/explanation/unsaid.md` | Mixed spelling and the added fact. |
| S4 | D2 | revised | `d2-copy-edit/revised/files/semantic/specification/interface-language/explanation/unsaid.md` | Consistent terms and the kept US spelling. |
| S5 | E1 | current | `e1-design-exchange/current/transcript.md`, turns 1 and 2 | Whether the human could state the decision back. Turns 3 to 10 are a proxy artefact. |
| S6 | E1 | revised | `e1-design-exchange/revised/transcript.md` | The restatements and the changed recommendation. |

The two specs from E1 are in `e1-design-exchange/*/files/docs/courier/specs/retry-policy.md`. A reader
who wants the deliverable rather than the conversation can read those instead of S5 and S6.

## Method and reproduction

- Each run is one `claude -p` process in a fresh copy of the case fixture. The command loads only
  the arm's six plugins with `--plugin-dir`, reads project settings only (`--setting-sources
  project`) and loads no MCP server (`--strict-mcp-config`). A probe run showed that each arm lists
  only its packs. It found no user `CLAUDE.md`, MCP server or user hook.
- The writing plugin depends on the `technical-communication-hooks` plugin, and Claude Code disables
  the writing plugin without it. Both arms load that plugin from the writing pack. Its bytes are the
  same in both arms. Claude Code's built-in skills appear in both arms.
- The prompts in `prompts/` replace the driver parts of a Grove launch prompt. The fixtures have no
  Grove driver and no version control. Each prompt therefore tells the session to read the task
  file directly. It also tells the session not to retire, commit or signal.
- The model is `claude-opus-5-5`. The exchange proxy is `claude-sonnet-5-5`.
  `../cases/exchange/proxy-prompt.txt` drives it. The proxy picks keys from `../cases/exchange/answer-sheet.json`
  and writes no words of its own. The exchange ends when the session asks the human nothing, or after
  ten turns.
- Each run stores `meta.json` (revisions, prompt digest, cost, turns, changed files and the case
  gate), `final-message.md`, the changed files under `files/` and a compressed event trace.
- A call can emit more than one result event when a background task ends after the report. The
  runner keeps them all. The verbs `remeta` and `retranscript` rebuilt the records of the runs that
  an earlier runner version had cut short. Each record marks the repair.
- A session may leave a server running, and `claude -p` then never exits. After the first E1 failure,
  the runner gained a watchdog. It stops a call that stays open 300 seconds after its last result.
  No call in the stored runs reached that limit.
- To repeat a run, build the arms with `tools/build-arms.sh`, set `COMPARISON_SCRATCH`, and run
  `tools/run.py doc CASE ARM` or `tools/run.py exchange ARM`. `tools/run-all.sh` runs all eight.

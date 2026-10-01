# dispatch-documentation-k71

**Reviews:** dispatch-documentation-k41

## Goal

Review the documentation of harness-dispatch's first release against the
requirements' acceptance cases. The root brief's last acceptance case makes
this review mandatory: "Review documentation against these acceptance cases."

## Context

`dispatch-documentation-k41` closed with three leaves, and their three commits
are what this review reads. Find each by its handle in the commit messages.
This leaf's brief chain is the root alone, so the running logs below are not
on it. Resolve each with `grove-llm resolve <handle>` and read its
*Decisions (running log)*.

- `acceptance-walk-k67` names the test or task for each acceptance clause and
  each row of the spec's `#test-seams` table. Its *What a test cannot hold*
  lists the clauses that are this review's alone. `seam-controls-k70` runs
  before this review and may have renamed or added tests since that log was
  written, so a test name the log gives is a starting point.
- `current-state-documents-k68` rewrote the spec, the three ADRs, the visual
  README and viewer text, and the runtime evidence as current state. Its log
  has each judgement.
- `usage-agreement-k69` placed the package in the architecture document,
  brought the usage documents into agreement with one another and with
  `--help`, and rewrote the CHANGELOG's Unreleased section. Its log lists
  every disagreement it found and closed.

The documents, as an owner meets them:

- `crates/harness-dispatch/README.md`, with `harness-dispatch --help` and each
  subcommand's help;
- `docs/CONFIGURATION.md#harness-dispatch` and
  `docs/USAGE.md#if-a-dispatched-launch-refuses`, with the usage guide's
  inspection and review-composition passages;
- `plugins/grove/skills/configure-grove/`, chiefly `references/dispatch.md`;
- `docs/specs/harness-selection-and-execution.md` and the three dispatch ADRs;
- `docs/ARCHITECTURE.md`'s *The harness-dispatch package*, `CONTEXT-MAP.md`,
  `docs/RELEASING.md` and the CHANGELOG's Unreleased section.

What the acceptance cases ask of the documentation, each a clause no test
holds:

- **Activation.** Can an owner go from an installation to a dispatched Grove
  session from the documents alone? The policy and the Grove command are both
  personal, nothing installs either, and the shipped examples are inactive
  until imported. `usage-agreement-k69` ran the activation as written and
  changed the instructions to a copy command. Check that it works as now
  written, for the static starter and for the Grove review example.
- **Both inspection surfaces.** `grove config show` explains the wrapper and
  `harness-dispatch inspect` explains the selection. No document may promise
  that evaluating trusted TypeScript is free of side effects.
- **Launch-time validation.** Grove's pre-authoring check stops at the
  configured command, for static and computed policy alike. Task authoring can
  succeed and the launch then refuse, leaving the leaf live.
- **Which configuration owns selection, and the remedy for an incomplete
  mapping.**
- **The `**Creator:**` conventions and their remedies.** The two forms, who
  writes or removes each, the declaration, why a wrong but existing run passes
  launch, and where inspection shows it.
- **Later outcome entry.** A documented way to write an observation against a
  run, with absent measurements never read as zero.
- **The stated floors.** glibc 2.17 and the CPU floor are executed. The kernel
  range is Bun's, stated as documented and not executed.
- **Agreement.** A document that states a flag, a default, a bound or an exit
  code agrees with `--help` and with the spec's tables.

Attack these first. Each is a judgement a producer made alone.

- `CONTEXT-MAP.md` says harness-dispatch comes closest of any crate to a fourth
  bounded context and is not declared one, because it has no glossary of its
  own. By the map's own test, a language boundary, it may be one today. A
  finding that it should be declared, which means moving glossary entries, is
  the human's to settle.
- The architecture section states the boundary as three sides and says no
  Rust source Grove ships names the command. Check each side against the
  manifests and the source, not against the section.
- The CHANGELOG now describes the release as a whole. It must claim no more
  than `acceptance-walk-k67` found an instrument for, and it dropped the
  emulator details the release procedure carries.
- The spec's `#grove-integration` now says which Grove-side document explains
  what. `usage-agreement-k69` changed that sentence instead of adding
  activation and the `GROVE_SIGNAL_FILE` warning to the usage guide.
- `references/dispatch.md` says the Grove review example compares one
  provider, the original creator's, and that a `select` policy may fall back
  to its own table. Check both against the spec and the examples' source.

One subject is not documentation of the command. The root brief records, under
`creator-reference-k60`, that k60 rewrote the last paragraph of *Naming your
run on what you finish* in the plugin's `references/retire.md`, and that
nothing has reviewed the rewrite. Read that paragraph, and the node-close step
that carries it, for three cases: a leaf that cuts its own review, a node
close, and a multi-level close. `usage-agreement-k69` was the first two at
once. It cut this review, wrote the line above, and then retired, which closed
`dispatch-documentation-k41`. It had no `HARNESS_DISPATCH_RUN_ID`, so this
leaf carries no `**Creator:**` line. Its log says what it did at that step.

## Done when

- Each clause above has a finding or a stated reason there is none, with the
  document and line it rests on.
- The activation instructions were run as written, with a fake harness, and
  not only read.
- The `retire.md` paragraph has been read for the three cases.
- A review with findings worth acting on cuts its integration as its last act.

## Notes

## Decisions (running log)

**Scope and evidence.** This is an inspection-only review of the three producer
changes: `2ced9b446f45` (`acceptance-walk-k67`), `9b0837d1383b`
(`current-state-documents-k68`) and `4ddf0c130134` (`usage-agreement-k69`).
The current source also includes `seam-controls-k70`, `b28c2f94a53e`, and the
two intervening fixture repairs. No test, build, lint or format command runs
in this review. The task expressly requires executing the activation recipes;
that owner workflow is exercised with existing binaries and fake harnesses.

**The graph is older than this package.** Tier 2, project
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`,
generation `2026-09-29T11:18:53Z`, fast mode. Both bounded harness-dispatch
symbol searches returned zero with `has_more: false`. Coverage marks the
package paths `not_tracked`, the changed root artifacts `metadata_changed`,
and `docs/` and CHANGELOG excluded. None of those empty searches is evidence
of absence. Source and document claims therefore use direct reads and bounded
literal searches. Coverage was checked for the documents, manifests, examples,
adapter, SDK, task files and retirement procedure.

**Activation was executed.** In an isolated HOME and newly initialized jj
repository, the first KDL block under `docs/CONFIGURATION.md`'s *Activating it*
was copied without changes, and its shell copy block was executed verbatim.
A local `brew` stand-in supplied the existing checkout's `target` prefix; no
installation or build was performed. The installed example and worker bytes
were used. Before supplying the placeholder wrapper, static inspection refused
`program_not_found`, exit 127; with the two named fake wrappers, it selected
`lead-high`. `grove config show --kind impl --json` displayed all four slots,
and bare Grove launched the fake producer with its prompt, cwd, completion
channel and run identity. Activating the documented `grove-review` re-export
then inspected and launched `review-high` (anthropic), looking up that
producer's recorded openai origin. A copied `grove-review.ts` also inspected
successfully. The worker reported build
`721aab0848f66623c650abe7fc6372b97e6e6f70cd6367c594b2885a954e9ff9`, Bun
1.4.2, package 21.12.0. The fake sessions deliberately exited 42 without
signalling, to stop each isolated loop after its one launch.

The isolated evidence is
`/var/folders/z1/nw_wk3gj3hqchs4_73xrf2dw0000gn/T/dispatch-documentation-k71-0luka9tv/results.json`
and `harness.jsonl`; these are disposable local evidence, not durable project
artifacts. The summaries here carry what the integration needs.

**A dispatch refusal is not Grove's process exit.** Removing the isolated
review's creator line made direct `harness-dispatch run` exit 3 with
`policy_refused` / `creator_line_missing`. Bare Grove printed that child's
exit status 3 and left the review live, but itself returned 0. The usage
guide's refusal paragraph says otherwise; this is an actionable documentation
finding, not a request to change Grove's exit behavior.

**The copied review starter still owns no local catalog.** A second activation
exercise edited the static copy to use `owner-lead`, `owner-reviewer` and
owner model labels. Inspection accepted that copy. Switching to the documented
Grove review re-export, or copying `grove-review.ts` without changing its
imports, then refused with `program_not_found`, exit 127, naming
`my-claude-wrapper` again. `grove-review.ts:77` imports its catalog and routes
from the installed `grove-static` example; lines 106 and 135–136 keep them.
The review copy's local bytes stay fixed, but those selection values do not.
The own-catalog selector recipe exists in the README; the operator activation
recipe does not connect it to the previously edited starter. This earns F1.

**The embedding is a source dependency, not review enforcement.** The worker
entry imports the adapter at `worker/src/main.ts:45` and the Grove review
example at line 47, then registers them at lines 117 and 120. The adapter
callback reports explicit policy use lazily; an ordinary static inspection
correctly reported `adapter: null`. So the docs' unconditional statement that
the core never imports the adapter is false as a source-boundary claim, while
the intended behavioral separation holds. F3 asks for accurate scoping of the
documentation, not removal of the delivered embedding.

**The remaining architecture sides hold within the inspected scope.** The
package manifest has only external normal/build/dev dependencies, with bundled
SQLite and no Grove package. The root manifest's workspace inheritance is
visible and the architecture names its shared release version. A literal search
of every `.rs` file under the seven other shipped crates' `src/` directories
found no `harness.dispatch|HARNESS_DISPATCH` reference. The same flags with
`keyed.launch` found 111 rows in those source directories, and the original
pattern found 152 rows in the dispatch source and Grove tests. The deliberately
changed pattern is dirty on the source, so the clean command-name reading is
not credited solely for exiting successfully. This verifies the stated Rust
source fact; it does not claim the bundled methodology contains no reference.

## Findings

### F1 — [P2] Give the review activation a path to the owner's edited catalog

`docs/CONFIGURATION.md:621–625` switches the owner who just copied and edited
the static starter to a re-export of the installed Grove review example.
`plugins/grove/skills/configure-grove/references/dispatch.md:59–66` recommends
a copy as the way to keep selection independent of installation upgrades, and
then offers a copy of the review example in the same procedure. That copy
imports `harness-dispatch/examples/grove-static` at
`crates/harness-dispatch/worker/examples/grove-review.ts:77`; its catalog and
non-review routes are aliases of that installed module (lines 106, 135–136).

Consequently, adopting the provider rule through either shown path restores
the installation's placeholder wrappers/models instead of using the edited
static catalog. With only the owner's wrappers available, the activation
refuses at exit 127. Copying the review file alone also leaves its candidate
and non-review route choices dependent on future installation changes. The
default-wrapper exercise passed; the owner-wrapper exercise above did not.
The README's own-catalog `groveReviewSelector` recipe at
`crates/harness-dispatch/README.md:865–887` makes the behavior possible, but it
is not linked into the Grove activation instructions.

Document a complete way to retain/customize the catalog when activating the
review rule, and qualify what copying only `grove-review.ts` freezes. A local
static module plus an explicit relative import, or the reusable-selector
recipe over the owner's catalog, can serve that account. This is a guidance
repair; the delivered example's default behavior is internally consistent.

### F2 — [P2] Distinguish the refused child's status from Grove's process exit

`docs/USAGE.md:459–460` says Grove stops the loop "with harness-dispatch's
exit status". In the executed missing-creator case, dispatch returned 3, Grove
reported that child's status 3, and Grove itself returned 0 with the leaf still
live. The same guide explicitly gives that contract at `docs/USAGE.md:521–522`.
An owner using the refusal paragraph to interpret a shell conditional or a
job runner would expect a nonzero Grove result that does not occur.

Say Grove reports the child's status and stops, while its own unsignalled-loop
exit remains 0. Keep the displayed refusal transcript and existing driver
behavior; the discrepancy is the paragraph's attribution of the exit status.

### F3 — [P3] Scope "the core never imports it" to the actual generic boundary

`docs/ARCHITECTURE.md:565–568` and
`docs/specs/harness-selection-and-execution.md:60–62` justify the adapter's
extraction boundary with the assertion that the core never imports it.
`CONTEXT.md:948–952` repeats it. The policy host is part of the command's core
and statically imports and registers the adapter and Grove review example in
`crates/harness-dispatch/worker/src/main.ts:45–47,117–120`. Ordinary dispatch
therefore depends on their presence in the compiled host even when no policy
uses their task parser. The import/reporting distinction is explicit in that
source at lines 125–132.

State the boundary that exists: the Rust front has no Grove relationship
semantics, the worker embeds/registers these optional policy modules, and a
policy explicitly opts into calling the adapter. Moving the adapter to a
different source package also needs its host/build registration reconciled.
The docs should not use lack of a source import to prove extraction, because
that lack is not true. No same-provider review or ambient-policy execution was
observed or inferred from this documentation error.

### F4 — [P3] Name the lines the adapter actually reads in the release summary

`CHANGELOG.md:162–164` says the dispatch policy's adapter reads the
`**Reviews:**` and `**Integrates:**` lines. The adapter's two marker constants
are `**Reviews:**` and `**Creator:**`
(`crates/harness-dispatch/worker/grove/index.ts:80–81`), and `markerLines` at
lines 229–237 collects only those. The README's grammar at
`crates/harness-dispatch/README.md:783–802` and the spec's supplied-policy
section agree with that parser. An integration task's `**Integrates:**` line
does not opt it into the review policy.

Retain the statement that Grove does not read relationship lines, but correct
the explanation so that it names the adapter's actual two inputs. The release
summary currently attributes implemented behavior to the wrong marker.

## Documentation clauses and challenged judgements

| Clause | Finding, or why none |
|---|---|
| Personal activation, examples inactive until selected | Both default starter paths were executed successfully against fake wrappers, from the exact KDL and static-copy recipes in `docs/CONFIGURATION.md:580–625`. F1 covers moving from the edited static catalog to review activation and the review copy's upgrade dependence. `docs/CONFIGURATION.md:617–619` correctly says installation writes neither personal file. |
| Both inspection surfaces and effects | `docs/CONFIGURATION.md:688–701` assigns wrapper inspection to Grove and selection inspection to dispatch, expressly allowing TypeScript effects. `crates/harness-dispatch/README.md:1120–1127` calls inspection a proposal and allows effects. The captured help agrees. `docs/USAGE.md:172–190` does not promise effect-free policy evaluation and links the fuller account. No finding. |
| Pre-authoring versus launch validation, static and computed | `docs/CONFIGURATION.md:705–714` explicitly covers both policy forms, successful authoring and later refusal. `plugins/grove/skills/configure-grove/references/dispatch.md:91–101` says the same. The refused isolated review remained live. F2 concerns Grove's reported versus actual exit, not that boundary. |
| Selection owner, incomplete mapping and fallback | `docs/CONFIGURATION.md:565–577,776–793` identifies both personal sources and the refusal remedy. `plugins/grove/skills/configure-grove/references/dispatch.md:117–121` permits owner computation to consult its own declared table; `docs/specs/harness-selection-and-execution.md:118–125` expressly permits that `select` composition, and `--choice` still forbids a different ID. No fallback is invented by dispatch. The language is consistent with the rejection of automatic fallback. No finding beyond F1's activation guidance. |
| Creator forms, writers, removal, declaration and wrong existing runs | `docs/CONFIGURATION.md:719–760` states both forms, retirement/node-close ownership, deletion of stale lines, owner declaration after finish, shared store and the attestation limitation. `crates/harness-dispatch/README.md:740–856` supplies the grammar, refusals and inspection rows. The adapter and selector source implement those distinctions; missing-creator refusal was observed. No finding. |
| Later outcome entry and unknown values | `crates/harness-dispatch/README.md:1431–1518` gives the import command, a complete document, the supported fields, corrections and unobserved defaults. `record observe --help` provides the same usable example and states the assertion boundary. k67's observation instruments, strengthened by k70, are recorded evidence rather than rerun tests. No finding. |
| Floors and evidence limits | `crates/harness-dispatch/README.md:65–80`, `docs/specs/harness-selection-and-execution.md:983–1006` and `docs/RELEASING.md:249–286` consistently distinguish executed glibc/CPU floors from the documented kernel range and require rechecking Bun on upgrade. k67 records all three targets and firing controls on this unchanged worker build. The dated runtime-evidence CPU run on Docker 6.10.14 remains a dated measurement, beside the later Docker 7.0.14 run; it is not evidence of an old-kernel test. No finding and no new target/floor claim from this review. |
| Flags, defaults, bounds, exits and current state | The actual `--help`, `inspect`, `run`, `record`, `record show` and `record observe` help were read. Their flags/ranges agree with `crates/harness-dispatch/README.md:904–953,1520–1598` and the spec's input, bound and exit tables (`:70–95,484–495,924–944`). F2 is the Grove exit attribution. The two source changes in k68 are comments only, verified by their diff. No other disagreement found in these surfaces. |
| Architecture's three sides | The manifests and direct source survey support the external-dependency and opaque-Grove-command sides (`docs/ARCHITECTURE.md:518–536`). The generic relationship-parser boundary holds behaviorally, as the source keeps calls to the adapter inside policy composition. F3 corrects its stronger source-import claim. |
| Whole-release CHANGELOG | Its claims have the instruments k67 identifies, including the delivered examples, records, cancellation, three targets and both process seams; k70 changes test evidence rather than shipped behavior. Dropping emulator details is appropriate because `docs/RELEASING.md:259–280` retains them. F4 is the marker-name error. No claim of measured kernel support, acceptance from exit zero, calibrated routing quality or accumulated contributor exclusion appears in the summary. |
| One provider versus contributors | `plugins/grove/skills/configure-grove/references/dispatch.md:107–115` accurately limits the delivered example to the original creator while leaving multi-contributor checking with the operator. `reviewSelector` compares one looked-up/declared origin. This matches `docs/specs/harness-selection-and-execution.md:630–635` and the accepted first-release simplification. No finding. |
| Division between usage and configuration | The spec's account at `docs/specs/harness-selection-and-execution.md:905–914` matches the documents: usage carries inspection/refusal and links configuration for activation; configuration and configure-grove carry the completion-authority warning. Keeping schema-level configuration out of the usage guide does not hide the warning from the activation path. No finding. |

**The context-map decision needs human settlement if it is to change.**
`CONTEXT-MAP.md:3–8` defines a context by a language boundary, and
`:93–105` acknowledges that dispatch has a separate language while withholding
declaration because it lacks its own glossary. Its collision table at
`:215–224` is evidence that the language boundary already exists. Glossary
location is an organizational choice, not evidence that the boundary does not
exist. My recommendation is to recognize the language boundary and then decide
where its glossary/record ownership should live; alternatively, document the
current ownership as an explicit organizational exception rather than a new
glossary prerequisite. This is a consistency judgement with a glossary-move
trade-off, not an implementation defect. The producer brief reserves that
choice to the human. This review does not declare a context, move entries or
silently turn either outcome into an integration obligation.

**The revised retirement paragraph works for all three cases.**
`plugins/grove/skills/grove/references/retire.md:46–52` requires cutting the
review and writing its relationship before retirement, and distinguishes the
last act of work from the later bookkeeping. For a leaf's own review, that
creates the review before the leaf's creator-line step. For a node review,
the review is cut outside the closing node, so node-close step 4 at
`:127–135` names the closing session's run without reopening it. In a
multi-level close, the same step applies to each node that actually closes;
a review inserted inside an ancestor leaves that ancestor live, so the cascade
stops there and never falsely names it as closed. The instruction requires no
review for a node that is still open. k69's review was cut at the root before
retirement, named k41, and had no creator line because its session had no run.
That is the stated direct-harness behavior, not a defect. No finding against
the rewritten paragraph.

**Handoff.** F1–F4 warrant an `integrate-review-impl` sibling. It reads this
review by handle and triages these findings; it owns any fixes and all
post-fix verification. The context-map judgement stays explicitly the human's
choice.

**Review completion check.** The task's documentation clauses each have a
disposition above, both activation paths were executed with fake harnesses,
the customized-catalog failure and the refused child's/Grove's distinct exit
statuses were read from their per-case output, and the retirement paragraph
was checked for all three cases. Finding citations were checked against the
current numbered source and corrected before handoff. `jj st` and the diff
show only this task file changed. No production/test edits and no suite runs
belong to this review. No later sibling currently follows this root leaf, so
its integration is appended and will be the next live work. Retirement cannot
close the root while that integration remains live.

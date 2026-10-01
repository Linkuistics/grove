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

# choice-and-refusals-k15

## Goal

Complete the static skeleton. An explicit `--choice` names one configured
candidate. Every refusal is actionable: a stable code, the stage, the input and a
remedy, plus the equivalent `inspect` invocation. The supplied static starter
examples ship as embedded package specifiers an owner can import.

## Context

The contract is in the spec's `#policy-and-choice` (explicit choice, static
examples) and `#diagnostics` sections. How `select` accepts or refuses a choice,
and `explicit_choice_mismatch`, are `computed-policy-k20`'s; refuse `select` as
before.

## Done when

- With `--choice ID`, a `routes` policy accepts any configured candidate,
  including one for a kind its table does not route. Inspection says the
  explicit choice selected it, not a route. An unknown ID refuses without
  selecting anything else.
- Every refusal before handoff carries a stable code, a stage, a message, the
  relevant input or source, and a remedy. With `--json`, a failure prints one
  JSON error on stderr and no partial stdout object. Text mode prefixes
  captured policy diagnostics on stderr.
- A refused `run` also reports the equivalent `inspect` invocation, without the
  prompt: a command line in text mode and an argv array in JSON.
- The exit results match the spec's table for the stages that exist: 2
  malformed CLI, 3 refusal, 4 required-record failure, 5 worker or protocol
  failure, 124 timeout, 126 and 127.
- Help carries independent-use and refusal-recovery examples. It has no pager,
  no interactive confirmation and no retry.
- The static starter examples give exact kind mappings. They explain effort by
  abstraction, uncertainty, consequences, downstream repair, reversibility and
  available checks, not as model rankings. Each is registered as a
  `harness-dispatch/examples/…` specifier with type declarations and a readable
  source. A test imports each from a temporary personal policy and selects
  through it.
- The package's usage documentation covers standalone inspect and run, policy
  authority, the static form, explicit choice, the required run record and
  `record show`. It states that `inspect` is a
  proposal: it is not a launch reservation, and evaluating trusted TypeScript
  is not promised to be free of side effects. The spec's notice states what is
  delivered.
- This node's `Done when` holds. Retiring this leaf closes the node. As this
  leaf's last act, cut the node's `review-impl` as the node's sibling,
  directly after it, never inside it. Run `grove-llm leaf-insert --kind
  review-impl dispatch-delivery-k16 static-dispatch`, targeting the first root
  entry after this node (today `dispatch-delivery-k16`). Give it
  `**Reviews:** static-dispatch-k12`, and write into its body the doubts the
  node brief names.

## Decisions (running log)

**An explicit choice under routes is looked up in the catalog, and the table is
never consulted.** `policy::select` takes the choice first. A known ID selects
with `selectedBy: "explicit_choice"`, whatever the kind or its route. The
reason says that the routes were not consulted. Inspection, the handoff notice
and the launch document carry `explicitChoice`, and the run record's
`selection.explicitChoice` is now filled for new runs. The worker's request
carries `explicitChoice` too, as the spec's request fields list it. A `routes`
policy never sees the request, but `computed-selection-k21` will hand it on.

**An unknown ID is `unknown_choice`, stage `selection`, exit 3.** The spec names
no code for it. It is a policy refusal rather than malformed input, because only
the catalog can say whether an ID exists. The remedy lists every configured ID.
`--choice` itself must be nonempty UTF-8, or it refuses as `malformed_input`,
exit 2. It does not take hyphen-leading values, like `--kind`; `--choice=-x`
reaches such an ID.

**A refused `run` names the equivalent `inspect` as argv plus cwd.** JSON puts
`error.inspect = {cwd, argv}`. Text adds `inspect: (cd CWD && ARGV…)`, one
POSIX command line in a subshell, so pasting it leaves the reader's directory
alone. The cwd is included because relative inputs, PATH resolution and the
request's `cwd` all depend on it, so argv alone would not reproduce the
selection. argv[0] is the process's own argv[0] as the caller spelled it, so the
reproduction reaches the same installation. Inputs appear in a fixed order.
A value that starts with a hyphen is written `--flag=value`, so it cannot read
as a flag. `--json` is carried over when the refusal was JSON. The prompt is
never included. Every failure `run` returns after its command line parses
carries it, including a record failure and an exec failure. A command line that
clap cannot parse has no parsed inputs, and so has no equivalent. `inspect`'s
own refusals carry none.

**Every refusal names an input or a source.** The gaps were `cwd_unavailable`
(now input `cwd`), the `worker_missing` branch where the front's own path is
unknown (now the layout as source), and two clap classes: an unknown
subcommand (clap's `InvalidSubcommand` context) and invalid UTF-8, for which
clap names nothing. The first non-UTF-8 argument is found, and the flag it is
the value of is named.

**Text-mode parse failures render as refusals.** Before, clap printed its own
error without a code or stage. Now both modes carry `malformed_input`, stage
`cli`, clap's message and the argument. The remedy keeps clap's `tip:` lines and
usage line, and points at the reached subcommand's `--help`. Help, version and
the help clap shows for a missing subcommand are not refusals and print as
before.

**Two static starter examples: `harness-dispatch/examples/static` and
`harness-dispatch/examples/grove-static`.** The first serves an independent
caller with its own kinds (`question`, `bugfix`, `feature`, `migration`,
`architecture`) over one harness at four efforts. The second maps all 23 of
Grove's session kinds exactly. Its lead and review roles follow Grove's own
modular configuration example, and its per-kind efforts follow the research's
starting mapping (`docs/research/grove-model-effort-routing.md`). Models are not
chosen per kind, so neither example ranks models. Each exports `catalog`,
`routes`, a `CandidateId` type and `policy`. An owner can then export the
policy whole, or put the routes over a catalog of their own. The programs are
illustrative wrappers, named as Grove's examples name theirs. A static table
cannot see a review's creator, and the Grove example says so rather than
implying a provider rule.

**Examples import `harness-dispatch/sdk` as owner policy does.** The readable
source is then a copy-and-edit starting point. The bundler resolves that
specifier through `paths` in `worker/tsconfig.json` to the SDK module that
`main.ts` imports. Bun's executables documentation for 1.4.2 says the bundler
already reads tsconfig when compiling, and the no-autoload switch governs the
compiled executable at run time
(https://github.com/oven-sh/bun/blob/bun-v1.4.2/docs/bundler/executables.mdx).
The resolver source finds the nearest tsconfig by directory. The build confirms
it: 5 bundled modules, one SDK. Declarations come from
`tsconfig.declarations.json`, renamed from `tsconfig.sdk.json`, now rooted at
`worker/`. It emits `sdk/*.d.ts` and `examples/*.d.ts` beside the worker, and
the readable sources are copied beside them. `build.rs` and `dispatch.sh` both
add `worker/examples` to the digested source set.

**`task dispatch:install` had been failing its own final check since
`harness-exec-k14`, and is repaired here.** Its throwaway install-check policy
had `args: []`. Once `prompt` became required exactly once, that policy refused
as `policy_invalid`, so the task exited 3 after installing. `task check` never
runs the install, so nothing caught it. The node's `Done when` includes
Taskfile tasks that install the pair, and this leaf closes the node, so the
policy now passes `{ slot: "prompt" }`. A scratch-prefix install now completes
and lists `bin/harness-dispatch`, the worker, `sdk/index.{d.ts,ts}` and
`examples/{static,grove-static}.{d.ts,ts}`. The installed front, run from
another directory, selected `deliberate` for `migration` through
`harness-dispatch/examples/static`.

**The repository check caught a test name, and the constant was renamed.**
`grove-llm`'s `removed_surface` sweep classifies every `GROVE_*` name that
starts at an identifier boundary as a launch surface. My test constant
`GROVE_ROUTES` is test data, so it is now `SESSION_KIND_ROUTES`. It was not
added to the sweep's roles.

**The non-UTF-8 search applies only to clap's `InvalidUtf8` error.** With two
non-UTF-8 arguments, where one is the value of a flag that accepts any bytes,
the first is named even if clap refused the other. That is documented at the
function, and it is an edge case of an edge case.

**No in-session reviewer.** The node's scheduled `review-impl`, cut by this
leaf, carries the doubts, so this producer's review is already scheduled.

**Controls seen to fire.** Each was a mutation of the sources, run against the
tests named and then restored. The digests of `src/*.rs` and `tests/*.rs` were
compared with those saved before the first mutation, and matched.

- A program refusal without its source: `every_exit_result_matches_the_spec_table`
  failed in the shared helper, with "refusal names neither its input nor its
  source".
- An unknown choice falling back to the route:
  `an_unknown_choice_refuses_without_selecting_anything_else` failed.
- The choice ignored: four of the five choice tests failed.
- No equivalent invocation attached: five of the eight refusal tests failed.
- Shell words never quoted: both unit tests failed. Run alone, the seam's
  `a_refused_run_names_the_equivalent_inspect_invocation_which_reproduces_the_selection`
  also failed, because the pasted line no longer reproduced the task identity.
- The prompt file carried into the invocation:
  `a_prompt_file_is_never_part_of_the_equivalent_invocation` failed.
- A refusal retried once: `a_refusal_is_evaluated_once_and_never_retried`
  failed.
- The non-UTF-8 input left unnamed, in the first form and then again after the
  `InvalidUtf8` restriction: `a_command_line_that_does_not_parse_refuses_in_both_modes`
  failed both times.
- The `grove-static` registration removed from the worker and the worker
  rebuilt: `the_grove_example_routes_every_grove_session_kind_exactly` and
  `an_owner_catalog_can_take_an_examples_routes` failed. With `main.ts`
  restored, the rebuilt worker had the same build ID as before.
- The type-check fixture's `@ts-expect-error` on an unknown candidate ID is its
  own negative control. `tsc` fails an unused expectation.

**Done-when instruments.** These are command-seam tests in
`crates/harness-dispatch/tests/`:

- Explicit choice (`choice.rs`): `an_explicit_choice_selects_a_candidate_for_a_kind_the_routes_do_not_name`,
  `an_explicit_choice_is_taken_over_the_kinds_own_route`,
  `an_unknown_choice_refuses_without_selecting_anything_else`,
  `a_malformed_choice_refuses_before_any_policy_runs` and
  `run_launches_the_explicit_choice_and_records_it`. The request field is
  covered in `worker.rs`, by `the_request_carries_the_task_inputs_and_explicit_choice_and_never_the_prompt`.
- The refusal contract: `support::Run::refusal` requires a code, stage,
  message, remedy and an input or source for every JSON refusal any test reads.
  `refusals.rs` adds `text_mode_prefixes_the_policy_output_before_the_refusal`
  and `a_command_line_that_does_not_parse_refuses_in_both_modes`.
- The equivalent invocation: `a_refused_run_names_the_equivalent_inspect_invocation_which_reproduces_the_selection`,
  which executes both forms, plus `only_a_refused_run_names_an_inspect_invocation`
  and `a_prompt_file_is_never_part_of_the_equivalent_invocation`.
- Exit results: `every_exit_result_matches_the_spec_table`, covering 2, 3, 4,
  5, 124, 126 and 127, with the launch control.
- Help: `help_carries_independent_use_and_refusal_recovery_examples` and
  `a_refusal_is_evaluated_once_and_never_retried`.
- The examples (`examples.rs`): `the_grove_example_routes_every_grove_session_kind_exactly`,
  `the_generic_example_routes_its_own_kinds_without_grove`,
  `an_owner_catalog_can_take_an_examples_routes`,
  `only_registered_example_specifiers_resolve` and
  `each_example_ships_declarations_and_a_readable_source_beside_the_worker`.
  The fixture `worker/typecheck/examples-policy.ts` is also type-checked by
  `task dispatch:typecheck`.
- Usage documentation: `crates/harness-dispatch/README.md` covers starter
  examples, explicit choice, inspection as a proposal, and the refusals with
  the equivalent invocation. The spec's notice, `#policy-and-choice`,
  `#diagnostics` and `#delivery` are updated.

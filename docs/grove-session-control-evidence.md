# Grove session control acceptance

`grove-session-control-acceptance-k61` closes `grove-session-control-k8` over
k59's lifecycle and k60's standalone implementation. Ravel remains the
independent published 0.1.0 pair; Grove owns both instructions and the meanings
of `grove-continue` and `grove-force-stop`. The tested implementation parent is
k60's `d9d9ff1d`; the acceptance changes are in the handle-named jj successor.

## Contract and witnesses

| k8 criterion | Accepted evidence |
|---|---|
| Continue, final completion and resumable force-stop | Real installed Ravel/shell-agent lifecycle tests advance to remaining work, stop explicitly without another launch, resume the live leaf and consume this launch's teardown. `force_stop_leaves_remaining_work_resumable_after_continuation` exercises resumption and descendant cleanup; `launch_directory_teardown_finishes_after_the_ravel_result` exercises the genuine teardown/force-stop combination. |
| Lifecycle precedence | After reap and terminal restoration, epoch invalidation gates interpretation. Driver interruption, including pending post-reap signals, wins; successful status with exact force-stop then exits 1 without completion; teardown otherwise wins over result/status/group; surviving group prevents continuation; successful exact continue alone relaunches. `force_stop_precedence_requires_a_successful_uninterrupted_supervisor`, `a_post_reap_interrupt_overrides_teardown_and_force_stop` and epoch-handoff tests cover the boundaries. Force-stop does not undo an already completed teardown. |
| Standalone completion/publication | `prompt_commands_complete_or_force_stop_without_touching_enclosing_channels` executes both commands from the actual prompt through installed Ravel. Continue publishes a checked whole output set; force-stop records failure and publishes none, even with valid outputs. Nested completion, force-stop and cancellation preserve enclosing channels; descendant cleanup is asserted. |
| Ordinary exit, refusal, cancellation and invalid results | Lifecycle/standalone suites retain natural exit, nonzero managed exit after acknowledged continuation, policy refusal/remedy, cancellation and absent/unsupported results. Exact-byte reader tests reject malformed bytes, newline/tails and missing capture; failed supervisors cannot give force-stop authority. These are public stdout results, not a private Ravel report protocol. |
| Stale epochs, nested authority, terminal recovery and reaping | Driver tests retain activation/invalidation order, stale admitted verbs, orphaned epoch guards and stale channels. Nine explicit authority cases pass. Existing lifecycle and local-runner PTY tests cover INT/TERM, raw-mode recovery, supervisor death and descendant reaping. Processes escaping a managed group remain outside its cleanup guarantee. |
| Prompt/methodology ownership | Lifecycle and standalone prompts supply both canonical commands and result meanings. Finish records caller teardown before its continue command; retirement/commit rules remain Grove's. Installed command tests assert unchanged policy prompt/task inputs. The official Ravel v0.1.0 source at `bbab823803bd2f2796f8bd9570f732dce900efa2` documents unchanged prompts and opaque results. No new Ravel API is required. |
| Active prose and source-pinned pages | Guide/vocabulary summaries, policy guidance, agent-helper book, diagram sources, viewer captions and guide-coverage roll-up now agree with the caller spec/ADR and source. All six books pass final/all reconstruction with no deferred source. The existing [surface inventory](ravel-caller-surface-inventory.tsv) is refreshed; measured lexicon, occurrences and semantic passages are retained in `target/k61`. |
| Independent packaging and published pins | The existing Grove formula depends on `linkuistics/taps/ravel`. The Ravel formula, input preparer, [published pin table](ravel-target-inputs.md#pinned-public-inputs) and all three archive bytes agree. Native Darwin and both Linux/QEMU routes pass with direct and symlinked fronts, independent runtime/SDK imports and stale-context refusal. Fresh native Grove 22.2.0 debug payloads pass lifecycle/standalone/import checks beside a separate Ravel 0.1.0 prefix, on child PATHs without Bun/Node. No equal-version/shared-Cellar assumption or Ravel release participates. |

The retained names have the same explicit boundaries as the earlier
[caller audit](ravel-caller-surface-audit.md): original live-v22 instructions,
historical observations, source-exact fixtures/stable locators and Grove's local
runner are distinct from current Ravel ownership. Diagram summaries were checked
alongside their detailed views. All nine diagrams parse with pinned Mermaid
12.0.0; malformed input is rejected. The stale decision caption is absent from
the active presentation and found by the same instrument in an injected sample
and historical snapshot; a deliberately broken pattern fails its positive
control. Graph coverage is best-effort: Mermaid parse gaps and reported shell
ranges were read directly, and excluded Cargo/preservation/measurement surfaces
were checked through their source and executable seams.

## Verification boundary

The final `task check` passes all 18 principal checks: 1,385 selected workspace
nextest cases, nine doctests and six final/all books. All 1,385 runtime identities
and outcomes match k60 individually. The gate additionally executes nine ignored
installed caller-asset cases. Explicit commands, fixtures and authority tasks
pass 4, 16 and 9 cases respectively, including six remaining installed ignored
cases. The other default-gate skips are subprocess helper entry points exercised
by their owning tests, not independent acceptance tests to run directly.

Complete terminal records are `target/k61/check.log`,
`installed-{commands,fixtures,authority}.log`, `ravel-targets.log`,
`grove-installed-layout-final.log` and `legacy-verify-sourced.log`. Named outcome
records, per-case comparison, formula/pin comparison, native payload digests and
diagram/search controls are alongside them. The native layout's before/after
payload digests match, and its staged binaries equal the freshly built debug
binaries. This is source/native acceptance; final finish still rebuilds and
checks the tagged Grove archives before publication.

All 16,583 frozen files match before/after the final gate: tracked repository
source/docs/task inputs, adjacent Ravel/tap/default-Grove tracked files, complete
original Grove 22.1.0 keg, installed Ravel, owner configurations, policy/parser
dependencies, pinned Bun and published Ravel archives. The named manifests are
`target/k61/check-inputs{,-after}.json`. This is not a freeze of every host,
toolchain or generated file read. No frozen input changed during a measurement.
Only acceptance reporting, brief promotion, inventory refresh and retirement
follow verification; the measured inventory snapshot is retained separately.

The first incomplete gate was deliberately interrupted before the last prose
repair; its input digests matched and `check-interrupted.log` is not a passing
gate. The original-driver check first refused an unsourced shell; sourcing the
applied `.jj/legacy-driver-22.1.0/release-env.sh` supplied the required cleanup
protections and verification passed. Neither event changed an installation or
runtime policy. Ravel and the tap remain clean at `3916a4a1` and `20ee956b`.

## Driver-owned finish handoff

Every k8 criterion is met. The root's extraction criteria retain the accepted
k3/k4/k11 independent project/publication, k5 caller, k6 delivery, k12 prose and
k13 integrated 22.2.0 release evidence; this increment completes its caller-owned
session-control extension. Final Grove integration/release is still the existing
driver's finish work. No Ravel publication, permanent installation, second driver
or teardown occurred here.

Preserve the complete original 22.1.0 installation, applied preservation plan,
owned links and cleanup exclusions through finish's last original signal. Every
release/recovery shell re-sources that plan. This live session retains its
original `harness-dispatch exit` ending; installing the new source never changes
an already running session's launch/admission/ending contract.

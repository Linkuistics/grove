# codex-trials-k105 — brief

## Goal

Measure the common loading route in actual Codex sessions, with native copies
absent. Reuse the runner of claude-code-trials-k104 with a Codex adapter.

## Done when

- The runner launches real Codex sessions in the sandbox home of Ravel's
  configuration specification. Fake-harness tests prove the Codex adapter's
  isolation and recording.
- Codex discovers a craft skill from activity alone and reads it before the
  governed action. Mandatory, kind and supporting reads keep their order.
- Every explicit request row, every discovery problem, continuation, both
  compaction cases, one real delegate and the native-only missing-kind case
  have their own record.
- The record states each gap, and each row that does not apply to Codex with
  its evidence. No gap passes as a result.

## Decomposition

The children mirror the Claude Code node, case for case. The first child adds
the adapter. The other children run the trial groups and depend only on it.

1. `codex-trial-adapter-k115` mirrors `claude-trial-runner-k107`.
2. `codex-discovery-trial-k116` mirrors `claude-discovery-trial-k108`.
3. `codex-explicit-routes-k117` mirrors `claude-explicit-routes-k109`.
4. `codex-discovery-problems-k118` mirrors `claude-discovery-problems-k110`.
5. `codex-continuation-trial-k119` mirrors `claude-continuation-trial-k111`.
6. `codex-compaction-trials-k120` mirrors `claude-compaction-trials-k112`.
7. `codex-delegate-trial-k121` mirrors `claude-delegate-trial-k113`.
8. `codex-missing-kind-k122` mirrors `claude-missing-kind-k114`.

codex-continuation-trial-k119 inserted `trial-harness-adapter-k123` before
k120. The human expects clear layers with small surfaces. The adapter puts
each harness difference behind one seam before k120 and k121 add more.

## Notes

The sandbox `.codex` holds only a link to the real `auth.json`. Each run sets
`CODEX_HOME` to the sandbox `.codex`. Refuse an inherited `CODEX_HOME` in the
real home, also an empty value, before any launch of the harness.

Codex 0.166.0 has its own sandbox modes. Record the mode of each case, and
whether it runs inside Ravel confinement or in its place.

codex-trial-adapter-k115 delivered `task trial:codex OUT=… CASE=…`, which runs
any case in Codex through its own tree, with `--harness codex`. "The Codex
adapter" in `docs/common-loading-acceptance-record.md` states the route, the
`workspace-write` sandbox and its reason, the residue and the helper links.
The fields `compaction`, `expectDelegate` and `systemNote` refuse for Codex
until k120 and k121 add their mechanisms. k119 added the Codex continuation. The delegate
leaf also adds Codex delegations, which the extraction does not yet link.

codex-discovery-trial-k116 recorded "discovery in Codex", which passes, and
"discovery-confined in Codex", which is a gap. Codex exits before its session
under Ravel confinement. The confined environment holds no `CODEX_HOME`, and
the sandbox denies writes in `$HOME/.codex`. The confined case of
codex-missing-kind-k122 probably meets the same gap.

codex-explicit-routes-k117 recorded "Explicit request rows in Codex". Codex
0.166.0 has no native skill loader. It lists native skills by `SKILL.md`
path, and the agent reads that path with `exec_command`. The three rows of a
native tool therefore do not apply. A native copy in Codex is visible only as
a read of a path in the sandbox `.codex/skills`, of the class `native`.

Five cases pass, including the express restriction. In the failed concrete
path case, Codex ran the force-stop command, and Claude Code ran no ending
command.

codex-discovery-problems-k118 recorded "Discovery problems in Codex". Six
cases pass, and the two cases with a pack reference did not run. Codex
force-stopped after two of its three reports. In the named user-invoked case,
it stopped before the write, because `authoring-conventions` requires
`writing-skills`, which no source holds. A `python3` heredoc read has the
class `command`, so a section names such reads by hand.

codex-continuation-trial-k119 recorded "continuation in Codex". The
continuation is `codex exec <the route's options> resume <thread id>
<answer>`. Codex keeps the thread, appends to the first session file and
keeps the sandbox of the options before `resume`. The rows hold as in Claude
Code, and the re-read is the same gap: the resumed turn read only the retire
and commit procedures.

Three observations differ from Claude Code. Codex adds
a new skill listing at the resume. Its fresh launch read `coding-style-bash`,
so `count.sh` carries the marker. It committed only the script, as the leaf
says, and left the retirement in the working copy.

trial-harness-adapter-k123 put each harness difference behind one adapter in
`scripts/harness_trial.py`. `ClaudeAdapter` and `CodexAdapter` implement the
interface `HarnessAdapter`, and the registry `ADAPTERS` maps a name to its
adapter. No code outside them tests a harness name. "The harness seam" in the
acceptance record states the layers.

k120 adds the Codex compaction to `CodexAdapter`. It removes `compaction`
from `CodexAdapter.unsupported` and sets `compaction_mechanism`. The feeder
`harness_trial.py turns` speaks Claude Code's stream-json protocol, and the
compaction readers read Claude Code's session format. k121 adds the Codex
delegation link to `CodexAdapter.describe`, which now returns no
delegations. `ClaudeAdapter.delegations` is the Claude Code form. A control of
a harness difference breaks one adapter method, such as `CODEX.resume_args`.

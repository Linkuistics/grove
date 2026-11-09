# tc-hook-bytecode-k284

## Goal

Stop the technical-communication hooks from writing Python bytecode into
their source. A write in Grove's keg changes the `writing` source's
`resourceDigest`, and the switch then refuses.

## Context

- session-probes-k281 found the defect in its staging pass. The hooks plugin
  `technical-communication-hooks@writing-hooks` runs in place from the
  `writing` source of Grove's keg, as "Native residue" of
  `docs/specs/owner-skill-delivery.md` states.
- The document gate runs after a `Write` or `Edit`. `scripts/tc_hook.py`
  loads `ste-lint.py` through `importlib.util.spec_from_file_location`.
  `scripts/tc-hook` runs Python with `-I`, which does not stop bytecode
  writes. Python then writes `scripts/__pycache__/ste-lint.cpython-315.pyc`
  into the source.
- Ravel's `resourceDigest` covers that file. In the staging pass, the
  `writing` source gave `6be6f84a…` after one Claude Code session, not the
  accepted `2dea55cd…`. The evidence section "Session probes" of
  `docs/owner-skill-delivery-evidence.md` records the run.
- In the real switch, the Claude Code probe of step 7 makes the same write.
  The Codex probe and `mac-check` would then report a source digest that is
  not the accepted digest.
- The source is the WritingSkills repository,
  `~/Development/AgenticOrchestration/WritingSkills`. Grove's keg packages it
  as the `writing` source.

## Done when

- No hook path writes a file inside the plugin root or the skill source. The
  fix covers bytecode and every other cache or log of the hooks.
- A control runs each hook event against a read-only copy of the source and
  against a writable copy. The writable copy's tree digest stays unchanged.
- The changed `writing` source has a new accepted digest. The accepted
  inputs, the candidate keg and the affected staging trials are revised as
  "The switch inside finish", step 2, requires.
- The candidate keg's `writing` source holds no `__pycache__`.

## Notes

The staging runner of session-probes-k281 removes each stray `__pycache__`
from the candidate keg before each case. It records each removal. The
record therefore shows whether the write recurs, and each case measures the
accepted digests. After the fix, the runner must record no removal.

## Decisions (running log)

- Every hook event writes the bytecode, not only the document gate.
  `main` loads the checker before it reads the off file. A scratch copy
  showed the write for the `prompt` event.
- The fix has two parts. `tc-hook` runs Python with `-I -B`, and
  `tc_hook.py` sets `sys.dont_write_bytecode` before it loads the checker.
  `-I` already ignores `PYTHONDONTWRITEBYTECODE` and `PYTHONPYCACHEPREFIX`,
  so no environment can turn the write on again. The core's own setting
  covers a run without the guard.
- The control lives in WritingSkills as `scripts/hook-writes.py`, beside
  `hook-count.py`. It runs each declared hook command from the hooks files
  and the Pi extension, so its event list follows the declarations. Mutant
  copies without the fix, and with a log write into the root, must fail it.
- WritingSkills `578f84d1f5a9bb04573a704bca639530d93b6bca` holds the fix,
  the control and the specification rule. `task check` passed there. The
  control ran 17 declared commands and loaded the Pi extension in a real Pi.
  Both mutants failed it. The commit is on the local `main` only.
- Grove `ef8a79dd7621` pins that commit in `plugins/packs.json`. A
  provisional candidate keg of it, in the scratchpad, gave `writing`
  `56bcd07b87f46a7b9ff92eed5a5a128f10d3ce64be2f7d934be898e058782212`. The
  five other digests equal the accepted digests. The keg holds no
  `__pycache__`. Each Claude Code event ran from that keg, and Ravel then
  gave the same `writing` digest.
- The auto-mode classifier refused `jj git push --bookmark main` in
  WritingSkills as a public change. The candidate must be a published
  commit, and the release build fetches each pack from GitHub. The
  replacement of the retained candidate, the reruns and the accepted inputs
  therefore wait for the human's decision on publication.
- The affected staging trials are the source trials and the session probes.
  Only those compare the `writing` digest or run the hooks. The discovery
  trials read skill text, which the fix leaves unchanged, and the staging
  HOME registers no hooks plugin. The dry run of owner-switch-rehearsal-k255
  has not run, so no dry run needs repeating.
- The plan draft takes each accepted digest from the table "The ten Ravel
  sources" of `docs/repository-moves-record.md`. That table therefore
  receives the new `writing` digest. The draft receives the same one value,
  which equals its generator's output for that field. `task
  owner:switch-plan:draft` is not rerun, because it reads the real HOME
  again.
- The human chose publication and allowed the pushes. WritingSkills `main`
  is at `578f84d1f5a9`, and `origin/gh-issue-25` at `ef8a79dd7621`.
- The earlier candidate keg moved to
  `~/.local/state/owner-staging/candidate-2dea55cd`, and the retained path
  holds the published candidate. It gave the same six digests as the
  provisional keg.
- `task owner:switch-plan:check` refused the revised draft, because
  `known_marketplaces.json` in the real HOME drifted. The stored check
  outputs stay as they are. owner-switch-plan-k254 writes the draft again.
- The source trials passed, and the eight session probes each gave their
  wanted result. No case recorded a bytecode removal.
- No review leaf: the control and its two mutants cover the fix with an
  executable seam, and the probes measured it in real sessions.
- `bash scripts/check.sh` passed all 18 checks.

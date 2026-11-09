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

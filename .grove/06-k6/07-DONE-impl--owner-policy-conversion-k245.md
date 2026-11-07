# owner-policy-conversion-k245

## Goal

Convert the owner's Ravel policy to Rhai. Ravel 2.0 then evaluates
`~/.config/ravel/config.rhai`, and the running 1.1.0 driver still reads
`policy.ts`.

## Context

- Order 4 of the cutover contract in `docs/specs/owner-skill-delivery.md`
  holds this conversion first. The design's section "The final session and
  new launches" lets preparation write `config.rhai` beside `policy.ts`.
- The coordinated release design states that Ravel 2.0.0 reads only
  `config.rhai` and refuses a lone `policy.ts`. The owner's `config.rhai`
  must exist and pass before any owner default launch reaches 2.0. Keep
  `policy.ts` while any 1.1 launch remains.
- The paused Rhai workstream in `Ravel.gh-issue-2` holds
  `owner-policy-conversion-k39` for the same work. Its body is a good brief.
  Read it, but do not edit that tree. Rollout's brief-update text tells that
  workstream to prune or retarget k39.
- The staging trials need the converted policy in the staging HOME. Use the
  accepted 2.0 build that `scripts/harness-trials/builds.json` names.
- The owner changed `policy.ts` on 2026-11-07. Read the current file.

## Done when

- A candidate `config.rhai` exists outside `~/.config/ravel`.
- Private homes with fake harnesses compare the two files. The candidate
  runs under the accepted 2.0 build, and `policy.ts` under the 1.1.0 keg.
  Each kind that the policy routes gets equal argument vectors. The cases
  include a secondary jj workspace, for the grant of the main repository.
- The owner approved the candidate. Then the leaf copied it to
  `~/.config/ravel/config.rhai`, with `policy.ts` unchanged.
- `settings.json` is still absent, or unchanged if it exists.
- A committed record names the digests of both files and the results. It
  states that a later change to `policy.ts` needs a new conversion and a new
  equivalence run before the switch. It names what the owner does after
  this driver's reap.

## Notes

Writing in `~/.config/ravel` changes the owner's live, mise-tracked
configuration. Show the owner the candidate and the test results, and ask
before you write.

Launch no real harness with the new file. The staging trials do that.

If the translation needs a host capability that the accepted build lacks,
stop and report the gap.

## Decisions (running log)

- The candidate is `docs/evaluations/owner-skill-delivery/owner-config.rhai`.
  It converts `policy.ts` (sha256 `30a0e5fc…`, 2026-11-07 19:34) line for line.
  `ravel::read_text` replaces `statSync` and `readFileSync`. Its codes
  `source_missing` and `source_not_regular` give no grant, as a missing
  `.jj/repo` and a directory did. `path_resolve` and `path_parent` replace
  Node's `resolve` and `dirname`. `VERSION` becomes `harness-dispatch 3`.
- The conversion adds no skill-snapshot read grant. Such a grant changes the
  argument vectors when skill delivery is on. `owner-staging-discovery-k249`
  measures whether Claude Code needs one.
- Two edge cases differ by design. A non-UTF-8 `.jj/repo` refuses under Rhai,
  where Node decoded it with replacement characters. The unreadable-store
  refusal carries the host's own error text.
- `policy_equivalence.py` runs 123 cases: 24 kinds in five directories, and
  three refusals. All passed. Two mutated candidates failed only their own
  rows: an effort change failed 5 cases, and a one-level grant failed the 72
  secondary-workspace cases.
- The owner approved the candidate on 2026-11-07. The leaf copied it to
  `~/.config/ravel/config.rhai` (`bbde65ef…`). `policy.ts` is unchanged, and
  no `settings.json` exists. The task reran 123 of 123 cases green against the
  installed copy. In a private copy of the real directory, 1.1.0 read
  `policy.ts` and 2.0 read `config.rhai`.
- The record is "Policy conversion" in `docs/owner-skill-delivery-evidence.md`.
  The checker reports 26 hard findings, all in the earlier k242 review record.
  This leaf leaves that record as its reviewer wrote it.

# core-candidate-restaging-k300

## Goal

Build a new core candidate keg in the projected layout. Record core's new
digest. Rerun the staging trials that read core. The plan and the dry run
then consume this evidence.

## Context

- skill-keg-link-layout-k292 changed `plugins/grove/README.md`, so the
  `grove-core` digest differs from the staging acceptance. The section
  "Consequences for queued work" of
  `docs/homebrew-skill-delivery-evidence.md` records it.
- "Skill formulae" and "Cutover contract" of
  `docs/specs/owner-skill-delivery.md` define the candidate. It is the
  last published commit of this line that changes `plugins/grove` or
  `plugins/packs.json`.
- `task owner:candidate-keg` builds a candidate keg and compares digests
  with an `ACCEPTED` file. grove-keg-projection-k293 made it build the
  projected layout.
- The staging trials run through `task owner:staging:case`. "Rerun
  commands" of the evidence section "Staging discovery trials" names each
  one. The section "Staging acceptance" records the accepted pins, digests
  and staging paths.

## Done when

- The line that holds the candidate is pushed to its public bookmark. The
  evidence names the candidate commit and its `plugins/grove` tree ID.
- The new candidate keg holds the projected layout. Its five domain-pack
  digests equal the accepted values. Its `grove-core` digest is recorded
  as the new accepted value.
- The staging settings name the new candidate keg. Each staging trial that
  reads `grove-core` ran again and passed. The evidence names each rerun
  and states why each other trial stays valid.
- The section "Staging acceptance" names the new candidate, digest and
  staging paths. It retains the earlier values as history.
- The finish-handoff bullets of the parent brief name the new candidate
  and the new core digest.

## Notes

Do not change `plugins/grove` or `plugins/packs.json` after this leaf
without a new candidate and another rerun.

owner-switch-plan-k254 names the retained staging HOME and this candidate
keg in `accepted.staging`. owner-switch-rehearsal-k255 runs the dry run
with them.

The release bump later changes `grove --version`. discovery-rollout-k12
already owns that consequence for the finish handoff.

## Decisions (running log)

- The candidate is `fb8d99c0f9bd9fd84276c4539ab11ce841bf9ced`, the commit
  of skill-keg-link-layout-k292. The jj revset `(gh-issue-25..@-) &
  (files("plugins/grove") | files("plugins/packs.json"))` selected it and
  `e287b47462b8`, and it is the later one. Its `plugins/grove` tree ID is
  `961fddd04bb7b271e9896d07dc0225348fa8c378`, the same as at k299's
  commit. The blob ID of `plugins/packs.json` is unchanged.
- jj pushed `gh-issue-25` from `ef8a79dd7621` to `2a24d5e61f29`. The line
  holds the commit `5163a5784f0c` without a description. It adds
  `.claude/settings.json` with one `gh repo create` permission. A new
  description would rewrite the candidate and the subject commits that two
  review records cite, so the push used `--allow-empty-description`.
- The new keg replaces the old one at the retained path, as the precedent of
  tc-hook-bytecode-k284 did. The old keg moves to `candidate-8d92cd58`.
  The staging settings and `accepted.staging` keep their text.
- The keg of `fb8d99c0f9bd` failed `task owner:candidate-keg`: its
  `share/ravel/skills/design` is a real directory. That commit precedes
  grove-keg-projection-k293, so its own template installs the direct
  layout. The spec defines the candidate keg as built by the production
  formula's install method, and k293's template is the production one.
  The candidate is therefore `2a24d5e61f29`, the pushed tip. Its
  `plugins/grove` tree and `plugins/packs.json` blob equal those of
  `fb8d99c0f9bd`. The spec sentence that defines the candidate changes to
  state both conditions.
- The new keg gives `grove-core` the digest `d8eec388…`. The five pack
  digests equal the accepted values. The moves record and the plan draft
  name the new digest.
- The session probes and the probe resolve read the core snapshot and the
  draft's digests. They are not in `accepted.trials`, but this leaf ran them
  too.
- A real-HOME marketplace write failed `kind-skill-codex-3`. The rerun
  `kind-skill-codex-4` passed.
- `stale-copy-codex-2` passed its verdict, but Ravel 2.0 exited 5 with
  `supervision_failed`. That does not concern the core, so this leaf
  inserted `ravel-stop-eperm-k302` before the rollout planner.
- `codex-native-bare` failed on `listing` only, twice. Codex found the
  frozen `grove@linkuistics` copy by a file search. The limit goes to
  owner-switch-rehearsal-k255 and the evidence.
- `task owner:switch-plan:check` refused the draft, because the real
  marketplace file changed. k254 writes the draft again, as in the
  precedent of tc-hook-bytecode-k284.

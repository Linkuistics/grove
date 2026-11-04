# requirements-native-namespace-k194

## Goal

Prove that a native Claude Code installation loads a moved requirements
member under its plugin's actual namespace. Prove also that the session reads
the core family file under the `grove` plugin's namespace.

## Context

- `docs/specs/pack-membership.md`, the row "An external member of a
  cross-pack family" in "The acceptance of each move".
- `docs/common-loading-acceptance-record.md`, "Research native namespace", as
  the model. The trial runner field `nativePlugins` installs plugins from the
  marketplace.
- The removal commit from requirements-removal-k192, whose marketplace lists
  `requirements` with a `git-subdir` source.

## Done when

- An isolated Claude Code HOME adds the marketplace from a plain checkout of
  this workstream's line at the removal commit. It installs the
  `requirements` plugin from that marketplace. Record whether the dependency
  installed the `grove` plugin. The `skills` key is absent from the Ravel
  settings, so only the native route supplies skills.
- A Grove `review-requirements` launch with the bare-name mandate loads the
  kind as `requirements:grove-review-requirements`. The agent loads the spine
  under its actual namespace and reads `references/review.md` in it before it
  acts.
- The record names the marketplace commit, the installed plugin revisions,
  the Ravel build, the Grove build that renders the mandate and the
  observations. A gap is a named gap with a disposition, never a pass.
- `docs/repository-moves-record.md` links the record section.
- The owner's real Claude Code installation is unchanged.

## Notes

Codex has no native skill loader in the measured version, so this proof is
for Claude Code only. A real run spends subscription usage.

This is the node's last planned leaf. Its retirement closes
requirements-pack-k173 if no live leaf remains, by the four close steps.

## Decisions (running log)

- The case is `requirements-native-namespace`, on the route native-2.0. It
  copies the planted tree of `requirements-family-read` unchanged and names
  `nativePlugins: ["requirements@linkuistics"]`. It routes only
  `review-requirements`. The runner did not change.
- The instrument was the Ravel 2.0 build from `71f70fcd3f92`, installed by the
  research recipe into a scratch prefix. Its `bin/ravel` SHA-256 is
  `26b487afb957…`, the value of `builds.json`, and the prefix has no
  `libexec`. `task trial:prepare` built Grove at the removal commit
  `302f85429e4b`, with the `grove` and `grove-llm` digests of the earlier
  trials.
- Run `requirements-native-namespace-20261104T141432Z`: pass. The install of
  `requirements@linkuistics` also installed `grove` as a dependency
  (`auto: true`). Call 1 loaded `requirements:grove-review-requirements` from
  the cache of the pinned pack commit `9f7278c2d46b`. Call 2 loaded
  `grove:grove` from `plugins/grove` of the checkout. Call 3 read
  `references/review.md` from that base directory. The first command on the
  task came in call 5.
- Call 3 ended with exit code 1 after the read. The Bash tool ran zsh, and
  zsh expanded the word `======` of an `echo` as a command path. This is a
  quirk of the agent's command, not a gap: the full file was in the result.
  Call 4 repeated the two reads that the error skipped.
- The installed `SKILL.md` files equal the files of `9f7278c2d46b`. That
  commit and the published `main` `eb945d9b13ef` have the same tree
  `f3cf8128fd20…` for `plugins/requirements`.
- The owner's real `~/.claude` had the same digests before and after the run.
  The case files and the runner had the same digests before and after the run.
- `docs/repository-moves-record.md` links the new section "Requirements
  native namespace" from the RequirementsSkills section. The checker reports
  no finding in the new text of either record. The counts equal those of the
  committed files. The acceptance record has 0 hard and 57 advisory findings.
  The moves record keeps its one known hard finding at the TestAnyware section.
- Gate: `bash scripts/check.sh` passed all 21 principal checks, with 1345
  Rust tests and the harness trial runner controls. The changed files outside
  `.grove/` had the same digests before and after the run.
- No in-session reviewer was spent, and no review leaf was cut. The
  observations come from the runner's record and the transcript, and no code
  changed.
- Close of requirements-pack-k173: each criterion of its `Done when` holds.
  The evidence is the RequirementsSkills section of the moves record and the
  two trial sections. The remote bookmark `gh-issue-25` holds `53d092fa0e5` and
  `302f85429e4b`. No review names the node or this leaf. The environment has
  no `GROVE_PROVIDER_EVIDENCE`, so this session writes no Creator line. The parent
  brief takes the two facts that later moves need. repository-moves-k5 still
  has live leaves, so the cascade stops there.

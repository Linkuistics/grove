# writing-craft-removal-k211

## Goal

Remove `technical-communication`, `writing-code-walkthroughs`, the hooks of
the `linkuistics` plugin and the moved assets from Grove. Reconcile every
Grove consumer in the same change, so that Grove stays green with
WritingSkills as the owner. Then publish the pack commit that records the
removal commit. This is the contraction step of the move.

## Context

- `docs/specs/pack-membership.md`: "The always-on hooks", "The Grove assets
  of moving craft skills" and "The acceptance of each move".
- `docs/specs/external-pack-contract.md`: "Distribution" and "The checker
  without a copy".
- The WritingSkills sections of `docs/repository-moves-record.md` from
  k208, k209 and k210: the moved assets, the plugin names and the commit
  that the trial accepted.
- engineering-removal-k204 and editorial-removal-k206, with their record
  sections, as the models.

Consumers at planning. Before you rely on this list, enumerate the tokens
again over the whole tree with `--hidden`. Use positive and cross-tree
controls. The tokens are both skill names, `tc-hook`, `tc_hook`,
`ste-lint`, `pi-extension`, `.agents/plugins` and each moved asset's path.

- `plugins/linkuistics/hooks/hooks.json` leaves. No hook in Grove names a
  moved script. `plugins/linkuistics/.claude-plugin/plugin.json` drops the
  walkthrough and writing words from its description and keywords.
- `scripts/check.sh` drops the checker self-test, the Pi load test and the
  `tc-hook` and Pi test paths of its shell lint.
- `plugins/install.test.sh` asserts the real Pi extension link. That is a
  named read of a moving file, so it takes a fixture.
- `.agents/plugins/marketplace.json` leaves, because the pack root holds the
  Codex marketplace file.
- `Taskfile.yml` drops the two `design:technical-communication` tasks.
- `plugins/linkuistics/PROVENANCE.md` drops the two moved sections.
- `.claude-plugin/marketplace.json` pins the accepted pack commit for
  `writing`, and it lists the Claude Code hooks plugin from the same
  commit. The dependency of the writing plugin resolves only when this
  marketplace lists the hooks plugin. The descriptions of the marketplace,
  of `writing` and of `linkuistics` change to match.
- The documents that state where the skills, the hooks or the moved records
  live: `plugins/README.md`, `plugins/CONTEXT.md`, `CONTEXT-MAP.md`,
  `docs/USAGE.md` and `docs/ARCHITECTURE.md` among them. A Grove file that
  stays and links a moved record links its public URL.
- The citation registry of `plugin_fallback.rs` keeps its two
  `technical-communication` rows. They read Grove's own files.

## Done when

- Grove holds neither skill, no `linkuistics` hooks file and no moved asset.
  The enumeration above is clean, and the session saw each control fail.
- The off file still turns off every always-on layer. The pack's checks
  show it for the hook core and Pi, and k210 showed it in the native route.
  No Grove record still claims to own that rule.
- `task check:packs` passes with the new pin, and `bash scripts/check.sh`
  passes on the Grove line.
- The pack's `GROVE_COMMIT` names the removal commit. That pack commit is
  on the public remote. Its `task check` passes from a clean checkout, with
  the public `GROVE_REPOSITORY` and no override.
- `docs/repository-moves-record.md` records both roots and the pack
  commit. It records the Git tree ID and the `resourceDigest` of
  `plugins/writing`, both from a clean checkout. It names the hooks plugin
  and the tool requirements: `python3` and the shell of `tc-hook`. It
  records the asset classification. It also gives skill-marketplace-k185
  one result. The Codex plugin stays listed in the pack root, unless Codex
  resolves a source outside it.

## Notes

The owner's native plugins stay at Grove commit `238c3a839ab4` from the
public `main`. Do not install, update or remove a plugin in the owner's
home. Rollout makes the native change.

`personal-pack-k183` removes the rest of the `linkuistics` plugin. Leave
its other four skills and their checks unchanged.

This is the node's last planned leaf. Its retirement closes
writing-craft-pack-k181 if no live leaf remains, by the four close steps.

writing-craft-repository-k208 copied the moved documents to the pack with
their bytes, apart from links and script paths. The pack's specification,
user guide and decision records therefore still describe the delivery through
Grove's `linkuistics` plugin and the checks of Grove's `scripts/check.sh`. Those
present-tense claims become false with this removal. Correct them in the pack
commit of this leaf, unless writing-hooks-plugins-k209 already did. The
section "Byte changes of the writing craft" of the moves record names the
eight changed files.

## Decisions (running log)

- Core alone before the removal, at `88be3ee7198`: 86 single-source checks,
  480 loaded-path checks and 51 unpinned rows. Conformance reads no file of
  `plugins/linkuistics`, so the removal must leave these figures unchanged.
- The removal takes out 1380 tracked files: the 1378 moved files of
  writing-craft-repository-k208, `plugins/linkuistics/hooks/hooks.json` and
  `.agents/plugins/marketplace.json`. The only untracked file in those trees
  was the ignored `scripts/__pycache__/ste-lint.cpython-315.pyc`, which went
  with its directory. The empty `.agents/` went too.
- `.claude/settings.json` is the owner's local permission. It stays in the
  working copy and in no commit, as in the earlier removals.
- The pin stays at `4cafb0fd8e72`, which the trial accepted. No file under
  `plugins/writing` claims delivery through Grove. The last pack commit
  therefore changes only files outside the source directory, and its tree
  equals the pinned tree.
- Consumers changed with the removal: `plugins/linkuistics` (manifest
  description and keywords, `PROVENANCE.md`), `scripts/check.sh` (the
  checker self-test, the Pi load check and the two shell-lint paths),
  `Taskfile.yml` (the two `design:technical-communication` tasks) and
  `.claude-plugin/marketplace.json`. The marketplace pins `writing` at
  `4cafb0fd8e72` and adds `technical-communication-hooks` from the same
  commit, with the source path `plugins/writing/craft/technical-communication`.
  It now counts nine plugins.
- `plugins/install.test.sh` replaced its read of the real Pi extension with a
  fixture. A pack's `craft/` skill ships `harnesses/pi/crafted.ts`, and
  `--pack` must link it into the `craft/` path. The suite gave 48 passed. A
  copy whose fixture shipped `other.ts` failed that case alone, with 47 passed.
- `install.sh` claims an extension link only when the run scans its plugin, as
  for skill links. A run without `--pack` therefore keeps a pack's extension
  link. No installer change was needed.
- `task check:packs` reports `technical-communication-hooks` as a
  directory-only source, because it has no kind skill and no manifest. The
  comment of `check-packs.sh` and the directory-only paragraph of
  `pack-membership.md` now name the hooks-only case.
- Documents: `plugins/README.md` moves the craft table and the always-on
  paragraph to the `writing` section, and names the hooks plugin in the pin
  exceptions. `plugins/CONTEXT.md` changes the plugin, always-on and harness
  entry terms. `CONTEXT-MAP.md` says that WritingSkills owns the five moved
  records and links them by public URL. `README.md`, `docs/USAGE.md` (the
  `writing` source gains the root `craft`) and `docs/ARCHITECTURE.md` state
  where the skills and the hooks ship. `README.md` also lost a claim that
  was already stale: "the Linkuistics coding/design skills".
- Three files wrote `linkuistics:writing-code-walkthroughs`, and they now
  write the bare name: `walkthrough-books.md`, `grove-loop-book-structure.md`
  and `a-book-cannot-witness-its-own-corpus.md`. The Codex marketplace item of
  `external-pack-contract.md` now says that Grove has none. Three files that
  stay linked a moved record, and they now link its public URL:
  `pack-membership.md`, `owner-skill-delivery.md` and the editorial pilot's
  `preregistration.md`. Each URL returned HTTP status 200.
- Off-file ownership: every Grove mention of the rule cites the WritingSkills
  record. No Grove record states the rule as its own.
- A link check over every tracked and untracked Markdown file found no
  relative link that the removal broke. The 63 broken links before the
  removal include the moved evaluation records and the pre-existing
  walkthrough examples.
- The technical-communication checker gave no new hard finding in a changed
  Grove document after four long paragraphs and two long sentences were
  split.
- Stale-location sweep: `rg --hidden` over the whole tree, apart from `.jj`,
  `.git`, `target`, `.grove` and `.cache`, for the moved paths, the two
  namespaced names, `linkuistics-hooks`, `.agents/plugins`, `pi-extension-load`
  and relative links into the moved records. Each hit is a public URL, the
  move decision in `pack-membership.md`, baseline evidence in the two
  inventory files, `CHANGELOG.md` or this workstream's record. A planted path
  in a hidden directory was found, and a misspelt pattern found nothing. The
  same sweep over WritingSkills gave hits in the moved documents. Its hits of
  Grove's old paths are provenance or dated records.
- Gate: `bash scripts/check.sh` passed all 19 principal checks, 1344 Rust
  tests among them. The two fewer checks are the checker self-test and the Pi
  load check, which the pack now runs. `GROVE_TEST_RAVEL` resolved to
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. The changed files outside
  `.grove/` had the SHA-256 `775f318556f8…` over their digests before and
  after the run.
- The Codex command of the corrected user guide works on Codex 0.166.0 in a
  scratch `CODEX_HOME`. `codex plugin marketplace add Linkuistics/WritingSkills`
  added `writing-hooks`, and `codex plugin list` showed
  `technical-communication@writing-hooks`.

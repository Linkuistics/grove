# writing-craft-repository-k208

## Goal

Add the `craft/` root to WritingSkills with `technical-communication` and
`writing-code-walkthroughs`. Test 2 or 3 of the membership decision gives
some Grove assets to the pack. Move them in, with their checks. Publish the
pack commit. This is the first part of the expand step. Grove keeps its
copies, its hooks and its checks until writing-craft-removal-k211.

## Context

- `docs/specs/pack-membership.md`: "The always-on hooks", "The Grove assets
  of moving craft skills" and "The acceptance of each move".
- `docs/specs/external-pack-contract.md`: "Layout", "Provenance and
  licences", "Distribution", "Checkout boundary" and "Proven facts".
- DesignSkills and EngineeringSkills as the models of a `craft/` root, of
  its manifest field and of its README section. The WritingSkills section
  of `docs/repository-moves-record.md` gives the current pack commit
  `ae1649d181a5`.
- The baseline of owner-ravel-skill-delivery-k37:
  `docs/owner-skill-inventory.tsv` and
  `docs/evaluations/owner-skill-delivery/inventory.py`.
- The engineering expand, engineering-pack-repository-k203, and its record
  sections as the model of the byte changes, the provenance and the tool
  requirements.

The moving files at planning. Search again with `--hidden` and with
positive and cross-tree controls before you rely on this list.

- `plugins/linkuistics/skills/technical-communication/`: `SKILL.md`,
  `always-on/`, `harnesses/`, `references/`, `scripts/`, `LICENSES/` and
  `.codex-plugin/`. The working tree also holds `scripts/__pycache__`, which
  Git does not track.
- `plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md`.
- The two sections of `plugins/linkuistics/PROVENANCE.md` for these skills.
- Test 2: `docs/specs/technical-communication.md`,
  `docs/TECHNICAL-COMMUNICATION.md`, `docs/design/technical-communication/`
  with the two `design:technical-communication` tasks of the Taskfile, the
  decision records `the-off-file-is-the-only-switch`,
  `documents-are-gated-replies-are-measured`,
  `technical-communication-carries-no-standard-text` and
  `behavioral-acceptance-is-closed-unmet`,
  `docs/technical-communication-publication-review.md` and
  `docs/evaluations/writing-code-walkthroughs/`.
- Test 3: `docs/research/harness-hooks-a.md`.
- The checks: `scripts/pi-extension-load.test.sh`, the checker self-test
  and the `tc-hook` lint of `scripts/check.sh`.

## Done when

- `plugins/writing/craft/` holds both skills. The plugin manifest lists
  `"skills": ["./craft/"]`, and the README gives the Ravel roots
  `["skills", "craft"]`. `ravel skills resolve` lists both craft skills
  beside the editorial skills.
- `technical-communication` keeps every tracked resource. A diff against
  Grove's tracked files shows the same bytes and the same Git modes, or a
  recorded change. The three scripts have mode `100644` in Grove, so they
  stay `100644`. The capture holds no `__pycache__`.
- Each moved skill matches its baseline entry or a recorded change, as
  order 2 of the cutover contract requires. Check the `repository` and the
  description of `.codex-plugin/plugin.json`, which name Grove and
  `plugins/install.sh`.
- The moved Grove assets are in the pack, each at a path that the pack
  README names. A link between a moved file and a Grove file that stays is
  a public URL. A finished evaluation record keeps its bytes. The harness
  scripts of the evaluations name the skill's new path, and the record names
  each such change.
- The pack's `task check` runs the checker self-test, a `tc-hook` lint and
  the Pi load test, with the controls that each has in Grove. Watch each
  control fail once in the pack. The pack has the two design view tasks.
- `plugins/writing/PROVENANCE.md` holds the two moved sections. The
  checker's licence stays in `technical-communication/LICENSES/`.
- The pack commit is on the public remote. The pack's `task check` passes
  from a clean checkout of it, with the public `GROVE_REPOSITORY`.
- The moves record has a section for this commit. It names the source
  paths, the Grove commit and the byte changes. It also names the asset
  classification, the checks and the published commit.
- Grove's `bash scripts/check.sh` passes. Grove changes only the record.

## Notes

The hooks plugins and the Codex marketplace file are
writing-hooks-plugins-k209. This leaf adds no `.claude-plugin/` to the
skill directory and no `hooks/` anywhere in the pack. Native users of the
writing plugin get no hooks from this commit. Grove's marketplace still pins
`694140a18dc7`, so no native user gets this commit yet.

The membership decision lists the asset classes. A move that finds another
class applies the same three tests in order. Record the result. The
walkthrough books under `docs/walkthroughs/`, `docs/specs/walkthrough-books.md`
and the decision records `a-book-*` state Grove's own books, so they stay.
Check `docs/research/walkthrough-method*.md` and
`docs/evaluations/editorial-pipeline-pilot/` by the same tests.

Do not install, update or remove a plugin in the owner's home. The
always-on layer of this session comes from the owner's plugin cache.

The evaluation directory holds about 8 MB of evidence. Copy it, and check
its digest before and after.

## Decisions (running log)

- Source commit: Grove `31ec9117c9da`. It is public on `gh-issue-25`, it is
  the pack's `GROVE_COMMIT`, and no moving file changed between it and
  `76daf77cdb24`. `GROVE_COMMIT` stays, because no new check reads Grove.
- Layout: each moved asset keeps its Grove path relative to the repository
  root (`docs/...`, `scripts/pi-extension-load.test.sh`). The relative links
  between moved files, and the `docs/...` names in code comments, then stay
  true without an edit.
- The only link from a moved file to a Grove file that stays is
  `docs/TECHNICAL-COMMUNICATION.md` to `../README.md#install-the-skill-plugins`.
  It becomes a public URL. Three other unresolved targets are `/private/tmp`
  paths inside evaluation evidence, and they keep their bytes.
- Baseline: the cache entry `b639969cc4a1256a`, 20 files, counts
  `scripts/__pycache__/ste-lint.cpython-315.pyc`. Grove's tracked tree at the
  source commit is 19 files, `960a0f38bdb0c2af`. Its `.codex-plugin`
  description already differs from the cache, by commit `175e5c7d259`.
  `writing-code-walkthroughs` equals its baseline `b1e4795388bcf3ab`.
- `.codex-plugin/plugin.json` changes its `repository` to WritingSkills and
  names the pack's install routes in its description. A recorded change.
- The evaluation scripts that name the skill path: the three `harness.sh`
  files and `refinement-regression/template-test.sh`. The template test also
  reads the plugin manifest and asserts the name `linkuistics`. It now reads
  `plugins/writing/.claude-plugin/plugin.json` and asserts `writing`.
- `docs/research/walkthrough-method*.md` stay (test 1): they are the
  research for Grove's `ordinal-fs-tree` book, and no moving file cites
  them. `docs/evaluations/editorial-pipeline-pilot/` stays (test 3), as
  editorial-pack-k179 classified it.
- Two more links change because their targets move inside the pack:
  the guide's link to the skill and the walkthrough decision record's link
  to the provenance file. The skill's own `../../PROVENANCE.md` and the
  provenance file's `../../docs/...` links resolve unchanged.
- Scope: the moved documents keep their present-tense claims of Grove's
  delivery and of `scripts/check.sh`. They stay true until k211, and a note
  in k211 asks it to correct them.
- The Pi check compiled `ste-lint.py` in the working copy, and jj captured
  the bytecode. The pack's `.gitignore` gains `__pycache__/`, as Grove's has.
- The pack's checks: `check:tc-checker`, `check:tc-hook-lint` (ShellCheck on
  `tc-hook` and the Pi check, as Grove lints both) and `check:tc-pi`. The
  ShellCheck task requires ShellCheck. The Pi task skips visibly without Pi,
  as in Grove.
- Published `0eacee02f0f2` to the public `main`. Tree `ca83ff27dceb`, digest
  `293ed909955f…`, from the public clone and the jj working copy.
- Grove's `bash scripts/check.sh` passed all 21 principal checks with 1344
  Rust tests. The four changed files had the same digests before and after
  the run. `check:packs` still reads the pinned writing commit
  `694140a18dc7`.

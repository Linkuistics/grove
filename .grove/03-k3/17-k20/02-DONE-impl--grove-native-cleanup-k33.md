# grove-native-cleanup-k33

## Goal

Supply the optional manual cleanup procedure for installations left by the
removed Codex provisioner, within the same unreleased native increment.

## Context

Consume grove-native-provisioning-k21 and the delivery specification's existing
installations and active-readers contract. Use its evidence document's cleanup
consumer map. This leaf changes guidance and its disposable verification only.

## Done when

- Write the optional procedure in `docs/USAGE.md` and link it from plugin
  guidance. Require stopped writers/readers and evidence of ownership and lack
  of retained references before any removal. Path shape alone is insufficient.
- Exercise the procedure in disposable legacy layouts: stopped writers/readers,
  owned and foreign links, aliases, uncertain provenance and retained snapshot
  references. Record which entries remain and which evidenced owned entries can
  be removed. Preserve the owner's real installations.
- Verify the documentation links and applicable documentation/conformance
  checks. Record commands and observations on this task. The parent cannot
  close or publish the native increment without this procedure.

## Notes

Add no automatic cleanup, cleanup CLI, owner-configuration change or release.

## Decisions (running log)

- The procedure is a subsection of `docs/USAGE.md` under its own explicit
  anchor `usage-codex-cleanup`, after the earlier-installations paragraph. Plugin
  installation guidance in `plugins/README.md` and `README.md` links to it.
- The removed provisioner (`crates/grove/src/provision.rs` at `6b2ec873`)
  wrote no provenance record. It wrote absolute links of exactly one form,
  `<root>/current/plugins/<plugin>/skills/<name>`, with `<name>` equal to the
  link name. In `$CODEX_HOME/skills` it repointed only links whose target already
  had the Grove plugin shape. Those links were usually the owner's manual
  installation. Each snapshot holds only `plugins/`, copied from every regular
  file of the build's `plugins` tree.
- Ownership needs three separate pieces of evidence. The owner's own setup
  records show no other writer. The link has the exact provisioner form. The
  Grove root holds only its own entries. A snapshot must also match the `plugins`
  tree of a Grove release or of a known build checkout. Any missing piece makes
  the entry uncertain, and the procedure keeps it.
- Removal moves entries into a holding directory first, links before `current`
  and snapshots. The owner removes the holding directory after new sessions work.
- The exercise runs the documented commands verbatim with `HOME` set to a
  disposable directory in the session scratchpad. The owner's real
  installations are only read, never changed.
- The first exercise found that a real 22.8.0 snapshot holds an extra
  `plugins/linkuistics/skills/technical-communication/scripts/__pycache__`.
  The release checks write it, and Git ignores it. So the documented comparison
  is `diff -r -x __pycache__`, and the guide says why. Every other difference
  still makes a snapshot uncertain.
- The new section is inside a `tc: pending` region of `docs/USAGE.md`, so the
  file-level checker skips it. This session lints the section's lines directly.
  One departure is marked: `launch policy` is the name of the owner's policy file.
- A link of the inventory that stays is also a reference to each entry through
  which it resolves. The guide states this beside the reference search.
- The delivery specification now states the procedure as current and links it.

- This leaf spent its one in-session reviewer on an adversarial read of the
  section, the contract and the removed source. It gave 11 findings. All 11 are
  valid and actionable. The procedure is rewritten as follows.
  - The manual installer replaces each link at a name that it ships. It also
    removes links to Grove plugin skills that it no longer ships, also foreign
    ones. It
    also writes the Gemini CLI and Pi directories. The procedure now uses it
    only when the owner accepts those changes.
  - The installer writes only `~/.codex/skills` and ignores `CODEX_HOME`
    (`plugins/install.sh` line 164).
  - The provisioner also replaced owner links in `~/.agents/skills`. The first
    condition now asks who made a link at the path, not at the target.
  - The reference search now reports links to `~/.agents`, into it, and to or
    into the Codex skill directory. It excludes only paths inside `~/.agents`.
  - The search scope names locations outside the home directory and the
    `CODEX_HOME` directory. The procedure requires `realpath`.
  - Recovery moves entries back in reverse order. The root is removed only after
    the new sessions work.
  - The provisioner first shipped in v21.11.0 (`92a8bbee003`). The layout
    claims of the procedure now name 21.11 to 22.8. Earlier builds wrote real
    directories, so the writer step still covers every build up to 22.8. Other
    documents say only that 22.8 and earlier installed Codex skills, which
    stays true.
  - The table states the provisioner's actual ownership test and its condition
    for startup. It also lists the staging directories of an interrupted startup.
  - The `pgrep` options differ on Linux, and the search time is not minutes.
- Re-review after this substantive rewrite is the second need. A
  `review-impl` leaf beside this one reads the committed procedure.

## Verification record

Base `175e5c7d259`. All runs used the real installed `grove 22.8.0` as the old
writer, and Ravel 1.1.0 at `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`.

Method. Scratchpad scripts build disposable homes. The old `grove` runs through
`env -i` with that `HOME`, in a disposable jj repository. Its `PATH` holds `jj`
but not `ravel`, so startup provisions and then refuses. A second script extracts
each `sh` block of the section verbatim and runs it under `/bin/sh`. Only the
placeholders `NAME`, `bundle-XXXXXX` and the release tag are substituted.

The classification is the judgment step, and the script writes its evidence.

### Pre-review exercise

Guide digest `f5bf91d49f2d3b43` (SHA-256 prefix of `docs/USAGE.md`), before
and after. These observations still hold for the final text.

- One reader held a snapshot file open, and one lazy reader held only a skill
  path. `lsof +D ~/.agents/.grove` showed only the open reader.
  `pgrep -fl 'grove|codex|ravel'` showed both. Both stopped before step 2.
- The `__pycache__` difference above appeared here, between a real 22.8.0
  snapshot and the v22.8.0 release.
- Control: `find -lname '*/.agents/.grove/*'` found 48 links whose text names
  the root, `current` among them, but not `~/work2/planning-skill`. The
  documented `realpath` form found it.

The later findings of the reviewer superseded the rest of this run.

### Final exercise

Guide digest `48bd9da4df52dd3e`, before and after.

- Layout A, separate `~/.agents/skills` and `~/.codex/skills`. The provisioner
  wrote 44 exact-form links. It repointed an installer-shaped
  `~/.codex/skills/grove` and an owner link `~/.agents/skills/grove-art`. The
  hand-made cases follow.
  - A dangling exact-form `obsolete`.
  - A `retained` link directly to `snapshots/bundle-older`, a v22.7.0 tree.
  - A relative `lookalike`, a checkout link and a dangling foreign link.
  - A real directory, a real file, Codex's `.system` and a foreign file.
  - A partial `bundle-partial`.
  - Owner records: `dotfiles.sh` made `grove-art` at its path, and `links.sh`
    makes `grove-research-a`.
  - A settings file that names `grove-impl`.
  - `~/work2/planning-skill`, which chains through `grove-planning`.
- A search output: the chain, `~/.codex/skills/grove` and the three text files.
- A comparisons: the provisioner snapshot equals v22.8.0, and `bundle-older`
  equals v22.7.0. `bundle-partial` differs from both by 204 lines.
- A result: 41 entries moved, `obsolete` among them. Ten stayed: the four
  evidenced keeps, `retained`, `lookalike`, the checkout link, the dangling link,
  the real directory and the file. `~/.codex/skills/grove` stayed as needed. All
  kept links and the chain stayed readable. Recovery moved `grove-design` back,
  and it was readable. `rmdir` refused both directories as not empty.
- Layout B: `~/.codex/skills` is a link to `~/.agents/skills`, and `CODEX_HOME`
  names `~/codex-alias`, a link to `~/.codex`. Two staging directories were
  added: `link-AbC123` in the skill directory and `link-DeF456` in the root.
- B search output: the two aliases only. Both `realpath` lines were equal. The
  snapshot equals v22.8.0. With no need through either alias, all 45 entries,
  `link-DeF456`, `current` and the snapshot moved. After the hold was removed,
  step 6 removed the lock, `snapshots` and the root.
- Layout C: `CODEX_HOME` outside the home, with a `config.toml` that names
  `~/.agents/skills`. Also `~/.gemini/skills` links to `~/.agents/skills`,
  `~/agents-alias` links to `~/.agents`, and `~/unrelated` links to `/usr/bin`.
  The provisioner wrote no Codex links there, because that skill directory did
  not exist. `realpath` reported it, as the guide says.
- C search output: both aliases and `config.toml`, not `~/unrelated`. Nothing
  showed that the aliases are unused, so every entry stays.
- Control: the pre-guard search form, with the Codex skill directory absent,
  also printed `~/unrelated -> /usr/bin`.
- The owner's real `~/.agents/.grove`, `~/.agents/skills` and `~/.codex/skills`
  are unchanged. A digest of the type, modification time, inode and link text
  of each entry is `99a2418fc76a7ecb` before the first run and after the last.

### Checks

- Direct `ste-lint.py` on the final section: 0 hard, 0 advisory, 1 accepted. The
  touched paragraphs of `plugins/README.md`, `README.md`, `CHANGELOG.md` and the
  delivery specification have no hard findings.
- `cargo nextest run -p grove --test user_guide_coverage --test
  reference_navigation`: 17 of 17 pass on the final text. Control: a broken
  fragment in the new `plugins/README.md` link fails
  `every_repository_markdown_reference_resolves` with that fragment named. The
  file was then restored byte-identical.
- `GROVE_TEST_RAVEL=<Ravel 1.1.0> task check:skills` on the final text: the
  installer suite passes 28, conformance holds, and 10 of 10 tests pass.
- `GROVE_TEST_RAVEL=<Ravel 1.1.0> mise exec bun@1.4.3 -- bash scripts/check.sh`
  on the pre-review text: all 20 principal checks, 1,323 nextest tests and 5
  books pass.
- The same command on the final text: all 20 principal checks, 1,323 nextest
  tests and 5 books pass. The digest of the change was `a24ed1c8bc7ba2a4`
  before and after that run.

Limits: no real harness session ran in the disposable homes, because they have
no authentication. Readable `SKILL.md` files through each link replace that
step. The exercises ran on macOS only, with canonical home paths.

# grove and the linkuistics skills live in one repo

`Linkuistics/grove` holds both the grove CLI + methodology (`src/`, `content/`)
and the skill plugins `linkuistics` and `testanyware` (`plugins/`,
`.claude-plugin/marketplace.json`). It binds because the two change **in
lockstep**: most grove changes require a corresponding skill change, and while
they lived apart no commit could carry both. The former `Linkuistics/skills`
repo is archived, its 68 commits grafted in via a two-parent merge so blame on
the skill files stays continuous.

The evidence that forced this is in the skills repo's own history: a grove
workstream run there had to write three commits whose entire content was a
pointer at the other repo — *"Track grove's jj plumbing landing in
Linkuistics/grove"*, and two more like it. Placeholder commits are what a
coupled change looks like when the histories cannot hold it.

**Distribution stays dual, deliberately.** grove's methodology reaches all three
harnesses grove supports (`claude`, `codex`, `pi` — `src/harness.rs`) by being
embedded in the binary and swept to each harness's personal skill dir; the
plugins reach Claude Code through the marketplace. Collapsing onto plugins alone
is not available: Claude Code plugins are Claude-only, which is why the skills
tree carries `install.sh` for the other two. One repo, two delivery paths, and
the monorepo is what makes that a choice rather than an accident.

## Considered options

- **The skills repo absorbs grove instead.** Rejected: it relocates the Cargo
  workspace, the `build.rs` that embeds `content/` via `include_dir!`, the test
  suite, cargo-release and the brew tap, and forces grove's jj-native setup onto
  a plain-git repo — all to protect a marketplace URL that turns out **not** to
  be identity-bearing. A marketplace's identity is the `name` field inside
  `marketplace.json` (already `linkuistics` while the repo was `skills`), so the
  move changes only `source.repo`. *Reopens if* grove's Rust component is ever
  retired, leaving a markdown-only tree with no build pipeline to protect.

- **Keep two repos; deduplicate only the overlapping content, or unify only the
  distribution mechanism.** Rejected: both leave the cross-repo lockstep intact,
  and the lockstep is the actual pain. *Reopens if* the coupling weakens — if
  skill changes stop accompanying grove changes, the repos have no reason to be
  one.

- **Keep `Linkuistics/skills` alive as a pointer marketplace**, gutted to a
  `marketplace.json` whose entries `git-subdir` into this repo, so existing
  installs follow transparently with no user action. Rejected as unearned: the
  consumer set is effectively one machine, and a permanently live second repo to
  spare it two commands is not a trade worth making. *Reopens if* external
  consumers appear — the mechanism works and 220 of the 273 plugins in
  `anthropics/claude-plugins-official` use exactly this remote-source form.

## Consequences

- **`git log` shows two roots.** The graft is a merge of two previously unrelated
  histories; this is intended, and is what keeps `git blame` on
  `plugins/linkuistics/skills/*` continuous past the merge.

- **Two bounded contexts, so a `CONTEXT-MAP.md`.** The repo carries `CONTEXT.md`
  (grove) and `plugins/CONTEXT.md` (skills) behind a root map, rather than one
  conflated glossary — the multi-context shape `content/CONTEXT-FORMAT.md`
  already documents.

- **Archiving `Linkuistics/skills` fails silently for anyone who does not
  re-point.** `known_marketplaces.json` carries `autoUpdate: true`, and pulling
  from an archived repo keeps *succeeding* — the content simply freezes, with no
  error surfaced. The migration is therefore announced, not merely performed.

- **Plugin versions churn with unrelated grove commits.** With no `plugin.json`
  declaring a version, Claude Code versions a plugin by repo HEAD SHA
  (`installed_plugins.json` records `"version": "e0ba6f40f6e8"`). In a repo whose
  commit rate is grove's, every commit re-versions both plugins. An explicit
  `plugin.json` version decouples them.

# Paused-grove handoffs

Three workstreams paused while this grove split the skill corpus and released
Ravel 2.0.0. This document gives each of them the facts that it needs when it
resumes, and a block of brief-update text. The human copies that block into the
workstream's root brief before its next session.

paused-grove-handoffs-k320 wrote this document on 2026-11-13. It read the three
task trees and their version-control state without a change. It edited no file
in those workspaces, and it sent no signal to a session.

Finish fills the section [Promotion and switch receipts](#promotion-and-switch-receipts)
after the promotion and the owner's skill switch. The finish handoff names that
section.

## How to use a section

1. Wait until this grove's finish session ends.
2. Read the receipts section of this document.
3. Copy the block "Text for the root brief" of the workstream into its `_BRIEF.md`, as a new section.
4. Rebase the workstream's line as its block directs.
5. Start the workstream again.

A section's tables record what this grove saw on 2026-11-13. Read each moving
fact again before you act on it, such as a bookmark position.

## Facts for every paused workstream

### Where the skills are now

Each pack is a repository under `~/Development/AgenticOrchestration/`. Each
source directory keeps kind skills in `skills/` and standalone craft skills in
`craft/`. The [external pack contract](specs/external-pack-contract.md) defines
these terms.

| Content | Repository and path |
| --- | --- |
| The spine, the tree and artifact formats, `finish`, `planning`, the review and integration family files, and `configure-grove` | Grove, `plugins/grove/skills/` |
| The requirements kinds and `grilling.md` | RequirementsSkills, `plugins/requirements/skills/` |
| The design kinds and the visual guide `grove-design/references/visual-design-discussion.md` | DesignSkills, `plugins/design/skills/` |
| Design craft: `cli-tool-design`, `codebase-design`, `decision-records` and `model-led-development` | DesignSkills, `plugins/design/craft/` |
| The `impl` and `prototype` kinds | EngineeringSkills, `plugins/engineering/skills/` |
| Engineering craft: the coding styles, `doubt-driven-development`, `simplify-project` and `using-codebase-memory` | EngineeringSkills, `plugins/engineering/craft/` |
| The research kinds and their family skill | ResearchSkills, `plugins/research/skills/` |
| The editorial kinds `draft`, `copy-edit`, `art` and `proof`, and their family skill `editorial-family` | WritingSkills, `plugins/writing/skills/` |
| Writing craft: `technical-communication` and `writing-code-walkthroughs` | WritingSkills, `plugins/writing/craft/` |
| Personal craft: `authoring-conventions`, `machine-environment` and `using-jujutsu`, and the `guardrail` plugin | PersonalSkills, `plugins/personal/` |
| `using-testanyware` | TestAnyware, `plugins/testanyware/skills/` |

The directory `plugins/linkuistics` is gone. So are the installer
`plugins/install.sh`, its test and the listings of the Claude Code
marketplace. The [moves record](repository-moves-record.md) states each move
and its commits. [Pack membership](specs/pack-membership.md) gives the reasons
for each pack's contents.

### How a skill reaches a session

Homebrew installs every skill source, and Ravel is the only route that
delivers skills to a session. Grove's formula carries core and the five domain
packs, each at the commit that `plugins/packs.json` names. The `personal`,
`testanyware`, `superpowers` and `quint-llm-kit` formulae carry the other
sources. Ravel 2.0.0 reads each source from
`$(brew --prefix)/share/ravel/skills/<source-id>` when its settings name no
sources.

A change to a domain pack reaches users in four steps. "A pack change" in
[RELEASING.md](RELEASING.md#a-pack-change) states them:

1. Publish the change as a commit of the pack's repository.
2. Record that commit in `plugins/packs.json`, as a Grove change.
3. Run `task check:packs`.
4. Run `task release:patch` to publish Grove.

To try a pack checkout before its publication, name it as a `local` source in
the `skills` object of Ravel's settings. A `sources` array replaces the default
directory, so name core and every other pack that the session needs.
[USAGE.md](USAGE.md) shows the settings. Run such a trial under its own HOME
while a Ravel 1.1 session still reads the usual HOME.

### Kind charters and craft guidance

A kind charter is the `SKILL.md` of a kind skill in a pack's `skills/`
directory. It holds what one kind owns. A craft skill is a standalone skill in
a pack's `craft/` directory. Any session or project can load it, and it names
no kind as its owner. Keep each new rule in one of the two places, and do not
copy it into the other.

Pack text names another skill by its bare name, such
as "Load the `codebase-design` skill". It never uses a plugin-qualified name,
because a later move changes the plugin.

### When to rebase

Grove's `main` holds this grove's result only after finish integrates it.
On 2026-11-13, `main` was still `238c3a839ab4`. Rebase a paused Grove line
after finish moves `main`, not before.

## The editorial grove

| Subject | Value |
| --- | --- |
| Workspace | `~/Development/AgenticOrchestration/grove.gh-issue-14-documentation-skills-for-editorial-pipeline` |
| Base of its line | Grove `b4fa1c239312`, "finish-k2: remove completed grove task tree", 2026-10-24 |
| Last commit of its line | `52b613c70292`, "document-owned-case-runtime-k546". The working copy above it was empty |
| Files under `plugins/` that its line changes | None |
| Other product files that its line changes | `CONTEXT.md`, `CONTEXT-MAP.md`, `Taskfile.yml`, `scripts/check.sh`, `crates/grove/tests/reference_navigation.rs`, five decision records, `docs/specs/documentation-guidance.md` and its design views, three research documents, and new check scripts. `jj diff --from b4fa1c239312 --summary` lists each path |

Each of those files still exists in Grove. This grove did not change
`docs/adr/the-editorial-pipeline-is-four-kinds.md` after `b4fa1c239312`. The
other files changed on both lines, so the rebase can conflict in them.

### Affected entries of the editorial tree

| Entry | Reference | What changed |
| --- | --- | --- |
| `_BRIEF.md`, Goal and Done when | "Grove's installed skills" | The editorial kinds ship in WritingSkills. Grove's formula installs that pack, and Ravel delivers it |
| `_BRIEF.md`, line 240 | `plugins/grove/skills/grove/references/editorial.md` | WritingSkills, `plugins/writing/skills/editorial-family/SKILL.md`. Grove removed its copy in `31ec9117c9da` |
| `_BRIEF.md`, the same pointer | Each `grove-<stage>/SKILL.md` | WritingSkills, `plugins/writing/skills/grove-<stage>/SKILL.md` |
| `_BRIEF.md`, line 242 | `plugins/grove/skills/grove-design/references/visual-design-discussion.md` | DesignSkills, `plugins/design/skills/grove-design/references/visual-design-discussion.md` |
| `16-k8/09-k13/_evaluation-harness.md`, line 198 | `plugins/install.sh` and `plugins/install.test.sh` as precedents for the arm's installation | Removed. The revised arm names its pack checkouts as `local` Ravel sources instead |
| `16-k8/09-k13/03-k32/_codex-adapter.md`, line 20 | `plugins/install.sh` links each skill into `~/.codex/skills/` | Removed. No native copy exists for any harness |
| `17-planning--guidance-increments-k9.md` | Cuts the guidance increments | Each increment now changes a pack repository and reaches users through the pack-change steps |
| `18-impl--revised-trials-k10.md` | The revised-skill arm | The arm's skills come from the WritingSkills checkout and the other changed packs, beside core |

### Text for the root brief

#### Rebase onto the split skill corpus

Grove #25 moved the skill corpus into domain packs while this grove was
paused. `docs/paused-grove-handoffs.md` in Grove, section "The editorial
grove", records the moves for this grove. Read it before the next session.

- The editorial kinds and their family file ship in WritingSkills, in the
  plugin `writing`. The family file is the skill `editorial-family`. Each
  stage charter is `plugins/writing/skills/grove-<stage>/SKILL.md`.
- The visual guide is in DesignSkills, at
  `plugins/design/skills/grove-design/references/visual-design-discussion.md`.
- The requirements kinds are in RequirementsSkills. The spine and the tree and
  artifact formats stay in Grove's `plugins/grove`.
- Homebrew installs every skill source, and Ravel delivers them. `plugins/install.sh`
  is gone, and no harness holds a native copy of a skill.
- A guidance change lands in the pack repository that owns the skill. It
  reaches users through a pack commit, `plugins/packs.json`, `task check:packs`
  and a Grove patch release.

Keep craft guidance separate from kind charters. A rule that one editorial
stage owns goes in that stage's charter. A rule that the whole pipeline shares
goes in `editorial-family`. Reusable writing guidance goes in a craft skill in
`plugins/writing/craft/`, such as `technical-communication`. Name each skill by
its bare name.

The revised-skill arm cannot install the checkout's `plugins/grove`. Name the
WritingSkills checkout, Grove's `plugins/grove` and each other changed pack as
`local` sources in an isolated Ravel settings file. The current-skill arm keeps
the bytes of its base release, as its frozen record states.

Rebase this line onto Grove's `main` after Grove #25's finish moves `main`.
Expect conflicts in `CONTEXT.md`, `CONTEXT-MAP.md`, `Taskfile.yml`,
`scripts/check.sh` and `crates/grove/tests/reference_navigation.rs`. Then
correct the pointers at lines 240 and 242 of this brief. Also correct the
installer references in `16-k8/09-k13/_evaluation-harness.md` and
`16-k8/09-k13/03-k32/_codex-adapter.md`.

## The design and decomposition grove

| Subject | Value |
| --- | --- |
| Workspace | `~/Development/AgenticOrchestration/grove.improve-initial-design-decomposition-skill` |
| Base of its line | Grove `b4fa1c239312`, as for the editorial grove |
| Last commit of its line | `bf58ccae`, "case-x-herdr-manager-transfer-k930". The working copy above it was empty |
| Retained history | The local bookmark `grove-approved-history-k702` at `95b5067c` keeps the cited commits that an earlier rebase rewrote |

Its line changes these skill files. Each one moved or is new:

| File on its line | Current place |
| --- | --- |
| `plugins/linkuistics/skills/codebase-design/SKILL.md` | DesignSkills, `plugins/design/craft/codebase-design/SKILL.md` |
| `plugins/linkuistics/skills/model-led-development/SKILL.md` | DesignSkills, `plugins/design/craft/model-led-development/SKILL.md` |
| `plugins/linkuistics/skills/simplify-project/SKILL.md` | EngineeringSkills, `plugins/engineering/craft/simplify-project/SKILL.md` |
| `plugins/linkuistics/skills/authoring-conventions/SKILL.md` | PersonalSkills, `plugins/personal/craft/authoring-conventions/SKILL.md` |
| `plugins/linkuistics/PROVENANCE.md` and `.claude-plugin/plugin.json` | Each pack has its own `PROVENANCE.md` and manifest |
| `plugins/linkuistics/skills/decomposition-design/`, new on its line | No place yet. A design craft skill belongs in DesignSkills `plugins/design/craft/`, as the root brief's destination table states. That grove's rebase decides |
| `plugins/grove/skills/visual-design-discussion/SKILL.md`, new on its line | No place yet. The visual guide that it enters is in DesignSkills. That grove's rebase decides |
| `plugins/grove/skills/grove-design/SKILL.md` and `references/visual-design-discussion.md` | DesignSkills, `plugins/design/skills/grove-design/` |
| `plugins/grove/skills/grove/SKILL.md`, `references/grove.md` and `grove-planning/SKILL.md` | Still in Grove's core. This grove also changed them, so the rebase can conflict |
| `plugins/grove/skills/grove/references/decompose.md` | Still in Grove's core. This grove did not change it |
| `plugins/grove/README.md`, `plugins/README.md`, `plugins/CONTEXT.md`, `plugins/grove/conformance.sh`, its test and `conformance/rules.tsv` | Still in Grove. This grove also changed them |
| `.claude-plugin/marketplace.json` | Retired. It lists no plugin |

### Affected entries of the design tree

| Entry | Reference | Current target |
| --- | --- | --- |
| `_BRIEF.md`, lines 253, 255, 258 and 260 | `plugins/linkuistics/skills/codebase-design`, `model-led-development`, `simplify-project` and `plugins/linkuistics/PROVENANCE.md` | The places in the table above |
| `_BRIEF.md`, line 261 | `plugins/grove/skills/grove-design/SKILL.md` | DesignSkills, `plugins/design/skills/grove-design/SKILL.md`. `grove-planning` stays in core |
| `20-k18/_decomposition-guidance.md`, line 106 | `plugins/linkuistics/skills/authoring-conventions/SKILL.md` | PersonalSkills, `plugins/personal/craft/authoring-conventions/SKILL.md` |
| `20-k18/82-k29/_strategy-catalogue.md`, line 100. `21-k20/12-k42/_catalogue-solutions.md`, line 86. `21-k20/08-k38/_catalogue-coordination.md`, lines 1280, 1364, 1366, 1367, 1407 to 1411, 1447 to 1451 and 2389 | `plugins/linkuistics/skills/decomposition-design/…` | The place that the rebase chooses for `decomposition-design` |
| `22-k22/01-k592/` trial leaves k43 to k48 | The arms' frozen `plugins/linkuistics` and `plugins/grove` trees | Unchanged. `export_trees.py` reads each tree from git objects at its pinned revision, not from a working copy |
| `22-k22/01-k592/22-impl--guidance-revision-k24.md` | Revises the skills, the catalogue and the views | Each revision lands in the pack that owns the skill after the rebase |

The pinned revision `M_REVISION`, `c14276be9401`, is a commit of that line. It
is on no remote. The bookmark `grove-approved-history-k702` keeps it
reachable. A rebase rewrites the line, so the bookmark is the only reference
that keeps the trial inputs readable.

### Text for the root brief

#### Rebase onto the split skill corpus

Grove #25 moved the skill corpus into domain packs while this grove was
paused. `docs/paused-grove-handoffs.md` in Grove, section "The design and
decomposition grove", records each moved file and reference. Read it before
the next session.

- `plugins/linkuistics` is gone. `codebase-design` and `model-led-development`
  are design craft in DesignSkills `plugins/design/craft/`. `simplify-project`
  is engineering craft in EngineeringSkills `plugins/engineering/craft/`.
  `authoring-conventions` is in PersonalSkills `plugins/personal/craft/`.
- `grove-design` and its visual guide are in DesignSkills
  `plugins/design/skills/grove-design/`. `grove-planning`, the spine and its
  `decompose.md` stay in Grove's `plugins/grove`.
- Each pack has its own `PROVENANCE.md`. The Claude Code marketplace lists no
  plugin.
- Homebrew installs every skill source, and Ravel delivers them. No harness
  holds a native copy of a skill.
- A change to a pack lands in its repository. It reaches users through a pack
  commit, `plugins/packs.json`, `task check:packs` and a Grove patch release.

Keep craft guidance separate from kind charters. `decomposition-design` and
`visual-design-discussion` are craft skills on this line. Put each in the
`craft/` directory of the pack that owns its subject. Put a change to the
design kind's charter in `grove-design`. Name each skill by its bare name.

Keep the bookmark `grove-approved-history-k702` through the rebase. It keeps
the pinned trial revision `c14276be9401` and the cited approval commits
readable. The frozen trial trees stay valid, because `export_trees.py` reads
them from git objects.

Rebase this line onto Grove's `main` after Grove #25's finish moves `main`.
Expect conflicts in the core spine, `grove-planning`, the conformance files,
`plugins/README.md`, `plugins/CONTEXT.md` and the Taskfile. Move the skill
changes of this line into the packs. Then correct the references at lines 253
to 261 of this brief. Also correct them in `20-k18/_decomposition-guidance.md`
and in the three catalogue node briefs.

## The Rhai workstream

| Subject | Value |
| --- | --- |
| Workspace | `~/Development/AgenticOrchestration/Ravel.gh-issue-2` |
| Working copy | Empty change `069f5cf97bff` |
| Its line | One commit, `78f8ba783e2c`, "rhai-book-use-k74: save the reconfiguration stop point". It changes only `.grove/19-requirements--rhai-book-use-k74.md` |
| Base of its line | Ravel `24a545236185`, the earlier `main` |
| Ravel `main` on 2026-11-13 | `1435acc79024`, also on origin |

Ravel's `main` and the tag `v2.0.0` carry this workstream's `.grove/` tree.
No commit between `24a545236185` and `main` changes a file under `.grove/`.
Thus the rebase of `78f8ba783e2c` onto `main` meets no conflict. Each Ravel
workspace at `main` holds the same live leaves, such as
`release-ravel-2-0-0-k37`.

### Release receipts

ravel-two-publication-k313 cut Ravel 2.0.0 once, on 2026-11-12. It
delivered the release through the keg-only formula `ravel@2` only. The
section [Ravel 2.0.0 publication receipt](coordinated-release-evidence.md#ravel-200-publication-receipt)
holds the full receipt.

| Subject | Identity |
| --- | --- |
| Channel | `versioned-2`, formula `ravel@2`, keg-only |
| Tag | `v2.0.0`, annotated object `d2f74e6d634369538a191dd1a7347b2c0563d392` |
| Tag commit | `ed6cedbf9a14cd088bea13fcfb8b86300cb6aaca` |
| GitHub release | Published `2026-11-12T10:00:02Z`, with three archives |
| Book | `gh-pages` commit `eb704fbf0700d92b1b9a6a1d338782d6e1a061d0`, served at `https://linkuistics.github.io/Ravel/v2.0.0/` |
| Tap | Commit `f7f9b9b0bc4d6353dcd8cd41fb47deabafc14c09`. It adds `Formula/ravel@2.rb` only |
| `ravel-v2.0.0-aarch64-apple-darwin.tar.xz` | SHA-256 `5957840ab33f4331b794539dbd455d801f71c6c6485c781dc290cfb48490231d` |
| `ravel-v2.0.0-aarch64-unknown-linux-gnu.tar.xz` | SHA-256 `a4750ea439450e88621be411183e7bf30d2616f9a545b64a30445df3a9f02bee` |
| `ravel-v2.0.0-x86_64-unknown-linux-gnu.tar.xz` | SHA-256 `906e589f72161ed5dce5db683e6af10963c1952bdf177c6b343d1918928c3dd1` |
| Release evidence | Ravel `target/release-evidence/v2.0.0.json`, untracked. Grove's `docs/ravel-v2.0.0-release-evidence.json` is its byte copy |

The release task stopped once, at its evidence step. The resume followed the
recovery row "Release evidence failed" of book chapter 11, and no second cut
happened. ravel-main-landing-k312 wrote the 2.0.0 changelog. It names the
configuration-format change, `config_format_retired` and chapter 4.

Three commits came after the tag on `main`. `ca3b39ee5567` and
`f34db9a944ce` change book chapter 3 about the tap's trust step.
`1435acc79024` adds Grove's upgrade order and the off value to the 2.0.0
notes. ravel-evidence-step-invocation-k332 can add one more commit, for the
Taskfile's evidence step.

Finish promotes the same archives to the unversioned `ravel` formula, after
Grove's release and its installed verification. Its receipt goes in
[Promotion and switch receipts](#promotion-and-switch-receipts).

### Owner-conversion receipt

owner-policy-conversion-k245 converted the owner's policy to Rhai on
2026-11-07. The section [Policy conversion](owner-skill-delivery-evidence.md#policy-conversion)
holds the record.

| Subject | Value |
| --- | --- |
| Source | `~/.config/ravel/policy.ts`, SHA-256 `30a0e5fca7c1446999bd19289566fc078053a503c75f03ca7056e641f836f131` |
| Approved candidate | Grove's `docs/evaluations/owner-skill-delivery/owner-config.rhai`, SHA-256 `bbde65efccbfd09e8ac84ab41e1322d35fa31b9726b8451466ce873dd22f88d8` |
| Equivalence | `task owner:policy-equivalence`: 123 of 123 cases passed for the candidate and for the installed copy. Two mutated controls failed only their own rows |
| Installation | The owner approved the candidate on 2026-11-07. The leaf copied it to `~/.config/ravel/config.rhai`, beside the unchanged `policy.ts` |
| Settings | No `~/.config/ravel/settings.json` existed, and the leaf wrote none |

The owner changed the installed `config.rhai` after the conversion. On
2026-11-08 its `lead` selected `sol` and its `review` selected `sonnet`, with
SHA-256 `0ccd5cdaa954…`. On 2026-11-13 the installed file had SHA-256
`21ca2e515718ef638b6a5fecea4e4b099e93c7f4c4a50b6e4b7a0434a4336a6d`, modified
2026-11-11. No record names that digest. `policy.ts` still had the converted
digest `30a0e5fc…`.

After the reap of the last driver on Ravel 1.1.0, these owner steps remain:

- Remove `~/.config/ravel/policy.ts`.
- Remove the 1.1.0 keg with `brew cleanup ravel`. Do it outside the finish
  session of its last driver. The owner's manual states when the owner may
  remove the Homebrew cleanup protection for `ravel`.
- Add any new settings keys, such as `harnesses`. Ravel 1.1.0 refuses them.

### The book work of this grove

This grove ran the editorial pipeline over chapters 7 to 11 before the cut.
The human chose that scope on 2026-11-11. All the work ran in the workspace
`Ravel.skill-directory-delivery`, on the bookmark `skill-directory-delivery`.

| Stage | Leaves | Result |
| --- | --- | --- |
| Draft | ravel-book-inside-k329, -k330 and -k331 for chapters 7, 8 and 9. ravel-book-inside-k308 for chapters 10 and 11 | Each chapter describes 2.0.0. Chapter 11 holds the complete example that refuses a lone `policy.ts` |
| Copy-edit | ravel-book-inside-k309 | Ravel change `lsvquslm` |
| Art | ravel-book-inside-k310 | Ravel commit `7d9526e9`. It revised five figures and added three |
| Proof | ravel-book-inside-k311 | Ravel commit `424af53d3f0c`. It read all eleven chapters, and its reviewer checked each Read 1 pattern in each chapter |
| Landing | ravel-main-landing-k312 | Ravel commit `935f0dbc8f7a`, then `main` |

The proof's evidence is Ravel's `docs/evidence/ravel-book-k311/README.md`.
Lesson L75 in `docs/documentation-lessons.md` records the eight
installed-prefix counterexamples and the correction with explicit settings.
These stages are not the owner's reads. Read 1 and Read 2 stay with the Rhai
workstream.

### Text for the root brief

#### Receipts from Grove #25

Grove #25 released Ravel 2.0.0 and converted the owner's configuration while
this workstream was paused. `docs/paused-grove-handoffs.md` in Grove, section
"The Rhai workstream", holds the receipts. Its section "Promotion and switch
receipts" holds the promotion. Read both before the next session.

- **Release.** Ravel 2.0.0 is public. The tag is `v2.0.0` at
  `ed6cedbf9a14`, and the formula is the keg-only `ravel@2`. The book is at
  `https://linkuistics.github.io/Ravel/v2.0.0/`. The release ran once, on
  2026-11-12.
- **Promotion.** Grove #25's finish promotes the same archives to the
  unversioned `ravel`. The 1.1.0 keg stays until the owner removes it.
- **Configuration.** owner-policy-conversion-k245 installed the owner's
  `~/.config/ravel/config.rhai` on 2026-11-07, after 123 equivalence cases
  passed. `policy.ts` stays for the 1.1.0 driver.

Do not run `release-ravel-2-0-0-k37`. Its `task release:major` would cut
3.0.0, because `release-prepare.sh` bumps when `main` has changes. Never cut
another version for 2.0.0. Prune or retarget these three leaves with the
human's confirmation:

- `release-ravel-2-0-0-k37`: the release receipt meets its goal. Prune it,
  or retarget it to read the receipt into this workstream's records.
- `homebrew-upgrade-k38`: the promotion receipt meets its goal. Prune it. If
  the receipt section is empty, stop and ask the human. Do not run
  `brew upgrade ravel`.
- `owner-policy-conversion-k39`: the conversion receipt meets its goal. Prune
  it. The owner steps after the reap of the last 1.1.0 driver stay with the
  owner.

Grove #25 ran draft, copy-edit, art and proof over chapters 7 to 11 before the
cut. Its proof read all eleven chapters. Ravel's
`docs/evidence/ravel-book-k311/README.md` holds the evidence. The owner's Read
1 and Read 2 stay with this workstream.

Reconcile `rhai-book-inside-k35` and `rhai-book-inside-k36` with the released
text. Compare each Done when with the chapters at `main` and with the proof
evidence. Retire a leaf that the released text meets, and retarget a leaf
with a remaining gap. `main` changed chapter 3 after the tag.

Resume Read 1 in `rhai-book-use-k74` at the setup section of chapter 4. Cut
Read 2 after Read 1's corrections, as section 4 of `docs/book/contract.md`
requires. TestSubject moved to
`/Users/antony/Development/AgenticOrchestration/TestSubject`. Correct the old
path at line 467 of `rhai-book-use-k74`.

Rebase the commit `78f8ba783e2c` onto Ravel's `main` before the next session.
No commit of `main` after `24a545236185` changes a file under `.grove/`.
Root Done when items "Ravel 2.0.0 is released" and the owner's installation
now have receipts. This workstream decides whether they are complete.

## Promotion and switch receipts

Finish fills this section. Until finish writes a value, each row states
"Not yet recorded". finish-k343 wrote each value on 2026-11-13. An empty row means that the step did not happen. A paused
workstream treats an empty row as a reason to stop and ask the human.

| Receipt | Value |
| --- | --- |
| Grove release that the promotion follows | Grove 22.9.0, tag `v22.9.0`, release commit `31fbab3ea935`. `task release:minor` published it on 2026-11-13. |
| Promotion: `task release:promote` run, tap commit and `Formula/ravel.rb` version | `task release:promote VERSION=2.0.0` exited 0 on 2026-11-13. Tap commit `06c719813671`. `Formula/ravel.rb` names version 2.0.0. The receipt is Ravel's `target/release-evidence/v2.0.0.promotion.json`. |
| Promotion: `ravel --version` on `PATH`, and the state of the 1.1.0 keg | `ravel 2.0.0`. The 1.1.0 keg `/opt/homebrew/Cellar/ravel/1.1.0` stays, and it reports `ravel 1.1.0`. The `ravel@2` keg stays too. |
| `native-off` after the promotion | It wrote the off value, and the real HOME resolved as `disabled`. `status` reported the health "native". |
| Owner skill switch: result, receipt path and time | Switched at 2026-11-13T11:46:30Z by attempt `20261113T114254090051Z-21a4bc`. The receipt is `~/.local/state/owner-switch/receipts/20261113T114254090051Z-21a4bc.json`. Two earlier attempts failed and restored native delivery, as [Finish](owner-skill-delivery-evidence.md#finish) states. |
| Remaining owner actions after finish | Release the Claude Code hold. Run the purge report before purge. After the last Ravel 1.1.0 driver ends, remove `~/.config/ravel/policy.ts`, the 1.1.0 keg and the Grove 22.8.0 keg. Run the manual's Synchronise the record workflow for the checker's skill rows. [After finish](finish-handoff.md#after-finish) gives each action. |

## Checks of this document

A sweep repeated the search of "References that this grove cannot edit" in
the [moves record](repository-moves-record.md#references-that-this-grove-cannot-edit)
on 2026-11-13. It used `rg --hidden --no-ignore` with the same pattern over
the live entries of the three trees. It found the references of that table and
no other. The same command found matches in this grove's own tree, the
positive control. A mutated pattern found none in the design tree.

The VCS readings used `jj --ignore-working-copy`, so no paused workspace
recorded a snapshot or an operation.

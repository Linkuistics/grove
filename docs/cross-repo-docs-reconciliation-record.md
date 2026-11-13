# Cross-repository documentation reconciliation record

`cross-repo-docs-reconciliation-k319` made the documents and configuration of
the other affected repositories describe the final common path. This record
holds the enumeration that came before each correction, the class of each
statement, and the publication of each change. It records no release and no
owner switch.

## Scope

The enumeration covers these subjects:

| Subject | Location | `main` before the edits |
| --- | --- | --- |
| RequirementsSkills | `~/Development/AgenticOrchestration/RequirementsSkills` | `c51ec8bebd81` |
| DesignSkills | `~/Development/AgenticOrchestration/DesignSkills` | `aa088156d1a5` |
| EngineeringSkills | `~/Development/AgenticOrchestration/EngineeringSkills` | `8d1a2bc3e07b` |
| ResearchSkills | `~/Development/AgenticOrchestration/ResearchSkills` | `8f321f2e99e0` |
| WritingSkills | `~/Development/AgenticOrchestration/WritingSkills` | `578f84d1f5a9` |
| PersonalSkills | `~/Development/AgenticOrchestration/PersonalSkills` | `cddcfde901cd` |
| TestAnyware | `~/Development/AbstractingComputers/TestAnyware` | `cd2bcf5f35de` |
| The tap | `~/Development/Delivery/homebrew-taps` | `f7f9b9b0bc4d` |
| The maintained manual | The Obsidian note `Configuration/AI Dev Tooling`, with its maintenance implementation under `_meta/scripts` | Not under version control |

`jj git fetch` found no change in each repository, and each local `main` was
the remote `main`.

A change inside an accepted source root moves the root's `resourceDigest`. The
roots are a pack's `skills` and `craft` directories. No edit of this leaf is
inside a root, so no pin in Grove's pack record or in a formula moves.

## Method

A seed grep listed each line outside the roots that matched an installation,
tap, trust, marketplace, plugin, Ravel, native-route, keg or skill-delivery
term. It read the tracked text files of each pack repository and the
documents, templates and scripts of TestAnyware. The list was a seed, not the
scope. This leaf read each hit in its context, with its section.

Each statement has one of three classes, as in
[the Grove reconciliation record](grove-docs-reconciliation-record.md):

- **current**: true of the final common path, before or after promotion as
  it says.
- **historical**: a dated record of a past state. It stays unchanged.
- **stale**: presented as a current fact or instruction, but false. It was
  corrected.

Line numbers name the file before its edits.

The final common path has these facts:

1. Homebrew 6.0.0 and later do not trust a tap that `brew tap` adds. A fully
   qualified name trusts only the formula that it names. Homebrew's
   [Tap Trust](https://docs.brew.sh/Tap-Trust#installing-from-a-tap) page
   states both rules.
2. Grove's formula loads its dependency `ravel` from the tap.
   homebrew-tap-trust-docs-k337 observed that a new user's short-name
   install of `grove` fails without tap trust. Homebrew's page does not state
   the rule for a dependency.
3. Ravel 1.x delivers no skill. While the unversioned `ravel` formula is 1.x,
   Ravel 2 is the keg-only `ravel@2`. Grove's README, section "Skills", and
   chapter 3 of Ravel's book give its route.
4. Grove's formula carries core and the five domain packs from the release
   after 22.8. The tap's `grove.rb` is still 22.8.0.
5. `personal` and `testanyware` load no other formula from the tap.

## RequirementsSkills, DesignSkills, EngineeringSkills and ResearchSkills

The four READMEs share one template. The table gives the lines of
RequirementsSkills. The lines of the others are in the last column.

| Lines | Section | Statement | Class | Design, Engineering, Research |
| --- | --- | --- | --- | --- |
| 30-33 | Layout | The source directory is the Claude Code plugin root and the Ravel source directory | current | 44, 54, 22 |
| 41-46 | Install | `brew tap Linkuistics/taps`, then `brew install grove` | stale: fact 1. Now `brew tap`, `brew trust`, `brew install grove`, with the reason and a link to Grove's README | 61-66, 72-77, 33-38 |
| 48-51 | Install | Grove's formula carries core and the five domain packs | stale: fact 4. Now "from the release after 22.8" | 68-71, 79-82, 40-43 |
| 51-53 | Install | The keg holds the source at `share/ravel/skills/<id>`, and Homebrew links it | current | 71-73, 82-84, 43-45 |
| 55-59 | Install | Ravel 2.0.0 reads that directory with no sources. No native marketplace entry or installer step | current. "Ravel delivers" became "Ravel 2 delivers" | 75-79, 86-90, 47-51 |
| none | Install | The Ravel 1.x condition | stale by omission: fact 3. A paragraph now names it and links Grove's README | |
| 63-66 | Development | A checkout as a trial source in the `skills` object | current | 83-86, 94-97, 55-58 |
| 83-84 | Checks | `check:plugin` runs `claude plugin validate` | current | 104-106, 112-113, 75-76 |

The plugin manifests describe the kinds and the craft skills only. They are
current.

## WritingSkills

| File | Lines | Statement | Class |
| --- | --- | --- | --- |
| `README.md` | 44-79 | The two hooks plugins, Ravel delivers the skill, no route installs the writing plugin, the keg's marketplace files | current |
| `README.md` | 136-137 | `brew tap Linkuistics/taps`, then `brew install grove` | stale: fact 1. Corrected as in the other packs |
| `README.md` | 140-141 | Grove's formula carries the five domain packs | stale: fact 4. Corrected |
| `README.md` | 146-150 | Ravel 2.0.0 delivers the skills of both roots | current. This leaf added the Ravel 1.x paragraph: fact 3 |
| `README.md` | 152-183 | The hook adapter commands for Claude Code, Codex and Pi | current. They equal Grove's `docs/USAGE.md`, "The hook adapters" |
| `README.md` | 250-339 | Provenance of the moves and of the Homebrew commit | historical |
| `docs/TECHNICAL-COMMUNICATION.md` | 56-57 | Grove's formula carries this pack, and Ravel delivers the skill | current |
| `docs/TECHNICAL-COMMUNICATION.md` | 59-62 | `brew tap Linkuistics/taps`, then `brew install grove` | stale: fact 1. Now with `brew trust`, the reason, and the Ravel 1.x condition |
| `docs/TECHNICAL-COMMUNICATION.md` | 66-112 | The hook adapter commands, the Codex trust review, the Pi link, Gemini CLI | current |
| `docs/specs/technical-communication.md` | 47, 75-94, 469-479, 545 | The keg holds the marketplace files, no route installs the writing plugin, Codex re-registration, the Pi link | current |
| `docs/adr/technical-communication-carries-no-standard-text.md` | 10-13 | Grove's keg carries the skill. Grove published it earlier through its marketplace and installer | current, and historical for the earlier routes |
| `docs/design/technical-communication/harness-map.d2` | 38-40 | Hooks plugins and the Pi extension from Grove's keg | current |
| `scripts/hook-count.py` | 6-7 | "a native user of the writing plugin also gets the hooks" | stale: no route installs the writing plugin. The docstring now says so and states what the count keeps |
| `scripts/hook-writes.py` | 4-5 | A Homebrew keg holds the `writing` source, and Ravel digests it | current |
| `docs/research/harness-hooks-a.md`, `docs/evaluations/`, `docs/technical-communication-publication-review.md` | all | Dated surveys, evaluations and reviews | historical. The README states that these keep their bytes |

The two hooks plugin manifests sit inside the `craft` root. Their descriptions
say that Ravel delivers the skill from Grove's Homebrew keg, which is current.

## PersonalSkills

| Lines | Section | Statement | Class |
| --- | --- | --- | --- |
| 30-32 | The guardrail plugin | `guardrail` is a native plugin, not a Ravel skill. The formula carries it and a marketplace file | current |
| 83-90 | The dependency | A native user of the former `linkuistics` plugin received `guardrail`. No route installs the `personal` plugin now | current, and historical for the former plugin |
| 101-104 | Install | `brew tap Linkuistics/taps`, then `brew install personal` | stale: fact 1. Now `brew install linkuistics/taps/personal`, with the trust rule and fact 5 |
| 106-110 | Install | The keg layout and the marketplace file | current |
| 112-116 | Install | Ravel 2.0.0 reads that directory | current. This leaf added the Ravel 1.x paragraph, with a link to chapter 3 of Ravel's book: fact 3 |
| 120-130 | The guardrail hook adapter | `claude plugin marketplace add "$(brew --prefix)/opt/personal"`, then `claude plugin install guardrail@personal` | current. It equals Grove's `docs/USAGE.md` |
| 147-153 | Checks | No installer reads `harnesses:` | current |
| 171-207 | Provenance | The moves and the Homebrew commit | historical |

## TestAnyware

| File | Lines | Statement | Class |
| --- | --- | --- | --- |
| `README.md` | 227 | `brew install Linkuistics/taps/testanyware` | current: facts 1 and 5 |
| `README.md` | 236-237 | The formula installs the skill, and Ravel delivers it | current |
| `README.md` | 643-648 | The archive and keg layout of the skill source | current |
| `README.md` | 650-651 | "`brew install` and `brew upgrade` are the whole skill setup" | stale: the formula does not install Ravel, and Ravel 1.x delivers no skill. Now "install and update the skill source with no other step" |
| `README.md` | 655-658 | Ravel 2.0.0 delivers the skill | current. The Ravel 1.x paragraph was added, with a link to chapter 3 of Ravel's book |
| `README.md` | 664-670 | A checkout as a trial source, `task plugin:check` | current |
| `LLM_INSTRUCTIONS.md` | 183 | `brew install --cask linkuistics/taps/testanyware-app` | current: a fully qualified name |
| `docs/specs/testanyware-app.md` | 193-196 | The formula installs the Ravel source, and `release_candidate.py verify` compares it | current |
| `docs/specs/admission-queue.md` | 940-941 | "The `using-testanyware` skill in the Grove repository" in "Documents that change" | historical: the change of 2026-10-26 was in Grove. A sentence now names the skill's present path |
| `scripts/release-publish.sh` | 136-137 | `brew install linkuistics/taps/testanyware` and the cask | current |
| `scripts/templates/testanyware.rb.tmpl` | 44-62, 85-90 | The private payload and relative alias of the keg layout | current |
| `docs/superpowers/plans/2026-05-29-migrate-legacy-groves.md` | 64 | `.claude/skills/grove/` | historical |

The release template keeps the keg layout of
testanyware-keg-projection-k296. This leaf did not change it. `task
release:test` asserts the private payload, the relative alias and the absence
of a real directory in `share/ravel/skills`.

## The tap

The tap has no README. Its formula documentation is each formula's `desc`,
comments and caveats.

| Formula | Statement | Class |
| --- | --- | --- |
| `personal`, `superpowers`, `quint-llm-kit` | The caveats name the private copy and say that Ravel 2.0.0 delivers it through the alias with no sources | current |
| `testanyware` | The comments describe the keg layout | current |
| `grove` | The 22.8.0 caveats: Codex provisioning, the Claude Code marketplace, the installer, `policy.ts` | current for the published 22.8.0. Grove's release replaces the formula from `scripts/templates/grove.rb.tmpl`, which is current |
| `ravel`, `ravel@2` | No caveats. `ravel@2` is `keg_only :versioned_formula` | current |

No formula carries the short-name route that fact 1 refuses. The release
tooling of each project renders its formula, so no tap file changed.

## Homebrew's link suggestion for `ravel@2`

promotion-upgrade-rehearsal-k318 observed the caveat `Run brew link ravel@2
to switch the active version to this keg.` beside a linked `ravel` 1.1.0.
No Grove or Ravel page repeats that suggestion for states B and C.

The search used `rg --hidden` with the pattern ``brew link( |`|$)``, case
sensitive, over Grove's workspace without `.grove/` and over Ravel's `main`
at `1435acc79024`. These were the controls:

- The positive control found the quotation in
  `docs/promotion-upgrade-rehearsal-record.md` of Grove.
- A mutated pattern, ``brew linkx( |`|$)``, found nothing.

Each other match described what `brew link` does, in a comment, a test or a
record. One match was an instruction: "For an affected old formula, `brew
unlink <formula>` followed by `brew link <formula>` is recovery" in "The
keg layout" of `docs/specs/codex-skill-provisioning.md`. It concerns
the skill formulae of an old layout, not `ravel@2`.

## The maintained manual

The manual records the owner's live machine. It is append-only: a correction
is appended and no earlier observation is rewritten. The owner design's
section "Cutover contract" states that rule.

| Lines | Section | Statement | Class |
| --- | --- | --- | --- |
| 100 | System map | Global commands, with Grove from Homebrew | current |
| 101-104 | System map | Claude marketplace plugins and skills, Codex skills, Pi skills, `~/.agents/skills` | current until the switch |
| 105 | System map | "Grove/Linkuistics in `~/Development/grove`" | stale. The directory no longer exists. Corrected by the appended section "Correction, 2026-11-13: Grove's checkout" |
| 115-118 | Claude Code | Marketplace plugins, and the `guardrail@personal` registration of 2026-11-11 | current until the switch, and dated |
| 154-157 | Shared skills | Claude marketplace plugins, Codex provisioning by the installed Grove binary, the Pi and Gemini installer, Quint links | current until the switch. Installed Grove is 22.8.0 |
| 160-164 | Shared skills | The machine-environment guide deployed through Grove 22.7.0, on 2026-10-25 | historical |
| 166 | Shared skills | Grove and Quint are development working copies | current |
| 183-192 | Shared skills | The checkout move, and a build from an empty temporary `~/Development/grove` | historical |
| 257 | Configuration ownership | mise does not track `skills/`, because the installers and Grove provision them | current until the switch |
| 273-288 | Bootstrap and recovery | Step 4 restores Claude marketplaces. Step 5 uses Grove's Codex provisioning and `plugins/install.sh` | current until the switch. Grove `main` at `238c3a839ab4` still holds `plugins/install.sh` |
| 305 | Safety | Grove/Linkuistics and Quint are protected checkouts | current |

The statements marked "current until the switch" become stale at the owner
skill switch. The switch program appends only its activation observation.
Their correction therefore needs an observation after the switch.
finish-handoff-k324 now carries that obligation.

The maintenance implementation defaulted `AGENT_TOOLING_GROVE_REPOSITORY` to
`/Users/antony/Development/grove`. The check skips a repository whose path
does not exist, so the status page lost its row "Grove / Linkuistics
source". The default now names `/Users/antony/Development/AgenticOrchestration/grove`.
The fixture key of the same path in the test file moved with it. The switch
plan names the manual by path only, and it pins no vault file.

The manual's verification procedure gave these results:

- `ruff check`, `uvx pyright`, `shellcheck -s bash` and `zsh -n` passed.
- `pytest` gave 9 failures and 33 passes, with and without the change. The
  failing tests were the same. Each runs the command-line program with `PATH`
  set to the test stubs, `/usr/bin` and `/bin`. The launcher then starts
  Xcode's Python 3.9, which has no `datetime.UTC`. This failure is older than
  this leaf.
- The forced check exited 20. Its status page shows the row "Grove /
  Linkuistics source: Live checkout is status-only". The failed rows are the
  dangling Pi skill links, three Obsidian community plugins and Codebase
  Memory. The page says that the last fully successful check was
  2026-08-18.

This leaf read the manual back after the append. Its bytes equal its bytes
before the append, followed by the correction.

## Checks and publication

Each changed repository passed its own checks before publication:

| Repository | Checks | Result |
| --- | --- | --- |
| RequirementsSkills, DesignSkills, EngineeringSkills, ResearchSkills | `task check`: conformance alone and in the aggregate, `check:plugin` | passed |
| WritingSkills | `task check`: conformance, `check:plugin`, `check:hooks` with its controls, `check:hook-writes`, the checker and Pi tasks | passed |
| PersonalSkills | `task check`: `check:plugin`, `check:frontmatter`, `check:guardrail-test`, `check:shellcheck` | passed |
| TestAnyware | `task release:test` and `task plugin:check` | passed |

The checker `ste-lint.py` of `technical-communication` gave the same counts
before and after the edits for each changed document.

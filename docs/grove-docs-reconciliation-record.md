# Grove documentation reconciliation record

`grove-docs-reconciliation-k317` made Grove's guides, specifications, formula
caveats and release notes describe the final common path for the compatible
Grove release. This record holds the enumeration that came before each
correction. It records no release and no owner change.

## Method

A script listed each line of the named files that matched one of eight topic
classes:

| Class | What it seeds |
| --- | --- |
| provision | Removed provisioning claims: `~/.agents/skills`, `install.sh`, plugin installation, snapshots |
| embed | Embedded-file references |
| inventory | Package inventories: the packs, `plugins/packs.json`, the other formulae |
| native | Native installation routes |
| fmt | `policy.ts`, `config.rhai`, `config_format_retired` |
| ravel2 | `ravel@2` |
| defdir | Ravel's default skill directory |
| market | The marketplace |

The list was a seed, not the scope. Each statement was read in its context,
and nearby statements on these topics that the seed missed were added. Each
statement has one of three classes:

- **current**: true of the compatible release, before or after promotion as
  it says.
- **historical**: a record of a past state that says so, such as "Grove 22.8
  and earlier". It stays unchanged.
- **stale**: presented as a current fact or instruction, but false for the
  compatible release. It was corrected.

"Native route" stayed a current term. It names Ravel's caller route that reads
an existing native installation. Only a statement that sets up a native
installation, or offers one as a setup route, was stale.

The tables give the lines of each file before its edits. The section
`## Unreleased` of `CHANGELOG.md` was enumerated by entry.

## Additions after the enumeration

- `docs/USAGE.md` gained the subsection "Promotion" under "Launch policy". It
  states the deprecation of `ravel@2`, the `config_format_retired` refusal
  and the conversion to `config.rhai`. It also states the route for a Grove
  that Homebrew cannot upgrade.
- `docs/USAGE.md` gained "Known limits" at the end of "The directory route".
  It states the `token_budget` limit of Codex and the recovery finding of
  `RAVEL_SKILL_DIRECTORIES`. It names the later design that puts the variable
  in Claude Code's system prompt or Codex's developer instructions. The
  [gap register](common-loading-acceptance-record.md#gap-register) holds the
  evidence.
- A sweep after the edits found two more claims that `brew install` is the
  whole skill setup, in "What happens in a session" of `docs/USAGE.md` and in
  "The keg layout" of the delivery specification. Both now carry their
  condition.
- The paragraph "Before the cut" in "Packaging and rollout" of the delivery
  specification moved to the past tense. It names `grove-ravel-two-pins-k314`
  as the repeat against the published archives.

## Counts

| File | Statements | Current | Historical | Stale |
| --- | --- | --- | --- | --- |
| `docs/USAGE.md` | 60 | 47 | 5 | 8 |
| `docs/ARCHITECTURE.md` | 49 | 36 | 8 | 5 |
| `docs/specs/codex-skill-provisioning.md` | 104 | 93 | 2 | 9 |
| `README.md` | 13 | 11 | 1 | 1 |
| `docs/RELEASING.md` | 17 | 12 | 1 | 4 |
| `docs/specs/harness-selection-and-execution.md` | 9 | 5 | 0 | 4 |
| `scripts/templates/grove.rb.tmpl` | 11 | 8 | 0 | 3 |
| `CHANGELOG.md`, `## Unreleased` | 17 | 12 | 0 | 5 |

A current row can still carry an addition, such as the promotion text of the
README. Its "Correction" cell says so.

<!-- tc: off because the tables below are the enumeration, written as terse records -->

## Enumeration

### docs/USAGE.md

Lines refer to the file before the edits. "missed" marks a statement that the
grep seed did not list. FP marks a grep hit on an unrelated sense of the word.

| # | Lines | Section | Tags | Statement (quote or paraphrase) | Class | Reason | Correction |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 22-24 | Using Grove (intro) | inventory, missed | The methodology a session executes is "the `grove` plugin's own skills" | stale | Most kind skills ship in the five domain packs, not in the `grove` plugin | "the skills of the `grove` plugin and the five domain packs" |
| 2 | 45-48 | Launch policy | native | Three routes. Directory route supplies Homebrew's skills. Native routes use a native installation that already exists. Guidance sets up none | current | Matches facts 6 and 7 | |
| 3 | 52 | Launch policy | native, fmt | Table row: native on 1.1, Homebrew's `ravel`, `policy.ts` schema 2, 1.1 settings keys, TypeScript set | current | Fact 11, spec "Supported routes" | |
| 4 | 53 | Launch policy | native, fmt, ravel2 | Table row: native on 2.0, keg-only `ravel@2` until promotion, `config.rhai` v4, `"sources": []` while keg sources are installed | current | Qualified by "until promotion"; facts 7, 9 | |
| 5 | 54 | Launch policy | native | Table row: directory on 2.0, no settings or settings without `sources` | current | Facts 6, 9 | |
| 6 | 56-57 | Launch policy | ravel2, missed | "Install Ravel independently with `brew install linkuistics/taps/ravel`" | stale | Grove's formula depends on unversioned `ravel`, so Homebrew installs it with Grove (fact 9, spec "Installation and bootstrap") | Now says Grove's formula installs Homebrew's `ravel` as a dependency, and gives the brew command as the other route. Rest of the paragraph kept |
| 7 | 58-60 | Launch policy | ravel2 | "Homebrew installs Ravel 2.x as the keg-only `ravel@2`, so its `ravel` is never the default command" | stale | False after promotion (state D), when unversioned `ravel` is 2.0.0 (fact 10) | Qualified with "Until a tap promotion makes Homebrew's `ravel` 2.x" and "only"; "never" becomes "not" |
| 8 | 62-66 | Launch policy | ravel2 | Refusal names both majors and the `ravel@2` directory. Grove refuses 1.0.x, prereleases, 3.x | current | Refusal content, admission rule | |
| 9 | 68-72 | Launch policy | fmt, missed | Both configuration files coexist. The settings file cannot serve both majors with every key | current | Fact 11 | |
| 10 | 74-78 | Launch policy | fmt | `policy.ts` on 1.x, `config.rhai` with `SCHEMA_VERSION` 4 on 2.x. Grove converts no configuration | current | Fact 11 | |
| 11 | 80-83 | Launch policy | ravel2, missed | "Ravel has exactly two commands, `start-session` and `end-session RESULT` ... It has no init, inspect or record command" | stale | Ravel 2.0.0 also has `skills resolve` (checked with `ravel@2 --help`), which line 164 tells the user to run | Restricted the two-command claim to Ravel 1.x, added "Ravel 2.x adds `skills resolve`", and "Neither has an init, inspect or record command" |
| 12 | 99-110 | Launch policy | fmt | The same route in `config.rhai` for 2.x | current | Fact 11 | |
| 13 | 112-117 | Launch policy | embed | Caller assets, one set per major. "Ravel does not embed them as modules" | current | Fact 11. Not a Grove embedding claim | |
| 14 | 135-138 | The directory route | native | Ravel 2.0.0 adds source locations to the prompt, so no harness needs a native copy | current | Facts 5, 6 | |
| 15 | 140-150 | The directory route | inventory | Grove's formula installs core and the five domain packs; table of six sources | current | Fact 4 | |
| 16 | 152-156 | The directory route | defdir | Homebrew links each source into the default skill directory. Ravel 2.0.0 reads it with no `sources`, also without settings | current | Facts 5, 6 | |
| 17 | 158-162 | The directory route | inventory | `personal`, `superpowers`, `quint-llm-kit`, `testanyware` formulae install their skills | current | Fact 4 | |
| 18 | 164-166 | The directory route | ravel2, missed | Run `ravel skills resolve` with the Ravel of the invocation | current | Ravel 2.0.0 has the command | |
| 19 | 168-170 | The directory route | market, missed | Hooks are not skills, so Ravel does not deliver them | current | Spec "Hook adapters" | |
| 20 | 173-175 | The directory route | market | Keg holds a marketplace file for hooks. No remote marketplace lists a hook adapter | current | Spec "Hook adapters"; not the retired `linkuistics` marketplace | |
| 21 | 177-182 | The directory route | provision, market, defdir | Registration commands for Claude Code, `guardrail`, Codex and Pi | current | Identical to the spec's documented commands | |
| 22 | 184-192 | The directory route | market, missed | Claude Code runs through `opt`; Codex copies and needs trust review; caveats print the commands | current | Spec "Hook adapters" | |
| 23 | 194-202 | The directory route | defdir, native | Optional `skills` object. `sources` replaces the default. `"sources": []` turns delivery off; native on 2.0 uses it | current | Facts 6, 7, 10 | |
| 24 | 204-215 | The directory route | defdir | Example settings with `/opt/homebrew/share/ravel/skills/...` paths | current | Fact 5 | |
| 25 | 227-231 | The directory route | native | One HOME serves a native 1.1 launch and a directory 2.0 launch | current | Fact 11, spec "Settings and HOME" | |
| 26 | 233-235 | The directory route | defdir | A separate HOME does not move the default skill directory | current | Spec "Settings and HOME" | |
| 27 | 242-245 | The directory route | provision | A running session keeps its Ravel snapshot across `brew upgrade` | current | Ravel capture, not Grove provisioning | |
| 28 | 310-315 | Running a standalone task | provision, native | Directory route reads captured skills. On a native route the harness loads native guidance from HOME | current | Fact 7 | |
| 29 | 590-597 | Running Grove | missed | Missing Ravel stops Grove; transcript remedy `brew install linkuistics/taps/ravel` | current | Transcript of the current refusal text | |
| 30 | 601-603 | Running Grove | inventory, missed | `grove-requirements` ships in RequirementsSkills; Grove's formula installs it; Ravel delivers it | current | Fact 4 | |
| 31 | 646-653 | If a launch refuses | fmt | Transcript of a `policy.ts` refusal | current | Native on 1.1 remains a route | |
| 32 | 773-774 | The task tree and its filename grammar | missed | Kinds that exist are the `grove-<kind>` skills you have installed | current | Homebrew installs them; true on both routes | |
| 33 | 870-876 | What happens in a session | inventory, missed | "the `grove` plugin ... ships one `grove-<kind>` skill per session kind ... the session loads it through its harness's own skill-loading affordance" | stale | Kind skills ship across the plugin and five domain packs; on the directory route the agent reads the skill from Ravel's listed sources (lines 135-137) | Spine sentence kept with its link; "That plugin and the five domain packs ship one skill per kind"; loading split into directory route (reads from a source that Ravel lists) and native route (harness affordance). "A kind exists iff" sentence kept |
| 34 | 879-884 | What happens in a session | inventory | Homebrew installs, Ravel delivers; binary installs none; `brew install`/`upgrade` is the whole setup | current | Facts 1, 4, 6 | |
| 35 | 886-887 | What happens in a session | native | Native route reads an existing installation; guidance sets up none; no installer | current | Facts 2, 7 | |
| 36 | 889-892 | What happens in a session | missed | No skill check before launch; pack change arrives through a Grove release | current | Spec "Installation and bootstrap" | |
| 37 | 895-901 | What happens in a session | provision | Grove 21.11 to 22.8 installed Codex skills; later releases leave the entries | historical | Dated record; the "later releases" clause is also current (fact 1) | |
| 38 | 903-905 | What happens in a session | native, missed | "Keep them, or replace them with links that you choose" | stale | Tells the user to make per-harness skill links, which no Grove guidance now sets up (facts 3, 7) | "You can keep them." Cleanup link sentence kept |
| 39 | 910-914 | Optional cleanup | provision | Cleanup removes only provably provisioner-written, unneeded links; no command or release runs it | current | Spec "Existing installations" | |
| 40 | 916-921 | Optional cleanup | provision | Procedure keeps the Grove root; 15 snapshots used 13 MB in one 22.8 installation | current | Spec; dated measurement kept as history | |
| 41 | 923-931 | Optional cleanup | provision | Each startup of 21.11 to 22.8 wrote the layout under `CODEX_HOME` and each HOME | historical | Past provisioner behaviour | |
| 42 | 933-940 | Optional cleanup | provision | Table of what the provisioner wrote; "the removed manual installer" | historical | Past provisioner and removed installer (fact 2) | |
| 43 | 942-948 | Optional cleanup | provision | Provisioner removed links at names it did not ship; kept no record | historical | Past behaviour | |
| 44 | 950-966 | 1. Stop every writer and reader | provision | Stop drivers, harnesses, and any 22.8-or-earlier executable | current | Spec "Existing installations" | |
| 45 | 970-1001 | 2. Make an inventory | provision | Read-only inventory commands; no Codex directory means no links there | current | Procedure | |
| 46 | 1003-1020 | 2. Make an inventory | provision | Search for retained links and texts | current | Procedure | |
| 47 | 1159-1166 | 3. Classify each entry | provision | Three classes; definition of the provisioner form | current | Procedure | |
| 48 | 1178-1188 | 3. Classify each entry | provision | Conditions for a removable link | current | Procedure | |
| 49 | 1190-1197 | 3. Classify each entry | provision | Conditions for a removable `link-XXXXXX` directory | current | Procedure | |
| 50 | 1199-1202 | 3. Classify each entry | provision, missed | "Keep each link in a separate Codex skill directory, or replace it yourself" | stale | "replace it yourself" invites a new per-harness skill link (facts 3, 7) | Dropped ", or replace it yourself"; the reasons that follow are kept |
| 51 | 1204-1205 | 3. Classify each entry | native | "If you need a skill, keep its link. Alternatively, replace the link yourself with a link to a copy that you choose" | stale | Second sentence tells the user to set up a native skill link (facts 3, 7) | Kept "If you need a skill, keep its link."; removed the "Alternatively" sentence |
| 52 | 1207-1210 | 3. Classify each entry | provision | Manual installer, removed in the release after 22.8, wrote four directories | historical | Fact 2 | |
| 53 | 1212-1222 | 3. Classify each entry | provision | Entries that are always foreign or uncertain | current | Procedure | |
| 54 | 1226-1249 | 4. and 5. | provision | Move entries to a holding directory; restore on failure | current | Procedure | |
| 55 | 1525-1527 | Cutting a research pair | inventory, missed | Research kinds ship in ResearchSkills; Grove's formula installs it; Ravel delivers it | current | Fact 4 | |
| 56 | 1599-1602 | Review composition and escalation | missed | In-session reviewer rules are the `grove` plugin's skills (`references/execute.md`) | current | The spine lives in the `grove` plugin | |
| 57 | 1696-1701 | Two habits for the human in the loop | inventory | Links to `plugins/grove/.../execute.md` and RequirementsSkills `grilling.md` | current | Both paths exist | |
| 58 | 1751-1754 | Supported workspace layouts | native (FP) | "Native (`jj git init`)" jj layout | current | Unrelated sense of "native" | |
| 59 | 1779-1782 | Supported workspace layouts | provision (FP) | jj never snapshots the control directory | current | Unrelated sense of "snapshot" | |
| 60 | 1824-1828, 1845-1846 | What teardown guarantees | provision (FP) | Jujutsu snapshots the working copy | current | Unrelated sense of "snapshot" | |

Totals: 60 rows. 47 current, 5 historical, 8 stale. The stale paragraph at 56-60 is now two paragraphs, to keep each within six sentences.

### docs/ARCHITECTURE.md

Line numbers refer to the file before the edits. The grep seed held 89 rows. Adjacent rows that form one statement are grouped. Rows marked "extra" were not in the seed.

Code checks: `crates/grove/tests/native_skill_delivery.rs`, `crates/grove-llm/tests/instructed_verbs.rs`, `crates/grove-loop/tests/prompt.rs`, `crates/grove-loop/src/ravel_job.rs` (`pub fn launch`), `grove_loop::driver`/`tree_lifecycle::transition_to_current`, `testing/ravel.rs`, `plugins/grove/conformance.sh`, `plugins/grove/conformance.test.sh`, `plugins/grove/conformance/rules.tsv`, `plugins/packs.json`, `plugins/README.md`, Taskfile `check:packs` all exist. The `ravel@2` remedy text is in `crates/grove/src/dispatch/ravel.rs:41`.

| Lines | Section | Tags | Statement | Class | Reason | Correction |
|---|---|---|---|---|---|---|
| 5-7 (extra) | Intro (pending) | native | methodology "delivered through each harness's installation path" | stale | Homebrew installs and Ravel delivers; native installation is no setup route | Replaced with "installed by Homebrew and delivered by Ravel" |
| 48 | Reading map | inventory | Methodology is `plugins/grove/skills/grove/SKILL.md` plus one skill per kind | current | files exist | none |
| 113 | Repository products (tc on) | inventory, defdir | `plugins/grove/` stored at `libexec/skill-sources/grove-core/`, alias `share/ravel/skills/grove-core`, Ravel delivers | current | matches keg layout | none |
| 114 | Repository products | inventory, native | external packs; `guardrail` native plugin; packs.json pins packs; personal and testanyware formulae | current | matches keg layout and spec line 132/155; "such as" list | none |
| 117-120 | Repository products | embed | neither binary embeds or installs the skill source; Homebrew installs, Ravel delivers | current | facts 1 and 4; role statement matches ADR | none |
| 130-135 | Runtime flow (pending) | embed, provision | two advisory steps went with the embedded corpus at `delete-provisioning-k19` | historical | records a removal | none |
| 135-139 | Runtime flow | provision | startup writes no skill installation; the Codex provisioning of Grove 22.8 and earlier is removed | current | fact 1, with history | none |
| 160-163 | Command surfaces | embed | the `methodology` verb served the embed; it went with the mandate machinery | historical | records a removal | none |
| 163-167 | Command surfaces | provision | `--content-hash` went with provisioning at `delete-provisioning-k19` | historical | records a removal | none |
| 192, 196, 207 | Observation / viewer | provision | "snapshot" of the task tree, `ReadGuard::into_snapshot`, `select_snapshot` | current | false positive: tree snapshots, not skill snapshots | none |
| 230, 235, 245, 263, 308 | Observation evidence | native, provision | native lock controls, byte snapshots, `select_snapshot` | current | false positive: OS locks and tree snapshots | none |
| 331-332 (extra) | Launching a session (tc on) | ravel2 | Grove admits 1.x from 1.1.0 and 2.x from 2.0.0 | current | fact 6 and spec | none |
| 334-336 | Launching a session | ravel2 | remedy names both majors and the keg-only `ravel@2`, recommends no shared upgrade | current | matches `dispatch/ravel.rs:41` | none |
| 341-343 | Launching a session | fmt, native | `policy.ts` for 1.x, `config.rhai` for 2.x; "These are the native routes on 1.1 and on 2.0" | stale | `config.rhai` serves both the native and the directory route on 2.0; settings choose | Second sentence replaced: `policy.ts` serves the native route on 1.1, `config.rhai` serves the native and the directory route on 2.0. The paragraph was split after that sentence to keep six sentences or fewer |
| 343-347 | Launching a session | fmt | settings keys differ by major, each refuses the other's | current | fact 11 | none |
| 351-353 | Launching a session | fmt | old `config.kdl` and `.grove.kdl` remain ignored | current | fact 11 | none |
| 412, 420, 430, 438-439 | Observation evidence | provision, native | filesystem snapshots; native process-death; native macOS/APFS controls; native-locking limits | current | false positive: OS platform evidence | none |
| 489 (extra) | Independent Ravel | ravel2 | Ravel 2.x installs one executable, no SDK | current | facts 11 | none |
| 520 | Independent Ravel | native | owner supplies `declared <provider>`, no store | current | false positive: provider evidence | none |
| 525-532 (extra) | Independent Ravel | native | formula depends on Ravel; archive users install Ravel; 22.1.0 installation for the live driver | current | facts 9 and legacy-driver transition | none |
| 540 | Process ownership | native | VCS seam for native, secondary and colocated jj workspaces | current | false positive: jj workspace types | none |
| 626, 683-684, 753, 763 | Task tree / lint | provision | snapshot read and `grove_loop::select_snapshot` | current | false positive: tree snapshots | none |
| 807, 841, 844, 845, 846 | Library refusals table | provision | guard snapshot, `WriteGuard::delete` | current | false positive | none |
| 890, 908, 920 | Library refusals | native, provision | Grove's own check, next snapshot, failed open | current | false positive | none |
| 1040, 1099-1100, 1116 | Lock and store | provision | snapshots, shared lock builds the snapshot, Jujutsu snapshots | current | false positive | none |
| 1208, 1249, 1267, 1269, 1272 | Lint | provision, native | guard snapshot, lint scans snapshot, Grove writes no such file | current | false positive | none |
| 1275, 1282, 1292, 1316 | Key allocation | embed, native, provision | content embeds the key, `NewEntry::empty`, prediction snapshot | current | false positive | none |
| 1450, 1508, 1518 | VCS | provision, native | Jujutsu snapshots the working copy; native jj workspace | current | false positive | none |
| 1537-1542 | How the methodology reaches a session | native | plugin decides any native namespace; missing skill force-stops | current | naming rule, not a setup route | none |
| 1543-1547 (extra, 1546 seeded) | How the methodology reaches a session | provision | "Ravel delivers them to a session in every harness"; no installer; Grove installs no skill | stale | Ravel 1.x delivers no skill, and the default installation stays on the native route on 1.1 | First sentence now says Ravel 2.0 delivers them; added "Ravel 1.x delivers no skill." |
| 1549-1554 | How the methodology reaches a session | provision, embed, native | Grove 22.8 and earlier embedded and provisioned Codex links; writer removed; links left unchanged; test compares entries | historical | explicit 22.8 history; test file exists | none |
| 1557-1563 (extra) | Delivery and build boundaries | provision | skills reach users through a Grove release whose keg holds them | current | keg layout | none |
| 1569-1571 | Delivery and build boundaries | embed, provision | `content/` embedded methodology; `delete-provisioning-k19` removed it | historical | records a removal | none |
| 1572-1573 | Delivery and build boundaries | embed | Codex adapter removed after 22.8 "when skill delivery became the user's choice" | stale | the reason names a superseded design: Homebrew installs and Ravel delivers | Reason replaced with "when Homebrew and Ravel took over skill delivery" |
| 1575-1579 | Delivery and build boundaries | inventory, native | conformance.sh, `task check:packs` over packs.json; Rust tests check no native entry is written and native-only lifecycle completes | current | files and task exist; native-only lifecycle is a current caller route | none |
| 1583 | The seven constraints | inventory | `SKILL.md` carries the spine | current | file exists | none |
| 1653-1654 | Rule ownership | inventory | `rules.tsv` read by conformance.sh | current | files exist | none |
| 1657-1660 | Rule ownership | inventory | only the skills of a pack's source directory are installed | current | keg copies the source | none |
| 1662, 1669 | What is measured | inventory | conformance.sh is the instrument; `check:packs` runs each pack in packs.json | current | files exist | none |
| 1677 | Budgets | provision | `delete-provisioning-k19` deleted the corpus | historical | records a removal | none |
| 1724 | Budgets | provision | budgets deleted at `delete-provisioning-k19` | historical | records a removal | none |
| 1776, 1783 | Budgets | inventory | conformance.sh and `rules.tsv` | current | files exist | none |
| 1799-1802 | The boundary is a build | native, provision | formula installs skills and binaries together; a native installation has its own update boundary; retained skill snapshot | current | an existing native installation stays usable and does not follow `brew upgrade` | none |
| 1810-1813 (1813 seeded) | The boundary is a build | native | "update a native installation through its own route" | stale | the marketplace lists no plugin and no installer exists, so no update route remains | Replaced with "An existing native installation keeps its own contents." |
| 1896 | Verification | fmt | `GROVE_TEST_RAVEL_RUNTIME`: 1.1 for `policy.ts`, 2.0 for `config.rhai`; `testing/ravel.rs` | current | file exists and matches | none |
| 1915, 1922 | Verification | native | native/colocated jj trees; natively on macOS | current | false positive | none |
| 1930 | Verification | embed | anchor `embed-test-seam` | current | anchor kept | none |
| 1937-1939 | Verification | embed | `methodology` used to be a seam over `include_dir` | historical | records a removal | none |
| 1940 | Verification | inventory | corpus is ordinary files under `plugins/grove/skills/` | current | true | none |

### docs/specs/codex-skill-provisioning.md

Line numbers refer to the file before edits.

| # | Lines | Section | Tags | Statement | Class | Reason | Correction |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 3-5 | (intro) | native, provision, embed | Grove implements its native-only route: no embedding or provisioning, startup leaves installations unchanged | current | CHANGELOG Unreleased | - |
| 2 | 5 | (intro) | provision | "Grove 22.8 and earlier still provision them" | historical | names past releases | - |
| 3 | 9-13 | (intro) | native | admits 1.x and 2.x, supports both native routes and the directory route, each passed acceptance on macOS | current | dual-Ravel stage delivered | - |
| 4 | 19-22 | (intro) | native | k216 amended the design; native skill installation retires | current | decision provenance | - |
| 5 | 23-26 | (intro) | market, status | four sections "are new. Their implementation is ahead." | stale | keg layout, hook adapters, pack record and retired marketplace are implemented and tested | Replaced "Their implementation is ahead." with "Grove implements and tests them." |
| 6 | 29-30 | (intro) | defdir | k218 settled the default skill directory | current | provenance | - |
| 7 | 34-38 | Ownership | inventory | Grove's formula carries core and the five domain packs | current | fact 4 | - |
| 8 | 46-48 | Ownership | native | decision record: why native installation retires, nothing writes one automatically | current | ADR | - |
| 9 | 62-67 | Installation and bootstrap | inventory, defdir | formula depends on Ravel, Ravel 2.0.0 reads the default directory, so brew install and upgrade are the whole skill setup | stale | until promotion the dependency is 1.1.0; directory delivery needs `ravel@2` selected (state C) | Prefixed "After promotion," to the whole-setup sentence and added "Until promotion, directory delivery also needs `ravel@2`, selected as Prefix selection states." |
| 10 | 69-72 | Installation and bootstrap | - | settings optional; each hook adapter needs one registration | current | | - |
| 11 | 74-77 | Installation and bootstrap | native | 1.x delivers no directory; finds existing native installations; guidance sets up none | current | fact 7 | - |
| 12 | 79-81 | Installation and bootstrap | provision | harness reads the retained snapshots | current | Ravel's snapshots, not Grove provisioning | - |
| 13 | 87-91 | Installation and bootstrap | embed | binary embeds no core-only fallback | current | fact 1 | - |
| 14 | 113-116 | Installation and bootstrap | - | Ravel reads local directories; Homebrew puts each keg source in place | current | | - |
| 15 | 128-134 | The keg layout | defdir, native, market | keg path table | current | template and tap formulae implement it | - |
| 16 | 136-142 | The keg layout | defdir, native | alias links into the default directory; brew install, upgrade, uninstall keep it current; no native entry | current | about directory content; route selection is in Prefix selection | - |
| 17 | 144-150 | The keg layout | defdir | Ravel 2.0.0 reads the default directory without `sources` | current | 2.0.0 published | - |
| 18 | 152-158 | The keg layout | market, inventory | formula, sources, roots table | current | fact 4, `scripts/skill-formulae.json` | - |
| 19 | 160-165 | The keg layout | ravel2 | `ravel@2` does not link those entries before promotion | current | state C | - |
| 20 | 167-171 | The keg layout | defdir | one token names a source in keg, directory and settings | current | | - |
| 21 | 175-201 | The keg layout | defdir, market | Homebrew linking facts | current | | - |
| 22 | 203-206 | The keg layout | - | rules in Homebrew 7.0.12; experiment reproduces failure and proposed remedy | historical | dated reading | - |
| 23 | 207-208 | The keg layout | status | "This layout amendment is a design. Formula implementation and publication must precede reliance" | stale | Grove's template and all four tap formulae implement it; tap formulae published projected | Replaced both sentences with "Grove's formula template and each published tap formula implement this layout. Grove's formula publishes it with Grove's compatible release." |
| 24 | 210-213 | The keg layout | inventory | tap owns personal, superpowers, quint-llm-kit; both must carry the layout | current | | - |
| 25 | 215-218 | The keg layout | - | install methods move payload; archives keep layout | current | | - |
| 26 | 220-222 | The keg layout | status | Ravel's spec and Homebrew view "must explain this packaging rule before Ravel 2.0.0 publishes" | stale | Ravel v2.0.0 holds "Packaging a default source" and `homebrew-directory.mmd` | "must explain this packaging rule before Ravel 2.0.0 publishes" became "explain this packaging rule" |
| 27 | 224-232 | The keg layout | - | migration rules | current | | - |
| 28 | 234-241 | The keg layout | defdir | acceptance resolves the shared default directory | current | seam implemented | - |
| 29 | 243-248 | The keg layout | - | identity by `resourceDigest` | current | | - |
| 30 | 250-265 | The keg layout | inventory, defdir | other formulae pin commits; TestAnyware archive path | current | | - |
| 31 | 267-274 | The keg layout | provision | Ravel copies a keg source into a retained snapshot | current | Ravel snapshots | - |
| 32 | 276-292 | The keg layout | - | rejected layouts | current | decision | - |
| 33 | 296-298 | Hook adapters | market | keg ships a marketplace file per adapter; no remote marketplace lists one | current | | - |
| 34 | 300-305 | Hook adapters | - | adapter table | current | | - |
| 35 | 311-317 | Hook adapters | market | marketplace files at keg root; no adapter marketplace named `linkuistics`; WritingSkills removes its root marketplace file | current | WritingSkills has no root marketplace file | - |
| 36 | 319-324 | Hook adapters | provision, market, defdir | registration command table | current | caveats print them (CHANGELOG) | - |
| 37 | 326-350 | Hook adapters | market, provision, defdir | registration lifecycle readings | current | | - |
| 38 | 352-355 | Hook adapters | - | caveats print the four commands | current | | - |
| 39 | 359-363 | Supported routes | - | three routes, owner selects | current | | - |
| 40 | 365-369 | Supported routes | native, fmt, ravel2, defdir | route table | current | facts 7, 9, 11 | - |
| 41 | 371-375 | Supported routes | native, defdir | only directory route delivers; native rows stay caller routes | current | fact 7 | - |
| 42 | 379-383 | Actual minimum and separate proofs | native | two separate proofs | current | | - |
| 43 | 385-387 | Actual minimum and separate proofs | status | "ships both in one 2.0.0. An earlier native-only 2.0.0 release would need its own explicit owner" | stale | 2.0.0 is published with both; the hypothetical presumes 2.0.0 unpublished | "ships both in one 2.0.0" became "shipped both in one 2.0.0, published through `ravel@2`"; removed the native-only 2.0.0 hypothetical |
| 44 | 391-402 | Settings and HOME | fmt | file coexistence and settings refusal | current | fact 11 | - |
| 45 | 404-407 | Settings and HOME | defdir, native | default directory belongs to the installation | current | | - |
| 46 | 416-425 | Prefix selection | ravel2, native | `ravel@2` until promotion; default stays native on 1.1 | current | state C | - |
| 47 | 429-435 | Caller assets | - | Rhai port; "the dual-Ravel stage delivers it" | current | delivered | - |
| 48 | 437-442 | Caller assets | embed | Ravel 2.0.0 embeds no module | current | | - |
| 49 | 506-509 | Runtime parity | provision | grant per `snapshotRoot` | current | | - |
| 50 | 526-532 | Runtime parity | - | checks and caveats show both formats | current | | - |
| 51 | 550-557 | Launch and compatibility | native | "first" qualification; load name selects no native tool | current | `prompt.rs` | - |
| 52 | 559-561 | Launch and compatibility | native | bare target; namespace from supplying plugin | current | | - |
| 53 | 563-568 | Launch and compatibility | native, status | "Before the first pack moves, the mandate must stop asserting that every kind belongs to the grove plugin" | stale | packs moved; `prompt.rs` already names the bare skill | Replaced the "Before the first pack moves, the mandate must stop asserting" instruction with "The mandate does not assert that every kind belongs to the `grove` plugin." |
| 54 | 574-579 | Launch and compatibility | native | admit 2.x once native 2.0 passes; advertise directory after its acceptance | current | gate rule, met per lines 9-13 | - |
| 55 | 581-584 | Launch and compatibility | ravel2 | remedy names `ravel@2` until promotion | current | | - |
| 56 | 586-591 | Launch and compatibility | native | native route needs no 2.0.0 | current | | - |
| 57 | 597-604 | Launch and compatibility | fmt, native | failure table | current | | - |
| 58 | 612-616 | Existing installations and active readers | provision, embed, native | remove provisioning and embedded snapshot; leave old entries untouched | current | delivered requirement | - |
| 59 | 618-621 | Existing installations and active readers | provision, native | manual cleanup after readers and writers stop | current | | - |
| 60 | 631-640 | Existing installations and active readers | provision | cleanup keeps legacy root; plugin guidance links the procedure | current | `plugins/README.md` links it | - |
| 61 | 642-652 | Existing installations and active readers | native | coexistence; explicit loader | current | | - |
| 62 | 654-657 | Existing installations and active readers | native | live endings preserved | current | | - |
| 63 | 665-668 | Resources and conformance | native | no sibling, native or cache layout assumed | current | | - |
| 64 | 676-680 | Resources and conformance | native | `harnesses:` has no reader; installer gone | current | fact 2 | - |
| 65 | 684-687 | Packaging and rollout | inventory, native, market | archive contents; no installer, no marketplace package | current | | - |
| 66 | 689-694 | Packaging and rollout | ravel2, status | "Prepare Ravel 2.0.0 and compatible Grove caller assets..."; "Ravel 2.0.0 publishes first" | stale | 2.0.0 published 2026-11-12; Grove release and promotion still ahead (kept) | Folded "Prepare Ravel 2.0.0 and caller assets in separate prefixes and isolated homes" into the route check; "Ravel 2.0.0 publishes first" became "published first" |
| 67 | 696-707 | Packaging and rollout | - | tap pairing and publication rules | current | | - |
| 68 | 709-713 | Packaging and rollout | native, status | "The source split later updates native packages and injected directory sources" | stale | split done; native packages retired | Replaced with "The split pack sources keep the same resource-preservation checks." |
| 69 | 713-714 | Packaging and rollout | - | design changes no owner installation | current | | - |
| 70 | 716-721 | Packaging and rollout | status | before the cut stubs; after the cut repeat against published archives | current | procedure keyed to Ravel's cut; k314 pinned the archives | - |
| 71 | 723-725 | Packaging and rollout | - | focused review integrated | current | | - |
| 72 | 729-734 | The pack record and the release | inventory | record names five domain packs | current | | - |
| 73 | 736-754 | The pack record and the release | defdir, inventory, market | release steps and formula install | current | implemented | - |
| 74 | 756-759 | The pack record and the release | - | four steps of a pack change | current | | - |
| 75 | 763-769 | The retired marketplace | market | lists packs until finish moves main; then empty | current | fact 3 | - |
| 76 | 771-773 | The retired marketplace | - | manifests stay as metadata | current | | - |
| 77 | 775-783 | The retired marketplace | defdir, market | release notes four steps | current | CHANGELOG Unreleased holds them | - |
| 78 | 785-788 | The retired marketplace | - | refresh effect | current | | - |
| 79 | 790-794 | The retired marketplace | provision, market | PersonalSkills sentence change, pinned by personal | current | PersonalSkills `cddcfde` (2026-11-06) holds it | - |
| 80 | 803-811 | Test seams | - | `directory_route.rs` | current | | - |
| 81 | 813-815 | Test seams | provision | standalone directory part | current | | - |
| 82 | 817-825 | Test seams | native | standalone native part | current | | - |
| 83 | 827-837 | Test seams | provision, native | Linux floor; no native floor case | current | | - |
| 84 | 841-842 | Test seams | native, fmt | native acceptance rows | current | | - |
| 85 | 843 | Test seams | native, defdir | directory bootstrap | current | | - |
| 86 | 846-848 | Test seams | native, fmt, defdir | loader, compatibility, settings isolation | current | | - |
| 87 | 849 | Test seams | ravel2 | prefix selection | current | | - |
| 88 | 850 | Test seams | native | no native mutation | current | | - |
| 89 | 853 | Test seams | provision, native | standalone row; confined results historical | current | | - |
| 90 | 856 | Test seams | native | installed distribution | current | | - |
| 91 | 857 | Test seams | defdir | shared default directory row | current | | - |
| 92 | 858 | Test seams | native, market | hook adapter row | current | | - |
| 93 | 859 | Test seams | market | retired listing reading before finish | current | evidence holds the reading | - |
| 94 | 860 | Test seams | native | actual harness behaviour | current | | - |
| 95 | 862-870 | Native-only hook acceptance | native | guardrail trial | current | | - |
| 96 | 872-878 | Shared default directory acceptance | defdir | seam before formula publication | current | | - |
| 97 | 880-886 | Shared default directory acceptance | - | five skill formulae participate | current | | - |
| 98 | 888-892 | Shared default directory acceptance | defdir | resolve with no settings | current | | - |
| 99 | 911-917 | Shared default directory acceptance | - | failure control links published direct formulae and two fixtures | current | code links any published direct formula, now none, plus fixtures | - |
| 100 | 919-920 | Shared default directory acceptance | native | stale native copy coexistence trial | current | | - |
| 101 | 936-941 | Scope and remaining work | - | implementation updates guidance alongside behaviour | current | | - |
| 102 | 943-944 | Scope and remaining work | inventory, market, status | "`homebrew-skill-delivery-k215` plans the implementation of ..." | stale | planning complete; all four implemented and tested | "plans" became "planned"; added "Grove implements and tests all four." |
| 103 | 945-946 | Scope and remaining work | - | k236 removed the manual installer | current | provenance | - |
| 104 | 948-952 | Scope and remaining work | native | open implementation questions; no native cleanup | current | scope statement | - |

### README.md

| Lines | Section | Tags | Statement | Class | Reason | Correction |
|---|---|---|---|---|---|---|
| 33-34 (extra) | Install Grove | defdir | `brew install` "is the whole setup of Grove and its skills" | stale | In tap state C the default `ravel` is 1.1.0, which delivers no skill | Says Homebrew installs Grove and its skills, and Ravel 2 delivers them |
| 40 | Install Grove | inventory | Homebrew installs core and the five domain packs | current | fact 4 | none |
| 43-45 | Install Grove | defdir | only a Homebrew installation links the sources into the default directory | current | fact 5 | none |
| 49-60 | Install Grove (pending) | fmt | 1.x reads `policy.ts`, 2.x reads `config.rhai`, copy the sample for your Ravel | current | fact 11 | Added: the files coexist, 2.0.0 refuses a lone `policy.ts` with `config_format_retired`, write `config.rhai` before the default `ravel` moves to 2.x, keep `policy.ts` while 1.x launches remain |
| 65 | Install Grove | fmt | earlier `config.kdl` files are ignored | current | fact 11 | none |
| 108-117 | Skills | inventory | table of the six sources of Grove's formula | current | fact 4 | none |
| 120-125 | Skills | defdir, native | Homebrew links each source into the default directory, Ravel 2.0.0 reads it, no harness needs a native copy | current | facts 5 and 6 | none |
| 127-134 | Skills | ravel2 | Ravel 1.x delivers no skill, select `ravel@2` by PATH while the unversioned formula is 1.x | current | fact 9 | Added the state after promotion: no PATH prefix, `ravel@2` receives no later release, write `config.rhai` first |
| 136-143 | Skills | inventory | other formulae: `personal`, `testanyware`, `superpowers`, `quint-llm-kit` | current | fact 4 | none |
| 145-149 | Skills | market | hook adapters registered by the documented commands | current | delivery spec "Hook adapters" | none |
| 151-163 | Skills | market | no marketplace lists a pack, the retired marketplace, four steps, refresh behaviour | current | fact 3 and the retired-marketplace design | none |
| 167-172 | Skills (pending) | provision | releases up to 22.8 installed Codex skills, later ones leave them | historical | names Grove 22.8 | none |
| 194 | Further reading | inventory | link to the runtime methodology | current | file exists | none |

### docs/RELEASING.md

| Lines | Section | Tags | Statement | Class | Reason | Correction |
|---|---|---|---|---|---|---|
| 33, 98 | Prepare | provision | "the task snapshots" the working copy | current | false positive: a jj snapshot | none |
| 68-87 | A pack change | inventory, defdir | core and five packs, `plugins/packs.json`, four steps, copy into `share/ravel/skills` | current | delivery spec "The pack record and the release" | none |
| 131-143 | Release notes | fmt | release notes need the owner policy at `policy.ts`, copy the `policy.ts` sample | stale | after promotion the default `ravel` reads only `config.rhai` | Names both files, copies the sample for the installed Ravel, states the `config_format_retired` refusal |
| 233-245 | Caller assets | fmt, inventory | each set checked against its own build, six keg sources resolved, OLD keg digests | current | release tasks | none |
| 246-269 | Caller assets | market, native, defdir | marketplace files validated, shared seam, tap formula comparison, 22.8 upgrade | current | release tasks | none |
| 271-272 (extra) | Caller assets | ravel2 | "a build of Ravel's 2.0 feature line reports `ravel 1.1.0` until the cut" | stale | Ravel 2.0.0 is cut and published | Past tense, as the reason why the tasks take explicit prefixes |
| 300-312 | Target smoke | provision | both lanes are published archives, the floor directory case publishes skill snapshots | current | Ravel captures; lanes are published | none |
| 327-364 | Targets | native | runs "natively" | current | false positive: CPU architecture | none |
| 616 | Signal-contract cutover | fmt | sample policy derives the grant | current | the sample exists | none |
| 623-626 | Signal-contract cutover | provision | a candidate writes no skills, Grove 22.8 and earlier provision Codex skills | historical | names Grove 22.8 | none |
| 653-655 | Signal-contract cutover | market, native | "Grove installs no skills: update the owner's marketplace plugins and native installations" | stale | the keg carries the skills, and no native route remains | The keg carries the skills with the binaries, so the installed release updates both |
| 699-705 | Build and publish | embed, inventory, defdir | no embedding, archive layout, Homebrew links, Ravel delivers, existing native installations unchanged | current | facts 1, 4 and 5 | none |
| 707-711 | Build and publish | native | no manual installer, an existing native installation can differ from the CLI | current | fact 2 | none |
| 718-757 | Build and publish | fmt, inventory, defdir, ravel2 | archive listing, keg relocation, unversioned dependency, `ravel@2` opt-in | current | delivery spec | none |
| 866-875 | Installed verification | ravel2 | formula test lanes, `ravel@2` by hand, `-` when the dependency is 2.x | current | template `ravel_builds` | none |
| 880-884 (extra) | Installed verification (pending) | none | a formula test fails "at Ravel's nested sandbox probe" | stale | Ravel confines nothing after the withdrawal | Generalised to a failure of the test itself |
| 886-891 | Installed verification | fmt, ravel2 | caveats show both formats, default directory, hook commands, `ravel@2` | current | template | Added: they state the deprecation of `ravel@2` and the `config_format_retired` remedy |

### docs/specs/harness-selection-and-execution.md

| Lines | Section | Tags | Statement | Class | Reason | Correction |
|---|---|---|---|---|---|---|
| 8-11 | Introduction | provision, native | Grove admits 1.x and 2.x, provisions no Codex skills, supports native and directory routes | current | delivery spec | none |
| 13-17 | Introduction | native | "Skill availability is the user's responsibility through native installation or Ravel directory injection" | stale | Homebrew installs and Ravel delivers | Homebrew installs the sources and Ravel delivers them, outside version admission |
| 56-64 | Resolution | native, fmt, ravel2 | admission, remedy names both files and `ravel@2` until promotion | current | `crates/grove/src/dispatch/ravel.rs` | none |
| 66-67 | Resolution | native | "Native skill delivery does not require Ravel 2.0.0" | stale | delivery is Ravel's; the native route reads existing installations | "A launch on a native route does not require Ravel 2.0.0" |
| 98-102 | Skill delivery | native | "configured directory injection", "Native-only launches need no such configuration" | stale | the default directory needs no configuration | Directory delivery on 2.0 from the default directory or the settings; a native route receives none |
| 121-125 | Owner policy | fmt | 1.x reads `policy.ts`, 2.x reads `config.rhai`, settings "supply optional skill-directory delivery" | stale (last clause) | delivery is on by default; settings name or turn off sources | Settings can name sources or turn off the default sources |
| 127-145 | Owner policy | provision, embed, inventory | settings keys per release, caller assets are ordinary files, configure-grove | current | delivery spec | none |
| 191 | Supervision | native | "native" job control | current | false positive | none |
| 479 | Acceptance | provision, native | common delivery acceptance matrix, no native writes | current | delivery spec "Test seams" | none |

### scripts/templates/grove.rb.tmpl

| Lines | Section | Tags | Statement | Class | Reason | Correction |
|---|---|---|---|---|---|---|
| 29-35, 46-60 | sources and install | defdir, inventory | six sources, private payload, public alias, default directory | current | fact 5 | none |
| 38-40 | install | inventory, native | no step writes a native installation | current | fact 1 | none |
| 61-64, 89-98 | install, caveats | market, provision | hook marketplace files and their registration commands | current | delivery spec "Hook adapters" | none |
| 69-77 | caveats | defdir | Ravel 2.0.0 reads the default directory; "brew install and brew upgrade are the whole skill setup" | stale (last clause) | in state C the default Ravel is 1.x | Removed the claim; points to `ravel@2` while the unversioned formula is 1.x |
| 79-80 | caveats | none | Ravel 1.x delivers no skill, `ravel skills resolve` | current | fact 6 | Says to run it with Ravel 2 |
| 82-83 | caveats | native | Grove leaves any native installation unchanged | current | fact 1 | none |
| 105-118 | caveats | fmt | each major reads its own file, copy both samples | current | fact 11 | none |
| 120-126 | caveats | ravel2 | select `ravel@2` by PATH while the unversioned formula is 1.x | current | fact 9 | Added that `ravel@2` reads the keg with no settings |
| 128-130 | caveats | fmt | after the move the default reads only `config.rhai` and refuses a lone `policy.ts` | stale (incomplete) | the Done when requires `config_format_retired` and the remedy | Names `config_format_retired`, exit 3, the conversion remedy, keeping `policy.ts`, `ravel@2` deprecation and the behaviour after the upgrade |
| 134-148 | ravel_builds | fmt, ravel2 | lanes per runtime; "The tap has no ravel@2 before Ravel 2.0.0 publishes" | stale (comment) | Ravel 2.0.0 is published | The comment names a tap in state A |
| 150-260 | test | fmt, inventory | asset list, aliases, lifecycle per lane, resolve with no settings | current | facts 5 and 11 | none |

The formula test `scripts/release-formula.test.rb` now also asserts the refusal and the remedy text.

### CHANGELOG.md, `## Unreleased`

| Entry | Tags | Statement | Class | Reason | Correction |
|---|---|---|---|---|---|
| Lead (extra) | all | no user summary of the release | stale (missing) | the Done when asks for a user description of the compatible minor release | Added a lead paragraph and an "Upgrading" part for both tap states; entries now sit under "Changes", user-facing first |
| Owner switch tooling | inventory | step 5 compares full package versions | current | internal tooling | Moved after the user entries |
| Linkuistics tap | inventory | published projected revisions | current | tap state | Moved after the user entries |
| Shared directory seam, tap formula test | defdir, native, market | release checks | current | release tasks | Moved after the user entries |
| Manual installer removal | provision | `plugins/install.sh` removed, links stay | current | fact 2 | none |
| Retired marketplace | market, defdir | four steps, refresh behaviour | current | fact 3 | none |
| Archives and formula | inventory, defdir | six sources, keg layout | current | facts 4 and 5 | none |
| Hook adapter marketplace file | market | `writing-hooks`, registration | current | delivery spec | none |
| Stop installing Codex skills | provision, embed | no embedding, links unchanged | current | fact 1 | none |
| Bare mandate | none | names the bare skill | current | code | none |
| Admit Ravel 2.x | fmt, ravel2 | "To use 2.x, install the keg-only `ravel@2`" with no time limit | stale | true only until promotion | "While the unversioned `ravel` formula is 1.x" |
| Directory route | defdir | sources from the `skills` settings; "a tool outside Ravel makes each checkout" | stale | the default directory needs no settings, and Homebrew installs the sources | Rewritten: default directory first, `skills` settings as an alternative |
| `grove run` confinement | provision | withdrawn confinement | current | fact 12 | none |
| `grove` formula | native, fmt, ravel2 | "the caveats show both setup routes, native installation and Ravel directory delivery" | stale | the caveats offer no native setup route | Rewritten to the current caveats, with the promotion refusal and remedy |
| Release checks | ravel2 | two published lanes | current | fact 8 | none |
| machine-environment, Development | none | owner skill, Bun pin | current | unrelated to delivery | Moved to the end |
| Known limits (extra) | none | none | stale (missing) | the two findings reach users | Added a known-limits entry pointing to `docs/USAGE.md` |

# homebrew-delivery-design-k219

**Reviews:** homebrew-delivery-design-k216, ravel-default-skill-directory-k218

## Goal

Read the Homebrew delivery design adversarially before
`homebrew-delivery-plan-k217` cuts implementation leaves from it. The design
is the amendments of both producers together, in Grove and in Ravel. Produce
findings, and no fixes.

## Context

- Grove: the commit of `homebrew-delivery-design-k216` ("Design Homebrew skill
  delivery") and the commit of `ravel-default-skill-directory-k218`. Each
  commit message names its handle. The running logs of both task files hold
  the decisions and the human's words.
- Ravel: commit `78d833d24364` on the bookmark `skill-directory-delivery` of
  the workspace `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
  It amends `docs/specs/skill-discovery.md`, adds the decision record
  `installed-skill-sources-are-the-default` and amends the glossary.
- The amended Grove documents: `docs/specs/owner-skill-delivery.md`,
  `docs/specs/codex-skill-provisioning.md`, `docs/specs/coordinated-release.md`,
  `docs/specs/external-pack-contract.md`, `docs/specs/pack-membership.md`, the
  decision record `homebrew-installs-skills-and-ravel-delivers-them` and
  `docs/homebrew-skill-delivery-evidence.md`.
- The owner design runs its switch against the real HOME inside finish. Its
  recovery rests on `docs/models/owner-switch.qnt`, and
  `task model:owner-switch` runs it.

## Done when

- Each finding names the document and section, the defect and its
  consequence. A finding states whether it concerns k216's amendments, k218's
  or both.
- If a finding is worth acting on, an `integrate-review-design` leaf is cut
  as the review's last act. Its body names this review's handle.

## Notes

The producers' specific doubts. Read each against the artifacts rather than
taking it as established:

- **The native settings.** k218 made the off value the earlier value of the
  `skills` cell. Finish writes it by a guarded step after promotion. The
  producer claims the five properties of the recovery model still hold
  without a model change, because the model's cells are abstract. Look for a
  state that the state table misreads. The guarded step, an interruption or
  an outside write can reach it. Look also at the rule that a classification
  never accepts another earlier value for that cell.
- **The upgrade interval.** Homebrew unlinks a formula's earlier keg before it
  installs the new one. Without `sources`, a launch in that interval delivers
  the other sources and refuses nothing. Judge whether accepting that silent
  absence is right, and whether every Grove and owner rule that assumed a
  refusal was amended.
- **The default belongs to the installation.** A separate HOME no longer
  isolates the keg sources. Find any check, trial, rehearsal or acceptance
  case that expects no keg source. Such a case runs a 2.0 build with a record
  in a HOME without `"sources": []`.
- **The installation record.** Ravel finds it two levels above its canonical
  executable. Check the keg-only `ravel@2`, Linux, a copied build, and the
  claim that no release archive holds a record.
- **Replacement and roots.** Explicit `sources` replace the default, and every
  default source has the roots `skills` and `craft`. Check each of the ten
  sources and the quint formula's changed source directory.
- **k216's amendments.** Read the keg layout, the identity of a keg source and
  the candidate keg. Read the hooks installation inside finish and the pack
  record. Read the retired marketplace and the Codex registration after each
  upgrade.
- **One owner per rule.** Grove's documents must cite Ravel's contract and
  restate none of it. Look for a Grove sentence that restates a Ravel rule,
  and for a Ravel rule that a Grove document contradicts.

## Findings

Reviewed Grove commits `93088bc51dfa` (k216) and `a7e7be78aef7` (k218),
and Ravel commit `78d833d24364`. Locations below refer to those committed
artifacts. This is an inspection-only review; no test, build, lint or format
command ran, and no design, implementation or model was changed.

### F1 — P1: native settings preparation blocks active and complete recovery

**Amendment:** k218. **Location:** `docs/specs/owner-skill-delivery.md`,
"The native settings inside finish", lines 561–579, and "The switch inside
finish", lines 752–757; compare "Resuming the switch", lines 934–936.

The retry instructions run the native settings step before the switch. That
step accepts only the off value or an absent `skills` key. Interrupt the
switch after its last exchange, before the receipt: all cells, including
`skills`, hold their switched values, and the open backup reads as Active.
The required prelude instead stops at item 5 because the switched value
contains the mandatory reference. The same obstruction occurs after a receipt
but before its copy into the manual, where the table requires Complete
continuation. Neither state can reach the promised automatic continuation
through the documented entry point. Restoring the off value by hand also
changes the layout of the open run; it is not the table's Active or Complete
continuation.

Route an open backup through its recorded program and state table before
initial native preparation. State explicitly when the prelude is applicable,
and cover both crash points through the whole retry entry point. The model's
`resumeActive` and `resumeComplete` actions enter recovery directly, so their
existing witnesses do not establish the new prelude's composition.

### F2 — P1: the new settings exchange lacks durable detection of outside writes

**Amendment:** k218. **Location:** `docs/specs/owner-skill-delivery.md`,
"The native settings inside finish", lines 564–581; compare "Concurrent
writers", lines 818–836, and `docs/models/owner-switch.qnt`, `stage` and
`exchange`, lines 343–353 and 422–442.

The native settings step compares displaced content after exchanging it,
before any switch backup exists, and expressly adds no record. Consider an
existing settings file without `skills`: preparation reads it, a kept
process writes another key, preparation exchanges its staged file into place,
then preparation is interrupted before the comparison. On retry, item 2 sees the off
value and changes nothing. No specified durable expectation identifies that
preparation's displaced content for subsequent flagging or merging. The
outside write has left the live settings without the report promised by the
recovery contract. A descriptor opened before the exchange can likewise write
to that displaced inode after the one comparison.

The switch avoids these cases by recording expected and staged content before
exchange and checking retained copies on later runs and in `mac-check`.
Preparation must participate in that durable detection, including recovery
before the first switch backup, or provide an equivalent specified mechanism.
Model the added transition before relying on the five recovery properties,
and extend the rehearsal to the exchange/comparison gap and a late descriptor
write. Changing only the abstract earlier cell value does not represent this
new exchange outside the recorded run.

### F3 — P2: Grove still states that settings are necessary for delivery

**Amendments:** both. **Location:** `docs/specs/codex-skill-provisioning.md`,
"Ownership", lines 36–40, and "Installation and bootstrap", lines 86–90;
`CONTEXT.md`, "Skill delivery / the retired sweep", lines 28–36 and 53–55.

The delivery design says that the user names the sources in settings and
that "only Ravel's settings deliver" core. The glossary is stronger: a launch
delivers "only the sources that the settings name". Ravel's amended
"Settings" contract instead delivers installed sources with no settings
file. Thus the overview and bootstrap instructions still describe the
explicit-settings route as necessary, while this amendment promises that
Homebrew installation is the whole skill setup. An implementer or user who
follows that statement can recreate the unnecessary per-source configuration
that the human rejected.

Remove these stale restatements and refer to Ravel's source-set contract.
Keep Grove's own installation obligations here. The new keg section already
names Ravel's owning section; it does not reconcile these earlier statements.

### F4 — P2: the membership decision still requires the retired Codex listing

**Amendment:** k216. **Location:** `docs/specs/pack-membership.md`, "The
always-on hooks", lines 117–118 and 133–135; compare
`docs/specs/codex-skill-provisioning.md`, "Hook adapters", lines 242–244.

The membership decision still assigns a Codex marketplace file to the
WritingSkills repository root and says that the listing stays there until a
remote source is proven. The delivery design now expressly requires
WritingSkills to remove that file and puts the listing at Grove's keg root.
These are incompatible instructions for the same artifact. The later
membership paragraph changes the delivery step but leaves this decision and
its rationale in force, so planning cannot satisfy both specifications.

Reconcile the membership decision with the keg-owned listing. Keep the
earlier native proof explicitly historical where it remains useful, as the
external-pack contract does for its retired distribution route.

## Review evidence and limits

The new record's canonical-executable lookup, the keg-only and Linux prefix
cases, explicit replacement of default sources, and the ten proposed source
roots have no additional supported design finding from this inspection.
The ten existing source layouts were read directly, including the narrowed
`quint-llm-kit-plugin` source. The hook-upgrade and retired-listing observations
remain acceptance work under the design's named seams, rather than measured
successes of this review. The silent absence during an upgrade is an explicit
accepted trade-off; the owner design now calls for no Grove loop launches
during an advance.

Graph evidence used Tier 2. Grove's checked document and Rust paths had no
recorded gap at generation `2026-11-06T06:41:56Z`; this is a best-effort
coverage signal. The graph returned no model symbols and no tracked model
freshness, so the relevant Quint source was read directly. Ravel's fast index
at generation `2026-11-02T05:30:19Z` excludes `docs/`; its amended contract,
decision records and release-channel text were therefore read directly from
the reviewed commit's unchanged working tree. The existing model results are
producer evidence, not a rerun or proof supplied by this review.

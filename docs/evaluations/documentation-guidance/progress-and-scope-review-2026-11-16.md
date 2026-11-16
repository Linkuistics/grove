# Progress and scope review, 2026-11-16

> **Limit, 2026-11-25.** The human chose option A of this review, and a curated
> change carried this review to `main`. The preregistration, the harness, the
> cases and the task tree that the review cites are not on `main`. They stay in
> the unpublished history of the grove workspace
> `grove.gh-issue-14-documentation-skills-for-editorial-pipeline`
> ([oversized evidence stays out of main](../../adr/oversized-evidence-stays-out-of-main.md)).
> The review below is unchanged from 2026-11-16.

Task `progress-and-scope-pruning-k646` wrote this review. The review is
read-only. It ran no tests, VMs or evaluations. Each count below comes from
the jj history or from the task tree under `.grove/`.

## 1. What the grove delivered

The grove forked from `main` at commit `b4fa1c239312` on 2026-10-24. Since then
it has made 448 commits and changed 2,475 files. It has 438 DONE tasks and 24
ABANDONED tasks.

| Item | Evidence |
| --- | --- |
| Research basis and evidence | `docs/research/documentation-evidence.md`, `docs/research/documentation-basis-a.md`, `docs/research/documentation-toolchain-a.md` |
| Agreed design with dispositions | `docs/specs/documentation-guidance.md`, agreed by the human on 2026-09-30 and again after its review |
| Decision records | Five ADRs, for example `docs/adr/a-document-declares-its-contract.md` and `docs/adr/a-reader-checkpoint-is-a-requirements-leaf.md` |
| Design figures | `docs/design/documentation-guidance/`, six D2 views with `regenerate.sh` |
| Figure checks | `figures:regenerate` and `figures:check` in `Taskfile.yml`, and `scripts/check-figures.sh` in `scripts/check.sh` |
| Preregistration | `docs/evaluations/documentation-guidance/preregistration.md`, with four review rounds and Amendments 1 to 4. It is not frozen, and `freeze.md` does not exist. |
| Evaluation harness | `docs/evaluations/documentation-guidance/harness/` and about 1 GB of cases, observer evidence and fixtures |

The grove changed no installed skill byte. No guidance increment exists.

## 2. Status of each root Done-when clause

| Clause | Status | Evidence |
| --- | --- | --- |
| D1. Each proposal in #14 to #19 has a disposition. | Met, in the workspace only | The Dispositions section of the spec gives each proposal an owner and an acceptance, or a narrowing, or a decline. |
| D2. Installed guidance covers the structure brief, the four stages, reader testing, format selection and exchanges. | Not met | No file under `plugins/` changed. The installed writing pack has no reader test, format selection, document contract or reader checkpoint text. |
| D3. The evidence survives outside the tree and is usable without a checkout. | Partly met | The research, spec and ADRs exist. They are not on `main`, because the history holds blobs above 100 MB and cannot be pushed (task k238). |
| D4. The agreed evaluation ran, with results and limits recorded. | Not met | The protocol is not frozen. The current-skill arm (k14) and the revised arm (k10) have not run. |
| D5. Repository checks pass, and figures generate through the Taskfile. | Partly met | The figure tasks exist. This review did not run `scripts/check.sh`. No integrated result exists to check. |

A user of the installed skills can use nothing from this grove now. Every
artifact exists only in this workspace.

The delivery target also moved. After the fork, `main` removed Grove's copies
of the editorial kinds. They now live in external packs.

The writing pack
(WritingSkills) holds `grove-draft`, `grove-copy-edit`, `grove-art`,
`grove-proof` and `technical-communication`. The requirements pack
(RequirementsSkills) holds `grove-requirements`. The six increments in the spec
name Grove paths such as `plugins/grove/skills/grove/references/editorial.md`.
Those paths must change to the external packs before any increment can land.

## 3. Trace from each live branch to the root

| Live branch | Root clause it serves |
| --- | --- |
| `18-planning--guidance-increments-k9` | D2, directly |
| `19-impl--revised-trials-k10` | D4 |
| `20-requirements--sample-reading-k11` | D4, seam 3 |
| `21-impl--adr-ownership-map-k239` | D3 and D5 |
| `22-impl--original-reading-history-k238` | D3, because integration needs a pushable history |
| `17-k8/10-impl--current-baseline-k14` | D4 |
| `17-k8/09-k13/176-impl--protocol-freeze-k37` | D4 |
| `17-k8/09-k13/177-impl--capture-held-pipe-fixture-k131` | D4, harness test hygiene |
| `17-k8/09-k13/175-k36/53` to `56` (k249 to k252) | D4, scoring |
| `17-k8/09-k13/175-k36/52-k280/` and its 12 leaves | D4, indirectly. The leaves reconcile envelope and receipt charters of the document scoring instrument. |
| `52-k280/108-k499/` and its 13 leaves | D4, indirectly. The leaves qualify the instrument for production admission with real eslogger collection. |

No live branch serves no clause. The last two rows serve D4 only through an
instrument standard that the root charter does not name. The root names three
seams: repository checks, a preregistered comparison with a model proxy, and a
short human read. Production admission of the scoring instrument is not one of
them. It grew from the harness charter.

## 4. Diagnosis of the loop

The harness node `17-k8/09-k13` holds 415 of the 438 DONE tasks. It still has
not met its own Done-when: a smoke test on final bytes and a frozen protocol.

The counts below show handoffs that left the acceptance unchanged:

- The qualification brief `_document-production-admission-qualification.md`
  has 26 sections from k558 onward. 23 of them record a split that keeps the
  aggregate acceptance verbatim.
- Leaf k644 nests 13 earlier charters verbatim, k618 to k643. Each one handed
  the same acceptance to its successor.
- Since 2026-11-01, the grove made 117 commits. All 117 are in the harness
  subtree, and none changed a root clause status.
- Leaf k644 asked the human three times: for a tool repin, for the VM scope,
  and then for this review. Each answer left the acceptance unchanged.
- The note "Current evidence, not acceptance" says that the prerequisites
  k547 to k557 grant no credit.

The mechanism is simple. Leaf k644 requires every original clause to hold on
the same enabled controller with real collection. A session cannot meet that,
so it delivers one interval guard. Then it splits off a successor with the full
charter. Each review cuts an integration leaf, and the cycle starts again.

These requirements cannot be met on this host, or not without human action:

- **Real eslogger collection.** `sudo -n` requires a password on this host.
  Real collection needs the pinned TestAnyware VM. The enabled selected path
  has no `GROVE_OBSERVER_VM` variant. Only seven observer modules have one.
- **Reviewed tool bytes.** The receipt profile pins the hashes of
  `Taskfile.yml` and `uv`. Each Homebrew upgrade and each Taskfile edit forces
  a human-approved repin, as in k645.
- **Protocol freeze.** Task k37 offers the answer sheet to the human.
- **History rewrite.** Task k238 needs an owner-authorised in-place jj rewrite.
- **Sample read.** Task k11 is a human read by design.

## 5. Pruning options

Each option keeps D1 and D3. Each option retargets k9 to the external packs.

### Option A. Ship guidance with a light comparison (recommended)

- **Delivers:** the six increments in the installed writing and requirements
  packs, and the Grove spine teaching. It also delivers a small comparison of
  current and revised skills. The comparison runs two or three document cases
  and one exchange through plain model calls. It stores the outputs, and the
  human reads them. D5 checks run on the integrated result.
- **Drops:** the preregistered blind protocol and the whole harness node
  `17-k8/09-k13`, with k14. The harness and its evidence stay in the history as
  a record of the attempt. The comparison becomes descriptive evidence, not a
  preregistered decision. This change to D4 and seam 2 needs your confirmation.
- **Also decides:** integrate a change that carries the research, spec, ADRs,
  design, figure scripts and comparison. Keep the 1 GB evaluation tree off
  `main`. Then k238 becomes unnecessary, and this review abandons it.
- **Sessions:** about 12. That is one for k9, six to eight for the increments,
  two for the comparison, one for the human read, and one for k239.

### Option B. Ship guidance and checks only

- **Delivers:** the six increments, repository checks and the human read of
  samples.
- **Drops:** D4 as an evaluation. The review records the omission as a limit.
  The tree change abandons the harness node and k10.
- **Sessions:** about 10.
- **Risk:** no evidence shows that the revised guidance is better than the
  current guidance.

### Option C. Keep the preregistered protocol, drop production admission

- **Delivers:** everything in the root charter. An amendment declares that the
  scoring instrument has no production qualification and states the limit.
- **Drops:** the `52-k280` and `108-k499` chains, 25 leaves in total.
- **Keeps:** k37 freeze, k14 baseline, k249 to k252, k10 and k11.
- **Sessions:** about 22, with a risk that more harness gaps appear.
- **Needs:** an amendment to the preregistration, which a review must read.

### Option D. Continue the current path

- **Needs:** a VM variant for the enabled selected path, real collection runs,
  and then the remaining owners k508, k543, k544, k561, k562, k573 and k593.
- **Sessions:** unknown. 117 commits in 16 days have not changed a root status.
- **Delivers:** no installed guidance until the whole chain closes.

## Recommendation

Choose Option A. It is the smallest option that delivers guidance to installed
users and still measures a difference. It uses the evidence that the grove
already holds. It also removes the dependency on a VM and on a pushable 1 GB
history.

Option B is smaller by about two sessions. It gives up the only check of
whether the guidance helps.

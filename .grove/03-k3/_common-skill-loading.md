# common-skill-loading-k3 — brief

## Goal

Deliver common Ravel skill discovery for Claude Code and Codex while the skill
sources still live in this checkout. Leave a working launch path on which the
external research-pack proof can build.

## Done when

- Ravel captures configured local sources, injects their skill-directory paths
  and prepares the prompt automatically before owner policy constructs the
  command. The original task and ending commands remain intact in their
  separate block.
- Ravel runs no VCS command and makes no network request for a source. A
  missing required source or root refuses as `skill_source_missing` before any
  worker, without substitution. A record of another `type` refuses as
  `settings_invalid` with the checkout remedy. Directories and resources
  remain readable for sessions and delegates, including confined launches.
- A checkout that a tool outside Ravel makes works as an ordinary local
  source. Acceptance uses a plain Git checkout at a commit and requires no
  checkout tool.
- Grove removes obsolete automatic native provisioning. Users may supply skills
  natively in each harness or through Ravel directory injection. Grove only names
  the skill and imposes no delivery route. Any core fallback uses the common path. No source
  pack has moved yet, and no kind registry or new workflow protocol exists.
- Public fixture tests cover launch/resolution, failures, launches without a
  network, concurrency/lifetime and confinement. Existing Grove tree/launch behaviour
  remains green.
- Real Claude Code and Codex trials activate a relevant craft skill without the
  task naming it, and record the read before the action. They use isolated homes
  or equivalent isolation with native copies absent. Preserve the owner's installs.
- The owning specifications, affected architecture/glossary/walkthrough and
  installation documentation agree with delivered behaviour. Applicable Ravel
  checks and Grove's full check pass using an explicit compatible Ravel build.

## Decomposition

The staged route now includes design repairs for upstream Rhai changes:

1. `coordinated-ravel-release-k36`: establish one Ravel release owner and a
   reviewed publication order for Rhai, directory delivery, Grove and the tap.
2. `owner-ravel-skill-delivery-k37`: inventory the full installed collection,
   including TestAnyware, and design Ravel delivery and safe native removal.
3. `grove-native-delivery-k20`: remove automatic native writes, keep native-only
   Ravel 1.1 launches working, supply the cleanup procedure, then repair the
   mandate's loading/failure wording. Fixtures prove Grove's interpretation.
   Actual missing-kind agent behaviour belongs to acceptance.
4. `ravel-rhai-skill-delivery-k34`: repair the directory contract against current
   Ravel main, then earn its focused design review before consumers run.
5. `grove-rhai-skill-delivery-k35`: repair Grove's adoption contract against that
   reviewed upstream artifact, then earn its design review before local planning.
6. `ravel-local-delivery-k23`: deliver local capture, retained directories and
   automatic public launch preparation with its settled schema, Rhai host and grants.
7. `ravel-pinned-delivery-k25`: withdrawn. During ravel-git-acquire-k77 the
   human removed Git sources from Ravel, and its remaining leaves are abandoned.
8. `ravel-local-sources-k78`: narrow Ravel's contract to local sources.
9. `owner-pack-checkouts-k79`: settle the owner's checkout tool outside Ravel.
   Three review chains followed, and `owner-switch-recovery-k87` rebuilt the
   switch's recovery with its own review chain.
10. `skill-loading-replan-k80`: reconcile the briefs with the local-source
    decision and grow the implementation leaves.
11. `ravel-local-sources-k90`: replace the temporary Git refusal with the
    permanent `type` refusal and remove the remaining Git code.
12. `grove-dual-ravel-k27`: prove the supported native and directory releases,
    explicit configuration examples and installed layouts against independent
    builds.
13. `common-loading-acceptance-k29`: measure actual Claude Code/Codex behaviour
    and reconcile the integrated checks and reproducible handoff to research.

The owner's checkout and switch implementation is in discovery-rollout-k6,
which owns the owner cutover.

Only the native stage has implementation detail now. The later planners consume
actual preceding commits and grow focused leaves, carrying fixture checks and
documentation with behaviour. Do not split a schema or internal resolver from
its only usable public reader into separately dead halves. Temporary independent
builds and isolated HOME settings supply acceptance before feature publication.

Feature release/tap activation remains discovery-rollout-k6. Rhai 2.0 native
compatibility and its release coordination are an earlier independent concern.
The human directed this grove to own Ravel release execution as well as Grove's
release. k36 settles the coordinated route before dependent work. The existing
rollout brief's versioned-formula path is the initial strategy. It does not
claim that an external workstream has already paused its release.

This subtree must remain live until implementation and acceptance satisfy its
Done when. Retiring its design and planning children alone does not close it.
Design producers add any earned review ahead of dependent work. Integrations
follow their reviews before later consumers edit the same artifacts.

## Pointers

- Ravel: `docs/specs/session-execution.md`, request preparation and confinement.
- Grove: `docs/specs/codex-skill-provisioning.md`,
  `docs/specs/harness-selection-and-execution.md`.
- Existing seams verified in skill-discovery-k2: Ravel request preparation before
  selection, and Grove's CLI provisioning and public session command.

## Notes

The human clarified the boundary during grove-skill-delivery-k8: Ravel injects
directory locations and does not resolve a requested skill. The earlier
ravel-skill-discovery-k7/k14 catalog design must be revised before planning.
Retain the local-source and lifetime requirements. Do not reintroduce a catalog
dependency into Grove. Test native-only setup as well as isolated directory
delivery. The adoption design's new review must consume that upstream clarification.

The historical reviewed Ravel directory contract is committed as
`b93bb927095ff09b91e9b78dc1ad05bdacfec649` by ravel-skill-directories-k18,
integrating ravel-skill-directories-k17 after producer ravel-skill-directories-k15.
It injects search directories and direct mandatory-file references. It retains
a directory-only descriptor for recovery/delegation. The integration defines
name versus explicit-loader outcomes and mandatory-before-caller read order.

Grove's mandate must qualify "first" with "after any mandatory guidance".
Its ordinary and plugin-qualified name requests must work with native copies
absent. Concrete native tools and file paths follow the explicit-route rules.
Shared settings remain a recorded trade-off. Mixed-major enablement needs
separate HOME settings, not just separate executable prefixes.

Whole-preamble loss needs an unprompted recovery trial. A retained path alone
does not satisfy it. Grove's adoption review consumes this revised handoff.
Ravel main `24a54523` now assigns 2.0.0 and schema 3 to Rhai native execution,
with unchanged caller prompts and worker protocol 1. The historical directory
contract's TypeScript SDK, schema and version assignments therefore cannot be
implementation targets.

ravel-rhai-skill-delivery-k34 settles the actual feature identity, request
fields, Rhai host reads, settings and frame bounds. grove-rhai-skill-delivery-k35
repairs the consumer specification and evidence. Their reviews precede
dependent work. Native-only Ravel 1.1 compatibility, Rhai 2.0 native
compatibility and directory acceptance are distinct obligations.

The adoption integration `grove-skill-delivery-k19` repairs the delivery spec,
records and caller views. Its evidence document owns the consumer handoffs.
Implementation supplies missing-skill force-stop and namespace-neutral mandate
wording before packs move. It also supplies optional manual-cleanup guidance
with the removal of provisioning. Native-only lifecycle success and confined
read access have separate acceptance cases. The first external pack proves its
native namespace.

Publication must preserve compatible tap dependencies at every intermediate state.

Keep one coordinated tree here. Work in other owning repositories must follow
their instructions, preserve unrelated changes, and leave discoverable commits.
Use independent temporary installed prefixes for acceptance if releases are not
yet available. Final publication belongs to discovery-rollout-k6. Establish any
required earlier dependency release explicitly rather than silently assume it.
Every session retains the ending command from its own launch prompt.

## Cross-repository handoffs

The coordinated route is now in `docs/specs/coordinated-release.md`, with
observed inputs and named owners in `docs/coordinated-release-evidence.md`. The
human acknowledged that this is the sole live workstream through completion.
coordinated-ravel-release-k39 reviewed the combined 2.0.0/schema-4 design, and
coordinated-ravel-release-k40 integrated the findings. Ravel's channel contract
and its decision record live in the isolated `Ravel.coordinated-release-design`
workspace from main `24a54523`. Consume the exact commit
`2a77ffbda3b2510f0997c740c0642b18afaeece7`, which k40 records. This grove completes remaining Rhai release
prerequisites itself, preserving the paused task tree.

The Ravel feature producer owns work in a separate jj workspace on
`skill-directory-delivery`, based on the exact main revision recorded by k34.
The local stage owner and ravel-local-sources-k90 each rebase and check at
entry. Each records
exact bases and output commits, and keeps the incomplete local subset off main.
Rollout owns the final rebase, full checks, main landing and complete-feature
release. The named Ravel owner must receive this integration contract before
implementation. No work here edits the `Ravel.gh-issue-2` tree or signals it.

This grove now owns Ravel release execution. coordinated-ravel-release-k36
records the human's single-owner handoff covering the other workstream's
`release-ravel-2-0-0-k37` and `homebrew-upgrade-k38`. It assigns tap changes to
the release stage that executes them. The human's instruction keeps all other
workstreams paused through completion. No cross-repository notification or
tree edit imposed that state.

Preserve the original 1.1.0 ending executable for every live driver that still
uses it. Rollout grows concrete Ravel integration, release, installed
verification and tap work under the reviewed plan. It is no longer merely a
request to another workstream.

owner-ravel-skill-delivery-k37 supplies the baseline of all installed skills,
resources, tools and hooks before moves. Repository moves preserve that set.
owner-ravel-skill-inventory-k38 reconciles the final set and grows actual owner
configuration, activation and native-removal leaves. The owner's `policy.ts`
to `config.rhai` conversion precedes any owner default launch on 2.0.

The owner-delivery design is `docs/specs/owner-skill-delivery.md`. Its
evidence is `docs/owner-skill-delivery-evidence.md`, and the per-entry baseline
is `docs/owner-skill-inventory.tsv`. Its requirements section lists what k34
must preserve or settle.

owner-ravel-skill-delivery-k41 reviewed the design, and
owner-ravel-skill-delivery-k48 integrated the findings before k34. Item 8 now
requires an authentication-only sandbox home, with a separate `CODEX_HOME`.
The repaired instrument records a change identity for each entry, Codex's
installed plugin cache and a coverage sweep. Its fixture controls are in
`docs/evaluations/owner-skill-delivery/controls.py`.

TestAnyware is an explicit acceptance case. A disposable proof cannot replace
the actual owner cutover. Old lazy readers and the final session's guidance
must remain readable.

The directory feature stays unpublished through local, dual-Ravel and
actual-harness acceptance. ravel-local-sources-k90 replaced the temporary Git
refusal of the local build with the permanent refusal of k78. A `type` other
than `local` now refuses as `settings_invalid`, exit 2, with the checkout
remedy.

ravel-rhai-skill-delivery-k34 settled the feature identity. The feature ships
in the combined Ravel 2.0.0, with script and request schema 4. A schema-3
script refuses, also with delivery disabled. Worker protocol 1,
selection-context schema 1 and the notice and failure schemas 2 keep their
numbers.

k34 committed the repaired Ravel contract as `fc87cb41a422`.
ravel-rhai-skill-delivery-k49 reviewed it, and ravel-rhai-skill-delivery-k50
integrated both findings. The reviewed contract is commit
`822c0ed2f67d9a446b74476fdad7d05922552b87` on the local bookmark
`skill-directory-delivery`, in the jj workspace `Ravel.skill-directory-delivery`.
Its parent is `fc87cb41`, above the channel contract `2a77ffbd` and main
`24a54523`.

Nobody pushed that line. Its `docs/skill-discovery-design-evidence.md` holds the
stage handoff for the Ravel owner and the consumer handoff for k35. The
integration changed two rules. The sandbox home for real harness runs refuses
an inherited `CLAUDE_CONFIG_DIR` and three Claude Code plugin-location
variables. Three frame-bound tests replace the one-byte control. The consumer
handoff for k35 did not change.

grove-rhai-skill-delivery-k35 repaired Grove's adoption contract against
`822c0ed2`. The Supported routes section of the delivery specification states
three routes: native on 1.1, native on 2.0 and directory on 2.0. Directory
delivery needs 2.0.0, and the two routes on 2.0 have separate proofs. The Rhai
contract mapping in the delivery evidence gives each upstream fact its Grove
owner.

The dual-Ravel stage ports every Grove caller asset to Rhai, which is the scope
of Grove issue 24. A build of the feature line reports `ravel 1.1.0` until the
cut, so each stage records its Ravel builds by commit and build identity.

The focused review grove-rhai-skill-delivery-k51 found one defect. The design
promised identical refusals, but Ravel's frozen comparison keeps three runtime
differences. grove-rhai-skill-delivery-k52 integrated it as the Runtime parity
section of the delivery specification. Grove accepts those differences for the
choice file and the jj store path. Receipt bytes and provider-safety checks
stay identical, and an unpaired surrogate in a receipt is an accepted
difference.

The reviewed Grove contract is the k35 commit with the k52 change. The shared
case table states the expected outcome for each runtime. The dual-Ravel stage
moves the runtime-neutral definition of a blank label into
`docs/grove-provider-evidence.md`.

ravel-local-handoff-k61 closed the local node ravel-local-delivery-k23. The
local stage output is Ravel commit `1d50dae5d29e`, on the unpushed bookmark
`skill-directory-delivery`, above `main` `24a545236185`. Its parent
`6436e66fdb30` is the build commit, with the build identity `2ee59f33657690da`.
The build reports `ravel 1.1.0`.

ravel-local-sources-k90 found Ravel `main` still at `24a545236185` and needed
no rebase. Its build commit is `78332fcc367d`, above `7a894c782037`. Its output
commit `4f831feb4d0f` is now the tip of the unpushed bookmark
`skill-directory-delivery`. The build identity is `9feb86b7982a3920`, and the
build reports `ravel 1.1.0`. The "Local-sources stage record" section of the
Ravel design evidence holds its inputs and checks. This build is the directory
build of grove-dual-ravel-k27.

The "Local stage record" section of the Ravel design evidence holds the build
inputs and checks. On macOS, `task smoke:pair OLD=<1.1 executable>
NEW=<prefix>` runs the Homebrew 1.1.0 release and a directory build at the same
time. Each run has its own temporary home, and a guard denies both the real
home. The dual-Ravel stage can reuse that task for its installed pair.

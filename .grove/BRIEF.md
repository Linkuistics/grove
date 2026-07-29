# grove.improve-signaling-to-herdr — brief

## Goal

Make a running grove **legible from outside the session**: herdr should show,
accurately and without screen-scraping, whether a grove pane is working, blocked
on a human, or finished — and what it is working on. The grove's name records
where this started (herdr's `blocked` state not firing when an agent stops to ask
a question); the work grew to cover how grove reports state at all, and how a
herdr plugin can render the task tree.

## Done when

- A herdr pane running `grove do` reports `working` / `blocked` / `idle` from
  grove's own knowledge rather than from regex over the terminal buffer — in
  particular, a grove stalled on a HITL question reads as **blocked**, not
  **done**.
- A herdr plugin renders the live `.grove/` tree, driven entirely by the
  artifacts on disk.
- herdr stays **optional**: with no herdr present, every grove behaviour is
  unchanged (*herdr-optional-ui*).

## Decomposition

Listed in position order, which encodes dependency and value-first sequencing —
but each item is named by its stable `<slug>-k<key>` handle, because positions
shift under `leaf-insert` and handles do not (*task-tree-scheme*).

- **herdr-pane-state-k2** — driver-level state reporting. Harness-agnostic:
  the driver is the parent process whatever it spawned, so this needs no hooks
  and no per-harness work. Now a node: measurement showed the unforked mechanism
  is vetoed by herdr, so the route was settled first (fork, two-hunk patch — ADR
  *herdr-optional-ui*), then patch → reporter. **Done.** Its fourth leaf, the
  upstream PR, was abandoned: grove does not contribute upstream. The reporter
  is written and tested; what it cannot reach — intra-turn state — is
  **herdr-turn-hooks-k4**, and what stops it being *live* is
  **status-surface-live-k23**.
- **task-kind-taxonomy-k3** — now a node. Opened as `harness-on-leaf`
  (planning): move harness selection from per-grove env onto the leaf. The
  grilling reframed it — the sequencing problem was a *taxonomy* problem, not a
  harness-location one — so it is now the five-kind → seventeen-kind rework plus
  both routing axes. Its brief carries the design. Independent of
  **herdr-pane-state-k2**.
  **Done.** Seventeen kinds, `work` → `impl` without breaking live groves, both
  routing axes (family fallback harness-major, per-leaf `**Harness:**`), and a
  hard failure on a kind that resolves no model var. The durable record is
  `docs/specs/task-kind-taxonomy.md` plus *task-kind-taxonomy* and
  *model-per-task-kind*, both reworked in place. Four of its ten leaves were
  consequences discovered by the sweep, not planned — the taxonomy's real cost
  was the doc and config surface, not the enum. Shipped as **v16.0.0** by
  `ship-release-k25`. Work in this repo still runs against
  `./target/debug/grove-llm` whenever it depends on unreleased behaviour — as of
  now, the whole v16.1.0 CHANGELOG entry, which is written but not cut.
- **herdr-notes-reverify-k17** — re-verify this brief's herdr Notes and the
  fork-maintenance spec against the fork's current state, once, ahead of the
  leaves that depend on them. A separate workstream had moved the fork.
  **Done.** Every Notes claim held; the spec needed two corrections, not a
  rewrite. Its real yield was two things nobody was looking for: the surface is
  inert in production (**status-surface-live-k23**) and the tap's caveats
  contradict the ADR (**tap-caveats-reconcile-k24**).
- **status-surface-live-k23** — ship the reporter and get the patched herdr
  server actually running, then pass the fork-maintenance acceptance test on a
  real `grove do` pane. Inserted ahead of the two leaves that refine a surface
  they currently cannot observe. Now a node: a `grove do` pane's ancestry is
  `herdr → shell → grove → harness`, so a session here is a grandchild of both
  processes that must be replaced, and cannot watch its own replacement. Hence
  **ship-release-k25** (cut the release; prove the installed binary reports) then
  **observe-live-surface-k26** (acceptance test under a new driver and a
  restarted server).
  **Done.** The surface is live and measured on a real pane — see Notes. The
  server was replaced by `herdr server live-handoff`, which preserved every pane
  including the one that issued it, so the "restart kills every pane" cost this
  node was sequenced around turned out not to exist. Three rows of ADR
  *herdr-optional-ui*'s table were out of scope here (SIGTERM/SIGHUP,
  version-skew stop, relaunch); all three were later observed by
  **observe-mid-turn-live-k31**.
- **herdr-turn-hooks-k4** — intra-session turn boundaries, per harness. Refines
  **herdr-pane-state-k2**; claude first (cleanest injection), codex and pi after.
  **Done.** claude launches under herdr now carry an inline `--settings` hook
  block wiring `UserPromptSubmit` ⇒ `working` and `Stop` ⇒ `blocked`-unless-
  signalled; measured live, on the real `claude`, on all three signal cases. The
  durable record is ADR *herdr-turn-boundary-hooks*. codex and pi are deferred on
  **facts, not effort** — codex has no turn-end hook event and persists hook
  trust per content hash; pi's own herdr extension reports `idle` at turn end,
  the same conflation. What it did *not* cover was mid-turn blockers — closed by
  **herdr-mid-turn-blockers-k30**, which widened the same ADR rather than adding
  one.
- **herdr-mid-turn-blockers-k30** — the gap inside a turn: a permission prompt
  stalls an unattended loop exactly as badly as a question, and grove's own
  authority suppressed the screen detection that used to catch it by accident.
  Deferred out of **herdr-turn-hooks-k4** because `blocked` there needs a paired
  restore that only a per-tool-call event gives — a different design, not a
  bigger version of the same one.
  **Done.** Two more rows in the injected block, as a pair: `Notification`
  (matched to three dialog types) ⇒ `blocked`, `PostToolUse` ⇒ `working`. Both
  design questions were settled by reading the shipped claude binary rather than
  its self-contradicting docs — see Notes. Redundancy suppression was
  *rejected*, contra the leaf's opening framing: grove's hook is a fresh process,
  so remembering costs what it saves, and a report per tool call is what
  re-asserts authority after a herdr restart. What it could not do is watch a
  real permission prompt end to end — that is **observe-mid-turn-live-k31**.
- **observe-mid-turn-live-k31** — the one measurement only production has: a real
  pane, parked on a real permission prompt, reading `blocked`. Carries the three
  rows of *herdr-optional-ui*'s table nobody has observed (SIGTERM/SIGHUP,
  version-skew stop, relaunch), which were homeless once **herdr-turn-hooks-k4**
  finished. Needs v16.1.0 shipped first; the version-skew guard makes that a
  single leaf rather than two (the leaf explains how).
  **Done.** v16.1.0 is shipped and installed, and under that driver a real
  permission dialog held past six seconds read **`blocked`** and returned to
  `working` on grant, mid-turn. All three carried rows were observed too, the
  last two by a detached observer acting after the arming session was dead — so
  *herdr-optional-ui*'s state table is now **observed in full, in production**
  (see Notes). Costs: two release-path frictions, both folded into
  **release-doctor-toolchain-gap-k27**, which is now a two-friction leaf rather
  than the doctor alone.
- **herdr-grove-plugin-k5** — the plugin. Depends only on the `.grove/`
  directory scheme, so it can follow **herdr-pane-state-k2** at any point.
  **Done.** `herdr-plugin/` — plugin id `linkuistics.grove`, a manifest plus one
  zero-dependency Python renderer, linked and measured on a real pane beside this
  very grove. Its three open design questions were all answered *cheaply*, which is
  the finding: the whole tree renders (finished nodes collapsed to counts), the
  invoking pane's cwd arrives free in `HERDR_PLUGIN_CONTEXT_JSON`, and a 1 s poll
  beats a watcher. The fourth question — sidebar tokens — was answered by
  *externalising* it as **herdr-sidebar-tokens-k32**: it is a push over the socket,
  not a read off disk, so it is a different decision entirely.
- **jj-first-coverage-k6** — the jj path is primary in code but untested, and
  the docs still lead with git.
  **Done.** `tests/jj_tree_verbs.rs` drives every tree-mutation verb end to end
  in a jj-native tree, and the rename-shaped ones in a colocated tree where
  git's index must come out untouched. The leaf's premise had half-decayed
  under it — `repo.rs`, `tree_rename.rs` and the migration's commit path had
  each grown jj tests since it was cut — so what was actually missing was
  per-*verb* proof that no verb carries a git-only side path, which no test of
  a seam can give. Both properties were **falsified by mutation rather than
  assumed**: removing jj-first from `rename_entry` fails exactly the 5
  colocated tests, removing `.jj/` from `vcs_of` fails all 13. The docs half
  was four small prose corrections; ADR *symmetric-vcs-rule* needed no rework
  but was under-described (`repo.rs` cites it while it named only the skills)
  and now states the rule binds the binary too.
- **compose-task-chains-k29** — make the review chain (`X` → `review-X` →
  `integrate-review-X`) and the research vendor pair the *habitual* shape a
  session cuts leaves in — in `SKILL.md`'s Decompose step, in `TASK-FORMAT.md`,
  and in the **bootstrap** prompt, which cuts the decomposition that shapes every
  later session — and give a cut chain a naming structure that makes it legible
  from `find .grove` alone. Encouragement only: grove validates no ordering
  between leaves and this must not start (*task-kind-taxonomy*). Raised by the
  human during **herdr-turn-hooks-k4**; independent of the herdr work. Sequenced
  ahead of the remaining planning leaf so that session cuts under the new
  guidance.
  **Done.** The premise was confirmed by this grove's own tree — 32 keys, 26
  leaves, **zero** chains and zero pairs, despite both patterns being documented
  in five places. All five were *reference* material; none was the guidance a
  session reads **while cutting leaves**. Fixed in the three surfaces in the
  order a session meets them (bootstrap prompt, `SKILL.md` Decompose,
  `TASK-FORMAT.md`), each stating the **escalation call** rather than only the
  shape — chain a load-bearing artifact, subagent a one-file change. The open
  naming question resolved to a **shared stem plus a terminal step suffix**
  (`<stem>-review` / `<stem>-integrate`; `<stem>-a` / `<stem>-b` /
  `<stem>-combine`): node-per-chain lost on what a node *already means* in a real
  tree, and a leading token loses because it scatters the chains it was meant to
  reveal. Durable record `docs/specs/task-kind-taxonomy.md`; `driving.md`'s two
  worked examples were both prefix-shaped and were corrected. Nothing gained a
  parser, and `resolve` was measured to match slugs **exactly**, so
  prefix-sharing slugs introduce no ambiguity. The guidance is embedded in the
  rebuilt binary but **not the installed one** — the same in-the-tree-not-in-the-
  binary gap as v16.0.0's reporter and v16.1.0's hooks, and it reaches real
  sessions only when **v16.2.0** is cut (entry written, not cut).
- **herdr-pane-misdetection-k11** — grove panes are labelled with the wrong agent;
  upstreaming is closed, so the route was ours to pick. Late because grove's own
  reports mask it whenever grove holds authority. Independent of everything above.
  Now a node: the planning session found a **fourth** route the leaf did not list,
  and it dominates all three — `HERDR_AGENT=<agent>` is a *documented upstream
  extension point* for exactly this shape (a host-visible wrapper hiding the real
  agent), so the fix is one environment variable on the harness child, on stock
  herdr as much as on the fork. The two candidates the leaf favoured both lost to
  it: restructuring the process group buys the same outcome by rewriting the
  driver's signal topology, and a third fork hunk is a permanent rebase obligation
  for a fix that needs no fork. Hence **agent-hint-k33** (set it) then
  **agent-hint-observe-k34** (measure it, and rework *herdr-optional-ui*'s
  "undecided, out of scope" paragraph). Needs no release: the observer drives a
  pane from `./target/debug/grove`.
  **Done.** One environment variable at two launch sites, and the A/B on real
  panes is unambiguous — identical process shapes read `codex` without it and
  `claude` with it. The measurement returned more than the label: the fix also
  repairs `fallback_state` (a stalled session now reads `blocked` off the right
  screen manifest), the hint is inherited by the harness's whole subtree, and it
  **dies with the harness** — which corrected a claim in the v16.2.0 changelog
  entry. Like **compose-task-chains-k29**, it is in the tree and *not* in the
  installed binary: real groves get it at **v16.2.0**. Only **claude** was
  measured; codex and pi follow by construction (same helper, same shape, both
  names are herdr labels) but are unobserved.
- **guard-loop-signal-k37** — `cargo test` in this repo killed the `grove do`
  session it was typed into. Raised by the human during **codex-grant-refused-k35**
  and sequenced ahead of it, because it makes every later session unsafe to test
  in and it is k35's own in-flight code that fires it.
  **Done.** The cause is that authority to end a session was **ambient**:
  `GROVE_SIGNAL_FILE` is inherited by every descendant, and the driver kills on
  the file's mere appearance. Pinned by measurement to five codex-launch tests,
  all of them k35's — its sandbox pre-flight spawns the harness binary outside
  `launch_session`, the one site that scopes the variable, so the suite's fake
  harness wrote the *live* path. Fixed on the suite side with two independent
  guards (`.cargo/config.toml` forced `[env]` override; the shared
  `tests/support` scrub list) asserted by `tests/env_hygiene.rs`; ADR
  *self-driving-loop* now states the invariant on the exporting side. The
  product-side half — no harness spawn but the session's own may inherit the
  control env — is recorded on **codex-grant-refused-k35**, whose code it is.
  A per-session **nonce was rejected**, and the reason generalises: the nonce is
  inherited too, so authority granted through the environment cannot be
  authenticated from inside it.
- **codex-grant-refused-k35** — a codex `grove do` launch dies at startup: codex
  refuses grove's `--add-dir` VCS-store grants because the effective permission
  profile is neither `workspace-write` nor off, and the loop stops on the mute
  exit. Raised by the human during **agent-hint-observe-k34**, from a different
  grove (`UIAnyware`, colocated jj); independent of the herdr work. Sequenced
  ahead of the remaining leaves because it blocks a live grove, and because it
  fires ADR *codex-gitdir-grant*'s own reopen condition ("unexplained codex
  launch failures surface in the field") verbatim.
  **Done.** The third case the ADR missed was the **default** one, not an exotic
  profile — see Notes. Every codex launch is now pre-flighted against codex's own
  verdict and **refuses before spawning**, naming both remedies; a refused launch
  parks the pane `blocked` with authority held, which is exactly what it is. It
  also carries the product-side half k37 handed here: one helper, called by all
  three harness spawn sites, and exactly one of them grants a signal path back.
  Both properties were **falsified by mutation**, not assumed. Like
  **compose-task-chains-k29** and **agent-hint-k33**, it is in the tree and not in
  the installed binary — real groves get it at **v16.2.0** (entry written, not
  cut).
- **chain-group-unit-k36** — design. grove has one decomposition mechanism and
  reuses it for two unlike things: genuine vertical-slice decomposition, and a
  **step chain over one artifact** (`X` → `review-X` → `integrate-review-X`; the
  research vendor pair), which exists only because a step boundary is the sole
  place grove can switch harness or model. Decide whether a chain is first-class —
  a unit `pick` will not wander out of, and whose close needs no confirmation —
  against constraints 3 and 5, which both push the other way.
  **compose-task-chains-k29** stopped deliberately at encouragement; this asks
  whether that was far enough. Raised by the human during
  **codex-grant-refused-k35**; independent of the herdr work.
  **Done — the answer is no, and k29's stopping point was far enough.** Two of the
  three costs motivating the construct **do not exist**: a chain's steps are
  already ordered by adjacency, which is the same ordering a node's children get
  (`pick` returns the first live leaf in pre-order and stops), and the Retire
  cascade's confirmation is asked **per node**, so a chain — deliberately not one —
  is never asked one. That second finding *reinforces* k29's rejection of
  node-per-chain on a new axis rather than reversing it: the confirmation is a cost
  a directory-shaped chain would introduce. The one real gap — a sibling-level
  `leaf-insert` can split a chain where containment would not — is repaired by one
  more insert, and closing it would cost `pick` its defining property. ADR
  *task-tree-scheme* now carries the invariant (**`pick` is a walk, not a
  scheduler**), which generalises past chains: answering *is a group in flight?*
  needs either state outside the tree or a rule that skips live work and ranks
  groups. It would also **gate** — a chain `pick` will not leave overrules the one
  verb that exists to preempt — so the request to remove a gate would have
  installed a larger one. Zero code changed; the guidance gained one operational
  habit (**cut a chain's steps together; `leaf-insert`, not `leaf-add`, for a step
  decided on after its producer ran**), which is the shape that actually breaks a
  chain in practice. Like k29 and k35, it is in the tree and not in the installed
  binary — real sessions get it at **v16.2.0**. Its unanswered half — whether
  *construction* needs more than encouragement — is **chain-construction-k38**.
- **tap-caveats-reconcile-k24** — the Homebrew formula's caveats still describe
  upstreaming as pending. Text-only, independent, low priority.
  **Done.** `Formula/linkuistics-herdr.rb` in `Linkuistics/homebrew-taps` now
  says the carry is permanent and names each half's real end condition
  (upstream separating identity from state; upstream shipping a geometry API).
  The `ui.layout` half went the same way: "unsubmitted upstream" read as pending
  intent, and the policy is blanket — the fork is ours to maintain, not a
  staging area. Its durable half is in `docs/specs/herdr-fork-maintenance.md`,
  which now records *why* that branch exists (Modaliser needs the drawn
  cell-rects; upstream has no `ui.layout` at all) and that Out-of-scope covers
  both patches. Committed and **pushed**, because the deliverable is what
  `brew info` prints, not what the checkout holds — a three-hop gap, the same
  in-the-tree-not-in-the-binary shape as v16.0.0's reporter. Verified by reading
  `brew info` after the tap clone caught up; no version bump, and `brew
  outdated` stays silent, so no user is rebuilt for a text fix.
- **release-doctor-toolchain-gap-k27** — two frictions the release path makes the
  operator remember. `release-doctor.sh` passes while the release build dies,
  because the doctor asks *rustup* what targets are installed and the build asks
  whatever `cargo` is on `PATH` (found by **ship-release-k25**, and it bit again
  in **observe-mid-turn-live-k31**); and `cargo release` refuses jj's
  always-detached HEAD, so a cut needs `--allow-branch HEAD` that `release.toml`
  should carry. Independent of the herdr work, sequenced last.
  **Done.** Both frictions are now configuration rather than memory — see Notes.
  The set-vs-diagnose question resolved to **set, then verify what you set**:
  `scripts/release-common.sh` (new, sourced by both doctor and build) prepends
  rustup's shim dir to `PATH`, and the doctor's target check interrogates the
  **resolved** toolchain instead of asking `rustup`. Three narrower remedies were
  measured and *rejected*, which is the finding — the gap is a **rustc-on-PATH**
  problem, not a cargo one. No ADR: a four-line PATH prepend fails the
  when-to-write test's first leg (hard to reverse), so the reasoning is a
  load-bearing script comment, this repo's existing idiom. No CHANGELOG entry
  either — release scripts ship nothing, and that file records what the repo
  ships.
- **session-leaf-binding-k28** — design. The driver resolves the leaf *before* the
  session exists (that peek is what binds harness and model), then hands the
  session no leaf identity, so the session re-picks independently. They agree only
  because nothing mutates `.grove/` in between — an unenforced coincidence whose
  failure mode is a session silently running the wrong model for its kind. Decide
  whether to bind, and reconcile the skill's Pick step with where the pick really
  happens. Raised by the human during **observe-live-surface-k26**; independent of
  the herdr work.
  **Done — do not bind; the tree wins.** The coincidence is real but the framing
  was inverted: the driver's peek runs *before the session exists*, so it is a
  **forecast**, and the session's own `pick` is the **fact**. Binding would make
  the forecast beat the tree — authoritative state outside `.grove/` (constraint
  1) that also silently discards a `leaf-insert`, the one verb that exists to
  preempt. Three more counts in ADR *model-per-task-kind*, which gained the
  invariant and the rejection rather than a new ADR (the same under-described-ADR
  repair **jj-first-coverage-k6** made to *symmetric-vcs-rule*). The divergence was
  **constructed, not assumed**, and it is worse than the leaf claimed —
  **cross-vendor**, not just a wrong model, and so *not* correctable in-session
  the way `/model` corrects the model axis. What retires the question is that the
  loop **self-heals in one iteration** off the same zero-state re-derivation that
  gives restart ≡ continuation, bounding the residual at one leaf, once. Its one
  open half — the misroute is invisible, since the launch line names harness and
  model but not the leaf — is **routed-leaf-diagnostic-k41**.
- **herdr-sidebar-tokens-k32** — design. Whether a `grove do` pane's **sidebar row**
  should carry the live leaf as `pane.report_metadata` tokens, and which side would
  report them. Split out of **herdr-grove-plugin-k5**, which decided it did not
  belong there: the plugin *reads disk*, tokens are a *push*, and a token renders
  only if the user has put `$name` in their own sidebar config. A well-argued **no**
  is the cheaper answer and retires the question for good.
  **Done — the answer is no, and it is a firmer no than the leaf framed.** The
  config demand turned out not to be the decisive cost, and the leaf's premise
  inverted on inspection: `pane.report_metadata` is **four** fields, and the two
  that render with *no* user configuration are the invasive ones — `display_agent`
  **replaces** the pane's agent label, and `title` outranks the human's own typed
  pane name in the navigator (see Notes). So a zero-config route existed and is
  worse, not better. What actually decides it is that **no reporter can hold the
  leaf honestly**: the driver's leaf is a forecast — **session-leaf-binding-k28**'s
  own measured, cross-vendor divergence — a grove verb is ruled out by
  *herdr-optional-ui*'s standing consequence, and the turn hooks reach claude only,
  so on codex and pi the row would be silently absent, where absence reads as *no
  grove* rather than *not reported*. ADR *herdr-optional-ui*'s "still fine as a thin
  display-only garnish" is **removed** — reworked in place, with a reopen
  condition — and `CONTEXT.md` gained the matching `_Avoid_`. Zero code changed. A
  user who wants a per-pane line already has one at zero cost to grove:
  `terminal_title_stripped` in their own rows, which the harness writes and a
  `live-handoff` carries, unlike metadata.
- **chain-construction-k38** → **chain-construction-review-k39** →
  **chain-construction-integrate-k40** — a review chain over one design: how to
  make the review chain and the research vendor pair **reliably constructed as a
  unit**, up to and including a `grove-llm` verb that cuts a whole chain in one
  call. Explicitly **construction only** — the human's framing is that sequencing
  needs no enforcement, which is also what **chain-group-unit-k36** independently
  concluded. This is k36's unanswered half and the next question after
  **compose-task-chains-k29**, which fixed the three cutting-time surfaces and
  stopped at encouragement on purpose. Raised by the human during
  **release-doctor-toolchain-gap-k27**; independent of the herdr work. Cut as a
  chain, at once, per k36's own operational habit — and the friction of doing that
  by hand (three `leaf-add` calls, three chances to stop after the first, the
  naming convention re-derived from this brief) is itself evidence for k38.
  **k38 done — the answer is yes: one call per shape, `leaf-add-chain` and
  `leaf-add-pair`.** The decision does *not* rest on the friction that raised it,
  which is thin (post-k29 this tree cut one chain in nine leaves, and that one was
  maximally primed — a design leaf about chains, cut right after k36 concluded on
  chains). It rests on ADR *cli-binary-split*'s own recorded rule — every
  deterministic step with a stable input/output shape is a verb rather than an
  instruction a model might paraphrase wrong — which as written would admit almost
  anything and now carries a **three-leg bar** (deterministic; derived from
  something grove already owns; the judgement stays outside). Leg 2 is what the
  verb actually buys: `<producer>` ⇒ `review-<producer>` ⇒
  `integrate-review-<producer>` is the parameterisation the seventeen-kind set
  bought and nothing had spent, and hand-transcribing it admits a
  **wrong-but-well-formed** kind — a silent misroute the `--kind` write-gate
  cannot catch, because `--kind review-impl` beside a `design` producer is a valid
  invocation. Leg 3 is why this is **not** a quiet reversal of
  **compose-task-chains-k29**: that rejection targets a `--chain` *flag on
  `leaf-add`*, where the verb would make the escalation call on the strength of an
  argument that meant one leaf, and it stands. Its closing generalisation ("the
  verbs stay one-leaf-per-call") does not — it was already false when written:
  `root-init` writes a root brief and a first leaf, `leaf-decompose` a brief and a
  first child. The kind set settles
  the two-shape question **by construction**: `research` is the one kind with no
  `review-` sibling, so it gets its own verb rather than a mode, and one verb
  dispatching on `--kind` was rejected because `research` is then the only legal
  value — a mode selector wearing a parameter's clothes. Constraints 3 and 5 are
  **not strained**: `leaf-add` is untouched, no tree is inspected, nothing is
  parsed, and the refusals are authoring-time argument validation with a human
  present. Zero code changed; no new ADR (it fails the hard-to-reverse leg). The
  named risk is k29's failure with a compile step — a verb nobody reaches for —
  and the design's only answer is **placement**, which is a claim the
  implementation has to make good on. Implementation is deliberately **not** cut
  here: **chain-construction-review-k39** has not read this yet, and a producer
  that banks its conclusion before the reviewer is the failure mode the chain
  exists to prevent.
  **k39 done.** The direction survived, with two behavioural holes that would have
  let either verb emit the wrong-but-well-formed result it exists to prevent, plus
  an ADR left internally incoherent by adopting verbs it did not enumerate. Its
  clean bills on constraints 3 and 5 are the ones that matter, and its one
  *conditional* verdict — encounter at cutting time — bound k40 to **replace** the
  hand-cut procedure rather than append to it.
  **k40 done — both verbs shipped, all three findings applied, none declined.**
  `leaf-add-chain <parent> <stem> --kind <producer>` and `leaf-add-pair <parent>
  <stem> --harness-a <n> --harness-b <m>`. The interesting outcome is **F2**: the
  review offered two remedies and both were wrong in the same way, because
  producer A's harness is not a fact to compare against — it resolves at *launch*
  time, so comparing it at construction compares an argument against a **forecast
  about the environment**, which is **session-leaf-binding-k28**'s inversion
  exactly. The third route makes the property durable: **both** producers are
  declared and an equal pair is refused, so "two corpora" is two `**Harness:**`
  lines in the tree and the check is between two *arguments*. What the caller
  gives up — *"the usual vendor, and a different one"* — is not expressible as a
  fact about a leaf, which is the point. **F1** made one call one mutation
  (validate-then-allocate-then-sweep-then-write, rollback, stdout buffered until
  success); its two arms are provoked by a *directory* wearing a leaf's name and
  by a derived name crossing `NAME_MAX` — the latter a hazard **specific to
  composite verbs**, since the derived names are longer than the stem the caller
  validated. Both properties, and the derivation itself, **falsified by mutation**.
  Two things worth carrying past this chain. The design's answer to k29's failure
  mode is **placement**, so placement is *asserted* (`--help` ordering is a test),
  not assumed — and the guidance **replaced** the hand-cut procedure in all five
  surfaces rather than sitting beside it, since parallel old-and-new guidance is
  how k29 failed the first time. And the chain's own history came back **negative**
  on the friction argument: `leaf-add-chain . chain-construction --kind design`
  would have emitted byte-for-byte what the three hand-cut `leaf-add` calls
  produced, so this chain is evidence for friction (which k38 already judged thin)
  and *not* for the error class the verb closes. Constraint 4 is recorded as
  **lightly strained** in the spec — adopted on reasoning about that error class,
  not on a measured incident. Like k29, k35 and k36, it is in the tree and not in
  the installed binary; real sessions get it at **v16.2.0**, whose `### Added`
  entry is written but not cut.
- **routed-leaf-diagnostic-k41** — make the driver's launch line name the leaf it
  routed on. Split out of **session-leaf-binding-k28**: the only part of that
  complaint k28's decision leaves standing is *visibility*, and a diagnostic
  closes it with no gate, no env export, and no state outside the tree. Stands
  alone too — *what it is working on* is this brief's own goal, and it is absent
  from the driver's own output. Deliberately **not** a chain (a one-file
  diagnostic; k29's escalation call says subagent, not `review-impl`).
  **Done.** `grove: launching claude (model: opus) — session-leaf-binding-k28
  (design)`. The leaf is named by its stable handle, not its path — so the
  driver's line and `--no-launch`'s deliberately disagree, the latter printing a
  path because it names something the operator opens *next* while scrollback
  outlives the position a `leaf-insert` moves. Both properties were **falsified
  by mutation**: removing the degrade makes the bootstrap line read `— ?
  (requirements)`, and rendering `name()` instead of `handle()` emits
  `02-picked-k7.md`. Two things it cost that the leaf did not predict. The
  handle had no renderer — it was spelled inline in three places — so the id
  model gained `Entry::handle()` as the position-free counterpart of
  `Entry::name()`, which is where task-tree-scheme §5's distinction belongs. And
  `kind` as an eighth parameter tripped clippy, which was right: the four
  routing fields are decided together by `resolve_launch` and only mean anything
  together, so `launch_session` now takes the whole `&Launch`. What the line
  **cannot** do is show a divergence by itself — it names the forecast, and
  seeing the disagreement still means comparing it against what the session
  reports working on; that is the `_Avoid_` added to **Kind routing**. In the
  tree, not the installed binary: real groves get it at **v16.2.0**.

## Pointers

- ADRs a session here must read: *herdr-optional-ui*, *self-driving-loop*,
  *task-tree-scheme*.
- *herdr-turn-boundary-hooks* — the second reporting mechanism (hooks grove
  injects into a claude launch), and why codex and pi are blocked on facts.
- *model-per-task-kind* — **task-kind-taxonomy-k3** reworked the mechanism that
  ADR describes.
- Glossary terms in play: herdr integration, Kind routing, HITL/AFK,
  Task kind (see `CONTEXT.md`).
- herdr's source is checked out at `~/Development/herdr` — `AntonyBlakey/herdr`,
  a fork with `ogulcancelik/herdr` as `upstream`. The reducer deciding whether a
  state report lands is `src/terminal/state.rs`; screen manifests are
  `website/agent-detection/<agent>.toml`; integration assets (the installed hook
  scripts) are `src/integration/assets/<agent>/`.
- The fork is **already in production**: `/opt/homebrew/bin/herdr` resolves to
  `linkuistics-herdr`, built from the fork and shipped from `linkuistics/taps`,
  the same tap grove ships from. How the carry is maintained — branch layout,
  rebase cycle, version suffix scheme, the required build environment, and the
  A/B check that verifies a rebase — is
  `docs/specs/herdr-fork-maintenance.md`. Read it before touching the fork.
- **We do not contribute upstream.** Offering the authority patch as a `fix:` PR
  was drafted and then decided against; the fork is a permanent carry (ADR
  *herdr-optional-ui*). Do not re-propose it, and do not file issues upstream
  either. Read the fork as ours to maintain, not as a staging area for
  contributions.

## Notes

Findings the leaves above depend on, **re-verified in full on 2026-07-27** by
`herdr-notes-reverify-k17` — against `upstream/master` at `dc2506ea`, with
`authority-fix` at `b1484e37` and `ui-layout` at `d17e0f42`. Every claim held;
two were widened. They are stated as **behavioural contracts, not line
references**, because `state.rs` moves under them. This is still a repo we do not
control: re-verify anything load-bearing before building on it.

- herdr's `claude` and `codex` integrations are **session-identity only, by
  design**. Each installed hook script gates on
  `case "$action" in session) ;; *) exit 0 ;;`, and the only RPC it can send is
  `pane.report_agent_session`; neither pair is in the authority allowlist. For
  those agents, 100% of `idle`/`working`/`blocked` comes from regex over the
  terminal buffer.
- herdr **used to** install lifecycle hooks for both and now installs exactly one,
  `SessionStart → session`. Both the install *and* the uninstall path strip the
  retired set, which is wider than this brief previously recorded — for claude:
  `SessionStart→idle`, `UserPromptSubmit→working`, `PreToolUse→working`,
  `PostToolUse→working`, `PostToolUseFailure→working`, `SubagentStop→working`,
  `PermissionRequest→blocked`, `Stop→idle`, `SessionEnd→release`. That list is
  herdr's own retired event→state mapping for claude, and it is where
  **herdr-turn-hooks-k4** should start rather than re-deriving one. Note it maps
  `Stop → idle`, which is exactly why it never helped — see the next point.
- **`done` is derived, not reported**: `Idle && !seen`, at three independent
  sites (the agent view's status name, the API's pane status, the navigator's
  `Done` filter). The real state machine is idle/working/blocked, so "finished"
  and "waiting on you" both land on `idle` unless something reports `blocked`.
- A state report whose `agent` label parses to a *different known agent* than the
  one herdr detected is **silently dropped**; an **unrecognised** label (`grove`)
  bypasses that gate, because the check only fires when the label parses to
  something — and `grove` is absent from herdr's agent-name table. It also
  prevents a screen-detected blocker from overriding the report. Both halves
  measured true by **herdr-pane-state-k2** — but they are not sufficient:
- **De-facto unforked authority does not exist.** A *third* gate drops any report
  whose `(source, agent)` differs from whoever owns the pane's **session
  identity** — and that owner is the harness's own herdr integration, claimed at
  every SessionStart. The session-identity-only integrations dismissed above as
  inert are exactly what lock grove out. This fired *herdr-optional-ui*'s own
  reopening condition for the fork option; `herdr-authority-route-k7` settled it,
  and the ADR carries the outcome.
- Full lifecycle authority is a **compiled-in allowlist** — still exactly six
  `(source, agent)` pairs (`pi`, `omp`, `mastracode`, `opencode`, `kilo`,
  `kimi`), with `hermes` alone in the separate session-identity-only category.
  Nothing reachable from outside the binary — a plugin included — can join it,
  which is why the plugin owns UI only, never state. It is *also* not the way in
  for grove, and the reason is sharper than "the allowlisted path demands a
  session_ref": a **non**-allowlisted report is waved straight through the
  routing step, whereas an allowlisted one whose label does not parse to the
  detected agent falls through to a branch requiring both a `session_ref` and a
  `seq`, and is dropped without them. Allowlist membership is a stricter path,
  not a fast lane.
- grove panes **were mis-detected, and now are not** (fixed and observed live
  2026-07-28, `agent-hint-observe-k34`). The cause: herdr prefers the
  process-group *leader*, and a grove pane's leader is **`grove` itself**,
  unidentifiable — so it falls back to scoring the whole group, where a
  `codex mcp-server` helper outranks the real harness. (The root brief originally
  blamed MCP servers inheriting the harness's process group; herdr already
  defends against precisely that — upstream #161, v0.5.11 — and grove sitting at
  the head of the group is what disables the defence.) The fix is one environment
  variable, `HERDR_AGENT=<harness>`, set on the harness child at both launch
  sites: herdr's probe reads **non-leader** hints before it reaches group
  scoring. No fork hunk, no process-group surgery.
  **The A/B, on real grove panes with byte-identical process shapes** (leader
  `grove`, live `claude`, `codex mcp-server` helper), against the installed
  `0.7.5-linkuistics.1`: the hint-less build read `agent: codex` with
  `agent_status: done` *while claude was alive* — the headline complaint, in
  miniature — and the hint-carrying build read `agent: claude`. On a real
  `grove do` pane with grove's authority released to expose detection, the pane
  re-acquired as `claude` after ~14s and herdr then read a stalled grilling
  session as **`blocked`** off claude's own screen manifest, before grove's turn
  hook self-healed back to `agent: grove`. So the fix buys more than a correct
  label: it makes `fallback_state` — which *herdr-optional-ui* used to disclaim
  for grove panes — actually work underneath grove's own reports.
  Two bounds worth carrying: the hint is **inherited by the harness's whole
  process subtree** (even the `codex` helper ends up carrying
  `HERDR_AGENT=claude`, so it cannot win the hint path either), and it rides the
  harness's **exec-time environment**, so it dies with the harness — which is why
  a pane released at `complete --done` never snaps back to a detected agent.
- **macOS discloses no environment at all for SIP-protected platform binaries**,
  and `kern_procargs2` is exactly what herdr reads — so a hint set on, say,
  `/bin/sleep` is invisible, while `claude`, `codex` and `grove` all read ~100
  environment tokens fine. This bit while building a synthetic rig, not in
  production (no harness is a platform binary), but it is the reason a
  hand-rolled reproduction with system binaries silently proves nothing. Copying
  a signed system binary to dodge it does not work either — the copy fails its
  code signature and is killed on Apple Silicon. Compile a throwaway instead.
- **The status surface is live in production, and verified end to end**
  (2026-07-27, `observe-live-surface-k26`). Both silences that made it inert are
  closed: `ship-release-k25` shipped **v16.0.0** so the installed `grove` carries
  the reporter, and the server was replaced by live handoff (PID 3825 → 77248),
  so the running herdr carries the patch. What was measured on a real pane whose
  `agent_session` was owned by `herdr:claude`: the fork acceptance test passes on
  all four points (report lands, a *second differing* report lands, release
  returns the pane, `agent_session` byte-identical throughout), and on a real
  `grove do` pane the driver reports `working` at launch, **`blocked` held** on a
  no-signal stop, and **releases** on `complete --done`. `working` was also seen
  overwriting a stale `blocked` — the ADR's self-healing claim, observed.
  Two observation nuances, both now in the spec: release reads `agent: null` /
  `agent_status: unknown` rather than snapping back to the detected agent, and
  the `idle` before release is not externally observable at 30 ms polling.
- **Hooks merge; they do not clobber** (measured 2026-07-27,
  `herdr-turn-hooks-k4`). claude's `--settings` takes inline JSON as an
  *additional* settings source and hooks are **unioned** across sources — proved
  twice, once against a project `settings.json` and once live, where herdr's own
  installed `SessionStart` hook claimed session identity in the same run as
  grove's injected turn reports. Hook subprocesses also inherit the driver's
  environment (`GROVE_SIGNAL_FILE`, `HERDR_*`), which is what makes the
  discriminator readable from inside the session. A `grove-llm` invocation costs
  **~3ms**, socket or no socket — so per-turn reporting is cheap, and the only
  real objection to going per-*tool-call* is socket chatter, not latency.
- **The shipped claude binary is readable, and it is the better source than the
  hooks docs** (measured 2026-07-27, `herdr-mid-turn-blockers-k30`). It is a
  Mach-O with the bundled JS inside, so `strings` over
  `~/.local/share/claude/versions/<v>` gives the actual implementation; the
  docs page contradicts itself about `Notification` matchers and the
  implementation does not. What that settled, all against 2.1.220:
  matchers **do** filter `Notification`, keyed on the payload's
  `notification_type`; a matcher drawn from `[A-Za-z0-9_|]` is compared as an
  **exact-string alternation**, not a substring regex; an unknown hook **event**
  name is dropped with a warning while the rest of the block still applies (an
  unknown *matcher* value is simply inert); `permission_prompt`,
  `elicitation_dialog` and `elicitation_url_dialog` come from one shared
  *idle-notify* helper that fires **once, after six seconds with no human
  interaction**; and `PermissionDenied` fires only for the auto-mode classifier,
  never on an interactive denial. `PostToolBatch` exists and fires once per model
  round trip, but appears in no changelog entry, so it is too new to assume.
- **A `Notification` cannot be provoked from `claude -p`** — it is raised by TUI
  dialog components, so print mode never reaches one. Everything else in the
  injected block *is* reachable: feed grove's own generated `--settings` to
  `claude -p` against a `UnixListener` and diff a tool-using prompt against a
  tool-free one. Driving interactive claude under `expect` was tried four times
  and did not get the model as far as a permission prompt.
- **codex has no turn-end hook event**, verified against codex-cli 0.145.0: the
  set is `pre_tool_use`, `permission_request`, `post_tool_use`, `pre_compact`,
  `post_compact`, `session_start`, `session_end`, `user_prompt_submit`,
  `subagent_start`, `subagent_stop`. Independently, hook **trust is persisted**
  per source-location and content hash in `~/.codex/config.toml`'s
  `[hooks.state]`, so a `-c`-injected hook has no trust record.
- **pi's herdr extension reports `idle` at turn end** — `agent_settled` with no
  outstanding `herdr:blocked` yields `idle`
  (`src/integration/assets/pi/herdr-agent-state.ts`). So pi is a full lifecycle
  reporter that still has the headline bug. It also dedups (`lastState`), which
  is the pattern to copy if grove ever needs redundancy suppression. And
  `pi -e <path>` **is** a per-launch injection route — contra
  **herdr-turn-hooks-k4**'s original note that none was found.
- **The turn hooks are in the installed binary from v16.1.0** (shipped
  2026-07-28, `observe-mid-turn-live-k31`). Until then everything
  **herdr-turn-hooks-k4** and **herdr-mid-turn-blockers-k30** built was invisible
  in production for the same reason the reporter was before v16.0.0: it was in
  the tree, not in the binary that launches sessions. `strings` on the installed
  `grove` now shows `report-turn`, `UserPromptSubmit`, `PostToolUse`,
  `Notification`, `permission_prompt` and `elicitation_dialog`. **The one-line
  check that a session is running under a hook-carrying driver is its own argv**:
  `ps -o command= -p $PPID` shows `--settings` or it does not.
- **The mid-turn pair is observed end to end in production** (2026-07-28,
  `observe-mid-turn-live-k31`), on claude 2.1.220 under the v16.1.0 driver. A
  real permission dialog left untouched ~10s reported **`blocked`**, and granting
  it returned the pane to `working` *mid-turn* — a further tool call followed in
  the same turn. Neither half is an artefact of hand-invoking `report-turn`;
  `PostToolUse ⇒ working` was separately isolated by putting the pane at
  `blocked` inside a tool call and watching the hook restore it with no report
  command running. The sidebar rendering is confirmed against herdr's own source:
  `state_dot` maps `Blocked → red`, and **red is unique to blocked** across all
  five rows (`src/ui/status.rs`), so the red dot an operator sees *is* `blocked`.
- **The version-skew stop row is observed**: `blocked` 14s after the signal, held
  5½ min, `agent=grove` never released — `plan_for(Stop::VersionSkew)` exactly.
- **ADR *herdr-optional-ui*'s state table is now observed in full, in
  production** — the last two rows on 2026-07-28 by `observe-mid-turn-live-k31`,
  from a detached observer armed before its arming session's completion signal
  (`target/k31-relaunch-interrupt.log`). **Relaunch**: across the session swap
  (child 39119 → 79090 under driver 39105) the pane held `agent=grove
  status=working` with **no `agent=null` sample** — nothing released, exactly
  `plan_for(Stop::Signal(Some(Relaunch)))`. **Interrupt**: a SIGTERM to the
  driver yielded `agent=null status=unknown` 2s later — released, reporting
  nothing, exactly `plan_for(Stop::Interrupted)`; the 2s is the child's kill
  grace, since the driver applies the plan only after the session it was killing
  is reaped. **SIGHUP is covered by code path, not separately observed**: both
  signals land on one handler and one `Stop::Interrupted`
  (`src/loop_driver.rs:1259-1278`).
- **The relaunch row's *silence* remains unobservable, by construction**, and
  rests on the unit-pinned policy table rather than on measurement: with the pane
  already at `working`, "reported nothing" and "re-reported the state it already
  held" produce identical samples (the `revision` finding), and arranging a
  different pre-state across a relaunch would need a third cross-session observer
  to buy a distinction no operator can see. What the row asserts *operationally*
  — the pane is never released mid-loop — is what was measured. Also observed at
  the release: `agent` stayed `null` and `agent_status` flapped
  `unknown`→`idle`→`unknown`, never re-acquiring an agent name, which is screen
  detection reading a pane whose harness is dead — not a stalled sweep.
- **Two bounds on how far the mid-turn row reaches**, both now in ADR
  *herdr-turn-boundary-hooks*. A **permissive permission mode raises no dialog at
  all** — under `defaultMode: "auto"` with `skipDangerousModePermissionPrompt`,
  an `rm -rf`, an explicit sandbox override and an un-allowlisted MCP call all
  ran unprompted, so nothing to report; provoking the row needs a *prompting*
  mode. And the **six-second timer is gated on human inattention, not elapsed
  dialog time**: dialogs held several seconds with the human present did not fire
  it. Both are the design working (it detects *unattended*), but they mean the
  row is not a general answer to "grove is stuck".
- **`herdr pane get`'s `revision` is not a report discriminator** — it tracks
  pane lifecycle, not agent state, and stayed put across three state changes. So
  "reported the same state" and "reported nothing" remain indistinguishable from
  the socket; the only way to tell a silent row from a re-reporting one is to
  arrange a *different* pre-state, or to watch for the `agent=null` that only
  release produces.
- **Cutting a release needs no workarounds any more** (fixed and measured
  2026-07-28, `release-doctor-toolchain-gap-k27`). Plain
  `cargo release <level> --execute` and `scripts/release-build.sh` now work as
  written: `release.toml` carries `allow-branch = ["*", "HEAD"]`, and the build
  pins its own toolchain. Run the cut from the **colocated default workspace**
  (`jj workspace root --name default`) — cargo-release drives git, and a
  jj-native secondary workspace has no `.git` at all. The rest of the path is
  unremarkable and jj-clean: git makes the detached release commit and tag, jj
  imports it, and `jj bookmark set main -r <release-change>` puts the bookmark
  on it.
- **The toolchain gap was PATH *ordering*, not a missing rustup** (measured
  2026-07-28, same leaf). `$HOME/.cargo/bin` was on `PATH` all along — at
  position 23, behind `/opt/homebrew/bin` — so every "is rustup installed?"
  check answered yes while the build got Homebrew's rustc. Three narrower
  remedies were measured and **do not work**: invoking `$(rustup which cargo)`
  by absolute path fails identically, because cargo resolves `rustc` from
  `$RUSTC` or `PATH` and rustup's cargo 1.93.1 happily drove Homebrew's rustc
  1.97.1; `rustup run <toolchain> cargo …` is a **no-op**, setting only
  `$RUSTUP_TOOLCHAIN` and never touching `PATH`, so on a machine whose `PATH`
  lacks the shim dir it changes nothing; and `rustup target list --installed`
  answers for rustup's toolchain whatever the build will use. What does work is
  prepending the shim dir, which yields one coherent toolchain (cargo, rustc and
  the `cargo-*` subcommands) and would honour a `rust-toolchain.toml`.
  The honest probe for "can this build the target" is
  **`"$rustc" --print target-libdir --target <t>` plus a directory-existence
  test** — the command prints a path for any recognised target whether installed
  or not, so the exit status proves nothing while the `-d` test reproduces the
  E0463 boundary exactly. The A/B, in one environment: the old check printed
  `✓ aarch64-unknown-linux-gnu` where the new one fails it and the build dies.
- **herdr's plugin surface is enough for a viewer and nothing more** (measured
  2026-07-28, `herdr-grove-plugin-k5`, against 0.7.5). Four facts, in descending
  order of how much time they cost: a **`plugin pane open --cwd` REPLACES the plugin
  root as the process cwd**, after which herdr resolves the manifest's *relative*
  command against the new directory and the pane dies on spawn with no log entry —
  and the override is unnecessary, because `HERDR_PLUGIN_CONTEXT_JSON` already
  carries `focused_pane_cwd` on **both** the split and the overlay path
  (`plugin_context_for_pane` / `current_plugin_context`). A **keybinding can only
  target a plugin action**, never a pane entrypoint, so a pane needs a one-line
  action wrapper calling `plugin pane open`. `herdr pane read` returns a snapshot
  **one column narrower than the pane**, which reads as a truncated right edge that
  is not there. And `pane.report_metadata` is display-only *and* inert until the user
  adds `$name` to `[ui.sidebar.agents] rows` in their own config — the fact that
  decides **herdr-sidebar-tokens-k32**.
- **This repo's own `cargo test` is a loop participant, not a bystander**
  (measured 2026-07-28, `guard-loop-signal-k37`). A meta-grove's suite runs as a
  *descendant* of the session under test, so it inherits `GROVE_SIGNAL_FILE` —
  the driver's kill channel, which fires on the file's appearance alone — and the
  fixtures write `"$GROVE_SIGNAL_FILE"` unconditionally. Two consequences worth
  carrying: **`.cargo/config.toml`'s `[env]` with `force = true` does override an
  inherited value** for everything cargo runs (measured — it is the only guard
  here that cannot be forgotten, since it needs no cooperation from the test),
  and the **acceptance test is a full `cargo test` from a live pane** with the
  real path in ambient env, checking the path stayed absent afterwards. Anything
  less proves nothing: a redirected run is safe by construction and would pass
  with the guard removed.
- **A codex grove needs the working tree *trusted*, and trust does not inherit**
  (measured 2026-07-28, `codex-grant-refused-k35`, against the installed codex-cli
  0.145.0). codex's effective sandbox is `read-only` for any untrusted project,
  and a scratch repo *inside* the trusted `[projects."/Users/antony"]` still reads
  `read-only` — so a fresh working tree, which is what `grove do` bootstraps into,
  is untrusted by construction. Under it `--add-dir` is **fatal** (exit 1 before
  any TUI, and before codex's own TTY check), while `-c
  sandbox_workspace_write.writable_roots` is silently *ignored* — which is why
  grove refuses rather than switching flag form. The asymmetry the pre-flight
  rests on: given the same flags that kill the TUI, `codex exec` prints its header
  (`sandbox: read-only`) and carries on. Two consequences for work in this repo:
  **every codex launch now spawns codex twice** (probe, then session), so a fake
  codex on the `GROVE_HARNESS_BIN*` seam must tell them apart — `exec` as argv[1]
  is the discriminator, and `tests/support`'s `fake_codex` carries the reply; and
  a codex leaf in *this* grove will refuse until this tree is trusted.
- **The driver's routing peek and the session's own `pick` really can disagree,
  and the misroute is cross-vendor** (constructed and measured 2026-07-28,
  `session-leaf-binding-k28`). With a `leaf-insert` landing between the two, a
  driver launched `claude/opus` — routed for a `design` leaf — while the session
  worked a `review-impl` leaf that `GROVE_REVIEW_HARNESS=codex` (this machine's
  live config) routes to codex. Three bounds worth carrying. The **window is
  ≥8s**, measured spawn-to-first-tool-call on a real `claude -p`, and a real
  session adds a skill-load turn on top; it is essentially all harness boot, so
  grove cannot shrink it, while `grove-llm kind --with-harness` costs ~0–30ms.
  The loop **self-heals at the next iteration** (the following session launched
  `codex/sol-xhigh` correctly) off the same zero-state re-derivation that gives
  restart ≡ continuation — but it heals the *routing*, never a leaf already
  executed under the wrong one, which is what bounds the residual at one leaf,
  once. And the **live path runs two picks, not three**: `picked_leaf`
  (`src/loop_driver.rs:1006`) is reached only from `readiness()`, the `--no-launch`
  dry run, which by definition never launches.
- **The routing decision's *outputs* already reach the session, through its own
  argv** — `ps -o command= -p $PPID` shows `--model opus`, the same one-line check
  the turn-hooks note below uses for `--settings`. Only the routing *input* (which
  leaf was peeked) is absent, and `grove do --no-launch` re-runs the identical
  `resolve_launch` against the current tree, so a divergence is checkable today
  with zero new code. Worth knowing, not worth mandating: it costs a subprocess
  per session, and it carries `--no-launch`'s side conditions (a codex probe
  spawn, the harness binary on PATH).
- **A fake harness on the `GROVE_HARNESS_BIN_<HARNESS>` seam must skip `exec` as
  argv[1]**, or a codex launch double-counts. This is `codex-grant-refused-k35`'s
  finding met from the other side: the sandbox pre-flight spawns the same fake
  before the session does, and a fake that logs both makes a two-iteration loop
  read as three. The scoped `GROVE_HARNESS_BIN_<HARNESS>` spelling is also the
  only one that survives a reroute — plain `GROVE_HARNESS_BIN` is deliberately
  ignored there (`harness_bin`), so faking *both* ends of a reroute needs the
  scoped form for each.
- **`pane.report_metadata` is four fields, not one, and the config-gated one is
  the *least* invasive** (read from herdr's source 2026-07-28,
  `herdr-sidebar-tokens-k32`, against `ui-layout` at `d17e0f42`). `tokens` render
  only where the user lists `$name` — `Custom(name) => entry.tokens.get(name)` is
  the whole path (`ui/sidebar/tokens.rs`) and the **default** agent rows are
  `[state_icon, workspace, tab]` and `[agent]` (`config/sidebar.rs`) — which is
  what `herdr-grove-plugin-k5` reported. What it did not surface is that the other
  three render **unconfigured**: `display_agent` *becomes* the pane's `agent_label`
  (`workspace/aggregate.rs`), replacing the `agent` row that **is** in the
  defaults; `title` becomes `effective_title()`, which the navigator checks
  **before** `manual_label` — the human's own typed pane name — so pushing it
  clobbers a rename; `state_labels` relabels `state_text`, which is not in the
  defaults. The guards (`agent`, `applies_to_source`) are **opt-in**, so an
  unguarded report is never dropped: the display half genuinely carries no
  authority machinery.
  Two bounds worth carrying. Limits are 16 token keys per request, 32 retained per
  resource, keys ≤32 chars, values ≤80 (`app/api_helpers.rs`). And **`PaneSnapshot`
  carries only cwd, label, agent name, managed kind, session ref and argv**
  (`persist/snapshot.rs`) — so *no* metadata survives a restart or a
  `live-handoff`, whereas `terminal_title` does (`handoff_runtime.rs`) and is
  written by the harness for free. That makes `terminal_title_stripped` in the
  user's own rows — herdr's own commented example in `main.rs` — the per-pane line
  that costs grove nothing.
- **A herdr restart need not kill every pane** — and the route is a **plain CLI
  subcommand**, `herdr server live-handoff --import-exe <path>`, listed in
  `herdr server`'s own help. Earlier notes here claimed no CLI path existed and
  that a raw `server.live_handoff` socket call was required; that was wrong.
  Demonstrated: every pane survived the swap, including the pane that issued it.
  Costs are bounded — TUI clients disconnect and must reattach, and a handoff
  carries at most 64 panes. It stays the human's call because it interrupts their
  UI. **Do not reach for `herdr update --handoff`**: that fetches *upstream*
  herdr and would clobber the fork.

# grove owns a closed set of five task kinds, gated on write and degraded on read

Every leaf task file declares a **kind** on its `**Kind:**` line, drawn from a
closed set of five: `planning`, `research`, `prototype`, `work`, `review`. The set
is closed — a sixth kind is a change to grove's code and docs, not a free-text
label a leaf may coin. A kind earns its place only by carrying behaviour beyond a
name: a distinct session discipline (`content/TASK-FORMAT.md`) and its own model
bucket (*model-per-task-kind*).

Only **`planning`** carries methodological force. It is the sole branch in the
loop's Execute step, and the only kind that grows the tree. The other four are
work-shaped sessions — they produce an artifact — differing in discipline, not in
what the loop does with them.

## Why the set is closed

The obvious alternative is a free-text label, and it is wrong for one concrete
reason: **grove reasons about the kind, not just reports it.** `leaf-decompose`
gives the new node's first child its parent leaf's kind, so a research leaf that
proves bigger becomes a research node. A label grove cannot enumerate is a label
grove cannot give defaults for, document a discipline for, or key a model bucket
on without the bucket namespace becoming unbounded.

The apparent tension with constraint 3 ("suggested shape, not enforced schema") is
not one: constraint 3 governs the **task file's body**, which stays freeform
markdown that nothing validates. The kind is a one-word declaration the CLI acts
on.

## Gate on write, degrade on read

The enforcement is deliberately **asymmetric**, and the asymmetry is the decision:

- **Write gates.** `leaf-add` / `leaf-insert` / `leaf-decompose` reject an unknown
  `--kind` with an error listing the five. A human is present at authoring time, so
  catching `reserch` there is cheap and actionable.
- **Read degrades.** An unrecognised `**Kind:**` line — hand-edited, or written by
  a future grove version — emits a warning and is treated as `work`. Reading never
  errors.

Read must degrade because the self-driving loop relaunches unattended: a task file
with a typo would otherwise jam the loop, and grove guides rather than gates
(constraint 5). Write may gate because gating there costs a human one retry, not a
stalled loop.

## Considered options

- **A free-text kind label (rejected).** Needs no enum, extends without a code
  change, and reads as truer to constraint 3. But `leaf-decompose` then has nothing
  it can meaningfully inherit, a typo becomes a silent new kind rather than an
  error, and the disciplines documented per kind become an unbounded set grove
  cannot describe. The closed set is what buys the inheritance and the defaults.
- **Upstream wayfinder's four types (rejected as a set).** `research` / `prototype`
  / `grilling` / `task` decompose *decision-reaching* work only — even wayfinder's
  `task` "earns its place by unblocking a decision, not by delivering the
  destination." They are a decomposition of grove's `planning`, not a superset of
  grove's binary: grove's `work` has no analogue there at all. grove takes
  `research` and `prototype`, maps `grilling` onto `planning`, and drops `task` —
  the tree already sequences prerequisites, so grove has no blocked-decision
  concept for it to earn its place against. `review` is grove's own addition.
- **Keeping the `planning`/`work` binary (rejected).** Cheapest, but it forces
  three genuinely different sessions — a citation-disciplined literature survey, a
  deliberately throwaway spike, and a fresh-context adversarial read — to share one
  label, one discipline, and one model bucket.

## Consequences

- Adding a sixth kind is a deliberate code change (`leaf::Kind`,
  `content/TASK-FORMAT.md`, `--help`, README) rather than a leaf coining a word.
  That friction is the point: a kind that cannot justify a discipline and a model
  bucket should not exist.
- Existing trees are unaffected — `work` and `planning` keep their labels and
  meanings, and no leaf changes kind on its own.
- Each kind is additionally marked **HITL** (`planning`, `prototype`) or **AFK**
  (`research`, `work`, `review`) in `TASK-FORMAT.md`. This is documented guidance
  with no machinery behind it: a HITL leaf reached by an unattended relaunch simply
  waits for a human, which is correct behaviour, not a fault to engineer around.

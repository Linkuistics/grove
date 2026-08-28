# Invariants and trade-offs
<!-- book-page id="invariants-and-trade-offs" slice="book-assembly-k18" order="8" -->
[Previous: Syllabus CLI](07-syllabus-cli.md) | [Contents](README.md)

`ordinal-fs-tree` stores an ordered tree directly in filesystem names. A
consumer supplies the name grammar; the library supplies traversal, stable-key
lookup, pure mutation decisions, and one filesystem interpreter. The design has
no sidecar index, journal, or private metadata format. Its guarantees therefore
depend on a small set of name laws, an advisory lock shared by cooperating
processes, and explicit recovery boundaries.

This final chapter combines the properties that cross the preceding source
layers. It owns no production fragments. The [source index](source-index.md)
records the complete fragment graph, and the sections below state the system
properties that emerge only when those fragments are considered together.

<a id="architecture-summary"></a>
## Architecture summary

A read takes a shared advisory lock on the directory containing the tree root,
classifies every reached directory entry without following symbolic links, and
builds an immutable `Snapshot`. A `ReadGuard` owns that snapshot and keeps the
lock descriptor alive for the same lifetime. The caller reads borrowed entry
and level views from the snapshot; the filesystem is not consulted again
through that guard.

A mutation takes the corresponding exclusive lock and captures the same kind
of snapshot. Every public mutation consumes its `WriteGuard`, so one snapshot
can authorize one decision only. The operation then crosses four internal
stages:

| Stage | Input and output | Property established at the stage |
|---|---|---|
| snapshot | filesystem names to an immutable parsed tree | every reached name is accepted, disclaimed, or causes the read to halt |
| algebra | snapshot plus request to `Decision` | the result is either a stated `Refusal` or an ordered `Plan`; this layer cannot reach the filesystem |
| guard | ordered effects to a self-consistent plan | each destination is checked against the state produced by earlier effects, not only against the original snapshot |
| interpreter | guarded plan to `Report` or `Error` | effects land in order; a reported forward failure triggers reverse unwind |

The `Report` describes effects that actually landed. It is the public mutation
result; `Plan` remains internal. This boundary prevents a consumer from applying
a plan after the snapshot on which it was based has become stale.

The root is a level but not an entry, so operations that add children accept
`Target::Root` or a stable key. Operations that transform an existing entry
accept only a key. Ordinals identify mutable sibling positions and therefore do
not cross this API boundary as durable targets. Paths are also unstable because
every rename changes them.

<a id="invariant-scope"></a>
## Invariant scope

The invariants are preservation properties. The library does not validate and
repair an arbitrary tree before operating on it. If the input already satisfies
an invariant, an operation preserves it within the limits stated below. A tree
built from empty by the library satisfies more properties than a hand-edited
tree, but the library cannot determine which history produced the current
directory.

The invariants also describe quiescent trees between operations. An operation
is a sequence of filesystem effects. Cooperating readers cannot observe its
intermediate states because the writer holds the exclusive lock, but a process
termination or a writer that ignores the lock can expose them.

| Invariant | Preserved statement | Boundary or exception |
|---|---|---|
| key uniqueness | no two quiescent entries share a key, and allocation uses the tree-wide maximum plus one | uniqueness is assumed of the input; a failed promotion rollback can leave the old leaf and new node sharing the preserved key |
| key non-reissue | a newly allocated key was not committed previously | promotion creates a new filesystem object with an existing key because the promoted entity retains its identity; this is preservation, not allocation |
| ordinal distinctness | positioned children of one level have distinct ordinals | promotion temporarily places the old leaf and new node at the same ordinal; failed rollback can leave that state behind |
| density by induction | a level built densely from empty stays dense | density is not established for hand-edited trees; append uses `max + 1`, and no operation fills an existing gap |
| distinguished shape | a distinguished child is a regular file and is unique by its one domain-provided name | a contradictory directory is malformed and halts the snapshot rather than hiding a subtree |
| recognised-name visibility | every reached recognised name parses completely or halts the operation | a foreign directory is disclaimed and skipped together with its descendants; a non-UTF-8 name cannot be offered to the consumer and therefore halts in the library |
| species agreement | a leaf name denotes a regular file, a node name denotes a directory, and a distinguished name denotes a regular file | `EntryName::parse` receives the observed filesystem species and must refuse contradictions |
| subtree preservation under insert | shifting changes only affected siblings' ordinals; keys, parts, bytes, and descendants stay unchanged | the plan proves that no descendant effect exists; the filesystem guarantee that a directory rename carries its subtree is below the model boundary |
| identity preservation under promotion | the promoted entry retains its ordinal and key while its bytes move into the distinguished child | the path and filesystem object change; stable identity is the key, not either of those |
| plan atomicity for reported failures | after a mutation reports a forward error, either every effect landed or reverse unwind restored the captured tree | process termination is outside the guarantee; an unwind failure returns a separate partial-rollback error and leaves repair work |

Highest-first sibling shifting connects ordinal distinctness to the interpreter.
Each rename vacates the next destination before that destination is needed. An
interrupted shift can therefore leave a gap but does not transiently duplicate
an ordinal. Lowest-first shifting would create a duplicate ordinal during every
non-empty shift and can also collide on a tree already carrying duplicate keys
and equal parts.

Promotion has no corresponding order that preserves every invariant throughout
the run. The destination node must exist before the leaf can move into its
distinguished child, and that node must already carry the leaf's ordinal and
key. The temporary duplicate is required by identity preservation. The
exclusive lock makes the state unobservable to cooperating readers; rollback
or manual repair is what removes it after failure or interruption.

The `EntryName` boundary supplies the remaining structural laws. Composition
must preserve its ordinal, key, and parts arguments. Positioned names and the
distinguished name must be disjoint. Formatting and parsing must be canonical
in both directions. A positioned name's species must follow from its parts, and
classification must agree with the observed filesystem species. The reusable
conformance check exercises representative values, while Rust's `NameView` sum
type and the static `positioned_species` function make several inconsistent
shapes unrepresentable. The library separately enforces that every rendered
name is exactly one path component because violating that law would address a
path outside the intended level.

<a id="outcome-map"></a>
## Failure and refusal map

A refusal is a successful algebraic decision that authorizes no effects. An
error is produced at the filesystem or name boundary, or while applying a plan.
The distinction determines both the state of the tree and the next action.

| Outcome class | Representative causes | Tree state | Appropriate response |
|---|---|---|---|
| snapshot or lock error | no containing directory for the root, lock or read I/O failure, non-UTF-8 filename, malformed or reserved name, species contradiction, rendered name with more than one path component | no mutation plan has run | correct the path, permissions, filesystem entry, or consumer name implementation, then read again |
| algebra refusal | target key missing, target not a node, insert ordinal unoccupied, promotion target already a node, no distinguished name, promotion parts not a node, rewrite species change, content supplied for a node, exhausted key or ordinal, destination occupied | unchanged | change the request or repair the pre-existing tree condition named by the refusal |
| clean apply failure | an effect returns I/O failure and every applied effect unwinds successfully | restored to the captured snapshot | inspect the reported filesystem cause; retry is safe with respect to this run after the cause is resolved |
| partial rollback | a forward effect fails and a reverse effect also fails | neither the captured state nor the intended state | inspect and repair the paths named by both failures before another mutation |
| process termination | the process stops between effects or during unwind | an intermediate state may remain and no library result classifies it | inspect the tree before retrying any non-idempotent operation |
| uncooperative concurrent write | another process ignores the advisory lock and changes a destination after the snapshot | the interpreter refuses the occupied destination or reports an I/O failure; rollback may then succeed or fail | coordinate writers, inspect the resulting error class, and repair before retry when rollback was partial |

Several algebra refusals define deliberate seams rather than missing
conveniences. `insert` does not become `append` when its ordinal is past the end,
because the two requests have different effects on later siblings. It also does
not fill a gap in a hand-edited level, because density is preserved but never
established. `rewrite` cannot change species because a rename cannot turn a file
into a directory. `promote` cannot invent node parts or a distinguished name
because both values belong to the consumer's opaque vocabulary. Supplying bytes
for a node is refused because silently discarding those bytes would report a
different operation from the one requested.

Destination occupancy is checked twice for different reasons. `Plan::guarded`
checks the ordered plan against its own prior effects, which rejects a plan that
would collide with itself before any filesystem change. The interpreter uses an
exclusive destination claim because advisory locking cannot constrain a process
that does not participate in the protocol.

<a id="model-evidence"></a>
## Model evidence and its boundary

The formal models are evidence for the design claims, not substitutes for the
Rust tests or filesystem contract.

`models/structure.als` checks single-state shape. Its assertions cover the
parse verdict's total and disjoint representation, distinguished-child
uniqueness, identity preservation by recomposition, positioned/distinguished
separation, promotion's ability to name its output, filesystem-species
agreement, and the rule that a recognised name is never silently skipped. Its
named witnesses demonstrate admitted or historically defective shapes,
including gapped levels, duplicate keys from hand editing, two canonical
filenames for one logical name under an insufficient grammar law, a positioned
name missing an ordinal under an insufficient representation, and the root as
a node that is not an entry.

`models/operations.qnt` checks reachable transitions through an effect-at-a-time
interpreter. Its invariants cover sibling-name uniqueness, child containment,
fresh-key allocation, reported-error atomicity, rollback ownership, insert-only
shifting, promotion identity, rewrite place preservation, append-only addition,
and the interpreter's destination assumptions. Separate instances establish
the important scope distinctions:

| Instance | Evidence produced |
|---|---|
| `pristine` | library-built trees retain density, uniqueness, successful operations, and the stated refusal cases |
| `hand_edited` | a gap is admitted and preserved while distinctness remains; insert into the gap is refused |
| `corrupted` | duplicate keys can be inherited without highest-first shifting adding an ordinal collision |
| `lowest_first` | the alternative order reaches both a refused shift and a transient duplicate ordinal |
| `no_distinguished` | promotion is refused when the consumer supplies no distinguished name |
| `unparseable` | one reached broken name halts the whole-tree snapshot |
| `failures` | forward failures restore the captured tree when unwind succeeds |
| `rollback_fails` | failed unwind can leave neither endpoint state, duplicate identity coordinates, and later operations stranded |

Both model suites include positive witnesses so a passing invariant is not the
result of an empty state space or an unreachable operation. Quint witnesses
also keep refusals as returned outcomes rather than disabled transitions, which
makes dead refusal branches visible.

The models omit four material boundaries. They do not model filesystem bytes or
the guarantee that renaming a directory carries its subtree. They model
reachability but not deterministic walk-order tie-breaking on a corrupted tree.
They do not model an uncooperative writer changing the filesystem during an
apply. They also cannot represent a Rust `Eq` implementation for parts that is
coarser than the model atom equality. The crate tests and the implementation's
explicit species comparison cover those seams. String-specific failures such as
non-UTF-8 names and multi-component rendering are also outside the models and
are exercised at the Rust boundary.

<a id="design-trade-offs"></a>
## Design trade-offs

| Choice | Benefit | Cost and excluded alternative |
|---|---|---|
| names are the only persistent representation | a tree remains legible and editable with ordinary filesystem tools; there is no index to synchronize | every walk parses names, a malformed reached name has whole-tree blast radius, and hand edits can violate assumptions; a sidecar database was not added |
| ordinal and key are separate | insertion can change order without invalidating durable references | filenames carry two coordinates and insertion renames later siblings; one position value cannot serve both purposes |
| one `EntryName` seam owns vocabulary | the algebra stays domain-independent and works only with ordinal, key, parts, species, and verdicts | consumers must satisfy canonical grammar laws; adding label lookup or a generic parts parser would expose domain concepts to the library |
| snapshot to decision to plan to interpreter | the algebra is pure, effect order is inspectable, and all mutations share one rollback implementation | every mutation reads a snapshot first and a plan remains internal; read-transform-diff would add a second algorithm whose ordering also needed proof |
| whole-tree snapshot | key allocation and lookup observe the complete tree, and malformed reached names stop the operation consistently | one broken reached name freezes every mutation and large trees cost a complete traversal; narrowing the snapshot is possible only if those visible properties remain unchanged |
| advisory lock on the containing directory | cooperating readers and writers share one protocol that also covers root creation and removal | the library cannot stop other processes from ignoring the lock, and the protocol supplies neither crash recovery nor a journal |
| highest-first shifts | ordinal distinctness holds throughout an insert and corrupted duplicate-key trees avoid one collision class | the report order is constrained by safety rather than presentation; lowest-first is simpler to describe but produces unsafe intermediate states |
| one-way promotion | a leaf can become a node without changing its key, and its bytes become explicit node content | the operation necessarily has a transient duplicate ordinal and key; demotion is absent because the library cannot decide what to do with children |
| no removal operation | `max key + 1` remains a durable allocation rule without persistent history | retirement must be encoded in consumer-owned attributes and applied with rewrite; deletion would require a separate non-reissue mechanism |
| no generic production CLI | the demonstration uses honest syllabus parts and verbs without widening the library seam | other consumers must build their own interface; accepting complete filenames or adding a second parts grammar would duplicate or distort the existing name contract |

These choices keep the library's interface small and the on-disk representation
direct. They also make its limits observable: corrupted input is not repaired,
locking is cooperative, rollback is bounded to reported failures, and a partial
rollback requires human inspection.

<a id="final-verification"></a>
## Final verification

The source ledger contains fifteen roots and 6,929 owned source lines. Every
top-level ownership block is `resolved`, every early-use row is `explained`, and
no `defer` directive remains. Recursive expansion of each source root is checked
byte for byte against its production file; Markdown validation separately
checks the canonical page inventory, identities, anchors, navigation, and local
links.

Assembly uses these commands:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/ordinal-fs-tree/book \
  --final \
  --check all
cargo test -p ordinal-fs-tree --all-targets
docs/ordinal-fs-tree/models/run-alloy.sh
docs/ordinal-fs-tree/models/run-quint.sh
```

The final book check reports all fifteen source files and all 6,929 source lines
complete with zero deferred ranges. The crate test command includes the CLI
contract tests and the guard proving that algebra modules cannot reach the
filesystem. The Alloy runner reports no counterexample for each check and an
instance for every named witness. The Quint runner reports every configured
invariant holding and every configured witness reached in its designated
instance.

[Previous: Syllabus CLI](07-syllabus-cli.md) | [Contents](README.md)

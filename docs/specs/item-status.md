# Item status and full-width viewing

## Problem

A reader needs to distinguish what work remains, what finished, what was
abandoned, and which item the driver is executing. These facts must stay visible
with deep trees, long handles, folded branches and a selected row. Reading a
file also needs the terminal's full width without losing the reader's place.

## Solution

`grove view` starts in Tree view. Tab switches between a full-width tree and
the selected item's full-width Markdown file. Lifecycle begins each row label;
RUNNING and NEXT have their own fixed column and a summary visible in either
view. Activity is a sampled observation, refreshed on the existing 500 ms
cadence, while lifecycle continues to come from the task tree.

[One live driver owns each working tree](../adr/one-live-driver-per-working-tree.md#read-only-activity-observation)
records why session witnesses extend driver ownership. This spec owns their
protocol, typed results and guard lifetimes. Module ownership remains in
[module-decomposition](./module-decomposition.md); the
[glossary](../../CONTEXT.md#tree-lifetime) defines tree lifetime, RUNNING and NEXT.

## Interfaces and responsibilities

### One typed observation operation

The loop exposes `try_observe` for an exact worktree location. It composes the
existing quiet tree reader with runtime observation, returning an
`ObservationGuard`: a typed tree result and an independent typed activity
result. A present tree exposes the same snapshot and canonical path operations
as the current reader, plus an opaque `TreeLifetime`. A vacant, busy or invalid
tree remains distinguishable. Activity failure does not discard a readable tree;
tree failure does not discard a verifiable mandate.

The operation copies rows and selected bytes internally and releases all
advisory guards before returning. `ObservationGuard` owns these captured values
and the retained `TreeLifetime`, which pins a directory without holding a tree
or epoch lock. The type grants no session admission or driver ownership. All
lock attempts are quiet and nonblocking; runtime discovery requires no launch
configuration, creates nothing, and never attaches an ancestor workspace's
driver to a tree observed in a subdirectory.

The runtime result has these meanings:

| Result | Meaning for the viewer |
|---|---|
| Idle | The observation establishes no current running session. Select ordinary NEXT from a valid current tree. |
| Running with mandate | A witnessed launch is current. The mandate carries a permanent key, launch-time tree identity and handle/kind, and its verified relation to this capture: same tree, previous tree, or no readable tree. |
| Busy with reason | Publication or observation is in progress, including a launch not yet witnessed as started. Show activity waiting; withhold current RUNNING and NEXT. |
| Unavailable with reason | Required evidence cannot be established, including unsupported or malformed metadata. Show activity unavailable; withhold current RUNNING and NEXT. |

No workspace/control namespace or a verified absence of a current session is
the ordinary idle case. Missing metadata beside an existing lease is
unavailable. Older active epochs lacking
observational metadata never become RUNNING by guessing from the tree. Missing
runtime support must leave temporary non-jj trees fully browsable.

The viewer retains its bounded two-capture consistency check. Each capture
releases its guards before the next. Compare tree rows and selected bytes as
before, and compare runtime identities, witness state and verified tree relation
separately. Changing
activity must not reject an otherwise consistent tree. An unstable runtime pair
produces Busy for activity and retries on the next deadline. A failing tree
capture suppresses NEXT and all activity attachments to retained rows; a fresh
mandate may still appear in the summary with `tree unavailable`.

Only an accepted, current observation supports row activity. When observation
fails or is contended, a retained tree remains explicitly WAITING or STALE and
previous row activity is cleared. Any retained activity description is labeled
STALE, never presented as the current RUNNING/NEXT pair. Both views, help and
undersized frames continue ticking; at most one retry is pending.

### Mandate and lifetime

Launch preparation retains an open task-root directory descriptor, acquired and
checked against the selected snapshot under its tree read guard. Its device/inode pair
is the **tree lifetime**, distinct from the working-tree-root identity in the
lease. On this same open directory description the driver takes an exclusive,
nonblocking `flock`: the **directory witness**. Before publication, launch
preparation transfers this pin and the private file witness into one
launch-observation value owned by the lease, and checks the pin again
before publishing the mandate. The ordinary copyable selection remains value
data; it does not own an OS descriptor. A mismatch before publication
stops that launch as a changed-tree error; no stale selection is rebound to the
new tree. No tree access or epoch guard survives across spawn. The directory
witness remains held; it locks the task-root directory itself, whereas ordinary
tree access locks its containing directory. It therefore does not serialize
task reads, mutation, retirement or root deletion.

Every launch may acquire either witness only after the predecessor's epoch has
been exclusively invalidated and its shared readers have drained. This also
binds a replacement driver waiting with predecessor lease bytes: it cannot
prepare a directory witness while the predecessor epoch is still observable.
Only the preparing driver takes either witness exclusively. Observers use shared
probes, and admission operations use neither. Failure to acquire the directory
witness, including contention or a reported directory-locking error, releases any
prepared observation resources and leaves activity Unavailable; launch and
mandatory admission continue normally. Neither acquisition waits or falls back
to a different lock primitive.

Active epoch records carry an optional, versioned observation extension:
permanent key, launch-time handle and kind, task-root identity, and the witness's
namespace-local name and descriptor identity. The extension is bound to the
record's existing lease nonce and signal path. The handle's key must agree with
the explicit key. Its recognized version requires both witnesses to have been
prepared; a file-witness-only record cannot satisfy that version. Missing or
unsupported observational fields cannot weaken validation of the mandatory
admission record and cannot establish RUNNING.
Admission does not require the extension, including when observation setup fails.

Keeping the task-root descriptor open prevents its inode being reused. This
rests on the native open-object lifetime assumption supported by the
[ADR's final-close sources](../adr/one-live-driver-per-working-tree.md#why-the-directory-witness-survives-process-death);
the replacement control below does not separately test inode non-reuse. With
the directory-witness check below, a viewer joining after root replacement
can reject a reused key without having seen the previous tree. The
pin and its advisory lock add no persisted generation or task-tree bytes.
Replacement after publication leaves the mandate attached to the old lifetime; it does not confer
authority over the replacement or change the existing admission rules.

### Witness publication and release

Each attempted launch allocates a new witness file in Grove's control namespace
using an independent OS-random 128-bit suffix and exclusive creation. Occupied
draws are retried with the same bounded policy as fresh signal allocation.
The driver tries to lock the empty regular file exclusively and nonblocking
before publishing its identity in the pre-spawn epoch. Contention is an
observation-only allocation failure: activity is Unavailable and launch proceeds
with admission intact. This lock is held by the driver alone, on a
close-on-exec descriptor. Its name is never deliberately reused. The accepted
random-collision limit is the same as the channel's, without tombstones.

The generic runner exposes launch events to its caller: **Started** after a
successful spawn and **Reaped** when it confirms that child's reap, including
escalated termination. These synchronous parent-side notifications introduce no
Grove vocabulary or child-side acknowledgement. Ordinary callers can run with
no observer. Notifications are infallible and do not change launch disposition;
Grove's callbacks take no epoch/tree lock and perform no waiting operation.

On Started, the driver writes the exact eight-byte marker `started` followed by
a newline to the previously empty witness. This is the file's only publication:
it is never rewritten to describe another phase, launch or item. Empty or
incomplete bytes cannot mean started; only the complete exact marker does. A
reader need not assume write atomicity. On Reaped, the driver releases the
witness immediately, before terminal recovery, exclusive epoch invalidation or
signal interpretation. Writing the completion signal and retiring the task do
not release it. Failed spawn publishes no marker and releases its prepared
witness before post-attempt invalidation.

The lease owns the launch-observation value until those events release it.
Its release operation closes the private witness first, then the locked tree
pin; lease drop invokes that same operation before releasing driver ownership, including on
unwind. Failed spawn and confirmed reap use the same release order. A
supervision error without confirmed reap must not invent a Reaped event or drop
the witness early through a helper's return. Driver exit, unwind or death
releases the lease and both witnesses through descriptor lifetime; all their
descriptors are close-on-exec and are never passed to another process. Child
processes cannot keep them alive after exec. A helper or iteration returning
cannot release either member of the lease-owned value. Process death may close
the directory and private witness in either order. Safety depends only on the
directory lock being removed before that same open description releases its
root reference, not on which descriptor closes first.

Observation-only allocation failure leaves activity Unavailable. Publication
failure leaves activity unverified: an empty file or valid marker prefix remains
Busy, and invalid bytes are Unavailable, until reap or driver exit. Either
failure emits a diagnostic without changing a successfully launched session's
authority or outcome. An epoch whose extension could not be prepared is still
valid for admission. Failure to write the mandatory epoch remains a launch
failure under the existing protocol. Witness cleanup happens after epoch
invalidation; a replacement removes abandoned witnesses only after it owns the
lease and has invalidated the old epoch. Cleanup failure leaves harmless bytes
and cannot change completion. No observer creates, cleans or repairs controls.

### One bounded observation

The VCS seam discovers an existing namespace in the exact observed workspace,
sharing path derivation and namespace validation with the creating operation.
Discovery creates nothing, follows neither a secondary workspace's repository
link nor an ancestor workspace, and uses no jj command, launch configuration
or ambient session context. A symlink alias of the exact workspace is allowed.

The loop's observer completes the tree capture before acquiring any epoch guard:

1. Attempt the ordinary quiet, nonblocking tree read. Pin and validate its
   task-root identity with the snapshot, copy rows and selected-file bytes, then
   release the tree guard. Retain the descriptor pin. A busy, vacant or invalid
   tree still permits a runtime summary, but no row attachment. Never open a
   later tree and join it to activity established by this capture.
2. Open existing controls read-only, nonblocking and close-on-exec. Require
   regular files for records and a directory for the namespace. Bound each
   lease/epoch read to 64 KiB; the witness accepts exactly the eight-byte marker
   and rejects extra bytes. Resolve the witness only from a plain basename
   inside this namespace, never an arbitrary path from the record.
3. Try a shared epoch guard without waiting. Contention returns activity Busy.
   An absent exact workspace, namespace or lease means Idle at that sample.
   When a lease exists but its epoch is missing, unreadable or malformed,
   activity is Unavailable. Under this guard validate the mandatory lease/epoch
   binding without locking the lease. Compare the observed working-directory
   descriptor's device/inode with the record's worktree-device/worktree-inode;
   path spelling is informative only. This accepts aliases such as /var and
   /private/var without attaching an ancestor's lease. Admission's existing
   canonical-path checks are unchanged. A matching inactive epoch means Idle;
   an active epoch requires the versioned observation extension. Invalid or
   mismatched records mean Unavailable.
4. Compare the captured, still-pinned task-root identity with the mandate's
   identity. If equal, probe the **captured directory descriptor itself** with
   a nonblocking shared `flock`; do not reopen its path. A successful probe is
   unlocked immediately, retaining the descriptor, and records an unverified
   tree binding. Contention records a verified same-tree binding. Other errors
   record an unavailable binding. Unequal identities mean previous tree and need
   no directory probe; no readable tree likewise permits only a summary.
5. Open the named private witness and compare descriptor/path identity with the
   published identity. Missing or mismatched evidence means Unavailable.
   Read its bounded marker, then probe through an independent descriptor with
   a nonblocking shared lock attempt. Success is released immediately, before
   any other work, and means Idle regardless of leftover marker bytes.
   Contention plus the exact started marker establishes Running, subject to the
   binding result: a numerically matching root whose directory probe succeeded
   makes activity Busy, and a directory-probe error makes it Unavailable. Neither
   case yields RUNNING or NEXT. A locked empty file or proper marker prefix means
   Busy; other bytes/errors mean Unavailable.
   Other viewers' shared probes cannot cause contention, and no admission
   operation or replacement driver takes this witness exclusively.
6. Copy the runtime result, including its tree relation, and release the epoch
   guard before returning on every path. It covers only bounded record reads,
   identity validation and the two witness probes: no tree traversal,
   selected-file read or caller work. Return separate typed tree/runtime results
   and the retained tree pin, with no advisory locks.

Every control-file acquisition/probe checks open descriptor identity against the
current path, with at most the existing eight identity-race attempts and no
sleep or blocking fallback. Tree and epoch guards are never held together by
observation. Neither an epoch guard nor a witness lock escapes to the caller.

### Process death and tree identity

For a same-tree result the order is: pin the captured root, take the short shared
epoch guard, validate the records, probe the directory, then probe the private
witness. The directory probe and the epoch publication rule close the gap that
pin-before-file-probe alone leaves:

| State when a numerically matching directory is probed | Consequence |
|---|---|
| The old directory witness is still exclusive | Its own open description still pins that root. The already-open observer descriptor therefore names the same lifetime; it keeps that identity valid through the final file probe. |
| The directory witness was released first, while the private witness remains locked | A shared directory probe succeeds. Even if an inode/key was reused before this observer opened the root, the old private witness cannot authorize a row attachment or NEXT exclusion. |
| The private witness was released first | Its final shared probe succeeds and produces Idle, regardless of an earlier directory-probe result. |
| A replacement driver owns the lease but old epoch readers remain | It cannot take a new directory witness before exclusive invalidation. Its lock cannot be mistaken for the old launch's lock while this observation holds the shared epoch guard. |

The [driver-lease ADR](../adr/one-live-driver-per-working-tree.md#why-the-directory-witness-survives-process-death)
provides the Linux and Darwin final-close evidence and the cooperating-process
boundary. The proof requires no relative cleanup order for separate descriptors.
The directory witness alone proves no launch; the private witness remains
necessary for Started evidence and for summaries when the old root is absent or
has a different identity. An observation backend must provide the stated native
file/directory lock and open-object lifetime semantics. Reported preparation
errors leave activity Unavailable; observation handles probe errors as specified
above, never by an identity-only fallback. Silently ineffective locking is
outside this boundary and is not detected by the protocol: if both shared
probes succeed despite a live launch, step 5 yields Idle and ordinary NEXT may
select the running item. No runtime capability self-check is specified.

Running is evidence at the witness probe, not a promise until the next frame.
After process teardown has released the private witness, a probe reports Idle
even while a replacement holds old lease bytes. Death after the probe is detected
next time; delivery of a kill signal itself is not evidence that teardown has
finished. The shared epoch guard blocks publication only during the short runtime
read, and keeps neither driver nor
witness alive. The two-capture comparison rejects observed change, not every
non-cooperating filesystem edit. Arbitrary administration-area corruption
remains outside the ADR's guarantee.

### One selection rule

The loop owns a selection operation over an already-read typed snapshot with
an optional excluded permanent key. Ordinary driver selection supplies no
exclusion. The viewer supplies a key only when the typed observation verified
that the witnessed mandate belongs to this snapshot's tree lifetime.

First validate the whole tree, including duplicate keys and multiple live finish
leaves. Exclusion cannot make malformed input valid. This deliberately strengthens
ordinary driver selection too: duplicate keys anywhere in the snapshot now
refuse selection with a duplicate-key diagnostic, even with no exclusion and
even if the duplicates are terminal or branch items. Previously the driver
could launch from such a tree while the viewer refused it. One validation rule
avoids an ambiguous mandate or exclusion; valid-tree selection is unchanged.
Implementation must document this refusal and key-repair guidance in the usage
guide's selection/viewing sections. Then remove the excluded
item from the live-leaf candidates, select the first non-finish leaf in
depth-first position order, or the sole remaining finish leaf when no ordinary
candidate remains. Return the canonical selection or none. This is selection
over the remaining candidates, not an ordinary pick with its result discarded.

A running leaf may still be LIVE or may already be DONE or ABANDONED. If it has
become a branch, only that item's identity is excluded; its children remain
eligible. Folding, visibility, cursor selection and label text do not enter
selection. NEXT never allocates a finish sentinel: no eligible leaf means none
until the driver materializes one in the tree.

### Binding a mandate to a row

Resolve a witnessed mandate by permanent key only when the typed observation
reports the same-tree relation. Comparing numeric identities in the viewer is
not a substitute for that verified relation. Use the item's current handle and
species when present, so renaming, moving, retirement and
decomposition preserve RUNNING. Only the directly named item gets the marker;
ancestors do not inherit it. Its lifecycle still reflects the current tree.

If the item disappeared from the same tree, the summary says `item absent` and
uses the launch-time handle; attach RUNNING to no row. NEXT still selects from
the current candidates. If the task root was replaced, the summary says
`previous tree`, attaches no RUNNING row, and excludes no key from the new tree.
The new tree may legitimately have a NEXT item with the same textual handle.
Removing the tree leaves the verified mandate in the summary with `tree absent`
and leaves NEXT unavailable. Root replacement clears old selection, expansion
and file positions; editing just the root brief preserves the lifetime.

### Typed row data

Rows carry item identity, handle, kind/species, depth, expansion, lifecycle and
descendant counts separately. Formatting belongs to rendering. Do not parse a
preformatted label to recover lifecycle or decide activity.

For a leaf the lifecycle is LIVE, DONE or ABANDONED. For root and branches it is
LIVE if any descendant leaf is live, otherwise DONE if any is done, otherwise
ABANDONED if any is abandoned, otherwise EMPTY. Counts include every descendant,
including folded subtrees, and retain the separate LIVE/DONE/ABANDONED totals.

## Tree layout

The status area has a constant width and precedes depth indentation:

| Area | Cells | Contents |
|---|---:|---|
| Cursor gutter | 2 | `> ` on the selected row, spaces otherwise |
| First part of label | 2 | `✓ ` for DONE, `✗ ` for ABANDONED, spaces for LIVE/EMPTY |
| Lifecycle | 10 | Full word, padded: LIVE, DONE, ABANDONED or EMPTY |
| Activity | 8 | RUNNING, NEXT or spaces, padded |
| Item | Remaining width | Bounded indentation, fold indicator, handle, kind and branch counts |

For example, the start of an ordinary retired row reads `✓ DONE`; a retired
item whose session is still finishing reads `✓ DONE` followed by `RUNNING`.
The tick/cross starts the label after the separate cursor gutter. Expansion
continues to use its own plus/minus indicator beside the indented item name.

Ordinary LIVE items use the normal foreground. DONE uses green marker, word and
item text; ABANDONED uses red; EMPTY uses the normal neutral foreground. Reserve
the active highlight for the RUNNING item: its item text and RUNNING word use
bold yellow, while its lifecycle marker and word retain their own treatment.
Thus a session that has retired its task still shows a green `✓ DONE` beside
its highlighted running name. NEXT uses bold normal-foreground text in its activity column only.

Selection adds the cursor gutter only, with no added row-wide style modifier;
it preserves lifecycle foreground colors and the RUNNING highlight. It never
applies a uniform highlight color or reverse-video style. Words, glyphs, cursor and fold indicators remain distinguishable with
color disabled. Selecting an ordinary LIVE item does not give it the active
highlight.

At the supported minimum of 60 columns, a bordered tree has 58 inner cells;
the fixed area uses 22 and leaves 36 for the item. Reserve at least 16 cells
for recognizable handle text and two for the fold indicator. Cap indentation to
the remaining budget, marking compressed depth with an ellipsis inside that
budget. Elide the slug before losing the permanent-key suffix; kind and counts
use the space left after the handle. Clip by terminal display width without
splitting graphemes or emitting control characters. Neither deep indentation nor
long text may consume status cells.

## View switching and chrome

Show one full-width body at a time. Chrome identifies Tree or File as active,
states `Tab` and its destination, and keeps one line each for RUNNING and NEXT.
Each summary line reserves its label and any `item absent`, `previous tree`,
waiting, unavailable or stale qualifier before eliding handle text. Prefer
truncating the slug to the permanent key. Keep tree-observation diagnostics
separate from activity freshness.

One location/view line, one tree-observation line, two activity lines and a
one-line key footer leave a bordered body with three visible content rows at
60 × 10. Longer diagnostics use the body where needed; they do not displace
either activity line during ordinary Tree or File viewing. Hidden, folded and
offscreen RUNNING/NEXT items remain identifiable in the summary.

Tree keeps the existing selection, parent/child and folding keys, including
Enter/Space. Tab works on root and branches as well as leaves: their File view
shows the appropriate brief. File keeps Markdown rendering, line/page movement,
Home/End and horizontal scrolling of code and tables. File scrolling actions
apply only while File is active. Help, refresh and quit remain global.
Help names the active Tree/File view and
explains Tab switching to the other full-width view, replacing “switch pane”; the
overlay pauses navigation, and the existing resize notice applies below
60 columns or 10 rows.

Switching views changes neither tree selection/expansion/viewport nor the
selected file's source reading anchor and horizontal offset. Keep independent
tree and file viewport state. Hidden views are not rendered into zero-sized
rectangles and are not clamped using the other view's dimensions. Reflow and
clamp the visible file from its saved source anchor after a resize. Returning to
Tree preserves its viewport, adjusting only as needed to keep the selection
visible after resize or tree changes.

Keep the existing per-item reading-position behavior across revisits and edits,
bounded to the accepted tree lifetime. Both views observe tree and selected-file
changes. Selection follows keys, reveals a moved item's ancestors, and falls
back to the nearest surviving old ancestor when its item disappears. File
errors retain saved reading positions. Changing views does not force a reload
or reset the automatic observation deadline.

## Acceptance scenarios and test seams

The shared application seam remains `Viewer::new`, `act`, `tick` and `render`,
using temporary trees and Ratatui TestBackend. Application tests and production
observation use the same typed loop operation; there is no test-only status
implementation. The existing controlled launcher, process fixtures and internal
lock/filesystem barriers exercise actual lease and witness behavior. The
terminal fixture owns key translation and terminal cleanup checks.

| Scenario | Required observation | Seam |
|---|---|---|
| DONE, ABANDONED and LIVE rows, selected and unselected | Leading tick/cross and lifecycle words survive selection and disabled color; DONE/ABANDONED have status colors and ordinary LIVE is unaccented | Application |
| RUNNING differs from cursor selection and includes an already-DONE item | Only RUNNING has the active item-text highlight; its leading lifecycle cue stays readable and the cursor remains independent | Application |
| Long handles and a deep tree at 60 × 10 | All status cells remain visible; recognizable handle/key survives bounded indentation | Application |
| Folded mixed and empty branches | Aggregate lifecycle and all descendant counts are correct | Application |
| Tab from root, branch and leaf; repeated switches | Correct full-width file; saved tree viewport, folds, file source anchor and horizontal offset survive | Application; terminal key mapping |
| Resize or edit a hidden/revisited file | Source reading location is preserved, with existing fallback/clamping rules | Application |
| RUNNING or NEXT folded/offscreen; File active | Both summary entries remain visible and are independent of cursor selection | Application |
| Session has retired or decomposed its item | DONE plus RUNNING, or RUNNING on the branch; descendants can be NEXT | Controlled launch through application |
| Only ordinary running leaf plus finish remains | Finish becomes NEXT after exclusion; no sentinel is created by viewing | Typed selection; application |
| Running ordinary leaf, later ordinary leaves and early finish | First remaining ordinary leaf wins; malformed duplicate finishes are still refused before exclusion | Typed selection |
| Successful, immediate-exit and failed spawn | Started evidence is published only on success; failure never reports RUNNING; reap clears activity before epoch handoff | Real runner/process fixtures |
| Completion signal precedes reap | RUNNING persists through grace/escalation until reap; it does not disappear at signal or retirement | Real runner/process fixtures |
| Driver killed with started bytes left behind | Next observation cannot report that launch RUNNING, even if a child still exists | Real process fixture |
| Multiple observers probe an unlocked started witness while replacement retains old lease bytes | Shared probes do not contend with one another and cannot manufacture RUNNING; observers never lock the driver lease | Process fixture and lock barriers |
| Replacement holds lease with old bytes while an old epoch guard blocks handoff | Old witness cannot establish RUNNING; viewer attempts return promptly and do not wait for handoff | Process fixture and lock barriers |
| Root replaced while a session runs, including reused key and first viewer opened after replacement | Old mandate is marked previous tree and never attaches to the new row; new NEXT uses no old-key exclusion | Real launch plus application |
| Directory witness closes before the private witness, with forced inode/key reuse before the observer opens the replacement | Numeric equality plus the old locked file witness yields Busy, no RUNNING row and no NEXT exclusion; the missing directory lock is decisive | Observer lock/filesystem barriers |
| Private witness closes before the directory witness, or both close | The final private probe yields Idle; the old directory lock alone cannot establish RUNNING | Observer lock/filesystem barriers |
| Root release occurs after the directory probe but before the private probe | The observer's retained pin still binds the captured lifetime; a later directory is never joined to this result | Observer lock/filesystem barriers |
| Item absent or whole tree removed during a launch | Summary retains the witnessed identity with the appropriate absence qualifier | Application plus real launch |
| Old, missing, malformed, truncated, oversized or mismatched observation records | Typed unavailable/busy state where evidence is missing; readable trees still browse | Typed observer; application |
| Witness allocation or started-marker write fails | Session authority and outcome are preserved; activity is Unavailable for missing metadata/invalid bytes or Busy for an incomplete marker | Runner/observer fault seam |
| Control path replaced by a FIFO or a directory; open/lock identity race | Bounded quiet failure/retry, with no blocking read or invented RUNNING | Existing filesystem barriers/process fixtures |
| Observer paused after tree/file capture, and separately inside runtime read | No epoch guard spans tree/file capture or caller work; runtime suspension alone can reach the documented handoff bound; release permits recovery | Lock barriers and process fixture |
| Supervision error without confirmed reap; unwind and normal lease drop | Lease-owned witness survives helper return; every orderly release closes witness before root pin and driver ownership | Runner/lease event trace |
| Duplicate keys with and without running-key exclusion, including terminal/branch duplicates | Driver and viewer refuse before selection/exclusion; valid trees retain ordinary ordering | Typed selection; driver/application |
| Exact workspace reached through symlink or /var alias | Same directory identity yields same activity; subdirectory observation never borrows ancestor runtime | Typed observer; application |
| Foreign shared holder of either the task root or a newly allocated private witness; reported directory-locking error | Exclusive acquisition returns promptly, prepared observation locks are released, launch/admission proceed and activity is Unavailable | Lock barrier and controlled launch |
| Directory witness held while a session mutates or deletes the root | The containing-directory tree lock remains usable; root replacement remains possible and never inherits the old root's witness | Real filesystem/process fixture |
| Help in both views; selected LIVE, DONE, RUNNING and NEXT with color disabled | Active view and Tab destination are named; gutter alone marks selection, NEXT is bold normal text, RUNNING retains its specified styles | Application; terminal fixture |
| Tree or epoch contention and rapid start/end changes | Current pair is withheld when unverifiable; navigation/quit remain responsive; cadence recovers | Application and process fixtures |
| Multiple viewers, non-jj location, missing namespace, absent tree | Browsing creates no files/directories or persisted state and never resolves launch configuration | Application filesystem snapshots; typed observer |
| Ambient old session after rotation; an admitted operation during replacement | Existing admission rejection and handoff semantics still hold; observation grants no authority | Existing lease/admission process fixtures |

Process assertions wait on fixture events and actual reap/lock transitions,
with a failure timeout; they do not infer death from file existence or elapsed
sleep. A stale-record rejection needs the positive control that the same
observer recognizes a genuinely running launch. The tree-replacement control
starts with the same key in both lifetimes. Render tests inspect semantic text
and styles, rather than pinning every border cell.

The death controls have two layers. On macOS and Linux, real subprocess fixtures
hold the two exclusive witnesses, acknowledge readiness through a pipe, and are
then killed and reaped with `waitpid` before observing the leftover records.
Use independent read-only descriptors to check both contended probes before
death and both successful probes afterwards. Rename/remove and recreate the
root while the original witnesses are held; observers opened afterwards must
see the replacement as a different lifetime. This checks identity comparison
and activity binding, not whether the pin prevents inode reuse; a host that
does not reuse inode numbers can pass it without a working pin. A separate
shared-holder process must permit another shared probe and make exclusive
preparation fail promptly.

The internal lock/filesystem barrier seam separately permits the two close
orders and makes a replacement report the old device/inode pair and task key.
Do not depend on the host allocator reusing an inode in a timeout. Pause once
between root-pin release and private-witness release, and once after a successful
directory verification but before the final private probe. These controls model
kernel interleavings; they are not a claim to have paused kernel teardown.
The first must reject the replacement with a still-positive private witness;
disabling the directory check must make that control attach activity wrongly.
Disabling replacement's epoch-before-directory-acquisition rule must similarly
expose a new directory witness under an old epoch. Keep the valid running-launch
control passing in both mutation checks, so blanket unavailability cannot pass.

## Scope

This design adds observation and presentation. It does not add lifecycle
outcomes, durable runtime files in the task tree, a scheduler, launch-policy
settings, clickable execution actions, or a per-viewer background service.
The witness protocol's concurrency claims require real process validation and
adversarial review; this document is not a formal proof of the implementation.

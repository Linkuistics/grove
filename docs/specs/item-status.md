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
owns the session-witness protocol, ownership, freshness and guard lifetimes.
This spec consumes that contract. Module ownership remains in
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

The guard owns whatever short read guards the operation acquired. Callers copy
rows and selected bytes, then drop it before another acquisition, layout, input
or waiting. A retained `TreeLifetime` pins a directory without holding a tree
or epoch lock. The type grants no session admission or driver ownership. All
lock attempts are quiet and nonblocking; runtime discovery requires no launch
configuration, creates nothing, and never attaches an ancestor workspace's
driver to a tree observed in a subdirectory.

Finalize activity only after the observation has pinned and validated its tree
lifetime. Row matching uses that same retained identity; never join an earlier
liveness result to a newly opened task root.

The runtime result has these meanings:

| Result | Meaning for the viewer |
|---|---|
| Idle | The observation establishes no current running session. Select ordinary NEXT from a valid current tree. |
| Running with mandate | A witnessed launch is current. The mandate carries a permanent key, tree lifetime and launch-time handle/kind for fallback description. |
| Busy with reason | Publication or observation is in progress, including a launch not yet witnessed as started. Show activity waiting; withhold current RUNNING and NEXT. |
| Unavailable with reason | Required evidence cannot be established, including unsupported or malformed metadata. Show activity unavailable; withhold current RUNNING and NEXT. |

No workspace/control namespace or a verified absence of a current session is
the ordinary idle case. Missing metadata beside an existing lease is
unavailable. Older active epochs lacking
observational metadata never become RUNNING by guessing from the tree. Missing
runtime support must leave temporary non-jj trees fully browsable.

The viewer retains its bounded two-capture consistency check. Each capture
releases its guards before the next. Compare tree rows and selected bytes as
before, and compare runtime identities and witness state separately. Changing
activity must not reject an otherwise consistent tree. An unstable runtime pair
produces Busy for activity and retries on the next deadline. A failing tree
capture suppresses NEXT and all activity attachments to retained rows; a fresh
mandate may still appear in the summary with `tree unavailable`.

Only an accepted, current observation supports row activity. When observation
fails or is contended, a retained tree remains explicitly WAITING or STALE and
previous row activity is cleared. Any retained activity description is labeled
STALE, never presented as the current RUNNING/NEXT pair. Both views, help and
undersized frames continue ticking; at most one retry is pending.

### One selection rule

The loop owns a selection operation over an already-read typed snapshot with
an optional excluded permanent key. Ordinary driver selection supplies no
exclusion. The viewer supplies a key only when the witnessed mandate belongs
to the snapshot's tree lifetime.

First validate the whole tree, including duplicate keys and multiple live finish
leaves. Exclusion cannot make malformed input valid. Then remove the excluded
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

Resolve a witnessed mandate by permanent key within its tree lifetime. Use its
current handle and species when present, so renaming, moving, retirement and
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
its highlighted running name. NEXT emphasizes its activity word without
highlighting the whole row.

Selection adds the cursor and emphasis without replacing lifecycle foreground
colors or the RUNNING highlight with a uniform highlight color or reverse-video
style. Words, glyphs, cursor and fold indicators remain distinguishable with
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
apply only while File is active. Help, refresh and quit remain global; the help
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
| Reap or driver death releases the old root pin during observation | The observed tree is pinned before the final witness probe; inode/key reuse cannot receive old RUNNING activity | Observer lock/filesystem barriers |
| Item absent or whole tree removed during a launch | Summary retains the witnessed identity with the appropriate absence qualifier | Application plus real launch |
| Old, missing, malformed, truncated, oversized or mismatched observation records | Typed unavailable/busy state where evidence is missing; readable trees still browse | Typed observer; application |
| Witness allocation or started-marker write fails | Session authority and outcome are preserved; activity is Unavailable for missing metadata/invalid bytes or Busy for an incomplete marker | Runner/observer fault seam |
| Control path replaced by a FIFO or a directory; open/lock identity race | Bounded quiet failure/retry, with no blocking read or invented RUNNING | Existing filesystem barriers/process fixtures |
| Tree or epoch contention and rapid start/end changes | Current pair is withheld when unverifiable; navigation/quit remain responsive; cadence recovers | Application and process fixtures |
| Multiple viewers, non-jj location, missing namespace, absent tree | Browsing creates no files/directories or persisted state and never resolves launch configuration | Application filesystem snapshots; typed observer |
| Ambient old session after rotation; an admitted operation during replacement | Existing admission rejection and handoff semantics still hold; observation grants no authority | Existing lease/admission process fixtures |

Process assertions wait on fixture events and actual reap/lock transitions,
with a failure timeout; they do not infer death from file existence or elapsed
sleep. A stale-record rejection needs the positive control that the same
observer recognizes a genuinely running launch. The tree-replacement control
starts with the same key in both lifetimes. Render tests inspect semantic text
and styles, rather than pinning every border cell.

## Scope

This design adds observation and presentation. It does not add lifecycle
outcomes, durable runtime files in the task tree, a scheduler, launch-policy
settings, clickable execution actions, or a per-viewer background service.
The witness protocol's concurrency claims require real process validation and
adversarial review; this document is not a formal proof of the implementation.

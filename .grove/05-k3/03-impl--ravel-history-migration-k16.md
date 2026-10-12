# ravel-history-migration-k16


## Goal

Make the runnable Ravel use the owner's preserved configuration and an isolated,
consistent snapshot of existing history while the old front remains usable.



## Context

- Requires `ravel-runtime-k15`. Read its Ravel commit and ordinary migration
  instructions. The parent `ravel-cli-k3` retains the full acceptance contract.
- Recheck actual `~/.config/harness-dispatch` and `~/.config/ravel` contents;
  planning inspection is not a lock. Recheck state-dir selections in settings.
- Keep the original installed Grove 22.1.0/dispatch ending contract intact.

## Done when

- Copy actual personal config to ~/.config/ravel, preserving source contents and
  destination edits. Preserve settings format, policy choices, routes, grants,
  bounds and local selections; change only necessary SDK/import locations.
  Inspect the copied policy from this Grove workspace and an ordinary directory
  without activating a sample or launching a model.
- Seed ~/.local/state/ravel/records.sqlite3 once through a consistent SQLite
  backup of the selected legacy store. Preserve IDs, launches, observations and
  immutable documents. Preserve an existing destination and refuse conflicting
  imports. Explicit legacy state-dir selections relocate for Ravel; neither
  migration nor runtime writes/upgrades the legacy store.
- Verify the old 22.1.0 front still opens, appends and reaps in its original
  store after Ravel writes its separate store. Confirm the legacy application's
  ID/version and pre-existing immutable documents stay unchanged; include a
  late-reap fixture using isolated stores/channels.
- Record the actual snapshot boundary and a final catch-up import procedure in
  ordinary Ravel documentation. Later old-supervisor records stay accessible
  through the retained old front/store; this is not continuous replication.
- Commit project artifacts through jj in Ravel, name the exact commit and
  config/store verification here, retire this child, reconcile the parent's
  complete Done when against k14/k15/k16, settle any parent-chain cascade and
  seal the coordination-tree commit before signalling with the old command.

## Notes

Release preparation/publication remain k4/k11, and new controlled-session
behavior remains k7. Do not overwrite source or destination edits to simplify
the copy. Preserve the old installed front and actual legacy store.

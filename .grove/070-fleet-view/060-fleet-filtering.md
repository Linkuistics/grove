# 060-fleet-filtering

**Kind:** work

## Goal

Add interactive **filtering** to the fleet nav (the piece deferred from the MVP at
070 Q5), satisfying the root brief's "filterable by repo / workstream / inbox-pending
count." Deliberately last in the node.

## Context

- Deferred from `040` to keep the first multi-repo cut tight (070 Q5). The grouping +
  sort + collapse shipped in `040`; this leaf adds the filter on top.
- Recommended shape (revisit at impl — it was *deferred*, not decided): a **fuzzy
  text filter** matching `repo/grove name`, plus a small set of **predicate toggles**
  (`inbox-pending > 0`, live-vs-seed). Empty repo sections hide while a filter is
  active. This was the leading option at Q5 before filtering was cut from v1; confirm
  or revise during this leaf rather than treating it as settled.
- Repo *filtering* is largely subsumed by the collapsible grouping (070 Q5 reasoning),
  so weight the design toward find-a-workstream-fast and show-what-needs-attention.
- An optional **sort toggle** (e.g. inbox-desc) belongs here too (070 Q5 deferred it
  from `040`).

## Done when

- The nav can filter the grouped fleet by grove/repo name and by inbox-pending; empty
  sections hide while filtering.
- Filter state is ephemeral (per session, not persisted — constraint 1).
- Single-repo (N=1) filtering still works (it is the same render path).
- The root brief's "filterable by repo / workstream / inbox-pending" done-criterion
  is met.

## Notes

Depends on `040` (grouped nav). Last leaf of the fleet node; after it the only
remaining grove-level leaf is `080-async-revisit`. If filtering turns out to want its
own small grilling (the shape is only *recommended*, not settled), open it with one —
don't force the deferred recommendation through.

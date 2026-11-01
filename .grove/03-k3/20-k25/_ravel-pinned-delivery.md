# ravel-pinned-delivery-k25 — brief

## Goal

Extend the working local route to exact pinned Git sources, completing Ravel's
generic source contract before Grove adopts directory delivery. Rhai native
compatibility is independent of this feature.

## Done when

- Public resolution and launch acquire exactly the configured commit/subdir,
  with object-format/filesystem validation, noninteractive bounded transport
  and no hook/filter/submodule execution or inherited completion authority.
- Verified exact cache hits require neither network nor Git. Missing cold pins,
  corrupt cache, object/subdir errors and timeouts retain distinct remedies;
  no nearby revision, branch or source is substituted.
- Pin changes, concurrent publication, mixed required local/Git sources and
  retained readers satisfy the same descriptor/resource and confinement contract.
- Ravel's full discovery acceptance fixtures, applicable repository/book checks,
  installed layout and Git transport dependency checks pass against a reproducible
  matched independent build. Owning documentation and commits record the handoff.

## Decomposition

The child planner consumes actual local-delivery implementation. Git transport
and extraction must reach existing public readers in each working increment;
private acquisition machinery alone does not close this node.

## Pointers

Ravel's reviewed `docs/specs/skill-discovery.md`, its exact-Git, publication,
failure and acceptance sections; the preceding local node's owning commits and
recorded installed-pair inputs. Use the repaired contract from
ravel-rhai-skill-delivery-k34, not the historical TypeScript/schema-3 design.
Follow Ravel's repository instructions.

## Notes

Only source acquisition extends here. No skill catalog, Grove dependency,
native installer or garbage collector is introduced. Final publication belongs
to discovery-rollout-k6; use isolated temporary installs for acceptance.
At entry the owning pinned implementation session rebases the recorded local
commits onto current Ravel main, checks the integrated local path and records
the exact base. Keep all work on `skill-directory-delivery`, outside main,
through acceptance. Rollout owns final rebase, checks and landing of the complete
feature. Remove the local subset's temporary Git refusal and its fixture when
the public Git path works; neither may survive into publication.

# witnessed-epoch-k33


## Goal
Complete witness-publication-k31 by publishing the selected mandate extension
and exact Started marker on the paired ownership seam from paired-witness-owner-k32.



## Context
Read the parent and launch-witnesses-k25 contracts in full. k32 intentionally
publishes no observation extension or marker; its empty witnesses are not RUNNING.

## Done when
- Pass selected key/handle/kind into preparation, bind tree/private identities,
  namespace-local basename, nonce and signal path in a recognized optional
  version. Mandatory admission ignores absent, malformed and unsupported extensions.
- Publish only with both locks prepared; Started writes exactly `started\n` once.
  Publication failures diagnose without changing successful launch outcome;
  mandatory epoch-write failure prevents spawn. Preserve guard-free callbacks.
- Complete every remaining writer criterion and fault/event/real-launch control
  from witness-owner-k29 and launch-witnesses-k25, including signal-before-reap,
  release order, confirmed/unconfirmed reap, unwind and replacement ordering.
- Production try_observe remains Unavailable even with the extension. Preserve
  admission/handoff/root-replacement controls; update docs/books and pass
  scripts/check.sh. Check every ancestor's full contract before closing.
- Native observer/platform and forced-reuse evidence stays with k26; usage/G6
  keep the witnessed-view deferral. Arrange protocol review as the enclosing
  witnessed-activity node requires, without preclaiming observer evidence.

## Notes

## Decisions (running log)

- Keep mandatory epoch serialization and field grammar unchanged. Append an ASCII,
  `observation-`-prefixed version-1 extension only after paired preparation and
  the mandatory active write succeed; encode text fields as hex so they cannot
  introduce mandatory record fields. Its nonce and signal binding is the enclosing
  epoch record, written under the same exclusive guard.
- Pass the already validated Selection to preparation; derive its explicit key
  from its typed Handle. Publish Started through the lease-owned pair at most
  once, retaining both locks on publication or unconfirmed-reap failure.
- Verification sequence: failing real-run marker/record tests; focused writer,
  admission and fault tests; source-derived book repair; principal gate. The
  enclosing k12 requires whole-protocol review after k27; k26 owns native
  observer/platform and forced-reuse evidence.
- Admission decodes unknown extension bytes tolerantly, while mandatory ASCII
  fields retain their validators. The single fresh-context review found a valid
  panic in hex-path decoding for non-ASCII input; reject it before string slicing.
  The admission test reproduces the panic and covers both mandatory path fields,
  including raw invalid UTF-8. This executable seam conclusively covers the fix;
  no second in-session reviewer is used.

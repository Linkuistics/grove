# lease-root-owner-k30


## Goal
Move the selected root pin into DriverLease before epoch publication and bind
its release to parent-side launch events.



## Context

## Done when
- Root replacement during exclusive epoch acquisition refuses launch; the root
  check runs under the acquired guard before activation.
- The lease retains the root across Started and unconfirmed supervision errors;
  failed spawn and confirmed Reaped release it before terminal recovery. Drop
  and unwind release it before driver ownership. Selection stays descriptor-free.
- Event/fault seam tests and real configured successful/failed launches exercise
  these transitions. Active records still observe as Unavailable.
- Shipped descriptions and affected books match, focused tests and the principal
  gate pass. Witness locks and publication remain in witness-publication-k31.

## Decisions (running log)

The current helper borrows SelectedTask and the lease. Transfer ownership at
preparation and use the existing Started/Reaped runner events to release the
lease's launch value. Rechecking under exclusive epoch acquisition closes the
wait window independently of witness publication.

The one in-session adversarial review found no production defect. Its weak
second-attempt assertion was actionable: use a replaced root and compare the
retained descriptor's identity with the original. Its drop-order evidence gap
is accepted explicitly: this child inspects the explicit Drop ordering and
checks driver release after unwind, while k31 owns native two-witness release
controls. No kernel-teardown or Linux evidence is claimed here.

## Notes

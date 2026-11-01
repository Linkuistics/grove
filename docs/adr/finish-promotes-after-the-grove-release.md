# Finish promotes after the Grove release

A dependency promotion that needs the published compatible Grove runs inside
the driver-created finish session. Finish runs it after Grove's minor release
and installed verification, and before its final signal. The operation is
prepared and rehearsed in ordinary leaves before finish. Finish's one
confirmation covers it as part of the concrete whole sequence. An incomplete
promotion is an incomplete finish, so the session never signals success.

The first case is the promotion of Ravel 2.0.0 to unversioned `ravel`. The
[coordinated release specification](../specs/coordinated-release.md) owns its
order, its handoff and its recovery rows.
Ravel's decision record `a-caller-transition-publishes-a-versioned-formula-first`
explains why the promotion must follow the compatible Grove release.

The trade-off is a longer finish session against a promotion with no owner.
Finish grows by one rehearsed public operation. That operation runs after
`finish-commit` removes `.grove/`, so no leaf can carry a repair. Only resume
rows apply there, and a failure ends without a successful signal.

The design rejected three alternatives. An ordinary leaf cannot follow Grove's
release, because that release belongs to finish and no leaf follows the finish
sentinel. Promotion before Grove's release would give fresh installations a
Ravel major that released Grove refuses. A promotion left to the owner after
the grove ends would let finish signal success while the default channel still
lacks the release.

Reopen this decision if Grove's release leaves finish. Reopen it also if a
compatible Grove that accepts both majors is public before the dependency cut.
Then the promotion needs no finish extension.

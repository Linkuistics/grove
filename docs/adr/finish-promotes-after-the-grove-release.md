# Finish promotes after the Grove release

A cutover operation that needs the published compatible Grove runs inside the
driver-created finish session. Finish runs it after Grove's minor release and
installed verification, and before its final signal. The operation is prepared
and rehearsed in ordinary leaves before finish. Finish's one confirmation
covers it as part of the concrete whole sequence. An incomplete operation is an
incomplete finish, so the session never signals success.

The first case is the promotion of Ravel 2.0.0 to unversioned `ravel`. The
[coordinated release specification](../specs/coordinated-release.md) owns its
order, its handoff and its recovery rows.
Ravel's decision record `a-caller-transition-publishes-a-versioned-formula-first`
explains why the promotion must follow the compatible Grove release.

The second case is the owner's skill-delivery switch. It activates Ravel's
directory delivery in the owner's real HOME and deactivates superseded native
skills. It needs the promoted Ravel and the compatible Grove. It also needs
the end of the driver's last Ravel 1.1 launch, which is finish's own launch.
The [owner skill delivery specification](../specs/owner-skill-delivery.md) owns
its steps, probes and rollback. The switch follows the promotion.

The trade-off is a longer finish session against operations with no owner.
Finish grows by two prepared operations. Promotion follows its release
recovery rows. The owner's serial switch follows an owner checklist and a disposable execution of the real plan.
It backs up before any write and restores native delivery on failure. It requires a quiet interval instead of preserving
concurrent edits.

Both operations run after `finish-commit` removes `.grove/`,
so no leaf can carry a repair. An interrupted switch requires the same restore
command before a fresh attempt. It never resumes partly applied changes.
A failure ends without a successful signal.

The design rejected four alternatives. An ordinary leaf cannot follow Grove's
release, because that release belongs to finish and no leaf follows the finish
sentinel. Promotion before Grove's release would give fresh installations a
Ravel major that released Grove refuses. An operation left to the owner after
the grove ends would let finish signal success while the cutover is still
incomplete. A switch in an ordinary leaf before finish would deactivate native
skills too early. Every later session of the old driver still loads them,
including finish itself.

Reopen this decision if Grove's release leaves finish. Reopen the first case
if a compatible Grove that accepts both majors is public before the dependency
cut. Reopen the second case if the driver that runs finish can deliver skill
directories. Then that case needs no finish extension.

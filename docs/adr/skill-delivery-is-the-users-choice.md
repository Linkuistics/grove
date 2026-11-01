# Skill delivery is the user's choice

Grove names a required skill and leaves its availability to the user. Native
harness installation and Ravel's skill-directory injection are alternative
routes. Grove does not install skills, resolve their locations, or require
Ravel discovery. The agent finds and reads the skill through the chosen route.
The [delivery specification](../specs/codex-skill-provisioning.md) owns setup
and acceptance.

Removing automatic Codex provisioning decouples the binary release from the
skill corpus. It costs the convenience of a binary-only first installation:
the user must also supply skills to every selected harness or configure Ravel's
directory delivery. Restoring implicit installation would change that public
setup contract and recreate the corpus dependency. This deliberate absence
could otherwise look like an incomplete bootstrap.

An embedded core-only fallback was considered. It cannot supply every open kind
or external resource, and it introduces version and source-precedence decisions
beside the user's chosen route. Reject it while explicit native or directory
setup can supply the required guidance. Reopen if a useful first launch with
neither route becomes a requirement; any new delivery must respect the common
source boundary rather than recreate automatic native provisioning.

Retaining the broad Codex snapshot installer would preserve convenience but
keep an automatic writer and embedded inventory coupled to moved skills.
Deleting its existing output would break readers whose lifetime Grove cannot
establish. The writer is removed; existing native installations and snapshots
remain. Optional manual cleanup by the user, under the procedure owned by the
[delivery specification](../specs/codex-skill-provisioning.md#existing-installations-and-active-readers),
is the accepted administration and storage cost.

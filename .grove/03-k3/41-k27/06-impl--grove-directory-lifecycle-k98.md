# grove-directory-lifecycle-k98

## Goal

Prove the directory route on 2.0 through Grove's lifecycle caller with local
sources and native copies absent. Then advertise the route in the owner
guidance.

## Context

- The "Directory bootstrap", "Source and skill failures", "Original mandate",
  "Settings isolation" and "Resource lifetime" rows of the delivery
  specification.
- Ravel's specification `docs/specs/skill-discovery.md` at the directory build.
  Its acceptance case "Local sources only" uses a spy `git`.
- Ravel's `task smoke:pair OLD=<1.1 executable> NEW=<prefix>` runs a 1.1 and a
  directory launch at the same time, each under its own HOME.

## Done when

- A lifecycle launch on the 2.0 build uses `skills` settings in a HOME of its
  own. Its sources sit outside cwd. One is a plain directory, and one is a
  plain Git checkout at a commit.
- The fake agent reads the spine, the named kind and a supporting resource from
  the injected directories. No native copy exists in that HOME.
- `request.originalPrompt` and the caller block of the prepared prompt each
  equal the composed mandate byte for byte. The ending commands are exact. The
  preamble names the mandatory reads before the caller block.
- A missing checkout refuses as `skill_source_missing` before any worker. Grove
  reports Ravel's remedy, the leaf stays live and the loop does not relaunch.
- One directory launch runs under a guard that denies the network. A spy `git`
  first on the PATH records no call.
- A native lifecycle launch on 1.1 and a directory launch on 2.0 run at the same
  time in separate HOMEs. A 1.1 HOME with `skills` settings refuses as
  `settings_invalid` and leaves the leaf live.
- A running session keeps its resolved directories while its source changes.
  The next launch receives the new view.
- The delivery specification, `docs/USAGE.md`, the configure-grove skill and
  the plugin installation guidance state the directory route as supported.
  They state the checkout boundary and the separate HOME.

## Notes

These fixtures prove delivery and Grove's interpretation, not the behaviour of
an LLM. common-loading-acceptance-k29 owns the real harness trials. The
explicit plugin-qualified name request also belongs to k29.

Never use the real HOME or the owner's `~/.config/ravel`. Controls: a fixture
whose fake agent reads a native copy must fail. A caller block that differs by
one byte must fail. Watch each fail once.

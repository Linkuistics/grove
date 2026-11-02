# owner-pack-checkouts-k79

## Goal

Settle the owner and the procedure that install the skill-pack repositories
as local checkouts and keep them current, outside Ravel. Revise the
owner-delivery design and the external-pack contract to match.

## Context

- The root brief records the decision of ravel-git-acquire-k77: Ravel reads
  local directories only.
- `docs/specs/owner-skill-delivery.md`, `docs/owner-skill-delivery-evidence.md`
  and `docs/owner-skill-inventory.tsv` describe the current owner design.
- The brief of external-research-pack-k4 still requires pinned Git sources and
  offline pins in Ravel.
- The machine-environment contract gives machine setup to the mise bootstrap.
  It is one candidate owner. The owner's projects use jj for checkouts.

## Done when

- The design names the owner of installation and update and the commands that
  each one runs.
- The design states how a revision is chosen and recorded, and what a
  launch does when a checkout is missing.
- The design states the behaviour without a network, after the first install.
- The owner-delivery specification and its evidence agree with the design.
  The text that replaces the pinned-Git clauses of external-research-pack-k4
  is ready for the replanning leaf.

## Notes

The choice of owner can be the human's to make. If it is, ask with a
recommendation and evidence, as the spine directs. Earn a focused review if
the design changes the owner-delivery specification materially.

## Decisions (running log)

**Base.** On 2026-11-02 this checkout was at `a20454a854b7`, the output of
ravel-local-sources-k78. The Ravel contract is the narrowed local-source
specification at `7a894c782037` on the unpushed bookmark
`skill-directory-delivery`.

**Owner.** On 2026-11-02 the human chose mise's `[bootstrap.repos]` part as
the owner of the pack checkouts. Reasons: the machine-environment contract
already gives machine setup to `mise bootstrap`, and the part does what the
owner needs with no new tool. The human rejected a Taskfile with a lock file in
skill-marketplace, and the Claude Code marketplace clones.

**Revision policy.** On 2026-11-02 the human chose an exact, published commit
for each pack. The owner advances a pack by an edit of its `ref` and a run of
`mise bootstrap repos apply`. The human rejected a checkout that follows the
default branch, and a mixed policy. The measured reason: without a `ref`,
`status` reports `current` after the remote moves ahead, and nothing records
the installed revision.

**mise is the owner's instance, not a requirement.** On 2026-11-02 the human
said: "We should not require mise however - I will do that but other ravel
users might do something else." The general contract therefore names no tool.

Ravel's section "Sources outside Ravel" already states what any checkout tool
does. The tool puts the directory in place and chooses its revision. It keeps
the directory quiet during a launch. It restores a missing directory outside
the failed session.

Ravel's specification, Grove's delivery specification and the external-pack
text stay tool-neutral. Pack
acceptance uses a plain Git checkout at a commit. No tool-neutral contract
names mise. The owner-delivery specification, its views, its instrument and
this grove's briefs name it as the owner's instance.

**No ADR.** The choice fails the "hard to reverse" condition of the ADR test.
A different checkout tool needs new entries and new clones, and Ravel and Grove
do not change. The specification section "Pack checkouts" records the decision
and the rejected alternatives.

## Handoff to skill-loading-replan-k80

Replacement text for the brief of external-research-pack-k4. Replace its third
Done when bullet with this bullet:

> - ResearchSkills is published on its remote. A checkout of a published
>   commit, which a tool outside Ravel makes, works as a local source. The
>   proof uses a plain Git checkout and requires no particular checkout tool.
>   A missing checkout or root refuses as `skill_source_missing` before any
>   worker. Run each test from a directory outside the Grove working copy, the
>   ResearchSkills working copy and the checkout. Use the actual session and
>   delegate read permissions.

In its last Done when bullet, replace "the proven layout, resource and
conformance boundary" with this text: "the proven layout, resource, checkout
and conformance boundary. The checkout boundary names the source directory and
roots inside any checkout of the pack."

Replacement text for the root brief follows. Find the Ravel paragraph of
"Ownership boundaries". Replace its sentence whose first words are "A design
leaf after the pinned node" with the quoted sentences below. The sentence whose first
words are "This paragraph supersedes" stays while another brief holds a pinned-Git
clause. Delete it as the last edit of the reconciliation:

> owner-pack-checkouts-k79 settled the mechanism. Ravel and Grove require no
> checkout tool. On the owner's machine, the `[bootstrap.repos]` part of mise
> holds each pack checkout at a published commit. `docs/specs/owner-skill-delivery.md`
> states the commands, the missing-checkout behaviour and the behaviour without
> a network.

The cutover contract of the owner-delivery specification now adds pack
publication to the repository moves (order 2). It adds the `[bootstrap.repos]`
entries to the drafts (order 4), and the checkout clones and the `mac-check`
change to the preparation (order 6). owner-ravel-skill-inventory-k38 grows
those leaves from the specification. Nothing in this handoff changes the Ravel
removal of ravel-local-sources-k78.

## Review triage

The in-session reviewer read the change without my conclusions and reported 22
findings. 21 are valid and actionable. Finding 18 is a contract that I stated
unclearly: the log said that only the specification names mise. Each fix is
in the changed files:

- 1: mise's status ignores files that Git ignores, and Ravel captures them. The
  specification forbids tools in a checkout and adds an ignored-file check to
  the switch. The script asserts the gap.
- 2: on a new machine, adoption restores the Ravel settings before the
  repository part. The command table states that order.
- 3 and 6: a clone inside the staging HOME added links, and Homebrew Git can
  reach the real keychain. The staging checkouts now lie under a separate HOME,
  with no system Git configuration and no prompt.
- 4: an advance during a launch can refuse or mix two commits. An advance now
  runs only while no Ravel launch runs.
- 5: one dirty checkout stops every entry and the later bootstrap parts. I
  reproduced it, and the specification states it.
- 7 and 11: the design states that it assumes public remotes, and that the
  settings name absolute paths.
- 8: every command names the HOME with `-C`, as `mac-bootstrap` does.
- 9: the tracked file holds the intended pin, and the checkout HEAD holds the
  delivered commit.
- 10: the `grove-core` pin follows Grove releases. The switch advances it to
  the commit of finish's release, before the status gate.
- 12: a rewritten third-party history needs a new pin.
- 13: the script now asserts the detached HEAD, the fetch, the exit codes, the
  plan and the ignored file. It passes all 33 cases, and `--mutate` fails one.
- 14: the complete bootstrap and the adoption need the real HOME or a new
  machine. The evidence says so.
- 15 and 19: the handoff keeps the supersession sentence until the last brief
  edit, and names the three directories that a test runs outside.
- 16, 17, 20, 21 and 22: `mac-check` reads `--json`, the staging rule names
  five Claude Code variables, three views changed, "at most one snapshot", and
  the layout is fixed.

**Escalation.** Fixes 3, 4 and 10 change the design, not only its wording. No
executable test covers the new finish step or the separate checkout HOME.
Under the review allowance, that second need is a review leaf, so
review-design `owner-pack-checkouts` runs before skill-loading-replan-k80.


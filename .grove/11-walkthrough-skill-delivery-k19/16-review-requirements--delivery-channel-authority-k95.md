# delivery-channel-authority-k95

**Reviews:** delivery-channel-authority-k94

## Goal

Adversarially try to disprove the `delivery-channel-authority-k94` requirements
decision before any replacement-campaign child is allowed to run.

## Context

- Producer artifact and stable handoff: `delivery-channel-authority-k94`, read
  from its own focused commit.
- Authority being preserved: `acceptance-replication-authority-k76` and the
  amended `walkthrough-skill-delivery-k19` brief chain.
- Finding that opened the gate: `supplemental-evaluation-k92` G1 and its
  disposition in `supplemental-evaluation-k93`.
- Blocked consumer: `paired-acceptance-campaign-k80`, which remains unable to
  run while the producer says `authorization = none`.
- Product decision that does not itself establish evaluation delivery:
  `docs/adr/skill-delivers-the-methodology.md`.

## Done when

- Re-derive the producer decision from the committed diff and available channel
  evidence. Try to find a currently provisioned direct request channel or
  authoritative harness receipt that the producer missed; reject local argv,
  filesystem, client-side digest, or asserted file-read evidence unless it
  demonstrably crosses the provider acceptance boundary.
- Try to disprove receipt authority in both directions: identify any false
  positive that could admit ignored, normalized, truncated, rejected, partial,
  or differently tooled treatment, and any requirement so strong that it rules
  out a channel that actually makes effective delivery checkable. Challenge the
  correlation, redaction, chronology, provider/runtime identity, and error
  taxonomy explicitly.
- Try to disprove treatment identity: recursively enumerate what the deployed
  skill can contain; challenge manifest exhaustiveness, deterministic ordering,
  UTF-8 and path rules, symlink and hard-link handling, byte preservation,
  framing visibility, control-arm delta, and the 65,536-byte cap. Reject any
  framing that silently turns the deployable skill into evaluation-specific
  semantic instructions.
- Try to disprove model-interface confinement. For intake and
  exposition/assurance, establish whether an explicit empty tool declaration is
  observable at the authoritative seam. For source/fragment, challenge whether
  one zero-argument `read_fixture` operation can be bound to the vendored
  manifest, called at most once, and prevented from accepting a path, traversing,
  following symlinks, mutating, or retrieving over a network.
- Try to disprove freshness: look for prior messages, response/session IDs,
  implicit harness state, caching, hidden tools, or a fixture-tool continuation
  that turns one fresh top-level request into a resumed conversation. Require
  the preserved request and receipt to enumerate every model-visible system,
  user, treatment, tool, fixture, and framing input.
- Check that leaving authority open changes no parent acceptance meaning and
  that the handoff is exact enough for `paired-acceptance-campaign-k80` to stop
  without making a new requirements choice.
- Record severity-ordered findings against the producer commit without editing
  the producer task. If actionable findings exist, commission a lazy
  `integrate-review-requirements` step with this bare stem where the walk reaches
  it before `paired-acceptance-campaign-k80`; if none exist, retire without
  creating one.
- Launch no evaluated treatment, control, scorer, or resolver context. Do not
  run a transport probe without separately authorized credentials and a channel.

## Notes

Assume the negative authorization is overconfident and the reopening contract
is internally inconsistent. A valid review may still conclude that no current
channel qualifies, but only after trying to produce a counterexample against
each of receipt authority, treatment identity, tool confinement, and freshness.

# delivery-channel-provisioning-authority-k97

## Goal

Obtain the human's explicit choice between provisioning an acceptance-authorized
delivery channel and closing the parent behavioral conjunct, then settle the two
model-visible treatment choices that depend on that answer before the blocked
paired campaign can run.

## Context

- Corrected authority and sufficient-evidence contract:
  `delivery-channel-authority-k94`, including the H1–H13 dispositions integrated
  by `delivery-channel-authority-k96`.
- Human-authority gap: `delivery-channel-authority-k95` H3 and H4.
- Parent acceptance authority: `acceptance-replication-authority-k76` and
  `walkthrough-skill-delivery-k19`.
- Blocked consumer: `paired-acceptance-campaign-k80`; no child may run while
  `delivery-channel-authority-k94: authorization = none`.
- Current primary-source mechanics: Anthropic request IDs and usage do not bind
  request contents; Anthropic cache hits require an identical prior prefix but
  do not establish that prefix's treatment identity; OpenAI request IDs are
  troubleshooting identifiers.

## Done when

- Ask the human to choose explicitly among these outcomes, recommending closure
  unless they are willing to provision and operate the required boundary:
  1. identify a provider that attests, at its final pre-inference boundary, the
     independently recomputable canonical input bytes and token sequence after
     every transformation;
  2. provision a pinned self-hosted inference runtime with an independently
     captured canonical input byte stream and token sequence, pinned tokenizer
     and chat-template behavior, and authenticated request/output correlation;
     or
  3. decline both and amend the parent brief to close the behavioral conjunct as
     unmet rather than pending.
- Price the options before asking. A hosted Claude/OpenAI/Codex credential plus
  request IDs and usage is insufficient under the current contract; no inspected
  provider offers option 1, and an upstream gateway cannot manufacture it;
  option 2 requires controlled model hosting and capture at the final
  transformation boundary; option 3 preserves all historical contrary evidence
  but ends this acceptance path.
- If the human chooses a channel, pin the exact provider/runtime, endpoint,
  model snapshot, schema, authenticated identity, request transformation,
  final-boundary attestation or capture, canonicalization, tokenizer, chat
  template, special-token rules, independently derived expected bytes and token
  sequence, sampling configuration, error taxonomy, redaction, retention, and
  operator procedure. Run only a bounded non-evaluated mechanics check; launch
  no evaluated context.
- Decide whether source/fragment receives its identical-arm fixture inline with
  zero tools or through one pathless, zero-argument, at-most-once tool. State
  whether choosing to call the tool is scored behavior. Recommend inline input
  unless that choice itself is acceptance-critical.
- Decide whether treatment delivery carries raw `SKILL.md` bytes including YAML
  frontmatter or reproduces deployed semantics in which frontmatter is metadata
  and only the body is instruction text. Record the raw digest, extraction or
  framing rule, outbound-reference closure, and every deployed-versus-delivered
  difference. Recommend deployed semantics unless the human deliberately wants
  the intervention to include metadata as instruction text.
- Replace `delivery-channel-authority-k94: authorization = none` with one exact
  authorized-channel value only after all conditions are met. Otherwise amend
  the parent behavioral wording according to the human's closure choice. Make
  the handoff exact enough that `acceptance-instrument-k81` has no requirements
  choice left to make.
- Commission a lazy `review-requirements` sibling with bare stem
  `delivery-channel-provisioning-authority`, inserted at the
  `paired-acceptance-campaign-k80` node's slot, before retiring if any channel or
  closure decision is produced. Any findings must settle through an earned
  `integrate-review-requirements` step at that same boundary before the campaign
  may run or the parent node may close.
- Edit no historical evaluation record, frozen rubric byte, or existing file in
  `paired-acceptance-campaign-k80`, and launch no evaluated treatment, control,
  scorer, or resolver context.

## Notes

This is a human-authority leaf because the remaining question is what cost is
worth paying and whether the acceptance claim remains worth pursuing. Credential
existence, client readiness, and technical possibility do not answer it.

If the human declines provisioning, closure is a legitimate requirements result,
not a failure to complete this leaf. Preserve the original failed evaluation and
its Case A limitation as contrary historical evidence in either branch.

## Decisions (running log)

The human chose outcome 3 after the three branches were priced: decline both a
hosted final-boundary attestation channel and controlled self-hosted inference,
and close the parent behavioral-acceptance conjunct as unmet rather than pending
or passed. `delivery-channel-authority-k94: authorization = none` remains the
exact channel value. No evaluated treatment, control, scorer, or resolver
context will run.

The options were priced on 2026-08-30 before the choice. No inspected hosted
Claude/OpenAI/Codex product documented the required final-boundary attestation,
so ordinary API spend bought no acceptance authority and a qualifying hosted
route had unknown enterprise procurement and engineering cost. Controlled
self-hosting was estimated at two to five engineering days plus model-dependent
compute; published on-demand rental examples ranged from US$0.27/hour for a
24 GB GPU to US$4.79/hour for an 80 GB H100. Closure required no inference
infrastructure or evaluated campaign spend, only this requirements change and
its review chain. Primary sources: [Anthropic request IDs](https://platform.claude.com/docs/en/api/errors),
[Anthropic token counting](https://platform.claude.com/docs/en/build-with-claude/token-counting),
[Anthropic prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching),
[OpenAI request IDs](https://developers.openai.com/api/reference/overview),
[llama.cpp server mechanics](https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md),
and [Runpod GPU pricing](https://www.runpod.io/pricing).

The root and `walkthrough-skill-delivery-k19` briefs state the closure directly.
They make no behavioral pass, causal-use, or skill-effect claim and preserve the
original failed evaluation, its Case A selection limitation, and its Case B/C
sample shortfalls as contrary historical evidence.

`paired-acceptance-campaign-k80` is considered on its merits and rejected. With
the human's explicit confirmation it is marked `ABANDONED` in place rather than
deleted, so its task files remain as history but no descendant remains live.

The source/fragment fixture choice and the raw-frontmatter versus deployed-body
treatment choice are inapplicable under closure. No evaluated intervention is
authorized, so choosing either would invent an unused acceptance condition
rather than settle a model-visible difference.

The human agreed the closure test seams: the brief chain states that behavioral
acceptance is unmet; the abandoned campaign has no live descendant; historical
evaluation records remain byte-identical; and a `review-requirements` sibling
adversarially checks the closure before the parent node may close.

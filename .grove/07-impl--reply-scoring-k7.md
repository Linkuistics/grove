# reply-scoring-k7

## Goal

Give the checker its two log modes, `--log-start` and `--log-reply`, as the spec
`docs/specs/technical-communication.md` defines in *The reply log*. This is the
half the reply log runs. A reply is scored and never held back or sent back for
a rewrite.

## Context

- The spec's section *The reply log* defines the location: one JSON Lines file
  for each session, in the state directory. It also defines the start and
  reply records and their fields, `seq` and `since_start`, the exemption marker
  and the checker digest. Last, it defines the lock, and the rule that a failed
  write writes nothing and exits 0.
  The log stores no reply text, no matched fragment and no exemption reason.
- `TECHNICAL_COMMUNICATION_STATE_DIR` moves the state directory. Use it for every
  run here, so that nothing touches the human's real log.
- The requirements fix the measure: hard findings per 100 words, with a count
  for each rule. Advisory findings (passive voice, compound tenses) are counted
  for each rule but not in `per_100`.
- `document-checks-k6` extended the same file. Build on its prose definition
  and its `auto` text type for reply text.

## Done when

- `--log-reply` scores the reply on standard input, appends exactly one
  well-formed reply record, and exits 0 whatever happens. A reply with no prose
  words writes no record.
- `--log-start` appends a start record, and `since_start` counts from the
  agent's last one.
- Concurrent writers cannot corrupt a session's file. Show it by running several
  writers at once in the scratchpad.
- A reply that carries the exemption marker is logged with `exempt: true`.
- The shipped self-test still passes and `bash scripts/check.sh` is green.
- Run over a few real replies from this session's transcript, in the
  scratchpad, it produces records a human finds sensible.
- Where the code meets a defect in the spec's contract, the spec is fixed in
  place, and the running log says why.

## Notes

- Which hook hands a reply to these modes, and how the switch is checked, belong
  to the always-on leaves.

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


## Decisions (running log)

**The leaf fits one session, so it is not decomposed.** The two modes are one
section of `ste-lint.py` (`# ours: begin the reply log`), a small change to
`analyse`, and a self-test. They share the state directory and the lock.

**An exempt reply is scored; the spec said so, the code did not.** `analyse`
returned at once for any valid `exempt` marker, so a reply with the marker had 0
words and, by the "no prose words" rule, would have written no record. The spec
says an exempt reply is "scored, and logged with `exempt: true`". So `analyse`
now returns early only when `reply` is false. With `reply` true the marker sets
`exempt` and the rules still run. The spec's `analyse` paragraph says this now.
The marker's own comment is masked, so it adds no words.

**The state directory is `technical-communication` inside `$XDG_STATE_HOME`.**
The spec said "the state directory" for the log and "`technical-communication/off`
in `$XDG_STATE_HOME`" for the switch, and let the variable move "the whole state
directory". Both read one way only if the variable names the
`technical-communication` directory itself, so the off file is
`$TECHNICAL_COMMUNICATION_STATE_DIR/off`. The spec now says so, and that a
relative `$XDG_STATE_HOME` counts as unset (the XDG rule). The hook core in
`claude-code-always-on-k11` reads the off file with the same function,
`state_dir()`.

**The checker does not read the off file.** The spec gives that to the hook
core ("reads the off file, then serves the event"), and the brief's notes leave
the switch to the always-on leaves. Called by hand, `--log-reply` writes.

**Decisions where the spec left room** (the spec now states each):

- A missing `harness` or `session` is `unknown` in the record and in the file
  name. The record keeps the raw session identifier. Only the file name is
  cleaned.
- Both log modes parse their own arguments (`log_main`) and exit 0 on a usage
  error, a bad stdin or a failed write. `main` routes to them before the normal
  parser, so `argparse` cannot exit 2.
- The writer opens the file `O_RDWR|O_CREAT|O_APPEND`, mode 0600. It polls
  `flock(LOCK_EX|LOCK_NB)` every 10 ms for 2 s, reads the whole file under the
  lock, skips lines that are not JSON, and writes the new line with one
  `os.write`. If the last line has no newline (a crashed writer), it writes one
  first, so two records never join.
- `since_start` counts only this agent's reply records after this agent's last
  start record. A start record of another agent does not reset it.
- Bytes that are not UTF-8 in a reply become U+FFFD, which is not a word. The
  reply is still scored.
- A faulty `exempt` marker in a reply is a hard `marker` finding and counts in
  `hard` and `rules`. It is the one place a reply can score on a marker.
- `rules` counts the unaccepted findings at both levels, keys sorted. In a reply
  no departure applies, so nothing is accepted.

## Checks and what they showed

- The self-test passes on Python 3.15 and on the system Python 3.9.6. It adds the
  log: counts and resets (own agent, other agent, compaction), the fields,
  advisory findings outside `per_100`, exempt, the four kinds of empty reply, a
  missing session, a torn last line, 24 racing threads, an unwritable state
  directory, three bad argument lists, bytes that are not UTF-8, and the
  default state directory.
- **Concurrency, with real processes** (scratchpad): 60 `--log-reply` processes
  and 6 `--log-start` processes at once on one session. The file had 67 lines,
  all valid JSON, and the reply `seq` values were exactly 1 to 60. Control: the
  same run on a copy with the `flock` call removed gave 60 lines but only 31
  distinct `seq` values, so the test does detect a lost serialisation.
- **A held lock**: another process held the lock for 4 s. `--log-reply` waited
  2.05 s, wrote nothing and exited 0.
- **Real replies**: 5 replies of this session and 8 of the previous leaf's
  session (assistant text of 150 characters or more, from the Claude Code
  transcripts) went through `--log-reply`. Records: 0 hard findings in 10 of 13,
  and one `long-sentence` (2.9 to 3.7 per 100) in each of three. No record holds
  text. `seq` and `since_start` count as the spec says.
- **`bash scripts/check.sh`: 18 of 19 pass**, "technical-communication checker
  self-test" among them. The one failure is the one `vendored-checker-k3` and
  `document-checks-k6` reported: `ravel:policy:typecheck` stops with "Bun 1.4.2
  is pinned; install it and retry" while this machine has Bun 1.4.3. I
  confirmed it by running `task ravel:review` alone. The pin is a repository
  decision, so I did not change it. The human needs to settle it before the
  finish cycle's `check.sh`.

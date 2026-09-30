# selection-deadline-k44

## Goal

Bound policy evaluation from its first import. When the whole-selection
deadline expires, `harness-dispatch` stops the worker, reaps it, launches
nothing and exits 124. Trusted TypeScript that hangs can no longer hold an
unattended caller forever.

## Context

The contract is the whole-selection row of the spec's `#bounded-context`
resource table and the deadline paragraph of `#execution-contract`. Static
`routes` policy already executes arbitrary TypeScript at import. So the bound
belongs to the first increment that evaluates policy, not to the evaluation
boundary (review `harness-selection-and-execution-k42`, finding F2). A
worker-side timer cannot end a synchronous spin. Nor can the worker, on its
own, see that a promise kept pending by live asynchronous work is stuck. Only
the front process's wall-clock deadline ends both.

Handled INT, TERM and HUP, and the post-result cancellation checks, stay with
`selection-cancellation-k28`. `computed-selection-k21` and `bounded-context-k22`
add the same hang shapes for `select` and `loadContext` once those callbacks
exist. The record-store lock wait arrives in the next leaf. It neither extends
nor consumes this bound.

## Done when

- `--timeout-ms` accepts 1 to 120 seconds, and the default is 30 seconds. The
  bound counts from worker start to its result and covers module import. An
  out-of-range value is malformed CLI input and exits 2.
- At expiry the front stops the worker with a cleanup grace of at most one
  second, then KILL, and reaps it before returning. It exits 124 with a
  structured diagnostic naming the bound, in text and in `--json`, and launches
  nothing. The deadline does not rely on the worker's cooperation.
- Command-seam tests run `inspect` and `run` with a short explicit bound. The
  policy spins synchronously at import in one case. In another, it awaits a
  promise that a live timer keeps pending. Each asserts exit 124, the
  diagnostic, no surviving worker process and no fake-harness marker.
- Positive control: variants of both fixtures whose spin or pending promise
  ends within the bound do reach the fake harness. No timeout case can then
  pass by never evaluating.
- Inspection reports the effective selection bound. The usage documentation
  states the bound, its ceiling and exit 124. The spec's notice states what is
  delivered.

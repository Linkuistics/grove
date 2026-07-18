# live-verify-k5

**Kind:** review

## Goal

Verify the fix live on both trial harnesses: a full `grove do` task cycle
ends hands-free (no manual `/exit`) on codex, and the pi side still works.

## Done when

- Codex: one real task cycle in a codex-stamped grove ends via `grove-llm
  complete` with the loop relaunching on its own.
- Pi: confirmed that pi's exec sandbox (if any) allows the `$TMPDIR`
  signal-file write, and a pi session ends hands-free the same way.
- Any failure comes back here as evidence, not a workaround.

## Notes

Human-in-the-loop leaf: needs a live trial session on each side.

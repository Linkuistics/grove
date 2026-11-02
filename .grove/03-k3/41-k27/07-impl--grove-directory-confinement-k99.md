# grove-directory-confinement-k99

## Goal

Prove the confined standalone route with directory delivery. Measure the
confined native case separately, with no directory-grant bypass.

## Context

- The "Confined standalone" row and the "Installation and bootstrap" section of
  the delivery specification.
- `crates/grove/src/standalone.rs` and `crates/grove/tests/standalone.rs`.
- Ravel's read grants for skill snapshots, from ravel-skill-confinement-k59.

## Done when

- A confined `grove run` on the 2.0 build with directory delivery reads the
  required resources outside cwd. Its writes and protected configuration reads
  still fail.
- With injection disabled and native trees outside every grant, the fake agent
  cannot read the guidance. It reports the failure and runs the exact
  force-stop command of its prompt. Grove publishes no output.
- The same case on the 1.1 build gives the same outcome. Grove adds no recursive
  native-directory grant and never retries unconfined.
- The cases run on macOS. Each Linux backend that the floor tooling supports
  runs them too, or the evidence names it as unmeasured.
- `docs/specs/standalone-invocations.md` and the overview book's standalone
  chapter agree with behaviour. Any regenerated book passes its check.

## Notes

Native installation and confined native readability are separate claims. This
leaf establishes no working native-only standalone configuration.

Controls: a grant of the native tree must make the unreadable case fail. An
unconfined retry must fail a test. Watch each fail once.

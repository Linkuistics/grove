# template-rule-tests-k118

## Goal

Add tests to `crates/keyed-launch/tests/templates.rs` for the three node-shape
rules `validate_node` enforces and nothing proves, so every rule the crate
refuses a configuration for has a test that names it.

## Context

- Found at `template-law-k111` while writing chapter 4 of the `keyed-launch`
  book, which states every rule with the line that enforces it, the diagnostic it
  produces and the test that pins it — and found three rows it could only leave
  empty. The chapter says so in `#the-rules` rather than implying coverage the
  suite does not have; this leaf is what that paragraph points at.
- The three, all in `validate_node` (`crates/keyed-launch/src/templates.rs`):

  | Rule | Enforced at | Message | Nearest existing coverage |
  | --- | --- | --- | --- |
  | type annotations, on the node or an entry | line 428 | `type annotations are not allowed` | none in the workspace |
  | exactly one positional argument | line 440 | `a key must have exactly one positional argument` | `crates/grove-loop/tests/session_config.rs` line 530 only |
  | the sole argument is a string | line 448 | `a key's sole argument must be a string` | none in the workspace |

- The property arm of the *fourth* node-shape rule is already covered:
  `schema_and_template_failures_are_aggregated_with_source_locations` loads a
  node with `extra=1` and requires
  `properties and child blocks are not allowed`. The **child-block** arm of that
  same rule has no test either, and is a fourth case worth adding while here.
- `a key's sole argument must be a string` is reachable through KDL values the
  crate never mentions: `impl 42`, `impl #true`, `impl #null` all parse and all
  take the `as_string() == None` arm. One of them is enough.
- The two-or-more direction of the argument-count rule is as untested as the
  zero direction; `impl "a" "b"` covers it.

## Done when

- `crates/keyed-launch/tests/templates.rs` names each of the three rules in a
  test whose assertion is the message the rule produces, in the file's existing
  `load_error` + `assert_contains` idiom.
- **No file under `crates/keyed-launch/src/` is touched.** `tests/` is evidence
  rather than a root — no book owns a byte of it — so the corpus freeze is not
  engaged, no fragment ledger moves, and no book needs revalidating.
- If a test fails, the source is right and the test is wrong until proven
  otherwise: this leaf was cut from a reading of the rules, not from an observed
  misbehaviour, and a genuine defect found here becomes its own leaf under the
  root brief's cross-book rule rather than an inline fix.
- `bash scripts/check.sh` passes.

## Notes

**Not a blocker for any book.** Chapter 4 is already accurate — it reports the
gap rather than papering over it — so nothing in
`docs/walkthroughs/keyed-launch/` waits on this. If it lands first, chapter 4's
rules table and its paragraph about the three untested rows both go stale and are
this leaf's to correct in the same commit; the table is at `#the-rules` and the
`Pinned by` column is where the new test names go.

**Placed ahead of `architecture-residue-k75`** with `leaf-insert`, as the root
brief's cross-book rule directs for work found while documenting.

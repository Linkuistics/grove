# ravel-skills-resolve-k62

**Reviews:** ravel-skills-resolve-k54

## Goal

Read the local resolver of Ravel commit `777e609f` adversarially, against
`docs/specs/skill-discovery.md` at `822c0ed2`. Produce findings, not fixes.

## Specific doubts

- The link-chain walk in `crates/ravel/src/skills/snapshot.rs` resolves each
  component inside the canonical source root. Can a chain of relative links
  escape the root, reach a `.git` or `.jj` entry, or loop without refusal? One
  mutation of the `..` check first survived the fixtures.
- The protected-overlap check in `skills/mod.rs` compares canonical and
  lexical forms of `$HOME/.config/ravel`, the settings file and the cache
  root. Is there an alias that escapes it?
- Publication seals the object contents, renames with no replacement and then
  seals the object root. A loser compares only the manifest bytes of the
  winner. Is any state visible to a reader before the object is complete?
  Deep checks of reused objects are `ravel-skill-publication-k56`. Judge only
  what this leaf claims.
- The discovery instructions in `skills/preamble.rs` are this leaf's own
  wording. Do they carry each obligation of "The preamble" and each row of
  "Agent discovery and native installations", with no added rule?
- The strict settings reader in `strict_json.rs` replaces `serde_json` for the
  whole settings file. Does any document that the old reader accepted now
  parse differently, apart from a repeated member name?

## Done when

- Each finding names a file, a line and a concrete failing input, or the review
  states that it found none.

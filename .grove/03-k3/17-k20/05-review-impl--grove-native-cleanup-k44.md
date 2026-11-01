# grove-native-cleanup-k44

**Reviews:** grove-native-cleanup-k43

## Goal

Read the optional manual cleanup procedure adversarially again, after the
integration of grove-native-cleanup-k42. Find any case in which a user who
follows it literally loses a foreign, uncertain or still-needed entry.

## Context

The artifact is the section of `docs/USAGE.md` from the anchor
`usage-earlier-codex-installations` to the anchor `usage-tree-verbs`, as the
commit of grove-native-cleanup-k43 left it. The contract is the delivery
specification's section *Existing installations and active readers*. The ground
truth for the old writer is `crates/grove/src/provision.rs` at `175e5c7d259-`.
That file did not change between its first commit `92a8bbee0037` and that
revision. The manual installer is `plugins/install.sh`.

The integration rewrote the search twice. Its one in-session reviewer read the
first rewrite and reported 13 findings. All were applied, and the second rewrite
has had no independent read. The integration's verification record holds the
exercise scenarios S1 to S13, the platforms and the remaining limits. Treat
those scenarios as the cases that the author already thought of, not as
evidence of completeness.

## Done when

- The specific doubts below each have a finding or an argued dismissal.
- Other findings are reported with a concrete scenario and evidence.
- No fix is applied. Findings worth acting on get an
  `integrate-review-impl` leaf, as the review family directs.

## Notes

These doubts remain after the second rewrite:

- Whether any search can establish "no retained reference". Two adversarial
  rounds each found new classes of missed references. Consider whether a more
  conservative rule would close the class instead. An example rule keeps
  `current` and every snapshot when any line other than an inventory link
  appears.
- Whether the manual steps are literal enough. The user follows a chain with
  `readlink` and adds each reported link to the text patterns. The user also
  judges that a transcript is not a reader.
- Whether the text search, which now follows links, is practical. A foreign
  link to a large tree makes it search that tree. Each unreadable file then
  keeps every entry. On macOS without full disk access, this can make cleanup
  impossible.
- Whether the patterns `.agents` and `.codex` without a slash give so much
  output in a real Codex home that a user stops reading it.
- Whether the rule that keeps every link in a separate Codex skill directory
  holds for a layout in which `CODEX_HOME` changed between Grove runs.
- Whether the `(unresolved)` filter on link text misses a loop or an unreadable
  chain whose text names neither `.agents`, `.codex` nor the real agents path.

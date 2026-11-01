# grove-native-cleanup-k46

**Reviews:** grove-native-cleanup-k45

## Goal

Read the optional manual cleanup procedure adversarially again, after the
integration of grove-native-cleanup-k44. Find any case in which a user who
follows it literally loses a foreign, uncertain or still-needed entry.

## Context

The artifact is the section of `docs/USAGE.md` from the anchor
`usage-earlier-codex-installations` to the anchor `usage-tree-verbs`, as the
commit of grove-native-cleanup-k45 left it. The contract is the delivery
specification's section *Existing installations and active readers*, which
k45 extended by one sentence. The ground truth for the old writer is
`crates/grove/src/provision.rs` at `175e5c7d259-`. The manual installer is
`plugins/install.sh`.

k45 settled a preservation rule before it changed the search. It replaced
manual chain following with lexical rules. Each line of the link search keeps
links by a fixed list. Any line outside the provisioner forms keeps `current`
and every snapshot.

Its one in-session reviewer read the first
k45 revision and reported seven loss findings. All were applied, and the
result has had no independent read. The k45 verification record holds the
scenarios N0 to N5, R1 to R7 and V1 to V7, the platforms and the limits.
Treat them as the cases that the author already thought of.

## Done when

- The specific doubts below each have a finding or an argued dismissal.
- Other findings are reported with a concrete scenario and evidence.
- No fix is applied. Findings worth acting on get an
  `integrate-review-impl` leaf, as the review family directs.

## Notes

These doubts remain after k45:

- Whether the six line rules and the keep-everything conditions together
  prevent every loss inside the searched locations. Look for a line whose
  text has an exact provisioner or skill-link form but whose chain still
  passes another entry.
- Whether `(leaves the search, real path R)` and the instruction to search R
  close the class of dependencies through links outside the roots. Consider
  file links, links that leave and return, and an R too large to search.
- Whether silence for a missing path, a non-directory and a loop is safe
  beyond the stated mount-namespace limit.
- Whether the exemption for `~/.agents` and the skill directories when they
  are links can hide a real alias.
- Whether the strict rules now make cleanup impossible in an ordinary home,
  and whether a narrower procedure would serve the contract better. Examples
  are removing only skill links, or keeping the whole Grove root, which held
  13 MB in one real installation.

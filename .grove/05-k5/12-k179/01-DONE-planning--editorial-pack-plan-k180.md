# editorial-pack-plan-k180

## Goal

Cut the implementation leaves of editorial-pack-k179.

## Done when

- Each Done when of the parent brief belongs to a leaf with a concrete,
  verifiable outcome.
- The leaves follow the expand and contract order of the contract. Each
  handoff leaves Grove's `bash scripts/check.sh` green.

## Decisions (running log)

- The node is one move with three leaves: the expand (k205), the
  contraction (k206) and the directory trial (k207). The first pack commit
  alone leaves the product working, and the pack-alone check proves it. The
  removal needs the second pack commit, because the aggregate fails on
  duplicates until the pack records the removal commit.
- No consumer step comes before the expand, unlike engineering-pack-k177. No
  conformance control copies an editorial kind, and no trial case launches
  one. The only named read is `MOVING_KIND_OPENINGS`, which the removal
  changes. An enumeration of the four kind names, `editorial.md` and
  `static(editorial)` with `--hidden` over the tree found the consumers that
  k206 lists. The same search found the moving files themselves, which is its
  positive control.
- Planning finding: a byte-for-byte copy of `editorial.md` with frontmatter
  alone fails the pack-alone check. A scratch pack under this session's
  scratchpad held the five skills and the eleven moved rows. The runner of the
  Grove line reported "missing file: editorial-family/SKILL.md names
  `references/execute.md`". The family skill loads nothing, so rule 3 of
  "References between skills" finds no skill that holds the path.
- Decision: the moved body says "the spine's" in place of
  "`references/execute.md`'s". With that change, the scratch pack passed
  alone with 11 single-source checks, 44 loaded-path checks and 4 external
  edges. A directed load of `grove` in the family skill and a change of rule 3
  were rejected. k205 holds the reasons. The node brief's Done when now
  permits a recorded body change where a path cannot resolve.
- When the removal empties `MOVING_KIND_OPENINGS`, the positive control of
  `no_skill_file_writes_a_plugin_qualified_name` has no subject. k206 gives it
  another subject and a seen failure.
- The membership row for this boundary owes a static control and a directory
  trial in each harness. It owes no native trial. The parent brief says that
  this node can copy `requirements-native-namespace`, but the research move
  already proved a family skill under a native namespace. No native leaf is
  cut.
- The parent's Done when maps to the leaves as follows. The layout, the family
  skill, the openings, the bare-name check and the first commit belong to
  k205. The spine's list, the README, the glossary and the descriptions belong
  to k206. The kind table, the named set, the rows' removal, the marketplace
  pin, the aggregate and the second commit also belong to k206. The proofs of the boundary belong to
  k206 (static) and k207 (trial). k205 and k206 append to the moves record,
  and k207 links its case from it.
- No in-session reviewer and no `review-planning` leaf. The subtree follows
  the pattern of three finished moves, and the membership specification had
  its review. The planning finding has an executable reading.

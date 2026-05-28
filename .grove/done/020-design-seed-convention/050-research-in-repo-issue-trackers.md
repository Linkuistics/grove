# 050-research-in-repo-issue-trackers

**Kind:** work

## Goal

Survey prior systems that embedded an issue-tracker-lite (or work
queue, or collaborative annotation store) **inside the git repo
alongside the code** and document **what went wrong**. The earlier
research (`docs/research/seed-capture-prior-art.md`) mapped paradigms
to pick a shape; this one is post-mortem-shaped — take the architecture
we have now (inbox files on a dedicated branch, CLI-mediated writes,
optional remote sync) and find out how similar designs fared after
contact with real multi-person, multi-machine, multi-year use. The
output is a focused document of **known failure modes**, with each
attached to a concrete prior system and a short note on how the
failure manifested.

The audience is the three open planning leaves (sync semantics, audit
of LLM/CLI boundaries, TUI) plus the rename leaf — each session should
be able to cite specific prior failures when making design decisions
rather than reasoning in a vacuum.

## Context

- `docs/research/seed-capture-prior-art.md` is the prior survey. Read
  it first to avoid duplication; the new artifact extends it, does
  not replace it. Where the prior survey says "model X exists and
  has properties Y," this one says "model X was used by tool Z and
  ran into problem W."
- ADRs 0002 and 0003 codify the design under audit. The research
  should call out, by name, the design choices most exposed to known
  failure modes (e.g. "dedicated branch is the same shape git-bug
  uses; their main complaint was…").
- The grove project is mid-flight; the research's job is to *de-risk
  forward decisions*, not to second-guess the convention wholesale.
  If it surfaces a fundamental flaw in the in-repo approach, that is
  important; if it surfaces normal-shaped trade-offs, the planning
  leaves handle them.

## Systems to investigate

The list is starting set, not exhaustive. The researching session
should expand it as it goes; conversely, any system that turns out
to be irrelevant after a quick look should be noted and skipped
rather than padding the document.

In-repo tracker / annotation systems with available post-mortems:

- **git-bug** — issues stored under `refs/bugs/*` as JSON objects.
  Pattern most similar to ours. Look for: multi-writer merge
  behaviour, adoption drop-off, tooling fragility, what users say
  drove them away or kept them in.
- **Fossil tickets / wiki** — built into Fossil rather than bolted
  onto git, but the *concept* (issue tracker shipped with the SCM)
  is identical. Different ecosystem; failure modes have a different
  flavour.
- **bugs-everywhere (BE)** — older Python tool; defunct or
  near-defunct. Why?
- **ditz** — Ruby, abandoned. Why?
- **ticgit / ticgit-ng** — abandoned. Why?
- **artemis** — Mercurial-based. Why?
- **git-appraise** — code review notes stored in `refs/notes/...`.
  Different content type (review threads, not issues) but same
  in-repo-collaboration architecture. Status, friction patterns.
- **Radicle COBs** — newer; CRDT-as-git-objects approach. Solves
  some of the merge problems older tools hit. Failure modes (if
  any) are newer and less well-documented; flag what is known.
- **Sapling / Jujutsu** — not issue trackers per se, but their
  treatment of "metadata adjacent to commits" (e.g. jj's
  change-id, sapling's smartlog metadata) carries lessons about
  multi-writer auxiliary data on a code repo.

In-repo file conventions that solved some of the same problems
without bespoke tooling:

- **TODO/FIXME comments in code** — the baseline "track work next
  to code" pattern. Known failure: scale, lack of cross-cutting
  view, no triage workflow. Worth one paragraph as the floor.
- **`docs/` or `notes/` directories with markdown files** —
  e.g. obsidian-style or zk-style note conventions, but checked in.
  Known failure: merge conflicts on shared files, no
  cross-machine sync separate from the code branch.
- **`CHANGELOG.md` / `RELEASE_NOTES.md` conventions** — append-only
  files. Known failure: the merge-conflict-on-append problem;
  Conventional Commits / Changesets / changie evolved partly to
  side-step it. Directly relevant to the inbox-as-single-file
  question.

## Specific questions the research must answer

Each open planning leaf has a question that the research should be
able to inform:

1. **For 060 (sync semantics + shape)**:
   - Of the surveyed tools, which used single-file-per-item vs
     directory-of-files-per-item shapes? What did they say about
     merge conflict frequency under multi-writer?
   - For tools that started single-file and migrated to
     directory-of-files (or vice versa), what triggered the
     migration?
   - For tools that auto-pushed on every write, what went wrong?
     For tools that left push manual, what went wrong differently?
   - Did any tool encounter a "users forgot to pull before drain"
     pathology? How did they address it?
2. **For 080 (LLM/CLI boundary audit)**:
   - For tools whose workflow was prose-coded (in README or docs)
     vs CLI-coded, was there a difference in user error rates?
     [This may not be answerable from public post-mortems; flag
     if so.]
3. **For 090 (TUI)**:
   - Which surveyed tools provided a TUI / interactive view, and
     which did not? Did that affect adoption?
   - Of the tools with a TUI, were there cases where the TUI
     drifted from on-disk state? What caused the drift?
4. **For 070 (rename + init)**:
   - The dedicated-branch architecture is most similar to
     git-bug's `refs/bugs/*` approach. What does the git-bug
     post-mortem say about that choice in particular?
   - Are there any reported pathologies around branch
     materialisation as a worktree (vs hidden refs)?

## Done when

- `docs/research/in-repo-issue-tracker-postmortems.md` exists,
  organised by system (one section each), with each section
  containing: a one-paragraph summary of the system, its
  architecture in one screen of detail, the documented or
  observed failure modes, and a one-line *takeaway for grove*
  pointing at which planning leaf the lesson applies to.
- The document ends with a "Synthesis" section that answers the
  four planning-leaf-specific questions above, citing the
  individual system sections for evidence.
- The document does **not** recommend abandoning the current
  convention unless the research uncovers a genuinely disqualifying
  pathology. If it does, the recommendation is recorded explicitly
  (with citations) and the planning session that picks up after
  this leaf decides what to do with it.
- The parent `BRIEF.md`'s "Pointers" section is updated to add the
  new research document.

## Notes

- **The earlier research is a sibling, not a parent**, of this one.
  Do not rewrite `seed-capture-prior-art.md`; cite it. Where the
  earlier survey already covers a system at the paradigm level,
  the new document extends it with the post-mortem layer.
- **Skip recommendation engines and project-management heavyweights**
  (Jira, Linear, GitHub Issues, Asana). They are not in-repo
  trackers; the earlier survey already established them as the
  baseline-to-beat, not candidates.
- **Bias toward primary sources**: project README "why we
  archived this" notes, retrospectives, last-commit messages on
  abandoned repos, and public threads where former users describe
  why they left. Secondary commentary (blog posts) is fine but
  cite primary sources where they exist.
- **Time-box the search.** This is a survey leaf, not a thesis;
  if a system is obscure enough that 20 minutes of searching turns
  up nothing useful, drop it and note that the search was
  fruitless. The synthesis matters more than completeness.
- **Walk-away-ability check.** As a side product, note for each
  surveyed system whether *its* artifacts would survive deletion
  of its tooling. The grove convention's plain-markdown-on-a-
  branch shape is partly justified by that property; the research
  may surface examples where the opposite choice failed.

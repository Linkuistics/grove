# plugin-skills-k27

**Reviews:** `plugin-skills-k7`

## Goal

Adversarially read the rationalised `linkuistics` corpus against the text it
replaced. The producer rewrote ~7,000 words across nine skills and deleted two;
the failure mode is a **rule that stopped being stated** and is invisible in the
artifact alone — the result reads as a coherent skill either way. Find those, and
find any place the compression left a skill saying something now false.

## Context

The producer's commit is the diff to read; every file below is in
`plugins/linkuistics/skills/`. What it did, so you can check each claim rather
than re-derive the disposition:

| skill | before | after | what it claims to have done |
|---|---|---|---|
| `codebase-design` | 1,536 | 953 | dropped two ASCII diagrams, the *Relationships* list and *Rejected framings*; folded the two useful "avoid" notes into the glossary entries; gained a naming-consistency principle |
| `decision-records` | 1,345 | 935 | dropped *Rejected framings* and shortened *What qualifies* to one prose paragraph |
| `simplify-project` | 1,191 | 1,116 | prose-level only; the *Common mistakes* table was kept deliberately |
| `using-jujutsu` + `git-to-jj-mapping` | 1,908 + 1,421 | 1,120 + 2,172 in `references/` | one skill; body keeps what decides behaviour, references take the command surface and the mapping tables |
| `using-codebase-memory` | 2,609 | 826 + 1,456 in `references/` | body keeps how to make a correct call; reference takes the silent-failure catalogue |
| `cli-tool-design` | 1,403 | 1,675 | softened, not compressed — gained an *Applicability* section; the audit reference is scoped to match |
| `coding-style` (generic) | 295 | deleted | `paths: "**/*"`, so it auto-loaded on every file; content was no-op advice or language-skill overlap |
| six `coding-style-*` | 189–350 | 225–389 | each gained a leading sentence making the repo's own config the authority |

Two of these are **grove's own dependencies**: `content/ADR-FORMAT.md` defers ADR
philosophy to `decision-records`, and `content/SPEC-FORMAT.md` defers "what a seam
is and how to judge one" to `codebase-design`. `content/references/commit.md`
defers the working-copy-as-commit lane to `using-jujutsu`. `plugin-fallback-k9`
audits those deferrals against this text, and `harness-compat-k8` edits the
frontmatter of these same files — which is why this review is sequenced ahead of
both.

The house rules are `plugins/CONTEXT.md`'s and `authoring-conventions`':
description shape (capability + *Use when*, never a workflow), progressive
disclosure (body under ~500 lines, a `references/` file over ~300 lines gets a
TOC, one level deep, never `@path`), and source citation with `UNVERIFIED`.

## Done when

Each of these has been checked against the pre-rewrite text and either cleared or
reported as a finding with `path:line`:

- **No rule was dropped in a compression.** Read the three compressed skills
  sentence by sentence against their predecessors. A sentence that vanished is
  fine only if some surviving sentence still binds the same behaviour; report
  every one that does not.
- **The three grove deferrals still resolve.** The ADR **AND** test and the
  minimum-coherent-set discipline in `decision-records`; the seam definition, the
  deletion test, two-adapters-mean-a-real-seam and the dependency-category table
  in `codebase-design`; the describe-early lane in `using-jujutsu`'s body.
- **The routers put the right half in the body.** For each of the two, ask of
  every rule now in `references/`: could a session that never opens the reference
  make a *wrong* call — not merely a slower one — because that rule is not in the
  body? `using-codebase-memory` is the sharp case: the body claims the guard
  wrapper and the `project` rule are the whole of what a correct call needs.
- **Nothing was lost in the fold.** `git-to-jj-mapping`'s tables and its
  *Still git* note (`submodule`, `lfs`) both survive, and its trigger clause was
  absorbed into `using-jujutsu`'s description rather than deleted with the file.
- **The `coding-style` deletion is honest.** Name anything it uniquely bound that
  no surviving skill now states — including for a language with no
  `coding-style-*` skill (Go, Java, C…), where its deletion means nothing fires.
- **`cli-tool-design`'s softening changes an audit's output.** The *Applicability*
  section either alters which findings a reviewer would raise on a real tool, or
  it is decoration; decide which by walking the audit checklist against one small
  single-purpose CLI in this repo (`plugins/install.sh` is one).
- **The repo-config-first sentences name real files.** Each of the six should
  name configuration that language actually uses, and should demote the house
  defaults rather than merely mention the repo.
- **Every description still triggers.** Compression must not have degraded
  model-invocation routing; frontmatter stays under the 1,024-byte spec limit.
- **The consumer sweep is complete.** `plugins/README.md`, `plugins/CONTEXT.md`,
  `authoring-conventions`' counts (it now says fifteen skills, four with
  `references/`), `CHANGELOG.md`, `.claude-plugin/marketplace.json`, and
  `plugins/install.sh`'s comments.

## Notes

- Cut the `integrate-review-impl` leaf only if there are findings worth acting on
  (`decompose.md`). A clean read creates nothing.
- The producer verified the repo-wide markdown-link test and the full `cargo test`
  suite green; no Rust changed. Re-running is cheap but is not the point of this
  leaf — the defects this review is for are ones no test can see.
- Word counts are means, not the deliverable. A skill that is shorter and now says
  the wrong thing is the failure this leaf exists to catch; a skill that stayed
  long because every sentence still binds is not a finding.

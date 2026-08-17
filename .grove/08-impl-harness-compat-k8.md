# harness-compat-k8

## Goal

Add harness compatibility metadata to the bundled skills and make installation
honour it, so Claude-only or harness-specific skills — and personal model or
profile assumptions — are not installed blindly into every harness.

## Context

`plugins/install.sh` today symlinks **every** `linkuistics` skill into
`~/.codex/skills`, `~/.gemini/skills` and `~/.pi/agent/skills` with no filter, and
`testanyware` ships by marketplace only. That is wrong in two distinct ways, and
both need covering:

1. **Harness-specific skills.** A skill whose content assumes Claude Code
   affordances — its tool names, its hook model, its slash commands, its skill
   invocation mechanism — installed into Codex or Pi is instructions a session
   cannot follow. `SKILL.md` is portable in principle, but not every skill is
   portable in fact.
2. **Personal model and profile assumptions.** A skill naming a specific model, a
   specific subscription, or a personal configuration profile is not shareable
   even where the harness matches.

The metadata design is the work. It needs to be declarative, live with the skill,
and be readable by the installer without parsing prose. Frontmatter is the
obvious carrier — the skills context already has `disable-model-invocation` and
`paths:` as precedent — but check whether every target harness tolerates an
unknown frontmatter key before choosing it.

Decide the **default** deliberately: absent metadata should mean the safe thing.
Which of "install everywhere" and "install to Claude Code only" is safe depends on
how many skills are genuinely portable, so measure before choosing rather than
picking the conservative-sounding one.

## Done when

- A metadata schema exists, is documented in `plugins/CONTEXT.md` or the skills
  authoring conventions, and covers both harness compatibility and personal
  assumptions.
- Every bundled skill in `plugins/linkuistics/` and `plugins/testanyware/` carries
  it, assessed individually rather than stamped uniformly.
- `plugins/install.sh` filters on it, and reports what it skipped and why —
  silence about a skipped skill is how a user concludes the install is broken.
- The behaviour on absent metadata is defined and tested, and the choice is
  justified by what was measured.
- The script still handles its existing concerns: the workspace guard, the
  jj-first repository resolution, and re-linking on re-run.
- Removing a skill's eligibility for a harness removes its existing symlink,
  rather than leaving it installed from a previous run.

## Notes

- This leaf is in the **skills** bounded context. Terms it settles belong in
  `plugins/CONTEXT.md`, not the root glossary.
- `grove`'s own provisioning is a **different** mechanism and is out of scope: the
  binary sweeps its embedded `content/` into every installed harness's skill
  directory deliberately, because an opaque configured command cannot be traced to
  one harness. Do not "fix" that sweep to match this filter — they answer
  different questions, and the reasoning for the sweep is in
  `docs/adr/skill-delivers-the-methodology.md`.
- The install script is the seam; test through it rather than through its
  internals. It already has a `.test.sh` neighbour pattern in the repo
  (`guardrail/scripts/guardrail-hook.test.sh`) if a shell-level test fits better
  than a Rust one.

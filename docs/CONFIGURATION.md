# Grove Configuration

Grove selects a primary agent harness for the workstream, then may route each
task kind to another harness and model. Configuration is read at launch time,
so changing an environment variable affects the next session without changing
the task tree.

## Harnesses

| Harness | Repository marker | Executable | Model argument | Provisioned skill directory |
|---|---|---|---|---|
| Claude Code | `.claude/` | `claude` | `--model VALUE` | `~/.claude/skills/grove/` |
| Codex | `.codex/` | `codex` | `--profile VALUE` | `~/.codex/skills/grove/` |
| Pi | `.pi/` | `pi` | `--model VALUE` | `~/.pi/agent/skills/grove/` |

The executable must be on `PATH`. Pi normally needs an explicit
`grove do --harness pi` because Pi does not create a repository-local `.pi/`
marker itself.

For Codex, each new working tree must be trusted. Run `codex` once in that tree
and accept its trust prompt, or configure `trust_level = "trusted"` for the path
in `$CODEX_HOME/config.toml`. Grove checks this before launch because an
untrusted, read-only Codex sandbox cannot use the VCS-store access that Grove
adds for commits.

## Primary harness and `.grove-stamps/`

The primary harness is resolved in this order:

1. `--harness claude|codex|pi`
2. `.grove-stamps/<grove-name>` in the repository root
3. the only detected repository marker
4. otherwise, an error asking for `--harness`

The grove name is the working-tree directory's basename. `grove do` writes a
stamp when `--harness` is explicit or when a repository has multiple harness
markers. A single unambiguous auto-detected harness needs no stamp.
`grove retire --harness` is intentionally one-session-only and never rewrites
the binding.

`.grove-stamps/` is therefore live local configuration, not history or a build
artifact. It is ignored by Git because the binding is machine-local. A stale
stamp can be deleted safely when its working tree no longer exists; the next
`grove do` will require an explicit or unambiguous harness again.

## Task kinds and families

Every live leaf declares one of seventeen kinds:

```text
requirements  design  planning  prototype  impl
review-requirements  review-design  review-planning  review-prototype  review-impl
integrate-review-requirements  integrate-review-design
integrate-review-planning  integrate-review-prototype  integrate-review-impl
research  combine-research
```

The five `review-*` kinds form the `review` family; the five
`integrate-review-*` kinds form the `integrate-review` family. Environment
suffixes uppercase labels and replace `-` with `_`, so `review-impl` becomes
`REVIEW_IMPL` and `integrate-review` becomes `INTEGRATE_REVIEW`.

The disciplines and composition rules behind these names are in
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

## Route a task to a harness

Most specific wins:

| Source | Scope |
|---|---|
| `**Harness:** NAME` in the leaf | This leaf only. An unknown or empty name is an error. |
| `GROVE_<KIND>_HARNESS` | One kind, such as `GROVE_REVIEW_IMPL_HARNESS=codex`. |
| `GROVE_<FAMILY>_HARNESS` | A family, such as `GROVE_REVIEW_HARNESS=codex`. |
| unset | The grove's primary stamped or detected harness. |

Per-leaf routing exists mainly for a research vendor pair: both leaves have
kind `research`, but each must run on a different harness. Ordinary routing
belongs in environment policy rather than task files.

## Select a model

Model lookup is harness-major, in this order:

| Precedence | Variable | Example |
|---:|---|---|
| 1 | `GROVE_<HARNESS>_<KIND>_MODEL` | `GROVE_CODEX_REVIEW_IMPL_MODEL` |
| 2 | `GROVE_<HARNESS>_<FAMILY>_MODEL` | `GROVE_CODEX_REVIEW_MODEL` |
| 3 | `GROVE_<KIND>_MODEL` | `GROVE_IMPL_MODEL` |
| 4 | `GROVE_<FAMILY>_MODEL` | `GROVE_INTEGRATE_REVIEW_MODEL` |

`<HARNESS>` is `CLAUDE`, `CODEX`, or `PI`. A task rerouted away from the
primary harness consults only the harness-scoped forms (1 and 2); an unscoped
model may have been written for the primary harness and can be invalid on the
other one.

A model is required for every launched leaf. Grove does not silently accept a
harness default. The only exemptions are a finish-cycle launch with no live
leaf and a future harness whose registry entry has no model flag. A brand-new
grove is routed as `requirements` by construction, so it needs
`GROVE_REQUIREMENTS_MODEL` or the harness-scoped equivalent before the first
`grove do`.

Claude and Pi values are model names accepted by their `--model` flags. Codex
values are profile names accepted by `--profile`; define a profile in
`$CODEX_HOME/<name>.config.toml` so it can bind both model and reasoning effort.
An in-session model switch overrides the launch value for that session only.

## Review target diversity

Grove owns the target of every scheduled `review-*` leaf through the same
harness and model policy above. A finishing producer records its actual launch
harness and exact model selector best-effort in the related review task. The
structured `kind --with-harness --json` peek validates that receipt under the
tree guard and nests the historical route beneath `producer-target`; the driver
retains it without a second metadata read. A direct receipt's source session is
the producer. A decomposed receipt may name the factual closing leaf separately,
plus a producer generation that changes on supported reopen. Review launch
resolves current policy again and warns unless both its harness and model
selector differ from the producer's. A harness-managed default compares equal
only to another default on the same harness.

This check is **advisory**. A matching axis, missing receipt, malformed stable
relationship, or unavailable historical target produces one warning in stderr
and the launched prompt but never changes or blocks the resolved command. A
one-harness installation therefore warns on every review by design. The notice
names a distinct validated source session and says it applies only when the
session's factual pick is the addressed review; a preempted session discards it.

`GROVE_SESSION_TARGET` carries the retained structured routing peek from the
loop driver to its foreground session so retirement can write that receipt. It
is reserved internal context, not user configuration: do not set or export it.
Auxiliary and nested harness spawns scrub it before launching.

## Example

This configuration keeps Claude as the primary harness, sends all reviews to a
Codex profile, and returns integration to Claude:

```sh
export GROVE_REVIEW_HARNESS=codex
export GROVE_CODEX_REVIEW_MODEL=sol-xhigh

export GROVE_REQUIREMENTS_MODEL=opus
export GROVE_DESIGN_MODEL=opus
export GROVE_PLANNING_MODEL=opus
export GROVE_PROTOTYPE_MODEL=sonnet
export GROVE_IMPL_MODEL=sonnet
export GROVE_RESEARCH_MODEL=opus
export GROVE_COMBINE_RESEARCH_MODEL=opus
export GROVE_INTEGRATE_REVIEW_MODEL=opus

grove do --harness claude
```

If the same shell drives groves with different primary harnesses, prefer the
harness-scoped model variables everywhere.

## Maintainer and diagnostic overrides

These are test, wrapper, or operational seams rather than normal routing:

| Variable | Meaning |
|---|---|
| `GROVE_HARNESS_BIN` | Override the primary harness executable. Ignored for rerouted tasks. |
| `GROVE_HARNESS_BIN_<HARNESS>` | Override one harness executable, including rerouted tasks. |
| `GROVE_LLM_BIN` | Override the internal `grove-llm` executable. |
| `GROVE_SKILL_DIR` | Provision and read the embedded Grove skill at one explicit path. |
| `GROVE_KILL_GRACE` | Seconds between a completion signal and `SIGTERM`. |
| `GROVE_KILL_GRACE_KILL` | Seconds between `SIGTERM` and `SIGKILL`. |

Grace values are clamped to 0–3600 seconds; invalid or non-finite values use
the built-in defaults. `GROVE_SIGNAL_FILE` and `GROVE_SESSION_TARGET` are
internal loop channels, not user configuration, and should not be exported
manually.

`GROVE_SIGNAL_FILE` also carries the launch's finish-attempt identity, which
`grove-llm finish-commit` writes into its teardown commit and requires again to
verify a lost result. A hand-set value that is not a real loop signal path fails
the finish outright; a stale one from another launch makes a retry unable to
recognise its own commit. Let the driver set it.

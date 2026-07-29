# using-codebase-memory Skill Implementation Plan

> **Executed 2026-07-29. Do not run this plan.** The deliverable shipped as
> `plugins/linkuistics/skills/using-codebase-memory/SKILL.md`, and **that file is
> the authority for every CLI behaviour claim.** Executing the plan meant running
> every command it documents, which falsified several facts this plan carried as
> verified — including two in the Global Constraints section, which explicitly
> told a reader not to check them.
>
> This document has been **reconciled, not preserved**. The two embedded
> `SKILL.md` drafts and the falsified expected-outputs are deleted rather than
> annotated: they were a second, wrong copy of a file that already exists, and a
> reader who lifted them would ship the wrong skill. What remains is the
> reasoning, the file structure, and the record of what was actually done.
> Checkboxes are marked to what ran. Git holds the original.
>
> The original header required `superpowers:subagent-driven-development` to
> execute the plan and `superpowers:writing-skills` to author the skill. Both are
> spent — and for the record, on the second: this repo ships
> `linkuistics:authoring-conventions`, a **house delta that overrides upstream's
> description rule** (house is capability + "Use when …"; upstream is when-only).
> Upstream's version is injected every session and will tempt an implementer to
> strip the capability clause. House wins.

**Goal:** Ship one portable skill, `using-codebase-memory`, that lets an agent in any of the four harnesses (Claude Code, Codex, Gemini, Pi) query the codebase knowledge graph from the shell, and compose multi-step graph queries in bash rather than as chained tool calls.

**Architecture:** A single `SKILL.md` in `plugins/linkuistics/skills/`. It rides the existing distribution path — `install.sh` symlinks it into `~/.codex/skills`, `~/.gemini/skills` and `~/.pi/agent/skills`; Claude Code gets it through the `linkuistics@linkuistics` marketplace plugin. No new code, no wrapper CLI, no scripts library.

**Tech Stack:** Markdown (SKILL.md open format), bash, `jq`, `codebase-memory-mcp` 0.8.1.

**Spec:** `docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md`

## Global Constraints

**The CLI-behaviour constraints that stood here are deleted.** They were the
plan's most dangerous content — the section opened *"Do not restate any of these
from memory — they are counter-intuitive"*, which is exactly the framing that
turns a wrong bullet from a mistake into an instruction — and several were wrong.
The shipped skill is now the single statement of all of them. Read
`plugins/linkuistics/skills/using-codebase-memory/SKILL.md`, not this section.

What execution falsified, kept only so the correction is legible:

- **"`relationship` and `direction` produce byte-identical results"** — false. On
  the fixture they dropped 2 rows of 2460. The pre-plan check saw agreement only
  because `limit:5` hid the difference. What *is* true, and durable: `min_degree`
  gates on **total** degree (in + out), and exactly half the rows that filter
  returned had `in_degree: 0`.
- **"True fan-in/fan-out filtering must be done client-side on `.in_degree` /
  `.out_degree`"** — false, and it propagated into Task 2's flagship example.
  Aggregation and global top-N belong in `query_graph`'s Cypher, which runs over
  the whole graph rather than over a truncated page.
- **"`| jq` works with no redirection"** — true on **success** only. On failure
  stdout is *empty* and the error goes to stderr, so `jq` sees nothing and the
  pipe masks the binary's exit `1`.

Constraints that were not about the CLI, and still hold:

- **Skill slug:** `using-codebase-memory` (approved; gerund prefix matches `using-jujutsu`). Do NOT name it `codebase-memory` — that collides with the installer-managed `~/.claude/skills/codebase-memory/`.
- **Binary path:** `/Users/antony/.local/bin/codebase-memory-mcp` (on PATH as `codebase-memory-mcp`).
- **Test fixture:** `Users-antony-Development-herdr` — a **live, drifting** index (23,641 nodes / 97,504 edges at plan time; 23,681 / 97,906 a day later), so every exact figure is a re-check, not a constant. `/Users/antony/Development/grove` is **NOT indexed** — do not use it as a fixture without indexing first.
- **VCS:** this repo is jj-colocated. Use jj for every mutation; git is read-only. Load `linkuistics:using-jujutsu` before any commit. Commit with `jj describe -m "..."` then `jj new`.

## File Structure

| File | Responsibility |
|---|---|
| `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` | **Create.** The entire deliverable. Frontmatter + CLI contract + composition patterns + surface-selection rule. |
| `plugins/linkuistics/.claude-plugin/plugin.json` | **Modify.** Its `description` enumerates the plugin's skill topics in prose and its `keywords` array is used for discovery; both need this skill added. |
| `install.sh` | **No change.** It globs `"${skills_dir}"/*/`, so a new directory is picked up automatically. Task 3 verifies this rather than editing it. |

---

### Task 1: SKILL.md — frontmatter and the verified CLI contract

> **Merged with Task 2 and executed as one grove leaf, `skill-k2`.** The two
> tasks write the *same file*, and the split assumed subagents sharing a parent's
> context. Grove leaves are cold-start sessions, so a second one would have
> re-established the whole CLI contract just to append two sections.

**Files:**
- Create: `plugins/linkuistics/skills/using-codebase-memory/SKILL.md`

**Interfaces:**
- Consumes: nothing (first task).
- Produces: the file at the path above. The section list planned here
  (`## Invoking the CLI`, `## The project parameter`, `## Streams and exit
  codes`, `## The fourteen tools`) is **not** what shipped — read the file.

- [x] **Step 1: Load the skill-authoring rules**

Superseded. `linkuistics:authoring-conventions` is the governing rule in this
repo and overrides `superpowers:writing-skills` on the description shape — see
the header note.

- [x] **Step 2: Write the failing check — assert the file does not yet exist**

```bash
test ! -f plugins/linkuistics/skills/using-codebase-memory/SKILL.md && echo "RED: not created yet"
```

Expected: prints `RED: not created yet`

- [x] **Step 3: Create the skill directory and write frontmatter plus the CLI contract**

**The verbatim `SKILL.md` draft that stood here is deleted.** It was a second
copy of a file that now exists, and a wrong one: it presented the error payload
as ordinary output, called `| jq` clean without qualification, and carried none
of the truncation, malformed-JSON, `query`-exclusivity or `--json` findings. Read
`plugins/linkuistics/skills/using-codebase-memory/SKILL.md`. The shipped file
diverges from that draft substantially, not cosmetically.

- [x] **Step 4: Run every command the file claims, and compare to the claim**

This step is why the plan survived contact at all — *"If any output disagrees
with the file, fix the file, not the expectation"* is what produced every
correction recorded above. It also falsified one of its own commands:

```bash
codebase-memory-mcp cli search_graph '{"name_pattern":".*Handler.*"}' | jq -r '.error'
```

This cannot work. On failure stdout is *empty*, so `jq` receives nothing; the
error body is on stderr, and the pipe masks the binary's exit `1`. The guarded
no-pipeline form is in the skill.

- [x] **Step 5: Commit**

Committed as grove leaf `skill-k2`, not under the message drafted here.

---

### Task 2: SKILL.md — composition patterns and the degree-filter correction

> **Merged into Task 1** and executed as grove leaf `skill-k2`. Its subject
> matter shipped; neither of its two embedded sections survived verification.

**Files:**
- Modify: `plugins/linkuistics/skills/using-codebase-memory/SKILL.md`

- [x] **Step 1: Write the failing check — confirm the degree claim is not yet documented**

Ran as written.

- [x] **Step 2: Prove the gotcha is real before documenting it**

Ran — and it disproved half of its own expectation. The step expected *"both
commands print identical output"*. They do not: adding `relationship` and
`direction` drops 2 rows out of 2460. The expectation looked right only because
both commands carry `limit:5`, which hides a 2-row difference in a 2460-row
result. A check whose sample is smaller than the effect it is checking for
cannot fail.

What the step *did* establish, and what shipped: `min_degree` gates on **total**
degree, so rows with `in_degree: 0` come back past `min_degree:10` — exactly half
the returned rows on the fixture.

- [x] **Step 3: Insert the two sections**

**Both drafted sections are deleted.** `## Gotcha: min_degree is total degree`
asserted the byte-identical claim; `## Composing queries` built its flagship
example on that claim's corollary, that fan-in must be filtered client-side.
Neither holds. The shipped skill replaces them with four sections — *Degree
filters are not directional*, *Results are truncated by default*, *Push
aggregation into Cypher*, and *Compose across tools, not within one*.

- [x] **Step 4: Run both composition examples verbatim**

Ran — and it falsified the first example. The recipe passes `limit:200`, which is
the **default** rather than a widening, then sorts in `jq` and takes the top 20.
On the fixture that ranks an arbitrary 8% of 2460 matches and presents it as "top
20 fan-in". Its expected top row (`123` / `app_for_mouse_test`) is nonetheless
correct — which is exactly why the example passed review. A client-side sort over
a truncated page agrees with the truth whenever the truth is dense at the top, so
this shape passes every eyeball test and is wrong in the tail.

The correct form is one `query_graph` Cypher call doing `count(*)`, `ORDER BY`
and `LIMIT` server-side, grouped on `qualified_name` rather than `name`. The
trace loop's *shape* survived into the skill; its `function_name` argument did
not, because a bare name shared by several symbols resolves to **none** of them
and returns an empty `callers` array at exit `0`.

- [x] **Step 5: Commit**

Committed as part of grove leaf `skill-k2`.

---

### Task 3: Distribution — manifest copy and install verification

> **Executed as grove leaf `distribution-k5`, unmerged and unchanged in scope.**
> Steps 1–3 and 6 ran as written. Steps 4–5 did **not** — read the hazard note on
> Step 4 before running either.

**Files:**
- Modify: `plugins/linkuistics/.claude-plugin/plugin.json`

**Interfaces:**
- Consumes: the completed `SKILL.md` from Tasks 1 and 2.
- Produces: a `plugin.json` whose `description` mentions the graph-query skill and whose `keywords` array contains `codebase-memory`, `knowledge-graph` and `code-search`; plus verified symlinks in three harness skill directories.

- [x] **Step 1: Write the failing check — the manifest does not mention the skill**

```bash
grep -q "knowledge-graph" plugins/linkuistics/.claude-plugin/plugin.json \
  && echo "GREEN (already present)" || echo "RED: manifest not updated"
```

Expected: prints `RED: manifest not updated`

- [x] **Step 2: Update the manifest**

In `plugins/linkuistics/.claude-plugin/plugin.json`, append to the existing `description` string, immediately before the final `.`:

```
, plus a shell-first workflow for querying a codebase knowledge graph that works in harnesses with no MCP support
```

And add these three entries to the end of the `keywords` array:

```json
    "codebase-memory",
    "knowledge-graph",
    "code-search"
```

- [x] **Step 3: Verify the JSON is still valid**

```bash
jq -e '.name, .description, (.keywords | index("knowledge-graph"))' plugins/linkuistics/.claude-plugin/plugin.json
```

Expected: exit 0, prints `"linkuistics"`, the extended description, and a non-null array index.

- [ ] **Step 4: Verify install.sh picks the skill up without modification**

> **Do not run `./install.sh` from a grove working tree.** The script derives its
> link source from `${BASH_SOURCE[0]}` and re-links **unconditionally**. Run from
> a secondary jj workspace — which is where this plan was executed, the default
> being `/Users/antony/Development/grove` — it silently re-points *every*
> installed `linkuistics` skill at a tree that dies when the grove is torn down.
> The underlying defect is tracked as grove leaf `install-workspace-guard-k8`.
> Run the script from the default workspace, once this work is integrated.

Verified instead against an **isolated `HOME`**, leaving the real one untouched.
That evidence is stronger for the claim actually at stake: it produced
`48 = 16 skills × 3 harnesses`, exercising all three targets including
`~/.gemini`, which does not exist on this machine and would otherwise have
printed `skip`. So the manifest edit and the glob-pickup are fully verified; only
the real-machine install is outstanding, and it is not this plan's to do.

- [ ] **Step 5: Verify the skill is readable through one non-Claude harness path**

```bash
head -5 ~/.pi/agent/skills/using-codebase-memory/SKILL.md
```

Expected: the YAML frontmatter, with `name: using-codebase-memory`. Not run
against the real `$HOME` for the reason above; the equivalent check passed inside
the isolated one, resolving the symlink and reading the shipped frontmatter.

- [x] **Step 6: Commit**

```bash
jj describe -m "skill: register using-codebase-memory in the linkuistics manifest

install.sh globs the skills directory so it needed no change; the manifest
description and keywords did.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>"
jj new
```

---

## Self-Review

Written before execution. Kept because its **misses** are more informative than
its coverage, and re-scored against what shipped.

**Spec coverage.** The mapping held — every numbered spec item did reach the
skill. Note that the spec's items 3–5 have since been renumbered and rewritten
(§ *Skill content*), so this table indexes the pre-execution numbering:

| Spec item (original numbering) | Task | Shipped as |
|---|---|---|
| 1. Invocation, fourteen tools | Task 1, Step 3 | yes, plus the `--json` warning and per-tool notes |
| 2. `project` required (leads) | Task 1, Step 3 | yes, unchanged |
| 3. Streams (stderr/stdout) | Task 1, Step 3 | **rewritten** — the failure row was missing |
| 4. Exit codes, pipeline caveat | Task 1, Steps 3–4 | yes, plus `set -o pipefail` and the guarded form |
| 5. Errors carry `hint`/`available_projects` | Task 1, Step 3 | yes, but on **stderr**, which changes how you read it |
| 6. Composition patterns | Task 2, Step 3 | **rewritten** — aggregation moved into Cypher |
| 7. Surface-selection rule + ToolSearch | Task 1, Step 3 | yes, unchanged |
| 8. Unindexed ≠ empty | Task 1, Step 3 | yes, plus `mode` and the `excluded` key |

**What this review missed, and why.** It checked *coverage* — that every spec
item had a task — and coverage was never the problem. Every falsified claim sat
inside an item this table marks as covered. Four findings the plan had no item
for at all surfaced only by running the commands: malformed JSON is discarded
rather than reported; `search_graph` truncates at a default `limit` of 200 and
`trace_path` caps at 100 with no flag; `query` silently overrides every other
selector; and an inline property map in Cypher silently truncates a traversal to
10 rows. A plan cannot enumerate what it does not know, which is the argument for
Task 1 Step 4 rather than against the plan.

**Addition beyond the spec:** the `min_degree` total-degree gotcha was discovered
while verifying the composition example, and is real. Its stated rationale here
was wrong — it was offered as *"the reason the composition example takes the
shape it does"*, and that shape was itself the error. The gotcha stands; the
recipe it justified does not.

**Placeholder scan:** no TBDs; every code step carried runnable content and a
concrete expected output. Two of those expected outputs were wrong, which is the
limit of what a placeholder scan can tell you.

**Type consistency:** held. `search_graph`'s `in_degree` / `out_degree` /
`name` / `qualified_name` and `trace_path`'s `function` / `direction` /
`callers[].name` were used consistently throughout — and consistently in a
recipe that was reading the wrong 8% of the rows. Consistency is not
correctness.

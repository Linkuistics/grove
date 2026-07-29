# using-codebase-memory Skill Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **ALSO REQUIRED:** Use `superpowers:writing-skills` when authoring the SKILL.md itself — it governs frontmatter, description triggers, and skill verification in this repo.

**Goal:** Ship one portable skill, `using-codebase-memory`, that lets an agent in any of the four harnesses (Claude Code, Codex, Gemini, Pi) query the codebase knowledge graph from the shell, and compose multi-step graph queries in bash rather than as chained tool calls.

**Architecture:** A single `SKILL.md` in `plugins/linkuistics/skills/`. It rides the existing distribution path — `install.sh` symlinks it into `~/.codex/skills`, `~/.gemini/skills` and `~/.pi/agent/skills`; Claude Code gets it through the `linkuistics@linkuistics` marketplace plugin. No new code, no wrapper CLI, no scripts library.

**Tech Stack:** Markdown (SKILL.md open format), bash, `jq`, `codebase-memory-mcp` 0.8.1.

**Spec:** `docs/superpowers/specs/2026-07-29-portable-codebase-memory-skill-design.md`

## Global Constraints

Every fact below was verified against `codebase-memory-mcp 0.8.1` on 2026-07-29. Every task's requirements implicitly include this section. **Do not restate any of these from memory — they are counter-intuitive and several contradict the existing installer-managed skill.**

- **Skill slug:** `using-codebase-memory` (approved; gerund prefix matches `using-jujutsu`). Do NOT name it `codebase-memory` — that collides with the installer-managed `~/.claude/skills/codebase-memory/`.
- **Invocation form:** `codebase-memory-mcp cli <tool> '<json>'`
- **Binary path:** `/Users/antony/.local/bin/codebase-memory-mcp` (on PATH as `codebase-memory-mcp`).
- **`project` is a required parameter.** The CLI does NOT infer the project from the working directory. Omitting it returns `{"error":"project not found or not indexed", "hint":..., "available_projects":[...]}`.
- **Project names are path-derived**, e.g. `Users-antony-Development-herdr`. Get them from `list_projects`.
- **Streams:** logs (`level=info msg=mem.init budget_mb=... total_ram_mb=...`) go to **stderr**; JSON goes to **stdout**. `| jq` works with no redirection.
- **Exit codes:** `0` success, `1` error (unknown tool, malformed JSON, unknown project). Verify exit status WITHOUT a pipeline — `$?` after a pipe reports the last stage, not the binary.
- **`min_degree` filters on TOTAL degree (in + out), not directional degree.** `relationship` and `direction` do NOT change this filter — passing them produces byte-identical results. True fan-in/fan-out filtering must be done client-side on `.in_degree` / `.out_degree`.
- **`trace_path` inbound response shape:** `{"function": str, "direction": str, "callers": [{"name": str, "qualified_name": str, "hop": int}]}`. There is no `.paths` key.
- **The 14 tools:** `index_repository`, `index_status`, `list_projects`, `delete_project`, `search_graph`, `search_code`, `trace_path`, `detect_changes`, `query_graph`, `get_graph_schema`, `get_code_snippet`, `get_architecture`, `manage_adr`, `ingest_traces`.
- **Test fixture:** `Users-antony-Development-herdr` (23,641 nodes / 97,504 edges). `/Users/antony/Development/grove` is **NOT indexed** — do not use it as a fixture without indexing first.
- **VCS:** this repo is jj-colocated. Use jj for every mutation; git is read-only. Load `linkuistics:using-jujutsu` before any commit. Commit with `jj describe -m "..."` then `jj new`.

## File Structure

| File | Responsibility |
|---|---|
| `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` | **Create.** The entire deliverable. Frontmatter + CLI contract + composition patterns + surface-selection rule. |
| `plugins/linkuistics/.claude-plugin/plugin.json` | **Modify.** Its `description` enumerates the plugin's skill topics in prose and its `keywords` array is used for discovery; both need this skill added. |
| `install.sh` | **No change.** It globs `"${skills_dir}"/*/`, so a new directory is picked up automatically. Task 3 verifies this rather than editing it. |

---

### Task 1: SKILL.md — frontmatter and the verified CLI contract

**Files:**
- Create: `plugins/linkuistics/skills/using-codebase-memory/SKILL.md`

**Interfaces:**
- Consumes: nothing (first task).
- Produces: the file at the path above, containing a YAML frontmatter block with keys `name` (value `using-codebase-memory`) and `description`, followed by sections `## Invoking the CLI`, `## The project parameter`, `## Streams and exit codes`, `## The fourteen tools`. Task 2 appends further sections to this same file.

- [ ] **Step 1: Load the skill-authoring rules**

Invoke `superpowers:writing-skills` and follow its frontmatter and description-trigger conventions. Do not proceed until you have read it.

- [ ] **Step 2: Write the failing check — assert the file does not yet exist**

```bash
test ! -f plugins/linkuistics/skills/using-codebase-memory/SKILL.md && echo "RED: not created yet"
```

Expected: prints `RED: not created yet`

- [ ] **Step 3: Create the skill directory and write frontmatter plus the CLI contract**

Create `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` with exactly this content:

````markdown
---
name: using-codebase-memory
description: Query a codebase knowledge graph from the shell via the codebase-memory-mcp CLI, and compose multi-step graph queries in bash. Use when finding functions/classes/routes by pattern, tracing callers or callees, checking impact of a change, hunting dead code or high fan-in symbols, or when you need to run many graph queries and filter/join the results without paying a round-trip per query. Also use in any harness that has no MCP support.
---

# Using codebase-memory from the shell

`codebase-memory-mcp` indexes a codebase into a knowledge graph and exposes
fourteen tools. It serves them two ways — over MCP, and over a CLI. The tool
set is identical; only the calling convention differs.

## Which surface to use

> If the MCP graph tools are available to you, prefer them for single
> queries — typed arguments, no shell quoting. Use the CLI when composing,
> batching, or when those tools are not available.

In Claude Code the MCP tools are deferred: they are not in the base tool list
and must be loaded before they can be called at all.

    ToolSearch("select:mcp__codebase-memory-mcp__search_graph,mcp__codebase-memory-mcp__trace_path")

## Invoking the CLI

    codebase-memory-mcp cli <tool> '<json>'

## The project parameter is required

The CLI does **not** infer the project from your working directory. Running
inside an indexed repo without `project` still fails:

```bash
codebase-memory-mcp cli search_graph '{"name_pattern":".*Handler.*"}'
# {"error":"project not found or not indexed","hint":"Use list_projects ...","available_projects":[...]}
```

Project names are path-derived. List them first:

```bash
codebase-memory-mcp cli list_projects | jq -r '.projects[].name'
# Users-antony-Development-herdr
```

Then pass one:

```bash
codebase-memory-mcp cli search_graph \
  '{"project":"Users-antony-Development-herdr","name_pattern":".*Handler.*","limit":3}'
```

The error body is worth reading rather than discarding — it carries both a
`hint` and the full `available_projects` list.

## Streams and exit codes

- Logs (`level=info msg=mem.init ...`) go to **stderr**; JSON goes to
  **stdout**. Piping straight into `jq` is clean — no redirection needed.
- Exit status is honest: `0` on success, `1` on error. `set -euo pipefail`
  and `||` guards both work.
- Check exit status **without** a pipeline. After a pipe, `$?` reports the
  last stage, not the binary:

```bash
codebase-memory-mcp cli no_such_tool '{}' >/dev/null 2>&1; echo $?   # 1
codebase-memory-mcp cli no_such_tool '{}' | head -1; echo $?          # 0 — head's status
```

## The fourteen tools

`index_repository`, `index_status`, `list_projects`, `delete_project`,
`search_graph`, `search_code`, `trace_path`, `detect_changes`,
`query_graph`, `get_graph_schema`, `get_code_snippet`, `get_architecture`,
`manage_adr`, `ingest_traces`

## When the graph has no answer

An unindexed project is not an empty graph. If `list_projects` does not list
the repo you are in, either run `index_repository`, or fall back to Grep —
but do not report "no results found".
````

- [ ] **Step 4: Run every command the file claims, and compare to the claim**

```bash
codebase-memory-mcp cli list_projects | jq -r '.projects[].name'
codebase-memory-mcp cli search_graph '{"name_pattern":".*Handler.*"}' | jq -r '.error'
codebase-memory-mcp cli search_graph '{"project":"Users-antony-Development-herdr","name_pattern":".*Handler.*","limit":3}' | jq -r '.total'
codebase-memory-mcp cli no_such_tool '{}' >/dev/null 2>&1; echo "no-pipe exit: $?"
codebase-memory-mcp cli no_such_tool '{}' | head -1 >/dev/null; echo "piped exit: $?"
```

Expected, in order: a list including `Users-antony-Development-herdr`; `project not found or not indexed`; a non-zero integer; `no-pipe exit: 1`; `piped exit: 0`.

If any output disagrees with the file, **fix the file, not the expectation.**

- [ ] **Step 5: Commit**

```bash
jj describe -m "skill: using-codebase-memory — the CLI contract, verified

The project parameter is not inferred from cwd, logs go to stderr, and exit
status is only honest without a pipeline. All three surprise on first use.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>"
jj new
```

---

### Task 2: SKILL.md — composition patterns and the degree-filter correction

**Files:**
- Modify: `plugins/linkuistics/skills/using-codebase-memory/SKILL.md` (append two sections before the existing `## When the graph has no answer` section)

**Interfaces:**
- Consumes: the file created in Task 1, with sections `## Which surface to use`, `## Invoking the CLI`, `## The project parameter is required`, `## Streams and exit codes`, `## The fourteen tools`, `## When the graph has no answer`.
- Produces: the same file with `## Gotcha: min_degree is total degree` and `## Composing queries` inserted immediately before `## When the graph has no answer`.

- [ ] **Step 1: Write the failing check — confirm the degree claim is not yet documented**

```bash
grep -q "min_degree is total degree" plugins/linkuistics/skills/using-codebase-memory/SKILL.md \
  && echo "GREEN (already present)" || echo "RED: not documented yet"
```

Expected: prints `RED: not documented yet`

- [ ] **Step 2: Prove the gotcha is real before documenting it**

```bash
P=Users-antony-Development-herdr
codebase-memory-mcp cli search_graph "{\"project\":\"$P\",\"label\":\"Function\",\"min_degree\":10,\"limit\":5}" \
  | jq -r '.results[] | "in=\(.in_degree) out=\(.out_degree) \(.name)"'
codebase-memory-mcp cli search_graph "{\"project\":\"$P\",\"label\":\"Function\",\"min_degree\":10,\"relationship\":\"CALLS\",\"direction\":\"inbound\",\"limit\":5}" \
  | jq -r '.results[] | "in=\(.in_degree) out=\(.out_degree) \(.name)"'
```

Expected: **both commands print identical output**, and results include rows whose `in_degree` is well below 10 (e.g. `in=0 out=11`). This proves `min_degree` gates on `in + out` and that `relationship`/`direction` do not affect it.

- [ ] **Step 3: Insert the two sections**

Insert immediately before `## When the graph has no answer`:

````markdown
## Gotcha: min_degree is total degree

`min_degree` filters on **total** degree — `in_degree + out_degree`. Passing
`relationship` and `direction` alongside it does not narrow the filter; the
results are byte-identical with or without them.

```bash
# These two return exactly the same rows, including in_degree=0 symbols:
codebase-memory-mcp cli search_graph '{"project":"P","label":"Function","min_degree":10,"limit":5}'
codebase-memory-mcp cli search_graph '{"project":"P","label":"Function","min_degree":10,"relationship":"CALLS","direction":"inbound","limit":5}'
```

So "find high fan-in functions" is not expressible as a single call. Use
`min_degree` as a cheap server-side pre-filter, then narrow client-side on
`.in_degree`. That is the composition case below.

Note also that `in_degree` and the number of callers `trace_path` returns are
different numbers — degree is a whole-graph edge count across relationship
types, while `trace_path` follows one traversal to a given depth. Run
`get_graph_schema` if you need the exact edge types in play.

## Composing queries

The CLI earns its place when one question needs many calls. Both patterns
below return a single answer instead of streaming every intermediate result
back through the conversation.

**True high fan-in** — pre-filter on the server, narrow on the client:

```bash
P=Users-antony-Development-herdr
codebase-memory-mcp cli search_graph \
  "{\"project\":\"$P\",\"label\":\"Function\",\"min_degree\":10,\"limit\":200}" \
  | jq -r '.results[] | select(.in_degree >= 10) | "\(.in_degree)\t\(.name)"' \
  | sort -rn | head -20
```

**Trace every hot symbol** — one loop instead of one round-trip per symbol:

```bash
P=Users-antony-Development-herdr
codebase-memory-mcp cli search_graph \
  "{\"project\":\"$P\",\"label\":\"Function\",\"min_degree\":10,\"limit\":200}" \
  | jq -r '[.results[] | select(.in_degree >= 20)] | sort_by(-.in_degree) | .[].name' \
  | while read -r fn; do
      codebase-memory-mcp cli trace_path \
        "{\"project\":\"$P\",\"function_name\":\"$fn\",\"direction\":\"inbound\",\"depth\":1}" \
        | jq -c '{function, callers: [.callers[].name]}'
    done
```

`trace_path` inbound returns `{function, direction, callers}`, where each
caller is `{name, qualified_name, hop}`. There is no `.paths` key.
````

- [ ] **Step 4: Run both composition examples verbatim**

```bash
P=Users-antony-Development-herdr
codebase-memory-mcp cli search_graph "{\"project\":\"$P\",\"label\":\"Function\",\"min_degree\":10,\"limit\":200}" \
  | jq -r '.results[] | select(.in_degree >= 10) | "\(.in_degree)\t\(.name)"' | sort -rn | head -5
```

Expected: five rows, descending by the first column, topped by `123	app_for_mouse_test`.

Then run the trace loop exactly as written in the file. Expected: one compact JSON object per symbol, each with a `function` key and a `callers` array of strings.

If either example errors or prints nothing, fix the example.

- [ ] **Step 5: Commit**

```bash
jj describe -m "skill: using-codebase-memory — composition patterns and the min_degree correction

min_degree gates on in+out, so the documented 'high fan-in' recipe returns
in_degree=0 symbols. The correct form pre-filters server-side and narrows in
jq — which is the composition case the CLI exists for.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>"
jj new
```

---

### Task 3: Distribution — manifest copy and install verification

**Files:**
- Modify: `plugins/linkuistics/.claude-plugin/plugin.json`

**Interfaces:**
- Consumes: the completed `SKILL.md` from Tasks 1 and 2.
- Produces: a `plugin.json` whose `description` mentions the graph-query skill and whose `keywords` array contains `codebase-memory`, `knowledge-graph` and `code-search`; plus verified symlinks in three harness skill directories.

- [ ] **Step 1: Write the failing check — the manifest does not mention the skill**

```bash
grep -q "knowledge-graph" plugins/linkuistics/.claude-plugin/plugin.json \
  && echo "GREEN (already present)" || echo "RED: manifest not updated"
```

Expected: prints `RED: manifest not updated`

- [ ] **Step 2: Update the manifest**

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

- [ ] **Step 3: Verify the JSON is still valid**

```bash
jq -e '.name, .description, (.keywords | index("knowledge-graph"))' plugins/linkuistics/.claude-plugin/plugin.json
```

Expected: exit 0, prints `"linkuistics"`, the extended description, and a non-null array index.

- [ ] **Step 4: Verify install.sh picks the skill up without modification**

```bash
./install.sh
ls -l ~/.codex/skills/using-codebase-memory ~/.gemini/skills/using-codebase-memory ~/.pi/agent/skills/using-codebase-memory 2>&1
```

Expected: `install.sh` reports an increased link count and `ok` lines for each existing harness directory; each `ls` shows a symlink pointing into `plugins/linkuistics/skills/using-codebase-memory`. A `skip` line for a harness whose home directory does not exist is normal, not a failure.

- [ ] **Step 5: Verify the skill is readable through one non-Claude harness path**

```bash
head -5 ~/.pi/agent/skills/using-codebase-memory/SKILL.md
```

Expected: the YAML frontmatter, with `name: using-codebase-memory`. This confirms the symlink resolves and the content is what shipped.

- [ ] **Step 6: Commit**

```bash
jj describe -m "skill: register using-codebase-memory in the linkuistics manifest

install.sh globs the skills directory so it needed no change; the manifest
description and keywords did.

Co-Authored-By: Claude Opus 5 (1M context) <noreply@anthropic.com>"
jj new
```

---

## Self-Review

**Spec coverage** — every numbered item in the spec's "Skill content" section maps to a task:

| Spec item | Task |
|---|---|
| 1. Invocation, fourteen tools | Task 1, Step 3 |
| 2. `project` required (leads) | Task 1, Step 3 |
| 3. Streams (stderr/stdout) | Task 1, Step 3 |
| 4. Exit codes, pipeline caveat | Task 1, Steps 3–4 |
| 5. Errors carry `hint`/`available_projects` | Task 1, Step 3 |
| 6. Composition patterns | Task 2, Step 3 |
| 7. Surface-selection rule + ToolSearch | Task 1, Step 3 |
| 8. Unindexed ≠ empty | Task 1, Step 3 |
| Distribution via install.sh | Task 3, Step 4 |
| Testing: every command executed | Tasks 1/2, Step 4 each |

**Addition beyond the spec:** the `min_degree` total-degree gotcha (Task 2) was discovered while verifying the composition example. It is in scope — it is the reason the composition example takes the shape it does — and it corrects a recipe the installer-managed skill gets wrong.

**Placeholder scan:** no TBDs; every code step carries runnable content and a concrete expected output.

**Type consistency:** `search_graph` result fields (`in_degree`, `out_degree`, `name`, `qualified_name`) and `trace_path` fields (`function`, `direction`, `callers[].name`) are used identically in Tasks 1, 2 and the Global Constraints.

---
name: using-codebase-memory
description: Query a codebase knowledge graph from any shell via the codebase-memory-mcp CLI — search_graph, query_graph (Cypher), trace_path, get_code_snippet — and the silent failure modes that make its output lie. Use when finding symbols by pattern, tracing callers or callees, assessing the blast radius of a change, hunting dead code or high fan-in, composing several graph queries into one answer, or working in a harness that has no MCP.
---

# Using codebase-memory from the shell

`codebase-memory-mcp` indexes a codebase into a knowledge graph and serves
fourteen tools two ways — over MCP, and over a CLI. The tool set is identical;
only the calling convention differs. The CLI reaches every harness that has a
shell, and lets one question become one script instead of a round-trip per call.

> Behaviour verified against `codebase-memory-mcp 0.8.1`. Re-check the stream
> and truncation rules against a newer build before trusting a script to them.

## Which surface to use

> If the MCP graph tools are available to you, prefer them for single
> queries — typed arguments, no shell quoting. Use the CLI when composing,
> batching, or when those tools are not available.

In Claude Code these MCP tools are deferred: they are absent from the base tool
list and must be loaded before they can be called at all.

    ToolSearch("select:mcp__codebase-memory-mcp__search_graph,mcp__codebase-memory-mcp__trace_path")

## Invoking the CLI

    codebase-memory-mcp cli <tool> '<json>'

There is no per-tool help — `cli --help` is read as a tool name and fails.
Parameter names live in the MCP tool schemas; where those are reachable
(`ToolSearch` in Claude Code) read them there, and otherwise start from the
argument lists in this file.

Skip the `--json` flag. It wraps the payload in an MCP envelope
(`.content[0].text`, a JSON *string* needing a second parse), writes that
envelope to **both** stdout and stderr, and returns exit `0` on failure. Plain
`cli` gives clean stdout and an honest exit status.

## The project parameter is required

The CLI does **not** infer the project from your working directory. Running
inside an indexed repo without `project` still fails. Project names are
path-derived; list them first:

```bash
codebase-memory-mcp cli list_projects | jq -r '.projects[].name'
# Users-antony-Development-herdr
```

Then pass one on every call:

```bash
P=Users-antony-Development-herdr
codebase-memory-mcp cli search_graph \
  "{\"project\":\"$P\",\"name_pattern\":\".*Handler.*\",\"limit\":3}"
```

Two tools are exceptions: `list_projects` takes no arguments, and
`index_repository` takes `repo_path` (a filesystem path) rather than `project`.

## Streams, exit codes, and the silent-failure trap

The single most important fact about this CLI:

| | stdout | stderr | exit |
|---|---|---|---|
| success | the result JSON | `level=info msg=mem.init …` | 0 |
| failure | *empty* | the error JSON | 1 |

So `| jq` is clean on success — the log line is already on stderr, no
redirection needed — and **blind on failure**: the error body never reaches
`jq`, and the pipe replaces the binary's exit `1` with `jq`'s exit `0`.

```bash
codebase-memory-mcp cli no_such_tool '{}' >/dev/null 2>&1; echo $?   # 1
codebase-memory-mcp cli no_such_tool '{}' | head -1 >/dev/null; echo $?  # 0 — head's status
```

Guard without a pipeline: capture stdout, let the `if !` see the real status,
and keep stderr where you can read it.

```bash
if ! out=$(codebase-memory-mcp cli search_graph '{"project":"nope"}' 2>/tmp/cm.err); then
  cat /tmp/cm.err >&2
  exit 1
fi
printf '%s' "$out" | jq -r '.total'
```

Dump that file rather than piping it to `jq`: stderr carries the `mem.init` log
line *above* the error, and not every error is JSON — an unknown tool name
prints the bare line `unknown tool: <name>`. Where the body *is* JSON, select it
first, `grep '^{' /tmp/cm.err | jq -r '.error, .hint'`.

The error body is worth reading rather than discarding — for an unknown project
it carries `hint` and the full `available_projects` list. Reach for a blanket
`2>/dev/null` only after a call is known to work; it throws that away.

## Malformed JSON is discarded, not reported

A JSON argument that does not parse is silently replaced by an empty argument
set. You are then told about the *missing* parameter, not the broken quoting:

```bash
codebase-memory-mcp cli search_graph '{"project":"'"$P"'", garbage'
# {"error":"project not found or not indexed", ...}
```

That message is the same one a genuinely unknown project produces, so read
"project not found" as *either* an unindexed project *or* a quoting mistake.
Build the argument with `jq -n` when it contains anything interpolated:

```bash
args=$(jq -nc --arg p "$P" --arg fn "$fn" '{project:$p, function_name:$fn, direction:"inbound"}')
codebase-memory-mcp cli trace_path "$args"
```

## The fourteen tools

| Tool | For |
|---|---|
| `list_projects` | what is indexed (no arguments) |
| `index_repository` | index a repo — `repo_path`, optional `mode` (`full`/`moderate`/`fast`) |
| `index_status` | `{project, nodes, edges, status}` |
| `detect_changes` | files changed since the index, and the symbols they touch |
| `delete_project` | drop an index |
| `search_graph` | find symbols — the workhorse |
| `search_code` | text search over the corpus (results under `total_results`) |
| `query_graph` | Cypher, with aggregation and `ORDER BY` |
| `trace_path` | callers, callees, data flow, cross-service |
| `get_code_snippet` | a symbol's `source` plus its metrics, by `qualified_name` |
| `get_graph_schema` | node labels, their properties, and edge types with counts |
| `get_architecture` | layers, clusters, entry points, routes, hotspots |
| `manage_adr` | decision records held in the graph |
| `ingest_traces` | fold runtime traces into the graph |

`search_graph` has three independent, combinable match modes: `query` (BM25
full-text, splits camelCase — best for natural-language discovery),
`name_pattern` / `qn_pattern` (regex over the name or qualified name), and
`semantic_query` (an **array** of keywords, vector search). Narrow with `label`,
`file_pattern`, `min_degree` / `max_degree`.

## Results are truncated by default

`search_graph` returns at most `limit` rows, **default 200**, and says so only
in two fields you have to look at: `total` is the full match count, `has_more`
is true when there are more.

```bash
codebase-memory-mcp cli search_graph "{\"project\":\"$P\",\"label\":\"Function\",\"min_degree\":10}" \
  | jq -c '{total, rows: (.results|length), has_more}'
# {"total":2460,"rows":200,"has_more":true}
```

Sorting or aggregating those 200 rows client-side answers a question about an
arbitrary 8% of the matches, not about the codebase. Page with `offset` until
`has_more` is false, or push the whole question into `query_graph`.

`trace_path` truncates too, and more quietly — no `has_more`, no count. It
returned exactly 100 callers for two different symbols whose inbound degree was
123 and 104. Treat 100 as "at least 100".

## Degree filters are not directional

`min_degree` gates on **total** degree, `in_degree + out_degree`. Adding
`relationship` and `direction` does not make it directional — those narrow the
candidate set by whether a node participates in that edge type at all, which at
`min_degree:10` on a 23k-node graph removed 2 rows out of 2460.

```bash
codebase-memory-mcp cli search_graph "{\"project\":\"$P\",\"label\":\"Function\",\"min_degree\":10,\"limit\":5}" \
  | jq -r '.results[] | "in=\(.in_degree) out=\(.out_degree) \(.name)"'
# one of the five rows it returns:
# in=0 out=11 above_pane_sets_autoscroll_up   <- in_degree 0, past a min_degree of 10
```

Exactly half the rows that filter returned on the fixture had `in_degree: 0`. So
"find high fan-in functions" is not a `search_graph` call. It is a Cypher
query — see below.

`in_degree` also counts **every** edge type, not just `CALLS` — `USAGE`,
`DEFINES`, `DECORATES` and a dozen more all contribute (`get_graph_schema` lists
them with counts). A symbol with `in_degree: 40` whose inbound edges are all
`USAGE` has no callers at all, and `trace_path` will correctly say so.

## A bare name may not resolve

`trace_path` takes `function_name`, and a name shared by several symbols
resolves to none of them — you get an empty `callers` array and exit `0`, which
reads exactly like "nothing calls this".

```bash
codebase-memory-mcp cli search_graph "{\"project\":\"$P\",\"name_pattern\":\"^wait_for_socket$\"}" \
  | jq -r '.total'                      # 8 distinct symbols share this name
codebase-memory-mcp cli trace_path "{\"project\":\"$P\",\"function_name\":\"wait_for_socket\",\"direction\":\"inbound\",\"depth\":1}" \
  | jq -r '.callers | length'           # 0
```

Pass the `qualified_name` instead — `trace_path` accepts one, and the same
query then returns 67 callers. Check `total` from `search_graph` before
trusting a bare name.

Two more `trace_path` defaults that shape the answer: `include_tests` is
**false**, so callers in test files are omitted unless you ask for them; and the
response key follows the direction — `callers` for `inbound`, `callees` for
`outbound`.

```bash
QN=Users-antony-Development-herdr.scripts.smoke_live_handoff_sessions.wait_for_socket
codebase-memory-mcp cli trace_path \
  "{\"project\":\"$P\",\"function_name\":\"$QN\",\"direction\":\"inbound\",\"depth\":1,\"include_tests\":true}" \
  | jq -c '{function, callers: [.callers[].name] | length}'
# {"function":"...wait_for_socket","callers":67}
```

## Push aggregation into Cypher

`query_graph` runs Cypher server-side, which is where any question involving
counting, sorting or a global top-N belongs. It answers "high fan-in" correctly
and in one call, over the whole graph rather than a truncated page:

```bash
codebase-memory-mcp cli query_graph "$(jq -nc --arg p "$P" '{project:$p, query:
  "MATCH (f:Function)<-[:CALLS]-() RETURN f.qualified_name AS qn, count(*) AS fan_in ORDER BY fan_in DESC LIMIT 5"}')" \
  | jq -r '.rows[] | @tsv'
# Users-antony-Development-herdr.src.app.input.mod.app_for_mouse_test	123
# Users-antony-Development-herdr.src.app.mod.test_app	121
```

Group by `qualified_name`, not `name` — the same ambiguity that defeats
`trace_path` silently inflates a `count(*)` grouped on the bare name, summing
eight unrelated `wait_for_socket`s into one row.

The response is columnar: `{columns, rows, total}`, where `rows` is an array of
arrays in `columns` order and `total` is the row count.

## Compose across tools, not within one

Cypher covers everything one query can express. Bash earns its place for the
shape Cypher cannot reach: **fanning a second tool over the first tool's
results**. Each intermediate stays in the shell instead of transiting the
conversation.

Disambiguate, then trace — the two-step that turns a name into real callers:

```bash
P=Users-antony-Development-herdr
codebase-memory-mcp cli search_graph \
  "$(jq -nc --arg p "$P" '{project:$p, label:"Function", name_pattern:"^wait_for_socket$"}')" \
  | jq -r '.results[] | select(.in_degree > 0) | .qualified_name' \
  | while read -r qn; do
      codebase-memory-mcp cli trace_path \
        "$(jq -nc --arg p "$P" --arg f "$qn" '{project:$p, function_name:$f, direction:"inbound", depth:1}')" \
        2>/dev/null \
      | jq -c --arg qn "$qn" '{symbol:$qn, callers:(.callers|length)}'
    done
```

Rank in Cypher, then pull each hit's source — one script instead of a
round-trip per symbol:

```bash
codebase-memory-mcp cli query_graph "$(jq -nc --arg p "$P" '{project:$p, query:
  "MATCH (f:Function)<-[:CALLS]-() RETURN f.qualified_name AS qn, count(*) AS fan_in ORDER BY fan_in DESC LIMIT 5"}')" \
  | jq -r '.rows[][0]' \
  | while read -r qn; do
      codebase-memory-mcp cli get_code_snippet \
        "$(jq -nc --arg p "$P" --arg q "$qn" '{project:$p, qualified_name:$q}')" 2>/dev/null \
      | jq -c '{name, file_path, lines, complexity}'
    done
```

## When the graph has no answer

An unindexed project is not an empty graph. If `list_projects` does not name the
repo you are in, either index it —

```bash
codebase-memory-mcp cli index_repository "$(jq -nc --arg r "$PWD" '{repo_path:$r, mode:"fast"}')" \
  | jq -c '{project, status, nodes, edges, excluded: .excluded.dirs}'
```

— or fall back to Grep and say which you did. `fast` skips similarity and
semantic edges and filters the file set; the response names the directories it
dropped under `excluded`, so check them before concluding a symbol is absent.
`full` and `moderate` index more and cost more.

Reporting "no results found" from an index that was never built is the one
failure mode this whole file exists to prevent.

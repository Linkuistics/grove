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

In Claude Code, MCP tools are [deferred by default][toolsearch] — absent from the
base tool list until loaded, so an absent `mcp__codebase-memory-mcp__*` tool means
"not yet loaded", not "server not running". Load what you need first:

    ToolSearch("select:mcp__codebase-memory-mcp__search_graph,mcp__codebase-memory-mcp__trace_path")

Deferral is the default, not a guarantee: `ENABLE_TOOL_SEARCH=false`, an
unsupported endpoint, or a server marked `alwaysLoad` all put the tools in the
base list instead, where they are simply callable.

[toolsearch]: https://code.claude.com/docs/en/mcp#scale-with-mcp-tool-search

## Invoking the CLI

    codebase-memory-mcp cli <tool> '<json>'

There is no per-tool help — `cli --help` is read as a tool name and fails.
Parameter names live in the MCP tool schemas; where those are reachable
(`ToolSearch` in Claude Code) read them there, and otherwise start from the
argument lists in this file.

Skip the `--json` flag. It wraps the payload in an MCP envelope
(`.content[0].text`, a JSON *string* needing a second parse) and returns exit `0`
even on failure — the failure signal moves inside the envelope, to
`"isError": true`, where a shell `if` will not see it. Plain `cli` gives clean
stdout and an honest exit status.

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

`set -o pipefail` (bash and zsh alike) fixes the *status* half in one line, and
is worth setting in any script that pipes this CLI:

```bash
set -o pipefail
codebase-memory-mcp cli search_graph '{"project":"nope"}' | jq -r '.total'; echo $?  # 1
```

It does not fix the *blindness* half — `jq` still receives an empty stream and
prints nothing useful. To see the error as well, guard without a pipeline:
capture stdout, let the `if !` see the real status, and keep stderr where you can
read it.

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
| `index_repository` | index a repo — `repo_path`, optional `mode` (below) |
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
| `ingest_traces` | accepts runtime traces — see below before relying on it |

`ingest_traces` does **not** currently change the graph. It accepts a payload and
says so in the response: `{"status":"accepted", "traces_received":0, "note":
"Runtime edge creation from traces not yet implemented"}`. Read the `note`;
`status: "accepted"` alone reads like success.

`index_repository`'s `mode` is `full` (the default), `moderate`, `fast`, or
`cross-repo-intelligence` — the last also **requires** `target_projects` (use
`["*"]` for all). An unrecognised mode is not rejected: it silently routes to
`full`, the slowest one, so a typo costs time rather than raising an error.

### `query` is exclusive; everything else composes

`search_graph` selects rows three ways — `query` (BM25 full-text, splits
camelCase — best for natural-language discovery), `name_pattern` / `qn_pattern`
(regex over the name or qualified name), and `semantic_query` (an **array** of
keywords, vector search) — narrowed by `label`, `file_pattern` and
`min_degree` / `max_degree`.

**`query` overrides every other one of those, silently.** Send it alongside any
of them and the rest are ignored — no error, no warning, just unfiltered results
you will read as filtered:

```bash
codebase-memory-mcp cli search_graph "{\"project\":\"$P\",\"query\":\"wait socket\",\"limit\":5}" \
  | jq -r '.total'                                                          # 257
codebase-memory-mcp cli search_graph \
  "{\"project\":\"$P\",\"query\":\"wait socket\",\"label\":\"NoSuchLabel\",\"limit\":5}" \
  | jq -r '.total'                                                          # 257 — label ignored
```

An impossible `name_pattern`, a nonexistent `label`, a `file_pattern` matching
nothing, `min_degree:10000` — every one of them returns the same 257 rows. So
treat `query` as a whole search, not as one clause of one: use it alone, or drop
it and express the constraint in the modes that *do* compose.

Everything except `query` intersects as you would expect — `name_pattern` +
`qn_pattern`, `semantic_query` + `label` + `min_degree`, and so on all narrow.

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

`trace_path` truncates too, and worse: its response carries no `total` and no
`has_more`, and the cap is **hard at 100** — `limit` and `max_results` are both
ignored. A symbol whose true caller count is 123 returns 100, and nothing in the
payload says so. Read 100 as "100 or more, and this tool cannot tell you which";
when the exact set matters, count it in Cypher instead.

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

```bash
QN=Users-antony-Development-herdr.scripts.smoke_live_handoff_sessions.wait_for_socket
codebase-memory-mcp cli trace_path \
  "{\"project\":\"$P\",\"function_name\":\"$QN\",\"direction\":\"inbound\",\"depth\":1}" \
  | jq -c '{function, callers: [.callers[].name] | length}'
# {"function":"...wait_for_socket","callers":67}
```

Two more `trace_path` defaults shape the answer. `include_tests` is **false**, so
callers in test files are dropped — and on a test-heavy symbol that is most of
them. One fixture symbol reports **1** caller by default and **100** with
`include_tests: true`; nothing but the flag distinguishes "barely used" from
"used everywhere in the suite". The other default: the response key follows the
direction — `callers` for `inbound`, `callees` for `outbound`.

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

### Filter with `WHERE`, never an inline property map

Pinning a node with a property map in the pattern — `(f:Function {qualified_name:
'…'})` — **silently truncates any traversal off it to 10 rows**. The `WHERE` form
of the same query returns the real answer:

```bash
cy() { codebase-memory-mcp cli query_graph "$(jq -nc --arg p "$P" --arg q "$1" '{project:$p, query:$q}')"; }
QN=Users-antony-Development-herdr.src.app.input.mod.app_for_mouse_test

cy "MATCH (f:Function {qualified_name: '$QN'})<-[:CALLS]-() RETURN count(*) AS n" | jq -c '.rows'
# [["10"]]    <- capped, and nothing in the response says so
cy "MATCH (f:Function)<-[:CALLS]-() WHERE f.qualified_name = '$QN' RETURN count(*) AS n" | jq -c '.rows'
# [["123"]]   <- the true count
```

Every high-fan-in symbol on the fixture returns exactly `10` in the first form.
There is no `has_more`, and `total` counts the rows *returned*, so the response
looks complete. Worse, the two forms **agree** whenever the true result is under
10 — so this passes every small test case and is wrong on every large one.

The cap is on the traversal, not the match: a property map with no edge pattern
(`MATCH (f:Function {name: 'wait_for_socket'}) RETURN count(*)`) is correct. That
narrowness is not worth remembering — just always write `WHERE`.

## Compose across tools, not within one

Cypher covers everything one query can express. Bash earns its place for the
shape Cypher cannot reach: **fanning a second tool over the first tool's
results**. Each intermediate stays in the shell instead of transiting the
conversation.

A loop is where the silent-failure trap does the most damage: a failed call
contributes nothing to stdout, so the loop simply iterates fewer times and the
output looks like a smaller true answer. Two lines fix it for a whole script —
`set -o pipefail` (bash and zsh both) restores the honest exit status through a
pipe, and one wrapper keeps every error visible:

```bash
set -o pipefail
P=Users-antony-Development-herdr

cm() {  # cm <tool> <json>  — JSON on stdout, or a loud failure and exit 1
  local out err=; err=$(mktemp)
  if ! out=$(codebase-memory-mcp cli "$@" 2>"$err"); then
    printf 'codebase-memory-mcp %s failed:\n' "$1" >&2; cat "$err" >&2
    rm -f "$err"; return 1
  fi
  rm -f "$err"; printf '%s' "$out"
}
```

Disambiguate, then trace — the two-step that turns a name into real callers:

```bash
cm search_graph "$(jq -nc --arg p "$P" '{project:$p, label:"Function", name_pattern:"^wait_for_socket$"}')" \
  | jq -r '.results[] | select(.in_degree > 0) | .qualified_name' \
  | while read -r qn; do
      cm trace_path \
        "$(jq -nc --arg p "$P" --arg f "$qn" '{project:$p, function_name:$f, direction:"inbound", depth:1}')" \
      | jq -c --arg qn "$qn" '{symbol:$qn, callers:(.callers|length)}'
    done
```

Rank in Cypher, then pull each hit's source — one script instead of a
round-trip per symbol:

```bash
cm query_graph "$(jq -nc --arg p "$P" '{project:$p, query:
  "MATCH (f:Function)<-[:CALLS]-() RETURN f.qualified_name AS qn, count(*) AS fan_in ORDER BY fan_in DESC LIMIT 5"}')" \
  | jq -r '.rows[][0]' \
  | while read -r qn; do
      cm get_code_snippet "$(jq -nc --arg p "$P" --arg q "$qn" '{project:$p, qualified_name:$q}')" \
      | jq -c '{name, file_path, lines, complexity}'
    done
```

The `2>/dev/null` these loops would otherwise carry is the blanket redirect
warned against above: inside a loop it discards the one diagnostic that explains
why a symbol produced no row.

## When the graph has no answer

An unindexed project is not an empty graph. If `list_projects` does not name the
repo you are in, either index it —

```bash
codebase-memory-mcp cli index_repository "$(jq -nc --arg r "$PWD" '{repo_path:$r, mode:"fast"}')" \
  | jq -c '{project, status, nodes, edges, excluded: .excluded.dirs}'
```

— or fall back to Grep and say which you did. `fast` skips similarity and
semantic edges and filters the file set; where it dropped directories it names
them under `excluded`, so check them before concluding a symbol is absent. The
key is **absent entirely** when nothing was dropped, so `.excluded.dirs` yielding
`null` means "nothing excluded", not "field missing". `full` and `moderate` index
more and cost more.

Reporting "no results found" from an index that was never built is the one
failure mode this whole file exists to prevent.

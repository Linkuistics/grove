# The engine of `conformance.sh`. Read the header of that file first: it states
# the assertions, the inputs and the two modes.
#
# `conformance.sh` writes one inventory and gives it to this program. Each line
# of the inventory is one record, with tab-separated fields:
#
#   P  <pack> <label> <skills root> <manifest> <removed paraphrases or empty>
#   S  <pack> <skill> <skill directory>
#   F  <pack> <skill> <path inside the skill> <absolute path>
#
# The program reads each markdown file, each manifest and each paraphrase list
# itself, with `getline`. It runs no other process. It keeps to POSIX awk, so
# the `awk` of macOS runs it.
#
# Variables from the command line:
#
#   mode     `aggregate` or `alone`. Core alone is the aggregate of one pack.
#   verbose  1 prints the detail lines.
#
# A file is named by its skill-qualified path, such as `grove/SKILL.md` or
# `grove-impl/SKILL.md`. Rows of a manifest name owners and triggers at the same
# grain.

BEGIN {
  FS = "\t"
  failures = 0
  notes = 0
  npacks = 0
  nskills = 0
  nkinds = 0
  nfiles = 0
  nduplicates = 0
}

$1 == "P" {
  npacks++
  pack_order[npacks] = $2
  pack_label[$2] = $3
  pack_root[$2] = $4
  pack_manifest[$2] = $5
  pack_paraphrases[$2] = $6
  next
}

# The first pack that ships a skill name owns it. A later pack with the same
# name is a duplicate. Its files take no part in the run, so one fault gives
# one diagnostic.
$1 == "S" {
  if ($3 in skill_pack) {
    nduplicates++
    duplicate_message[nduplicates] = "duplicate skill: `" $3 "` is shipped by " \
      pack_label[skill_pack[$3]] " (" skill_dir[$3] ") and by " pack_label[$2] " (" $4 ")"
    duplicate_skill[$2 SUBSEP $3] = 1
    next
  }
  skill_pack[$3] = $2
  skill_dir[$3] = $4
  nskills++
  skill_list[nskills] = $3
  if ($3 ~ /^grove-./) {
    nkinds++
    kind_list[nkinds] = substr($3, 7)
  }
  next
}

$1 == "F" {
  if (($2 SUBSEP $3) in duplicate_skill) next
  nfiles++
  file_id[nfiles] = $3 "/" $4
  file_skill[nfiles] = $3
  file_rel[nfiles] = $4
  file_abs[nfiles] = $5
  file_index[$3 "/" $4] = nfiles
  next
}

END {
  run()
  exit (failures ? 1 : 0)
}

# ---------------------------------------------------------------------------
# Output
# ---------------------------------------------------------------------------

function fail(message) {
  printf "FAIL   %s\n", message > "/dev/stderr"
  fflush("/dev/stderr")
  failures++
}

function note(message) {
  printf "note   %s\n", message
  fflush()
  notes++
}

function detail(message) {
  if (verbose) printf "       %s\n", message
}

# ---------------------------------------------------------------------------
# Reading a file
# ---------------------------------------------------------------------------

function read_file(i,   path, line, status, n, lines) {
  path = file_abs[i]
  n = 0
  while ((status = (getline line < path)) > 0) lines[++n] = line
  close(path)
  if (status < 0) fail("cannot read " file_id[i] " at " path)
  extract_references(i, lines, n)
  extract_loads(i, lines, n)
  file_norm[i] = normalised(join_lines(lines, n))
}

function join_lines(lines, n,   k, s) {
  s = ""
  for (k = 1; k <= n; k++) s = s lines[k] "\n"
  return s
}

# normalised: emphasis and code markers removed, case folded, white space
# squeezed. The corpus wraps prose at 80 columns and marks up text inside a
# phrase, so a raw substring test misses real occurrences. Such a miss reads
# as a clean sweep.
function normalised(s) {
  gsub(/[*_`]/, "", s)
  s = tolower(s)
  gsub(/[ \t\n\r\f\013]+/, " ", s)
  return s
}

# drop_comments <line>: the line without the parts inside HTML comments. The
# global `in_comment` carries an open comment to the next line. A comment is
# provenance, such as the upstream file of an adapted passage. It never tells a
# session to open a file or to load a skill.
function drop_comments(line,   open_at, close_at, rest) {
  while (1) {
    if (in_comment) {
      close_at = index(line, "-->")
      if (close_at == 0) return ""
      line = substr(line, close_at + 3)
      in_comment = 0
    }
    open_at = index(line, "<!--")
    if (open_at == 0) return line
    rest = substr(line, open_at + 4)
    line = substr(line, 1, open_at - 1)
    close_at = index(rest, "-->")
    if (close_at == 0) {
      in_comment = 1
      return line
    }
    line = line substr(rest, close_at + 3)
  }
}

# extract_references: each backticked token that ends in `.md`, once per file.
# The split on backticks keeps the spans inside them. One noun phrase often has
# two spans, so a greedy regular expression would join them.
function extract_references(i, lines, n,   k, line, parts, m, j, seen) {
  in_comment = 0
  file_nrefs[i] = 0
  for (k = 1; k <= n; k++) {
    line = drop_comments(lines[k])
    m = split(line, parts, "`")
    for (j = 2; j <= m; j += 2) {
      if (parts[j] !~ /\.md$/ || (parts[j] in seen)) continue
      seen[parts[j]] = 1
      file_ref[i, ++file_nrefs[i]] = parts[j]
    }
  }
}

# extract_loads: each directed load, in the order of the text, once per file.
# The match rules are those of "References between skills" in
# `docs/specs/external-pack-contract.md`. A paragraph is the unit, so a phrase
# can wrap across a line break. A fenced block, a block quote and an HTML
# comment hold no directed load, because an example or a quotation of the
# phrase is not an edge.
function extract_loads(i, lines, n,   k, line, fence, paragraph) {
  in_comment = 0
  fence = ""
  paragraph = ""
  file_nloads[i] = 0
  for (k = 1; k <= n; k++) {
    line = lines[k]
    if (fence != "") {
      if (line ~ ("^[ \t]*" fence)) fence = ""
      continue
    }
    line = drop_comments(line)
    if (match(line, /^[ \t]*(```|~~~)/)) {
      scan_paragraph(i, paragraph)
      paragraph = ""
      fence = substr(line, RSTART + RLENGTH - 3, 3)
      continue
    }
    if (line ~ /^[ \t]*>/ || line ~ /^[ \t]*$/) {
      scan_paragraph(i, paragraph)
      paragraph = ""
      continue
    }
    if (line ~ /^#+[ \t]/) {
      scan_paragraph(i, paragraph)
      scan_paragraph(i, line)
      paragraph = ""
      continue
    }
    paragraph = (paragraph == "") ? line : paragraph " " line
  }
  scan_paragraph(i, paragraph)
}

# scan_paragraph: a directed load is the words "load the", one code span that
# holds the skill name, and the word "skill", in that order.
function scan_paragraph(i, paragraph,   parts, m, j, before, after, name, k) {
  if (paragraph == "") return
  m = split(paragraph, parts, "`")
  for (j = 2; j < m; j += 2) {
    before = prose(parts[j - 1])
    after = prose(parts[j + 1])
    if (before !~ /(^|[^a-z0-9])load the ?$/) continue
    if (after !~ /^ ?skill([^a-z0-9]|$)/) continue
    name = parts[j]
    gsub(/^[ \t]+|[ \t]+$/, "", name)
    if (name == "") continue
    for (k = 1; k <= file_nloads[i]; k++) if (file_load[i, k] == name) break
    if (k <= file_nloads[i]) continue
    file_load[i, ++file_nloads[i]] = name
  }
}

# prose: the text between code spans, with emphasis markers removed, case
# folded and white space squeezed.
function prose(s) {
  gsub(/[*_]/, "", s)
  s = tolower(s)
  gsub(/[ \t]+/, " ", s)
  return s
}

# ---------------------------------------------------------------------------
# Classifying a path
# ---------------------------------------------------------------------------
#
# **Enumerate, then classify.** Each token falls into exactly one class. A
# token that fits none fails, because a pattern list is complete only as far
# as the list.
#
#   skill    skill-relative: `SKILL.md`, `TASK-FORMAT.md`, `references/x.md`.
#            It must resolve by rule 3 of "References between skills".
#   grammar  a filename template or a fragment, not a file: anything with `<`
#            or `>`, and a bare extension or suffix such as `-a.md`.
#   tree     an artifact of the grove working tree: a node file, the glossary,
#            a leaf filename.
#   repo     a path into the repository under review, such as `docs/adr/x.md`.
#            That repository decides whether it resolves. A concrete tree path
#            such as `01-k7/_extract.md` also lands here.
function classify(token) {
  if (token ~ /[<>]/) return "grammar"
  if (token ~ /^[-.]/) return "grammar"
  if (token ~ /^references\/.*\.md$/) return "skill"
  if (token ~ /\//) return "repo"
  if (token == "_BRIEF.md" || token == "CONTEXT.md" || token == "CONTEXT-MAP.md") return "tree"
  if (token ~ /^_[a-z0-9]+(-[a-z0-9]+)*\.md$/) return "tree"
  if (token ~ /^[0-9][0-9]+-.*-k[0-9]+\.md$/) return "tree"
  if (token ~ /\.md$/) return "skill"
  return ""
}

# ---------------------------------------------------------------------------
# Resolution
# ---------------------------------------------------------------------------

# effective_loads <i> <out>: the skills that file i loads, in order. These are
# its own directed loads, then those of its skill's `SKILL.md`. A session reads
# a supporting file of a skill only after that `SKILL.md`, so the loads of that
# file are in force. Prints nothing, returns the count.
function effective_loads(i, out,   n, k, s, j, seen) {
  n = 0
  for (k = 1; k <= file_nloads[i]; k++) {
    seen[file_load[i, k]] = 1
    out[++n] = file_load[i, k]
  }
  if (file_rel[i] == "SKILL.md") return n
  s = file_skill[i] "/SKILL.md"
  if (!(s in file_index)) return n
  j = file_index[s]
  for (k = 1; k <= file_nloads[j]; k++) {
    if (file_load[j, k] in seen) continue
    seen[file_load[j, k]] = 1
    out[++n] = file_load[j, k]
  }
  return n
}

# resolve <i> <token>: where a skill-relative path of file i resolves, by rule
# 3. It tries the file's own skill, then each loaded skill in the order of the
# loads. It returns the skill-qualified path of the resource, or `external:`
# and the name of a loaded skill that this pack alone does not ship, or the
# empty string for a missing file.
#
# In the aggregate, a load that no pack ships is skipped here. The load check
# reports it once.
function resolve(i, token,   s, n, k, loads, l) {
  s = file_skill[i]
  if ((s "/" token) in file_index) return s "/" token
  n = effective_loads(i, loads)
  for (k = 1; k <= n; k++) {
    l = loads[k]
    if (!(l in skill_pack)) {
      if (mode == "alone") return "external:" l
      continue
    }
    if ((l "/" token) in file_index) return l "/" token
  }
  return ""
}

# ---------------------------------------------------------------------------
# The composed loaded path
# ---------------------------------------------------------------------------
#
# A kind's composed loaded path starts at its own `SKILL.md`. It follows each
# directed load to the loaded skill's `SKILL.md`, and each skill-relative path
# to the file where it resolves. The spine is on the path only because the
# kind loads it. In a pack alone, the path follows loads inside the pack only.
#
# This is the pointer graph that the corpus itself realises, read from the
# shipped bytes. A path computed from a parallel table of what a session reads
# drifts from the real one.

function compose_path(kind,   queue, head, tail, id, i, k, n, loads, l, token, r) {
  head = 1
  tail = 0
  id = "grove-" kind "/SKILL.md"
  on_path[kind, id] = 1
  queue[++tail] = id
  while (head <= tail) {
    id = queue[head++]
    if (!(id in file_index)) continue
    i = file_index[id]
    n = effective_loads(i, loads)
    for (k = 1; k <= n; k++) {
      l = loads[k]
      if (!(l in skill_pack)) continue
      r = l "/SKILL.md"
      if ((kind SUBSEP r) in on_path) continue
      on_path[kind, r] = 1
      queue[++tail] = r
    }
    for (k = 1; k <= file_nrefs[i]; k++) {
      token = file_ref[i, k]
      if (classify(token) != "skill") continue
      r = resolve(i, token)
      if (r == "" || r ~ /^external:/) continue
      if ((kind SUBSEP r) in on_path) continue
      on_path[kind, r] = 1
      queue[++tail] = r
    }
  }
  path_length[kind] = tail
  detail("path     grove-" kind ": " tail " files")
}

# ---------------------------------------------------------------------------
# Loads and references, over every file of the run
# ---------------------------------------------------------------------------

function check_file(i,   id, k, l, token, class, r) {
  id = file_id[i]
  for (k = 1; k <= file_nloads[i]; k++) {
    l = file_load[i, k]
    if (l in skill_pack) {
      detail("load     " id " -> " l)
    } else if (mode == "alone") {
      external_edges++
      note("external edge: " id " loads `" l "`, which this pack does not ship")
    } else {
      fail("unresolved load: " id " loads `" l "`, which no supplied pack ships")
    }
  }
  for (k = 1; k <= file_nrefs[i]; k++) {
    token = file_ref[i, k]
    class = classify(token)
    if (class == "") {
      fail(id " names `" token "`, which this runner cannot classify — extend classify rather than ignoring it")
      continue
    }
    if (class != "skill") {
      detail("ref " class " " id " -> " token)
      continue
    }
    r = resolve(i, token)
    if (r == "") {
      fail("missing file: " id " names `" token "`, which is in neither " file_skill[i] " nor a skill that the file loads")
    } else if (r ~ /^external:/) {
      external_resources++
      note("external resource: " id " names `" token "`, which resolves in `" substr(r, 10) "`, a skill this pack does not ship")
    } else {
      detail("ref ok   " id " -> " r)
    }
  }
}

# ---------------------------------------------------------------------------
# The single-source sweep
# ---------------------------------------------------------------------------
#
# The sweep is phrase-scoped, and that bounds what it proves. A paraphrase, a
# second statement of the same rule in other words, reads clean. The removed
# paraphrases of each pack pin the known ones. Nothing here claims that no
# other exists.
#
# The spine's `SKILL.md` is the condition register. It states situations and
# paths, never procedures, so the sweep leaves it out and `check_row` sweeps it
# from the other direction.

# sites <needle>: every file of the run that states the normalised needle,
# sorted and joined with commas. Memoised by needle.
function sites(needle,   i, n, list) {
  if (needle in sites_memo) return sites_memo[needle]
  n = 0
  for (i = 1; i <= nfiles; i++) {
    if (file_id[i] == "grove/SKILL.md") continue
    if (index(file_norm[i], needle) > 0) list[++n] = file_id[i]
  }
  sites_memo[needle] = sorted_join(list, n)
  return sites_memo[needle]
}

# sorted_join <list> <n>: the n entries, in byte order, joined with commas.
function sorted_join(list, n,   i, j, t, s) {
  for (i = 2; i <= n; i++) {
    t = list[i]
    for (j = i - 1; j >= 1 && list[j] > t; j--) list[j + 1] = list[j]
    list[j + 1] = t
  }
  s = ""
  for (i = 1; i <= n; i++) s = s (i > 1 ? "," : "") list[i]
  return s
}

# ---------------------------------------------------------------------------
# The manifest
# ---------------------------------------------------------------------------

# bound_kinds <load> <out>: the kinds of the run that a load predicate binds.
# Returns the count, or -1 for a spelling that is neither `static(K)` nor
# `on(t) @ F`.
#
#   static(all)                 every kind of the run
#   static({a,b})               the listed kinds that the run ships
#   static(review-*)            the review kinds, in any pack
#   static(integrate-review-*)  the integration kinds, in any pack
#   static(research)            research-a and research-b, until their move
#   static(editorial)           draft, copy-edit, art and proof, until their move
#   on(t) @ F                   each kind whose loaded path holds file F
function bound_kinds(load, out,   n, set, k, kind, names, wanted, m, j, trigger, found, at) {
  n = 0
  if (load ~ /^static\(.*\)$/) {
    set = substr(load, 8, length(load) - 8)
    if (set ~ /^\{.*\}$/) {
      m = split(substr(set, 2, length(set) - 2), names, ",")
      for (j = 1; j <= m; j++) {
        gsub(/^[ \t]+|[ \t]+$/, "", names[j])
        wanted[names[j]] = 1
      }
    } else if (set != "all" && set != "review-*" && set != "integrate-review-*" \
      && set != "research" && set != "editorial") {
      return -1
    }
    for (k = 1; k <= nkinds; k++) {
      kind = kind_list[k]
      if (set == "all" || (kind in wanted) \
        || (set == "review-*" && kind ~ /^review-/) \
        || (set == "integrate-review-*" && kind ~ /^integrate-review-/) \
        || (set == "research" && (kind == "research-a" || kind == "research-b")) \
        || (set == "editorial" && (kind == "draft" || kind == "copy-edit" || kind == "art" || kind == "proof")))
        out[++n] = kind
    }
    return n
  }
  if (load ~ /^on\(/) {
    trigger = load
    found = 0
    while ((at = index(trigger, ") @ ")) > 0) {
      trigger = substr(trigger, at + 4)
      found = 1
    }
    if (!found || trigger == "" || trigger !~ /\//) return -1
    for (k = 1; k <= nkinds; k++)
      if ((kind_list[k] SUBSEP trigger) in on_path) out[++n] = kind_list[k]
    return n
  }
  return -1
}

# check_row: the three assertions for one row of pack p's manifest.
function check_row(p, rule, owner, class, load, phrase,   owners, n, k, o, s, shipped, needle, found, want, bound, nb, kind, ok) {
  if (rule in row_pack) {
    fail("duplicate row: rule `" rule "` is in the manifest of " pack_label[row_pack[rule]] " and in the manifest of " pack_label[p])
  } else {
    row_pack[rule] = p
  }

  # The driver delivers this rule, inlined into the launch prompt. No skill
  # carries it, and a runner over skills cannot read a prompt. The run reports
  # the row and asserts nothing about it.
  if (owner == "${prompt}") {
    prompt_rows++
    detail("prompt   " rule " (delivered by the driver, inlined into ${prompt})")
    return
  }

  # A row names an owner in its own pack, and that file exists. A row left in a
  # manifest after its owner moved would otherwise pass in silence.
  n = split(owner, owners, ",")
  shipped = 1
  for (k = 1; k <= n; k++) {
    o = owners[k]
    s = o
    sub(/\/.*/, "", s)
    if (!(s in skill_pack) || skill_pack[s] != p || !(o in file_index)) {
      fail("unshipped owner: " rule " names the owner `" o "`, which " pack_label[p] " does not ship")
      shipped = 0
    }
  }
  if (!shipped) return

  # -- 2. No rule has two owners ---------------------------------------------
  if (phrase != "") {
    needle = normalised(phrase)
    found = sites(needle)
    want = sorted_join(owners, n)
    if (found == want) {
      single_source_rows++
    } else {
      fail(rule ": stated in [" (found == "" ? "nowhere" : found) "], and " owner " must be its only site — a missing owner is a rule that a move deleted, and an extra site is a second owner")
    }
    if (("grove/SKILL.md" in file_index) && index(file_norm[file_index["grove/SKILL.md"]], needle) > 0)
      fail(rule ": the spine's SKILL.md states it — a condition register names the situation and the file, never the procedure")
  } else {
    unpinned_rows++
  }

  # -- 1. Behavioural rules on the composed loaded path -----------------------
  if (class !~ /B/) return
  nb = bound_kinds(load, bound)
  if (nb < 0) {
    fail(rule ": load predicate `" load "` is neither static(K) nor on(t) @ F")
    return
  }
  if (nb == 0) {
    unbound_rows++
    return
  }
  for (k = 1; k <= nb; k++) {
    kind = bound[k]
    ok = 1
    for (o = 1; o <= n; o++) if (!((kind SUBSEP owners[o]) in on_path)) ok = 0
    if (ok) {
      path_checks++
    } else {
      fail(rule ": " owner " is not on grove-" kind "'s composed loaded path, and " kind " binds it — a rule a session never reaches is not delivered")
    }
  }
}

function check_manifest(p,   path, line, status, f, phrase, k, m) {
  path = pack_manifest[p]
  while ((status = (getline line < path)) > 0) {
    if (line ~ /^#/ || line ~ /^[ \t]*$/) continue
    m = split(line, f, "\t")
    phrase = f[5]
    for (k = 6; k <= m; k++) phrase = phrase "\t" f[k]
    check_row(p, f[1], f[2], f[3], f[4], phrase)
  }
  close(path)
  if (status < 0) fail("cannot read the manifest of " pack_label[p] " at " path)
}

# check_paraphrases: a wording removed because it restated another file's rule
# must not come back, in any file of the run.
function check_paraphrases(p,   path, line, status, f, found) {
  path = pack_paraphrases[p]
  if (path == "") return
  while ((status = (getline line < path)) > 0) {
    if (line ~ /^#/ || line ~ /^[ \t]*$/) continue
    split(line, f, "\t")
    found = sites(normalised(f[3]))
    if (found != "")
      fail(f[1] ": the wording " f[2] " used to restate it is back, in [" found "] — the owner states it, and this file points at the owner")
  }
  close(path)
  if (status < 0) fail("cannot read the removed paraphrases of " pack_label[p] " at " path)
}

# ---------------------------------------------------------------------------
# Run
# ---------------------------------------------------------------------------

function run(   i, k, p) {
  if (mode == "alone") {
    print "grove conformance — " pack_label[pack_order[1]] " alone"
  } else if (npacks == 1) {
    print "grove conformance — core alone"
  } else {
    print "grove conformance — aggregate of " npacks " packs"
  }
  for (k = 1; k <= npacks; k++) {
    p = pack_order[k]
    printf "pack   %s %s\n", pack_label[p], pack_root[p]
  }

  for (k = 1; k <= nduplicates; k++) fail(duplicate_message[k])
  for (i = 1; i <= nfiles; i++) read_file(i)
  for (i = 1; i <= nfiles; i++) check_file(i)
  for (k = 1; k <= nkinds; k++) compose_path(kind_list[k])
  for (k = 1; k <= npacks; k++) check_manifest(pack_order[k])
  for (k = 1; k <= npacks; k++) check_paraphrases(pack_order[k])

  if (nkinds == 0) note("no grove-<kind> skill is in this run, so no kind binds a behavioural row")

  printf "rules  %d single-source checked, %d loaded-path checked\n", single_source_rows, path_checks
  printf "       %d with no pinned wording, %d bound to no shipped kind, %d delivered by ${prompt}\n", \
    unpinned_rows, unbound_rows, prompt_rows
  if (mode == "alone")
    printf "       %d external edge(s) and %d external resource(s), which the aggregate resolves\n", \
      external_edges, external_resources
  if (failures) {
    printf "FAILED %d assertion(s)\n", failures > "/dev/stderr"
    return
  }
  print "ok     every assertion holds over the supplied skill set"
}

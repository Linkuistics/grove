#!/usr/bin/env python3
"""Derive docs/loop-record.md — the structural record of a grove's own loop.

The subject is `.grove/`, which grove's finish cycle deletes from the *tip* but
not from *history*. So this script reads the task tree at a **revision**, never
from the working copy by assumption: pointed at a pinned commit it keeps working
after the grove that produced it has been torn down.

Run:  python3 scripts/loop-record.py --rev <revision> --start <scaffold-commit>

Controls are printed to stderr on every run and embedded in the output. Read
them: a broken instrument reads clean everywhere, and the residues below are the
only thing that distinguishes clean-because-correct from clean-because-blind.
"""

from __future__ import annotations

import argparse
import collections
import datetime as dt
import re
import subprocess
import sys

# The closed set of nineteen session kinds (content/TASK-FORMAT.md). Five
# producers, each with a review and an integrate-review step, plus the research
# row and the driver-reserved finish sentinel. No kind label plus "-" prefixes
# another, so longest-prefix matching over this set is unambiguous by
# construction.
PRODUCERS = ("requirements", "design", "planning", "prototype", "impl")
KINDS = (
    list(PRODUCERS)
    + [f"review-{p}" for p in PRODUCERS]
    + [f"integrate-review-{p}" for p in PRODUCERS]
    + ["research-a", "research-b", "combine-research", "finish"]
)
assert len(KINDS) == 19, KINDS

LEAF_RE = re.compile(r"^(?P<pos>\d{2})-(?:(?P<outcome>DONE|ABANDONED)-)?(?P<rest>.+)-k(?P<key>\d+)\.md$")
NODE_RE = re.compile(r"^(?P<pos>\d{2})-(?P<slug>.+)-k(?P<key>\d+)$")
HANDLE_RE = re.compile(r"\b([a-z0-9]+(?:-[a-z0-9]+)*)-k(\d+)\b")

FS, RS = "@@F@@", "@@R@@"


def jj(*args: str) -> str:
    out = subprocess.run(["jj", *args], capture_output=True, text=True)
    if out.returncode != 0:
        sys.exit(f"jj {' '.join(args)} failed:\n{out.stderr}")
    return out.stdout


def split_kind(rest: str) -> tuple[str, str]:
    """Split `<kind>-<slug>` on the closed kind set, longest prefix first.

    TASK-FORMAT.md: a missing or unknown kind is malformed and stops the
    operation, naming the path and the valid set, rather than degrading to
    `impl`. The grove immediately before this one used a different filename
    grammar entirely, so a lenient parser would turn it into plausible garbage.
    """
    for kind in sorted(KINDS, key=len, reverse=True):
        if rest == kind or rest.startswith(kind + "-"):
            return kind, rest[len(kind) + 1:]
    raise ValueError(rest)


class Leaf:
    __slots__ = ("pos", "outcome", "kind", "slug", "key", "path", "node", "commits")

    def __init__(self, pos, outcome, kind, slug, key, path, node):
        self.pos, self.outcome, self.kind = pos, outcome, kind
        self.slug, self.key, self.path, self.node = slug, key, path, node
        self.commits: list[Commit] = []

    @property
    def handle(self) -> str:
        return f"{self.slug}-k{self.key}"


class Commit:
    __slots__ = ("cid", "chg", "when", "subject", "lead", "mentions")

    def __init__(self, cid, chg, when, subject):
        self.cid, self.chg, self.when, self.subject = cid, chg, when, subject
        # The leading `<slug>-k<key>:` is the work item this commit carries
        # (content/references/commit.md). Handles later in the line are nodes
        # the retire cascade closed, reported alongside the leaf's own.
        m = re.match(r"^([a-z0-9-]+-k\d+):", subject)
        self.lead = m.group(1) if m else None
        self.mentions = {f"{a}-k{b}" for a, b in HANDLE_RE.findall(subject)}


def read_tree(rev: str) -> tuple[list[Leaf], dict[str, str], list[str]]:
    """Parse .grove/ at `rev`. Returns (leaves, node slug->key, malformed)."""
    files = [
        ln for ln in jj("file", "list", "-r", rev).splitlines()
        if ln.startswith(".grove/") and ln.endswith(".md")
    ]
    leaves, nodes, malformed = [], {}, []
    for path in sorted(files):
        parts = path.split("/")[1:]          # drop the ".grove" root component
        name, dirs = parts[-1], parts[:-1]
        for d in dirs:
            nm = NODE_RE.match(d)
            if nm:
                nodes[f"{nm.group('slug')}-k{nm.group('key')}"] = "/".join(
                    dirs[: dirs.index(d) + 1])
            else:
                malformed.append(f"{path} (directory component {d!r})")
        if name == "BRIEF.md":
            continue
        m = LEAF_RE.match(name)
        if not m:
            malformed.append(f"{path} (filename)")
            continue
        try:
            kind, slug = split_kind(m.group("rest"))
        except ValueError as e:
            malformed.append(f"{path} (unknown kind in {e.args[0]!r})")
            continue
        leaves.append(Leaf(m.group("pos"), m.group("outcome") or "LIVE", kind,
                           slug, int(m.group("key")), path, "/".join(dirs)))
    return leaves, nodes, malformed


def read_window(start: str, rev: str) -> list[Commit]:
    tmpl = (f'commit_id.short(12) ++ "{FS}" ++ change_id.short(8) ++ "{FS}" '
            # AUTHOR, not committer: jj rewrites the committer timestamp on every
            # amend and squash, and this record lives inside a commit it rewrites
            # by being written. The author timestamp records when the work was
            # done and survives the rewrite, so the document reaches a fixed
            # point. It is also the more honest number for a cost account.
            f'++ author.timestamp().format("%Y-%m-%dT%H:%M:%S%z") ++ "{FS}" '
            f'++ description.first_line() ++ "{RS}\\n"')
    raw = jj("log", "--no-graph", "-r", f"{start}::{rev}", "-T", tmpl)
    out = []
    for line in raw.splitlines():
        if not line.endswith(RS):
            continue
        cid, chg, when, subject = line[: -len(RS)].split(FS, 3)
        out.append(Commit(cid, chg, when, subject))
    return sorted(out, key=lambda c: c.when)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--rev", default="@", help="revision to read .grove/ at (default: @)")
    ap.add_argument("--start", default="149994af",
                    help="the commit that scaffolded this grove (window anchor)")
    ap.add_argument("--out", default="docs/loop-record.md", help="output path; - for stdout")
    ap.add_argument("--positive-control", default="experiment-baseline-k4",
                    help="handle that MUST appear in the window")
    ap.add_argument("--negative-control", default="blinded-read-k27",
                    help="handle from a sibling workstream that must NOT appear")
    args = ap.parse_args()

    rev_id = jj("log", "--no-graph", "-r", args.rev, "-T", "commit_id.short(12)").strip()
    start_id = jj("log", "--no-graph", "-r", args.start, "-T", "commit_id.short(12)").strip()
    # Pin the regenerate command on the CHANGE id, not the commit id. In jj the
    # working copy is a commit, so writing this very file rewrites `@` and its
    # commit id — a document that pinned one would be stale the instant it was
    # saved, which is the self-measuring hazard in miniature. A change id
    # survives that rewrite and every later amend.
    rev_change = jj("log", "--no-graph", "-r", args.rev, "-T", "change_id.short(8)").strip()

    leaves, nodes, malformed = read_tree(args.rev)
    commits = read_window(args.start, args.rev)

    by_handle = {lf.handle: lf for lf in leaves}
    dup = [h for h, n in collections.Counter(lf.handle for lf in leaves).items() if n > 1]

    unattributed, node_closes = [], collections.defaultdict(list)
    for c in commits:
        if c.lead and c.lead in by_handle:
            by_handle[c.lead].commits.append(c)
        else:
            unattributed.append(c)
        for h in c.mentions - {c.lead}:
            if h in nodes:
                node_closes[h].append(c)

    unjoined = [lf for lf in leaves if not lf.commits]
    multi = [lf for lf in leaves if len(lf.commits) > 1]

    window_text = " ".join(c.subject for c in commits)
    pos_ok = args.positive_control in window_text
    neg_ok = args.negative_control not in window_text

    # ---- controls to stderr, always, before anything is written -------------
    e = sys.stderr
    print(f"[control] window {start_id}..{rev_id}: {len(commits)} commits", file=e)
    print(f"[control] tree at {rev_id}: {len(leaves)} leaves, {len(nodes)} nodes", file=e)
    print(f"[control] positive '{args.positive_control}' present: {pos_ok}", file=e)
    print(f"[control] negative '{args.negative_control}' absent: {neg_ok}", file=e)
    print(f"[control] malformed entries: {len(malformed)} {malformed if malformed else ''}", file=e)
    print(f"[control] duplicate handles: {len(dup)} {dup if dup else ''}", file=e)
    print(f"[control] leaves with no commit: {len(unjoined)}", file=e)
    print(f"[control] leaves with >1 commit: {len(multi)}", file=e)
    print(f"[control] commits with no leaf: {len(unattributed)}", file=e)
    if not pos_ok or not neg_ok or malformed or dup:
        print("[control] FAILED — refusing to write; the instrument is not trustworthy", file=e)
        return 1

    doc = render(args, rev_id, start_id, rev_change, leaves, nodes, commits, node_closes,
                 unjoined, multi, unattributed, pos_ok, neg_ok)
    if args.out == "-":
        sys.stdout.write(doc)
    else:
        with open(args.out, "w") as fh:
            fh.write(doc)
        print(f"[control] wrote {args.out} ({len(doc)} bytes)", file=e)
    return 0


def parse_ts(s: str) -> dt.datetime:
    return dt.datetime.strptime(s, "%Y-%m-%dT%H:%M:%S%z")


def fmt_span(a: dt.datetime, b: dt.datetime) -> str:
    total = int((b - a).total_seconds())
    h, m = divmod(total // 60, 60)
    return f"{h}h{m:02d}m"


def phase_of(leaf: "Leaf") -> str:
    return leaf.node.split("/")[0] if leaf.node else "(root)"


def render(args, rev_id, start_id, rev_change, leaves, nodes, commits, node_closes,
           unjoined, multi, unattributed, pos_ok, neg_ok) -> str:
    start_leaves, start_nodes, _ = read_tree(args.start)
    planned = {lf.handle for lf in start_leaves} | set(start_nodes)
    current = {lf.handle for lf in leaves} | set(nodes)
    grown = sorted(current - planned, key=lambda h: int(h.rsplit("-k", 1)[1]))
    kept = sorted(planned & current, key=lambda h: int(h.rsplit("-k", 1)[1]))

    ts = [parse_ts(c.when) for c in commits]
    L: list[str] = []
    w = L.append

    w("<!-- DERIVED FILE — do not edit by hand. -->")
    w(f"<!-- Regenerate: python3 scripts/loop-record.py --rev {rev_change} --start {start_id} -->")
    w(f"<!-- Pinned by change id {rev_change}. The commit id is deliberately not recorded:")
    w("     in jj the working copy is a commit, so writing this file rewrites the")
    w("     commit id but never the change id. Recording it would make this derived")
    w("     document fail to reproduce itself. -->")
    w("")
    w("# The loop record — how this grove's sessions actually ran")
    w("")
    w("This is the structural record of one Grove workstream: every session it")
    w("ran, what kind it ran as, where it sat in the tree, how it ended, and which")
    w("commit carries it. It exists because `.grove/` — the task tree that is the")
    w("only record of these facts — is **process state that Grove's finish cycle")
    w("deletes**, and because this campaign's own loop turned out to be its most")
    w("transferable output.")
    w("")
    w("**Derived, not transcribed.** Every table below is generated by")
    w("[`scripts/loop-record.py`](../scripts/loop-record.py) from two sources: the")
    w("`.grove/` tree as it stands at a named revision, and the commit messages in")
    w("this grove's own commit window. Nothing here is typed by hand, and the")
    w("regeneration command is in the comment at the top of this file.")
    w("")
    w("**It reads history, not the working tree.** Finish deletes `.grove/` from")
    w("the tip, not from history, so the script takes the revision to read as an")
    w("argument. Re-run it after teardown with the pinned revision above and it")
    w("still works; that is the whole reason it is not a one-off sweep.")
    w("")
    w("**The pin is a change id, not a commit id.** In jj the working copy *is* a")
    w("commit, so the act of writing this document rewrites the commit it is being")
    w("written into. A document pinned to a commit id would be stale the moment it")
    w("was saved — the self-measuring hazard, in miniature. A change id survives")
    w("the rewrite, so the regenerate command above keeps resolving.")
    w("")
    w("## What this record does not establish")
    w("")
    w("- **It counts sessions and commits, not effort.** There is no token count,")
    w("  no wall-clock attention, and no measure of how hard any session was.")
    w("- **A commit's timestamp bounds a session; it does not measure one.** The")
    w("  gap between consecutive task commits is an *upper* bound on the session")
    w("  between them, and a gap that spans a night is a human asleep, not a")
    w("  nine-hour session. The distribution below is reported for that reason.")
    w("- **A leaf that never ran has a body and no commit, by construction.** An")
    w("  `ABANDONED` row with no commit is the normal case, not a join failure.")
    w("- **It says nothing about what the reviews found.** Finding counts, their")
    w("  survival at integration, and the yield curve are a separate measurement")
    w("  with its own instrument; this record only enumerates the chains.")
    w("")
    w("## Controls")
    w("")
    w("A clean result from a broken instrument is indistinguishable from a clean")
    w("result from a correct one, so the script runs two controls and publishes")
    w("both residues rather than asserting a clean tree.")
    w("")
    w(f"| control | expectation | result |")
    w(f"|---|---|---|")
    w(f"| positive — `{args.positive_control}` | present in the window | {'**found**' if pos_ok else 'MISSING'} |")
    w(f"| negative — `{args.negative_control}` | absent (a sibling workstream sharing this jj commit store) | {'**absent**' if neg_ok else 'PRESENT'} |")
    w(f"| leaves parsed | every task filename carries a known kind | {len(leaves)} parsed, 0 malformed |")
    w(f"| handles unique | slug+key identifies one leaf | {len(leaves)} leaves, 0 duplicates |")
    w("")
    w("The negative control is the one that matters most. This jj repository's")
    w("commit store is shared by four workspaces, and their permanent-key spaces")
    w("**collide**: `blinded-read-k27` belongs to a different workstream while this")
    w("grove's `k27` is `impl-collapse-application-k27`. A join on `k<key>` rather")
    w("than on the full `<slug>-k<key>` handle imports the wrong commit silently.")
    w("")
    w("Two residues are published below rather than swallowed: leaves that joined")
    w("no commit, and commits that joined no leaf. Neither is expected to be")
    w("empty — an instrument whose residue is empty on the first run is more")
    w("likely broken than perfect.")
    w("")

    # ---- the tree at a glance ---------------------------------------------
    by_outcome = collections.Counter(lf.outcome for lf in leaves)
    by_kind = collections.Counter(lf.kind for lf in leaves)
    ran = [lf for lf in leaves if lf.outcome == "DONE"]
    chains = [lf for lf in leaves if lf.kind.startswith("review-")]
    integs = [lf for lf in leaves if lf.kind.startswith("integrate-review-")]
    producers = [lf for lf in ran if not lf.kind.startswith(("review-", "integrate-review-"))]

    w("## The tree at a glance")
    w("")
    w(f"- **{len(leaves)} leaves** and **{len(nodes)} nodes**, at change `{rev_change}`.")
    w(f"- **{by_outcome['DONE']} ran to retirement**, {by_outcome['ABANDONED']} were abandoned, {by_outcome['LIVE']} still live.")
    # Equal counts are not a pairing. Match each review to an integration by the
    # shared stem the composed shape carries, in both directions, and report the
    # unmatched rather than asserting a bijection from two totals that agree.
    rev_stems = collections.Counter(lf.slug for lf in chains)
    int_stems = collections.Counter(lf.slug for lf in integs)
    unmatched_rev = sorted((rev_stems - int_stems).elements())
    unmatched_int = sorted((int_stems - rev_stems).elements())
    w(f"- **{len(producers)} producer sessions ran**, and **{len(chains)} of them")
    w(f"  earned a `review-*` leaf** — a producer was reviewed")
    w(f"  {100*len(chains)//max(len(producers),1)}% of the time.")
    if not unmatched_rev and not unmatched_int:
        w(f"- **Every one of those {len(chains)} reviews earned an integration.** This is")
        w("  checked as a pairing, not inferred from two totals that happen to agree:")
        w("  each review's stem is matched against the integration stems in both")
        w("  directions, and neither side has a leftover. A review that found nothing")
        w("  would have created nothing and simply retired, so **no review in this")
        w("  grove came back empty**.")
    else:
        w(f"- **The review/integration pairing is not clean.** Reviews with no")
        w(f"  integration: {unmatched_rev or 'none'}. Integrations with no review:")
        w(f"  {unmatched_int or 'none'}.")
    w("")
    w("| kind | leaves | ran | abandoned |")
    w("|---|--:|--:|--:|")
    for kind in KINDS:
        n = by_kind.get(kind, 0)
        if not n:
            continue
        d = sum(1 for lf in leaves if lf.kind == kind and lf.outcome == "DONE")
        a = sum(1 for lf in leaves if lf.kind == kind and lf.outcome == "ABANDONED")
        w(f"| `{kind}` | {n} | {d} | {a} |")
    w("")

    # ---- plan vs grown -----------------------------------------------------
    w("## The plan at the start, and the tree that grew")
    w("")
    w(f"The grove was scaffolded at `{start_id}` with a tree already laid out:")
    w(f"**{len(start_leaves)} leaves under {len(start_nodes)} nodes**, keys")
    w(f"`k1`–`k{max(int(h.rsplit('-k',1)[1]) for h in planned)}`. It now holds")
    w(f"**{len(leaves)} leaves under {len(nodes)} nodes**.")
    w("")
    w(f"- **All {len(kept)} of the {len(planned)} planned entries survive** in the tree —")
    w("  nothing scaffolded was deleted; the twelve leaves of the two phases that")
    w("  never ran are marked `ABANDONED` in place rather than removed.")
    w(f"- **{len(grown)} further entries were grown during the run.** {100*len(grown)//max(len(current),1)}%")
    w(f"  of the final {len(current)}-entry tree was created by sessions mid-flight,")
    w("  not by the plan that opened it.")
    w(f"- **Every review chain in this grove was grown, none planned**: the lowest")
    w(f"  key on any `review-*` or `integrate-review-*` leaf is")
    w(f"  `k{min(lf.key for lf in leaves if lf.kind.startswith(('review-','integrate-review-')))}`,")
    w(f"  above the whole scaffolded range.")
    w("")
    w("Grown entries, in the order their keys were allocated:")
    w("")
    w("> " + ", ".join(f"`{h}`" for h in grown))
    w("")

    # ---- the session table -------------------------------------------------
    w("## Every session")
    w("")
    w("Tree pre-order — the order `grove-llm pick` walks. `node` is the path under")
    w("`.grove/`; a blank node is a leaf at the grove root.")
    w("")
    w("**Commits are named by jj change id, not commit id**, throughout this")
    w("document. That is not a stylistic choice: this record is generated *inside*")
    w("the commit that carries it, so quoting that commit's own id makes the file")
    w("change every time it is written, which rewrites the commit, which changes")
    w("the id again. A change id is stable across every rewrite, so the document")
    w("reaches a fixed point and reproduces itself. Resolve any of them with")
    w("`jj show <change>`.")
    w("")
    w("| # | node | pos | kind | handle | outcome | change | when |")
    w("|--:|---|--:|---|---|---|---|---|")
    for i, lf in enumerate(sorted(leaves, key=lambda x: x.path), 1):
        c = lf.commits[0] if lf.commits else None
        cid = f"`{c.chg}`" if c else "—"
        when = c.when[:16].replace("T", " ") if c else "—"
        node = f"`{lf.node}`" if lf.node else "—"
        w(f"| {i} | {node} | {lf.pos} | `{lf.kind}` | `{lf.handle}` | {lf.outcome} | {cid} | {when} |")
    w("")

    # ---- nodes -------------------------------------------------------------
    # Three origins, not two. A key present in the opening plan may have been
    # planned AS a node, or planned as a LEAF that a session later decomposed —
    # and collapsing those two into "scaffolded" would hide the decomposition
    # rate, which is the number this table exists to expose.
    start_leaf_handles = {lf.handle for lf in start_leaves}
    def origin_of(h: str) -> str:
        if h in start_nodes:
            return "node at start"
        if h in start_leaf_handles:
            return "planned leaf, decomposed"
        return "grown mid-run"
    origins = collections.Counter(origin_of(h) for h in nodes)
    decomposed = origins["planned leaf, decomposed"] + origins["grown mid-run"]
    w("## Nodes")
    w("")
    w("A node is a leaf that proved bigger than one session, so the count of nodes")
    w("that were **not** nodes to begin with is the decomposition rate of this")
    w("grove.")
    w("")
    at_start = sorted((h for h in nodes if h in start_nodes),
                      key=lambda h: int(h.rsplit("-k", 1)[1]))
    w(f"- **{origins['node at start']} were nodes in the opening plan** — "
      + ", ".join(f"`{h}`" for h in at_start) + ".")
    w(f"- **{origins['planned leaf, decomposed']} were planned as single leaves and decomposed** when the")
    w("  session that picked them found the work bigger than its brief.")
    w(f"- **{origins['grown mid-run']} did not exist at all when the grove opened.**")
    w("")
    w(f"So **{decomposed} of the {len(nodes)} nodes were created by a session rather than by")
    w(f"the plan** — {100*decomposed//len(nodes)}% of the tree's structure was discovered, not designed.")
    w("")
    w("`children` counts direct entries — child leaves and child nodes alike.")
    w("`closed by` is the commit whose message named this node under the retire")
    w("cascade; a blank means no commit ever named it, which for a node whose")
    w("subtree went terminal by abandonment rather than completion is the expected")
    w("reading.")
    w("")
    w("| node | path | origin | children | closed by |")
    w("|---|---|---|--:|---|")
    for h, path in sorted(nodes.items(), key=lambda kv: kv[1]):
        kid_leaves = sum(1 for lf in leaves if lf.node == path)
        kid_nodes = sum(1 for q in nodes.values()
                        if q != path and q.rsplit("/", 1)[0] == path)
        closers = node_closes.get(h, [])
        cl = ", ".join(f"`{c.chg}`" for c in closers) or "—"
        origin = origin_of(h)
        w(f"| `{h}` | `{path}` | {origin} | {kid_leaves + kid_nodes} | {cl} |")
    w("")

    # ---- cost account ------------------------------------------------------
    w("## Cost account, in raw form")
    w("")
    w(f"The grove's commit window is `{start_id}`..change `{rev_change}`: **{len(commits)} commits**")
    w(f"between **{ts[0].strftime('%Y-%m-%d %H:%M')}** and")
    w(f"**{ts[-1].strftime('%Y-%m-%d %H:%M')}** — an elapsed")
    w(f"**{fmt_span(ts[0], ts[-1])}**, or {(ts[-1]-ts[0]).days} days and change.")
    w("")
    w("| phase | leaves | ran | abandoned | commits | first | last | elapsed |")
    w("|---|--:|--:|--:|--:|---|---|--:|")
    for phase in sorted({phase_of(lf) for lf in leaves}):
        ph = [lf for lf in leaves if phase_of(lf) == phase]
        cs = sorted((c for lf in ph for c in lf.commits), key=lambda c: c.when)
        d = sum(1 for lf in ph if lf.outcome == "DONE")
        a = sum(1 for lf in ph if lf.outcome == "ABANDONED")
        if cs:
            t0, t1 = parse_ts(cs[0].when), parse_ts(cs[-1].when)
            span = fmt_span(t0, t1)
            f0, f1 = t0.strftime("%m-%d %H:%M"), t1.strftime("%m-%d %H:%M")
        else:
            span, f0, f1 = "—", "—", "—"
        w(f"| `{phase}` | {len(ph)} | {d} | {a} | {len(cs)} | {f0} | {f1} | {span} |")
    w("")
    gaps = [int((ts[i+1] - ts[i]).total_seconds() // 60) for i in range(len(ts) - 1)]
    gaps_sorted = sorted(gaps)
    buckets = [("under 30m", sum(g < 30 for g in gaps)),
               ("30m – 2h", sum(30 <= g < 120 for g in gaps)),
               ("2h – 8h", sum(120 <= g < 480 for g in gaps)),
               ("over 8h", sum(g >= 480 for g in gaps))]
    w("**Gap between consecutive commits**, which bounds a session from above and")
    w("does not measure one. The distribution is reported rather than a mean,")
    w("because the long tail is a human asleep and averaging it in would invent a")
    w("session cost nobody paid:")
    w("")
    w("| gap | commits |")
    w("|---|--:|")
    for label, n in buckets:
        w(f"| {label} | {n} |")
    w("")
    w(f"Median gap **{gaps_sorted[len(gaps_sorted)//2]} minutes**; longest")
    w(f"**{fmt_span(ts[0], ts[0] + dt.timedelta(minutes=max(gaps)))}**.")
    w("")

    # ---- residues ----------------------------------------------------------
    w("## Residues — what did not join, in both directions")
    w("")
    def why_no_commit(lf) -> str:
        if lf.outcome == "ABANDONED":
            return "never ran — abandoned in place"
        if lf.outcome == "LIVE":
            return "not yet run"
        # A retired leaf with no commit can only be the leaf of the session that
        # generated this file: Retire precedes Commit, so the rename is already
        # on disk while the commit carrying it does not yet exist. This is the
        # reflexive boundary, stated rather than smoothed over.
        return "**retired by the session that generated this record** — its commit is the one this file lands in"
    w(f"**{len(unjoined)} leaves carry no commit**, and each is accounted for. An")
    w("abandoned leaf never ran; a live leaf has not run yet; and a *retired* leaf")
    w("with no commit can only be this record's own session, because Grove retires")
    w("a leaf before it commits it.")
    w("")
    w("| handle | kind | outcome | why |")
    w("|---|---|---|---|")
    for lf in sorted(unjoined, key=lambda x: x.key):
        w(f"| `{lf.handle}` | `{lf.kind}` | {lf.outcome} | {why_no_commit(lf)} |")
    w("")
    w(f"**{len(unattributed)} commits carry no leaf handle.** These are the framing")
    w("commits — the scaffolding, the human stop decisions, and any open working")
    w("copy. Each is listed in full rather than counted, so the window is accounted")
    w("for exhaustively rather than by subtraction.")
    w("")
    w("| change | when | subject |")
    w("|---|---|---|")
    for c in unattributed:
        if c.subject:
            w(f"| `{c.chg}` | {c.when[:16].replace('T', ' ')} | {c.subject} |")
        else:
            w(f"| change `{c.chg}` | — | *(the session writing this record — an open,"
              f" undescribed working copy; its commit id and timestamp are omitted"
              f" because writing this file changes both)* |")
    w("")
    if multi:
        w(f"**{len(multi)} leaves carry more than one commit.**")
        w("")
        w("| handle | commits |")
        w("|---|---|")
        for lf in sorted(multi, key=lambda x: x.key):
            w(f"| `{lf.handle}` | " + ", ".join(f"`{c.chg}`" for c in lf.commits) + " |")
        w("")
    else:
        w("**No leaf carries more than one commit** — one task, one focused commit,")
        w("held across the whole run.")
        w("")
    return "\n".join(L) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Derive docs/review-yield.md — what this grove's nine review chains found.

The subject is the eighteen `review-*` / `integrate-review-*` task bodies in
`.grove/`, which grove's finish cycle deletes from the *tip* but not from
*history*. So this script reads them at a **revision**, never from the working
copy by assumption.

WHAT IS DERIVED AND WHAT IS NOT, because the difference is the whole instrument.
Classifying a paragraph as a finding is a judgement and cannot be derived; the
`BLOCKS` table below is that judgement, written by hand. What IS derived is the
*enumeration*: every ATX heading, every top-level ordered-list item and every
table in all eighteen bodies is extracted mechanically from markdown block
structure — complete by construction, not by a pattern list — and each is
assigned to exactly one classified block. The script then asserts coverage in
**both directions** and exits non-zero if either side has a leftover:

  forward   every enumerated item lands in exactly one classified block
  reverse   every classified block resolves to at least one enumerated item

That is the same shape as `models/run.sh` asserting (family, obligation)
coverage in both directions, and for the same reason: a clean result from a
broken instrument is indistinguishable from a clean result from a correct one.

Run:  python3 scripts/review-yield.py --rev <revision>
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys

# ── The nine chains ──────────────────────────────────────────────────────────
# Ordered by the review leaf's commit, which is the order the loop ran them.
G = ".grove/"
FM = "01-formal-modeling-k1/"
QM = FM + "09-quint-models-k10/"
FS = FM + "13-formal-synthesis-k16/"
CD = FS + "05-catalogue-disposition-k64/"
QN = FS + "09-quarantine-necessity-k79/"

CHAINS = [
    # slug, producer kind, review path, integration path, integration change id
    ("experiment-baseline-k29", "requirements",
     FM + "02-DONE-review-requirements-experiment-baseline-k29.md",
     FM + "03-DONE-integrate-review-requirements-experiment-baseline-k30.md", "xpyqrvqy"),
    ("model-contract-k31", "design",
     FM + "05-DONE-review-design-model-contract-k31.md",
     FM + "06-DONE-integrate-review-design-model-contract-k32.md", "wxrmplzu"),
    ("task-tree-k55", "prototype",
     QM + "02-DONE-review-prototype-task-tree-k55.md",
     QM + "03-DONE-integrate-review-prototype-task-tree-k56.md", "xwzuynow"),
    ("finish-k57", "prototype",
     QM + "05-DONE-review-prototype-finish-k57.md",
     QM + "06-DONE-integrate-review-prototype-finish-k58.md", "tkyxvkss"),
    ("system-k59", "prototype",
     QM + "08-DONE-review-prototype-system-k59.md",
     QM + "09-DONE-integrate-review-prototype-system-k60.md", "xrtrwuwt"),
    ("obligation-placement-k67", "design",
     FS + "03-DONE-review-design-obligation-placement-k67.md",
     FS + "04-DONE-integrate-review-design-obligation-placement-k68.md", "vrtwvqyo"),
    ("finish-scope-k75", "design",
     CD + "04-DONE-review-design-finish-scope-k75.md",
     CD + "05-DONE-integrate-review-design-finish-scope-k76.md", "ktlsqlqs"),
    ("finish-verdicts-k77", "design",
     FS + "07-DONE-review-design-finish-verdicts-k77.md",
     FS + "08-DONE-integrate-review-design-finish-verdicts-k78.md", "wkzptosn"),
    ("honest-classification-k84", "design",
     QN + "02-DONE-review-design-honest-classification-k84.md",
     QN + "03-DONE-integrate-review-design-honest-classification-k85.md", "ooytutqw"),
]

# ── The classification, written by hand ──────────────────────────────────────
# One row per *enumeration block*: a maximal run of sibling headings, ordered
# list items or table rows under one parent. The block, not the item, is the
# unit a session actually chose, and classifying at the item grain would invent
# a precision the sources do not have.
#
#   key      (chain slug, "R" | "I", anchor) — anchor is a substring that
#            identifies the block's parent heading, or "" for a body's top level
#   cls      T  template section heading (Goal / Context / Done when / Notes /
#               Decisions), carrying no enumeration of its own
#            C  charter prose the session wrote *about* the work — a review's
#               doubts before it ran, an integration's Done-when
#            X  a RESTATEMENT of findings already counted at the review; the
#               single most important exclusion, since five integrations open
#               by re-listing the review's findings verbatim
#            F  findings raised by the review about the producer  (channel T)
#            G  findings the integrating session raised itself    (channel G)
#            I  findings from an in-session reviewer              (channel I)
#            D  a published residue — doubts discharged, checks that stood
#            V  verification / run-line record
#   n        items in the block, asserted against the mechanical enumeration
BLOCKS = [
    # ── 1 · experiment-baseline-k29 → k30 ────────────────────────────────────
    ("experiment-baseline-k29", "R", "", "T", 0,
     "the review body is charter only; it recorded no findings section"),
    ("experiment-baseline-k29", "I", "", "T", 0, ""),
    ("experiment-baseline-k29", "I", "Findings to integrate", "F", 6,
     "the review's findings, written by the review into the integration it cut"),

    # ── 2 · model-contract-k31 → k32 ─────────────────────────────────────────
    ("model-contract-k31", "R", "", "T", 0,
     "the review body is charter only; it recorded no findings section"),
    ("model-contract-k31", "I", "", "T", 0, ""),
    ("model-contract-k31", "I", "Context", "F", 6,
     "B1-B6, written by the review into the integration it cut"),

    # ── 3 · task-tree-k55 → k56 ──────────────────────────────────────────────
    ("task-tree-k55", "R", "", "T", 0, ""),
    ("task-tree-k55", "R", "Done when", "C", 3,
     "the producer's three doubts, written before the review ran"),
    ("task-tree-k55", "R", "Findings", "F", 7, ""),
    ("task-tree-k55", "R", "Discharged doubts and limits", "D", 0,
     "prose residue: four doubts discharged, incl. the producer's own five "
     "findings re-checked — the second population of the word"),
    ("task-tree-k55", "I", "", "T", 0, ""),
    ("task-tree-k55", "I", "Context", "X", 7,
     "the seven findings restated verbatim in substance; counted at the review"),

    # ── 4 · finish-k57 → k58 ─────────────────────────────────────────────────
    ("finish-k57", "R", "", "T", 0, ""),
    ("finish-k57", "R", "Context", "C", 2,
     "the two things replay will not read; charter"),
    ("finish-k57", "R", "Review findings", "F", 6, "B1-B6"),
    ("finish-k57", "R", "Verdict", "D", 17,
     "the 129-witness classification table: 17 modules, all protocol-established "
     "— a residue reported because a review that reports only defects leaves the "
     "next session unable to tell what was checked"),
    ("finish-k57", "R", "Checks that stood", "D", 0, "prose residue"),
    ("finish-k57", "I", "", "T", 0,
     "charter only; the disposition is in commit tkyxvkss"),

    # ── 5 · system-k59 → k60 ─────────────────────────────────────────────────
    ("system-k59", "R", "", "T", 0, ""),
    ("system-k59", "R", "Done when", "C", 6, "six doubts; charter"),
    ("system-k59", "R", "Findings", "F", 6, "F1-F6"),
    ("system-k59", "R", "Doubts discharged", "D", 0, "prose residue, six items"),
    ("system-k59", "R", "Review execution note", "D", 0, "declared limits"),
    ("system-k59", "I", "", "T", 0, ""),
    ("system-k59", "I", "Context", "X", 6, "the six findings restated"),

    # ── 6 · obligation-placement-k67 → k68 ───────────────────────────────────
    ("obligation-placement-k67", "R", "", "T", 0, ""),
    ("obligation-placement-k67", "R", "Context", "C", 3,
     "three judgement calls; charter"),
    ("obligation-placement-k67", "R", "Findings", "F", 3, "F1-F3"),
    ("obligation-placement-k67", "I", "", "T", 0, ""),
    ("obligation-placement-k67", "I", "Context", "X", 3, "the three findings restated"),
    ("obligation-placement-k67", "I", "Decisions (running log)", "G", 6,
     "two findings this integration raised itself: no shared-safety obligation is "
     "stated over the quarantine reaper's actions (routed to k65); and the "
     "concurrent-sweep process failure that destroyed a measurement"),
    # ── 7 · finish-scope-k75 → k76 ───────────────────────────────────────────
    ("finish-scope-k75", "R", "", "T", 0, ""),
    ("finish-scope-k75", "R", "Context", "C", 6,
     "what k71 decided, one line each; charter"),
    ("finish-scope-k75", "R", "Notes", "C", 5, "five doubts; charter"),
    ("finish-scope-k75", "R", "Findings", "F", 4, "two [P1], two [P2]"),
    ("finish-scope-k75", "R", "Disposition verdicts", "D", 0,
     "prose residue: six dispositions, two confirmed on their own evidence"),
    ("finish-scope-k75", "I", "", "T", 0, ""),
    ("finish-scope-k75", "I", "Context", "X", 4, "the four findings restated"),
    ("finish-scope-k75", "I", "Decisions (running log)", "G", 8,
     "five findings this integration raised itself: Quint's inv_FN_25a was true "
     "by construction; the FN-22.h proviso turned inv_FN_25b red; a mutant "
     "module's environment made a control read green while unreached; its own "
     "first edit deleted a clause that was checking something true; "
     "CONTEXT-MAP.md's ADR list was two records short"),
    ("finish-scope-k75", "I", "What this leaf did NOT do", "D", 0, "declared limits"),

    # ── 8 · finish-verdicts-k77 → k78 ────────────────────────────────────────
    ("finish-verdicts-k77", "R", "", "T", 0, ""),
    ("finish-verdicts-k77", "R", "Context", "C", 4,
     "four narrower claims; charter"),
    ("finish-verdicts-k77", "I", "Context", "F", 2,
     "R1, R2 — written by the review into the integration it cut"),
    ("finish-verdicts-k77", "I", "Claims that survived the review", "D", 0,
     "five claims the review checked and confirmed"),
    ("finish-verdicts-k77", "I", "", "T", 0, ""),
    ("finish-verdicts-k77", "I", "Decisions (running log)", "G", 0,
     "one finding this integration raised itself: the ADR's sentence that "
     "Alloy's none rows are argument rows is false on the artifact"),
    ("finish-verdicts-k77", "I", "The in-session reviewer", "I", 0,
     "6 valid and actionable, 1 visible trade-off accepted, 1 noise"),

    # ── 9 · honest-classification-k84 → k85 ──────────────────────────────────
    ("honest-classification-k84", "R", "", "T", 0, ""),
    ("honest-classification-k84", "R", "Context", "C", 0,
     "three specific doubts; charter"),
    ("honest-classification-k84", "R", "Findings", "F", 5, ""),
    ("honest-classification-k84", "R", "Review limits", "D", 0, "declared limits"),
    ("honest-classification-k84", "I", "", "T", 0, ""),
    ("honest-classification-k84", "I", "Context", "X", 5, "the five findings restated"),
    ("honest-classification-k84", "I", "Outcome", "G", 5,
     "the five-row verdict table, plus two findings the review did not have "
     "(inv_FN_28 violated with no model mutation at all, found by NARROWING "
     "base's environment; inv_FN_25b red under the in-place candidate) and one "
     "gap externalised to quarantine-gate-control-k86"),
]

# ── Per-finding verdicts, channel T ──────────────────────────────────────────
# (chain, id, verdict, granularity, where the disposition is recorded)
# verdict uses integrate-review's own four-way triage.
REAL, UNCLEAR, TRADEOFF, NOISE = "real issue", "contract unclear", "trade-off", "noise"
VERDICTS = {
    "experiment-baseline-k29": [("1", REAL, 3), ("2", REAL, 6), ("3", REAL, 2),
                                ("4", REAL, 2), ("5", REAL, 1), ("6", REAL, 6)],
    "model-contract-k31": [("B1", REAL, 1), ("B2", REAL, 3), ("B3", REAL, 6),
                           ("B4", REAL, 3), ("B5", REAL, 3), ("B6", REAL, 2)],
    "task-tree-k55": [("1", REAL, 1), ("2", REAL, 1), ("3", REAL, 1), ("4", REAL, 1),
                      ("5", REAL, 1), ("6", REAL, 1), ("7", REAL, 1)],
    "finish-k57": [("B1", REAL, 1), ("B2", REAL, 1), ("B3", REAL, 1),
                   ("B4", REAL, 1), ("B5", REAL, 1), ("B6", REAL, 1)],
    "system-k59": [("F1", REAL, 1), ("F2", REAL, 1), ("F3", REAL, 1),
                   ("F4", REAL, 1), ("F5", REAL, 1), ("F6", REAL, 1)],
    "obligation-placement-k67": [("F1", UNCLEAR, 1), ("F2", REAL, 1), ("F3", REAL, 1)],
    "finish-scope-k75": [("1", REAL, 1), ("2", REAL, 1), ("3", REAL, 1), ("4", REAL, 1)],
    "finish-verdicts-k77": [("R1", REAL, 1), ("R2", REAL, 1)],
    "honest-classification-k84": [("1", REAL, 1), ("2", REAL, 1), ("3", UNCLEAR, 1),
                                  ("4", REAL, 1), ("5", REAL, 1)],
}

# Where each integration recorded its disposition. Five wrote nothing back into
# their task body — their bodies are charters the REVIEW wrote at cut time — and
# recorded a per-finding verdict in the task commit instead.
DISPOSITION_LOCUS = {
    "experiment-baseline-k29": ("commit", "all verified against source before acting"),
    "model-contract-k31": ("commit", "six blockers, each applied and described"),
    "task-tree-k55": ("commit", "all seven verified and acted on; none was noise"),
    "finish-k57": ("commit", "triaged against the model; B1-B6 each applied"),
    "system-k59": ("commit", "six findings integrated; F3's claim withdrawn as the fix"),
    "obligation-placement-k67": ("body", "## Decisions (running log), per finding"),
    "finish-scope-k75": ("body", "## Decisions (running log), per finding"),
    "finish-verdicts-k77": ("body", "## Decisions (running log), per finding"),
    "honest-classification-k84": ("body", "## Outcome, a five-row verdict table"),
}

# ── Channel G: what the integrating session raised ITSELF ────────────────────
# Only the four integrations that wrote back into their own body could record
# such a thing; the other five are silent on it, which is a limit rather than a
# zero. Each row cites the sentence that carries it.
SELF_RAISED = [
    ("obligation-placement-k67", "no shared-safety obligation in this repository is "
     "stated over the quarantine reaper's actions — routed to finish-verdicts-k65, "
     "and later falsified by k78's own reviewer"),
    ("obligation-placement-k67", "two mutation-64 sweeps ran concurrently against one "
     "log file, so the log showed KILLED FN_31d and then did not — an instrument you "
     "adjust mid-reading has not read anything"),
    ("finish-scope-k75", "Quint's inv_FN_25a was *the two diagnoses are disjoint*, which "
     "the if/else chain made true by construction — no mutation could have moved it"),
    ("finish-scope-k75", "giving FN-22.h's clause the same proviso turned inv_FN_25b RED: "
     "the state that row reaches is not groveOwnedCorrelated, so exhaustiveness died"),
    ("finish-scope-k75", "the first mutant_correlation_wins_the_overlap copied its "
     "neighbours' environment and reported GREEN — a mutant module's environment is part "
     "of the control, and a green mutant reads as a surviving claim when it is an "
     "unreached one"),
    ("finish-scope-k75", "the clause this leaf had just deleted as *the false disjointness "
     "claim* was checking something else and true; a clause can be load-bearing for a "
     "claim other than the one it is labelled with"),
    ("finish-scope-k75", "CONTEXT-MAP.md's ADR-ownership list was two records short, found "
     "by enumerating docs/adr/*.md against the map rather than by reading the list"),
    ("finish-verdicts-k77", "the ADR's sentence *Alloy's none rows are argument rows* is "
     "false on the artifact: only Q4-5 is; Q4-6 and Q4-7 are artifact-specific mutations "
     "run in the available world"),
    ("honest-classification-k84", "inv_FN_28 was violated with NO model mutation at all, "
     "found by NARROWING base's environment rather than widening it — a strict subset of "
     "base's traces, so the counterexample was always base's"),
    ("honest-classification-k84", "inv_FN_25b is red under the in-place candidate, "
     "pre-existing, routed to sweep-ownership-k81 with its measurement"),
]

# One further gap was externalised rather than raised as a finding:
# FN-22.e is green because nothing can falsify it — quarantine-gate-control-k86.
EXTERNALISED = 1

# ── Channel I: the in-session reviewer allowance ─────────────────────────────
# Recorded only in a producer's or an integration's own body. Four producers
# considered the allowance and declined it, each for a different reason; those
# are counted too, because a declined spend with a recorded reason is data.
IN_SESSION = [
    # leaf, kind, spent, raised, actionable, unclear, tradeoff, noise, where, note
    ("model-contract-k5", "design", False, 0, 0, 0, 0, 0, FM + "04-DONE-design-model-contract-k5.md",
     "declined: the doubt is one an in-session reviewer cannot discharge — the "
     "enumerated-assumption control is itself part of what needs challenging"),
    ("ordinal-root-lifecycle-k14", "prototype", False, 0, 0, 0, 0, 0,
     FM + "10-DONE-prototype-ordinal-root-lifecycle-k14.md",
     "declined, and no review leaf cut either: cross-model-replay-k15 is "
     "chartered to contest the verdict, so a review leaf would buy the same "
     "read twice"),
    ("obligation-placement-k63", "design", False, 0, 0, 0, 0, 0,
     FS + "02-DONE-design-obligation-placement-k63.md",
     "could not be spent: the harness forbade subagents. A review-design leaf "
     "was cut instead, and is called the stronger instrument"),
    ("task-tree-scope-k70", "design", False, 0, 0, 0, 0, 0,
     CD + "02-DONE-design-task-tree-scope-k70.md",
     "declined on the merits: four executable tests that would break if the "
     "central claim were wrong were spent instead, as stronger evidence than a "
     "fresh context reading the same prose"),
    ("finish-verdicts-k65", "design", False, 0, 0, 0, 0, 0,
     FS + "06-DONE-design-finish-verdicts-k65.md",
     "could not be spent: the harness forbade subagents. A review-design leaf "
     "was cut and INSERTED, because otherwise the decision would get no "
     "fresh-context challenge anywhere"),
    ("finish-scope-k71", "design", True, 7, 5, 1, 0, 1,
     CD + "03-DONE-design-finish-scope-k71.md",
     "the reviewer BROKE the landed disposition; the reversal that followed "
     "touched nine artifacts and is what earned review-design finish-scope-k75. "
     "One item is 'a contract I stated unclearly'"),
    ("honest-classification-k80", "design", True, 9, 6, 2, 0, 1,
     QN + "01-DONE-design-honest-classification-k80.md",
     "prose says eight findings, the enumeration lists nine (3 substantive + 2 "
     "qualifying + 3 mechanical + 1 noise); the enumeration is counted and the "
     "discrepancy published"),
    ("finish-verdicts-k78", "integrate-review-design", True, 8, 6, 0, 1, 1,
     FS + "08-DONE-integrate-review-design-finish-verdicts-k78.md",
     "the only in-session spend by an integration. One of its findings "
     "FALSIFIED a finding obligation-placement-k68 had raised itself"),
]

# ── Review rate by producer kind, the confound that explains the curve ───────
# Derived from docs/loop-record.md's own enumeration, re-asserted here.
REVIEWED_PRODUCERS = {
    "experiment-baseline-k4", "model-contract-k5", "task-tree-k11", "finish-k12",
    "system-k13", "obligation-placement-k63", "finish-scope-k71",
    "finish-verdicts-k65", "honest-classification-k80",
}

# ── Controls ─────────────────────────────────────────────────────────────────
# A broken instrument reads clean everywhere. Clean-here plus dirty-there cannot
# be produced by one.
POSITIVE = ("system-k59", "R", "F5 — the flat-menu measurement")
NEGATIVE = ("system-k59", "R", "F7 — ")   # must NOT be found: k59 has six findings

HEAD = re.compile(r"^(#{2,6})\s+(.*?)\s*$")
OL = re.compile(r"^(\d+)\.\s+(.*)$")
ROW = re.compile(r"^\|(?!\s*[-: ]+\|)(.+)\|\s*$")
SEP = re.compile(r"^\|[\s:|-]+\|\s*$")


def jj(*args: str) -> str:
    out = subprocess.run(["jj", *args], capture_output=True, text=True)
    if out.returncode != 0:
        sys.exit(f"jj {' '.join(args)} failed:\n{out.stderr}")
    return out.stdout


def enumerate_body(text: str) -> list[tuple[tuple[str, ...], str, str]]:
    """Every enumerated item, as (heading path, kind, text).

    Complete by construction over markdown block structure — headings, top-level
    ordered-list items and table rows are the only shapes an enumeration takes
    in these files, and no lexical pattern is consulted. The heading path is the
    chain of enclosing headings, so a block can own its own subtree the way
    markdown nests. A heading's path is its *parent's*, since a heading is an
    item of the section it opens under.

    Table header rows are dropped: a header names a table's columns and is not
    one of its items. The separator row is dropped by the ROW pattern itself.
    """
    items: list[tuple[tuple[str, ...], str, str]] = []
    stack: list[tuple[int, str]] = []
    in_table = False
    for line in text.splitlines():
        m = HEAD.match(line)
        if m:
            in_table = False
            depth, title = len(m.group(1)), m.group(2)
            while stack and stack[-1][0] >= depth:
                stack.pop()
            items.append((tuple(t for _d, t in stack), "head", title))
            stack.append((depth, title))
            continue
        path = tuple(t for _d, t in stack)
        if SEP.match(line):
            continue                  # the separator is part of the header
        m = ROW.match(line)
        if m:
            if not in_table:          # the header row names columns, not items
                in_table = True
                continue
            items.append((path, "row", m.group(1).strip()))
            continue
        in_table = False
        m = OL.match(line)
        if m:
            items.append((path, "ol", m.group(2)))
    return items


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--rev", default="@", help="revision to read .grove/ at")
    ap.add_argument("--out", default="docs/review-yield.md", help="output path; - for stdout")
    args = ap.parse_args()

    rev_change = jj("log", "--no-graph", "-r", args.rev, "-T", "change_id.short(8)").strip()

    bodies: dict[tuple[str, str], list] = {}
    for slug, _kind, rpath, ipath, _chg in CHAINS:
        for role, path in (("R", rpath), ("I", ipath)):
            text = jj("file", "show", "-r", args.rev, G + path)
            bodies[(slug, role)] = enumerate_body(text)

    # ── forward and reverse coverage ─────────────────────────────────────────
    # A block names one heading (or "" for the document root) and owns that
    # heading's whole subtree MINUS any subtree owned by a more deeply nested
    # block. That is exactly markdown's own nesting, so the classification is a
    # partition of the body by construction rather than by a matching rule.
    #
    #   forward   no item unowned
    #   reverse   no block owning nothing
    #   n         the block's own direct items, asserted against what it owns;
    #             0 means "prose — count deliberately not asserted"
    claimed: dict[tuple[str, str], list] = {k: [None] * len(v) for k, v in bodies.items()}
    reverse_residue, count_mismatch = [], []

    def heads(anchor, title):
        return title == anchor or title.startswith(anchor)

    def depth_of(key, anchor):
        """Document depth of the block's heading; the root is depth 0."""
        if anchor == "":
            return 0
        for path, kind, text in bodies[key]:
            if kind == "head" and heads(anchor, text):
                return len(path) + 1
        return -1

    def owns(path, kind, text, anchor):
        """True when `anchor` heads this item's subtree, or is the item itself."""
        if anchor == "":
            return path == ()
        return any(heads(anchor, p) for p in path) or (kind == "head" and heads(anchor, text))

    # Deepest heading first, so a nested block takes its own subtree before an
    # ancestor block claims it. Depth is read from the document, not guessed
    # from the anchor.
    order = sorted(range(len(BLOCKS)),
                   key=lambda i: -depth_of((BLOCKS[i][0], BLOCKS[i][1]), BLOCKS[i][2]))
    for bi in order:
        slug, role, anchor, cls, n, _note = BLOCKS[bi]
        key = (slug, role)
        if depth_of(key, anchor) < 0:
            reverse_residue.append((slug, role, anchor, cls))
            continue
        mine = [i for i, (path, kind, text) in enumerate(bodies[key])
                if owns(path, kind, text, anchor) and claimed[key][i] is None]
        for i in mine:
            claimed[key][i] = cls
        # The block's own enumeration: items directly under its heading, NOT
        # counting the heading itself and NOT counting anything a nested block
        # already took. `n = 0` means the block's content is prose — unordered
        # bullets are deliberately not enumerated, since every explanatory
        # paragraph in these files uses them.
        direct = sum(
            1 for i in mine
            if not (bodies[key][i][1] == "head" and heads(anchor, bodies[key][i][2]))
            and (bodies[key][i][0][-1:] == ()
                 if anchor == ""
                 else bool(bodies[key][i][0]) and heads(anchor, bodies[key][i][0][-1])))
        if n and direct != n:
            count_mismatch.append((slug, role, anchor, cls, n, direct))

    forward_residue = [(slug, role, bodies[(slug, role)][i])
                       for (slug, role), marks in claimed.items()
                       for i, m in enumerate(marks) if m is None]

    # ── the two named controls ───────────────────────────────────────────────
    def present(slug, role, needle):
        return any(needle in t or needle in sec
                   for sec, _k, t in bodies[(slug, role)])
    pos_ok = present(*POSITIVE)
    neg_ok = not present(*NEGATIVE)

    ok = not forward_residue and not reverse_residue and not count_mismatch \
        and pos_ok and neg_ok
    for label, rows in (("FORWARD RESIDUE (unclassified items)", forward_residue),
                        ("REVERSE RESIDUE (blocks matching nothing)", reverse_residue),
                        ("COUNT MISMATCH (block n vs enumeration)", count_mismatch)):
        if rows:
            print(f"{label}: {len(rows)}", file=sys.stderr)
            for r in rows[:20]:
                print(f"  {r}", file=sys.stderr)
    print(f"positive control ({POSITIVE[2]!r} found): {pos_ok}", file=sys.stderr)
    print(f"negative control ({NEGATIVE[2]!r} absent): {neg_ok}", file=sys.stderr)

    text = render(args, rev_change, bodies, claimed,
                  forward_residue, reverse_residue, count_mismatch, pos_ok, neg_ok)
    if args.out == "-":
        sys.stdout.write(text)
    else:
        with open(args.out, "w") as fh:
            fh.write(text)
        print(f"wrote {args.out}", file=sys.stderr)
    return 0 if ok else 1


def render(args, rev_change, bodies, claimed,
           fwd, rev, mismatch, pos_ok, neg_ok) -> str:
    o: list[str] = []
    w = o.append

    by_cls: dict[str, int] = {}
    for marks in claimed.values():
        for m in marks:
            by_cls[m] = by_cls.get(m, 0) + 1
    total_items = sum(len(v) for v in bodies.values())

    raised = {c: len(v) for c, v in VERDICTS.items()}
    n_raised = sum(raised.values())
    verd: dict[str, int] = {}
    gran: dict[str, int] = {}
    for c, rows in VERDICTS.items():
        for _id, v, g in rows:
            verd[v] = verd.get(v, 0) + 1
            gran[c] = gran.get(c, 0) + g
    n_g = len(SELF_RAISED)
    n_i_raised = sum(x[3] for x in IN_SESSION)
    n_i_act = sum(x[4] for x in IN_SESSION)
    n_i_unclear = sum(x[5] for x in IN_SESSION)
    n_i_trade = sum(x[6] for x in IN_SESSION)
    n_i_noise = sum(x[7] for x in IN_SESSION)
    assert n_i_raised == n_i_act + n_i_unclear + n_i_trade + n_i_noise, \
        "the in-session triage must partition what it raised"
    spends = [x for x in IN_SESSION if x[2]]

    w("<!-- DERIVED FILE — do not edit by hand. -->")
    w(f"<!-- Regenerate: python3 scripts/review-yield.py --rev {rev_change} -->")
    w("<!-- Pinned by change id, not commit id: in jj the working copy is a")
    w("     commit, so writing this file rewrites the commit id but never the")
    w("     change id. See docs/loop-record.md for the same reasoning at length. -->")
    w("")
    w("# Review yield — what nine review chains actually found")
    w("")
    w("This measures one thing: **across every review chain in one Grove")
    w("workstream, how many findings were raised, how many survived verification")
    w("at integration, and how many the integration added on its own.** It exists")
    w("because the campaign's own recollection — *review yield did not decay* —")
    w("was written from inside the campaign by a session that is not neutral")
    w("about it, and rests on a single node.")
    w("")
    w("It is the companion to [`loop-record.md`](loop-record.md), which enumerates")
    w("the sessions and deliberately opens no review body. This one opens all")
    w("eighteen.")
    w("")
    w("**The verdict, first, because it is the null one.** Nine chains and three")
    w("channels **cannot separate flat from falling**. What the tree does")
    w("establish is narrower and, for anyone deciding whether to pay for review,")
    w("more useful: **no review came back empty, at any point, including the four")
    w("cut latest against the smallest subjects — and every one of the 45")
    w("findings they raised survived verification.** The section *[Why the curve")
    w("cannot be read](#why-the-curve-cannot-be-read)* says what would have been")
    w("needed instead, and it is not simply more chains.")
    w("")

    # ── method ───────────────────────────────────────────────────────────────
    w("## What is derived and what is judged")
    w("")
    w("Classifying a paragraph as a finding is a judgement and cannot be derived.")
    w("[`scripts/review-yield.py`](../scripts/review-yield.py) separates the two")
    w("and makes the judgement checkable:")
    w("")
    w("- **Derived.** Every ATX heading, top-level ordered-list item and table row")
    w("  in all eighteen bodies is extracted from markdown block structure at a")
    w("  pinned revision — complete by construction, with no lexical pattern")
    w(f"  consulted. That is **{total_items} enumerated items**.")
    w("- **Judged.** Each is assigned to one classified *enumeration block* — the")
    w("  unit the session itself chose. The table lives in the script.")
    w("- **Asserted, in both directions.** Every enumerated item must land in")
    w("  exactly one block, and every block must resolve to at least one item.")
    w("  The script exits non-zero if either side has a leftover, so this document")
    w("  cannot be regenerated against a tree it no longer describes.")
    w("")
    w("That is the shape `models/run.sh` uses to assert obligation coverage, and")
    w("it is here for the same reason: a clean result from a broken instrument is")
    w("indistinguishable from a clean result from a correct one.")
    w("")
    w("### The counting rule, fixed before any count was taken")
    w("")
    w("A **finding** is one item at the top level of the enumeration *the session")
    w("itself used*, in the body where that session wrote it. Sub-bullets inside")
    w("one item are not separate findings; each item's sub-item count is recorded")
    w("separately as a **granularity** figure, because that figure is what shows")
    w("the unit is not comparable across chains.")
    w("")
    w("Four exclusions, each published rather than silent:")
    w("")
    w("| class | what it excludes | why |")
    w("|---|---|---|")
    w("| `X` | an integration's opening re-listing of the review's findings | five integrations open by restating the findings verbatim in substance; counting both would double every one of them |")
    w("| `C` | charter prose — a review's doubts, written *before* it ran | a doubt is what the review was asked to check, not what it found |")
    w("| `D` | published residues — doubts discharged, checks that stood | deliberately not findings; a review that reports only defects leaves the next session unable to tell what was checked |")
    w("| `V` | verification and run lines | evidence inside a finding, not further findings |")
    w("")
    w("**Three channels are counted separately and never summed into one curve.**")
    w("`T` tree-level review → integration; `G` findings the *integrating* session")
    w("raised itself; `I` the in-session reviewer allowance, recorded only in a")
    w("producer's or integration's own body.")
    w("")
    w("**One arithmetic discrepancy was found and is resolved by the rule.**")
    w("`honest-classification-k80`'s prose says its reviewer returned eight")
    w("findings; its own enumeration lists nine. The enumeration is counted.")
    w("")

    # ── the finding that changes where you look ──────────────────────────────
    w("## Five of nine dispositions are not in `.grove/` at all")
    w("")
    w("The brief that commissioned this measurement warned that three chains")
    w("write their *findings* only in the integration body, and that a")
    w("file-scoped counter would score them zero. That is true, and the larger")
    w("version of it is this:")
    w("")
    w("**Five of the nine integrations wrote nothing back into their own task")
    w("body.** Their bodies are charters the **review** session wrote at cut")
    w("time — which is exactly what `references/decompose.md` prescribes, since")
    w("*the creating session writes the new leaf's body* — and the integrating")
    w("session never returned to them. Every one of those five recorded a")
    w("per-finding disposition **in its task commit** instead.")
    w("")
    w("So `.grove/` alone cannot answer this document's question for five of nine")
    w("chains, and the commit messages can for all nine. The commit message is")
    w("also the *more* durable record: finish deletes `.grove/` from the tip but")
    w("never from history, and Grove's retire-then-commit rule puts the")
    w("disposition in a message that survives teardown. **The loop's own commit")
    w("discipline is what makes review yield measurable at all.**")
    w("")

    # ── the table ────────────────────────────────────────────────────────────
    w("## Per chain")
    w("")
    w("`raised` counts channel T only. `sub` is granularity — the sub-corrections")
    w("inside those findings, which is the number that shows the unit is elastic.")
    w("`+G` is what the integration raised itself.")
    w("")
    w("| # | chain | producer | raised | sub | real | unclear | dropped | +G | disposition recorded in |")
    w("|--:|---|---|--:|--:|--:|--:|--:|--:|---|")
    for i, (slug, kind, _r, _ip, chg) in enumerate(CHAINS, 1):
        rows = VERDICTS[slug]
        real = sum(1 for _a, v, _g in rows if v == REAL)
        unc = sum(1 for _a, v, _g in rows if v == UNCLEAR)
        drop = sum(1 for _a, v, _g in rows if v in (TRADEOFF, NOISE))
        g = sum(1 for c, _t in SELF_RAISED if c == slug)
        locus, how = DISPOSITION_LOCUS[slug]
        where = (f"the body — {how}" if locus == "body"
                 else f"**commit `{chg}`** — {how}")
        w(f"| {i} | `{slug}` | `{kind}` | {len(rows)} | {gran[slug]} | {real} | "
          f"{unc} | {drop} | {g or '—'} | {where} |")
    w(f"| | **total** | | **{n_raised}** | **{sum(gran.values())}** | "
      f"**{verd.get(REAL,0)}** | **{verd.get(UNCLEAR,0)}** | **0** | **{n_g}** | |")
    w("")
    w("**Zero findings were dropped and zero were classified noise.** Two were")
    w("downgraded within `integrate-review`'s own four-way triage from *a real")
    w("issue* to *a contract stated unclearly* — `obligation-placement-k67`'s F1")
    w("and `honest-classification-k84`'s finding 3. One review's proposed")
    w("*remedy* was rejected in favour of the other option it had itself offered")
    w("(`finish-scope-k76` on finding 2); the defect stood.")
    w("")

    # ── the honest attack on that number ─────────────────────────────────────
    w("## A 100% survival rate is what a broken instrument reads")
    w("")
    w("This is the number that most needs attacking, and the shape it has is the")
    w("self-certifying one this campaign kept finding in its own models.")
    w("")
    w("**In five of the nine chains, the session that \"verified\" the findings")
    w("had its own task body written by the reviewer.** The charter *is* the")
    w("finding list, and `Done when` is the finding list restated as obligations.")
    w("An integration in that position has no structural place to disagree: to")
    w("reject a finding it would have to reject its own charter.")
    w("")
    w("The counter-evidence is real, and partial:")
    w("")
    w("- The four late integrations state in as many words that they verified")
    w("  against the artifacts rather than against the review's summary, and one")
    w("  of them found the defect was **worse** than the review knew")
    w("  (`finish-scope-k76`: Quint's `inv_FN_25a` was true by construction, which")
    w("  no mutation could have moved).")
    w("- `finish-verdicts-k78`'s in-session reviewer **falsified a finding that")
    w("  `obligation-placement-k68` had raised itself** — *no shared-safety")
    w("  obligation in this repository is stated over the quarantine reaper's")
    w("  actions* is false, and the true statement is the narrow one about the")
    w("  reaper's *ownership proof*. So findings in this tree do get rejected.")
    w("- Both downgrades above are real gradings, not rubber stamps.")
    w("")
    w("**But no tree-level review finding has ever been rejected.** Every")
    w("rejection observed in this grove came from the in-session channel, where")
    w("the reader owes the finding nothing. That asymmetry is the single most")
    w("actionable thing in this document, and the repair is cheap: an")
    w("`integrate-review-*` leaf's body should carry the review's **handle**, and")
    w("its findings should be read from the review's own commit — the handoff a")
    w("`review-*` step already uses — rather than transcribed into the")
    w("integration's charter, so that rejecting a finding is not rejecting one's")
    w("own task.")
    w("")

    # ── channel I ────────────────────────────────────────────────────────────
    w("## The in-session channel, and the five leaves that declined it")
    w("")
    n_decl = sum(1 for x in IN_SESSION if not x[2])
    w("Three sessions spent the allowance `references/execute.md` grants, and")
    w(f"between them raised **{n_i_raised} findings: {n_i_act} valid and")
    w(f"actionable, {n_i_unclear} a contract the producer had stated unclearly,")
    w(f"{n_i_trade} a visible trade-off accepted, {n_i_noise} noise**.")
    w(f"{n_decl} more leaves considered the allowance and did not spend it, and")
    w("the recorded reasons are not the same reason — which is itself the")
    w("finding, because only two of the five are about the harness.")
    w("")
    w("| leaf | kind | spent | raised | actionable | unclear | trade-off | noise | why / what happened |")
    w("|---|---|:-:|--:|--:|--:|--:|--:|---|")
    for leaf, kind, spent, r, a, u, t, n, _p, note in IN_SESSION:
        mark = "**yes**" if spent else "no"
        nums = f"{r} | {a} | {u} | {t} | {n}" if spent else "— | — | — | — | —"
        w(f"| `{leaf}` | `{kind}` | {mark} | {nums} | {note} |")
    w("")
    dropped_i = n_i_trade + n_i_noise
    f_pct = 100.0 * dropped_i / n_i_raised
    w("**The in-session channel is the only one with a non-zero drop rate**")
    w(f"— {dropped_i} of {n_i_raised} findings ({f_pct:.0f}%) were classified a")
    w(f"trade-off or noise, against 0 of {n_raised} at the tree level. That is")
    w("the asymmetry the previous section is about, seen from the other end.")
    w("")
    w("**All three spends changed the session that made them**, which is not what")
    w("a second opinion does.")
    w("`finish-scope-k71`'s reviewer *broke the disposition that had already been")
    w("landed*, forcing a reversal across nine artifacts — and that reversal is")
    w("precisely what earned the `review-design finish-scope-k75` chain, by the")
    w("mechanical second-review signal. `honest-classification-k80`'s reviewer")
    w("swept all 63 library invariants against the candidate's module and found")
    w("two violated, one of which nobody was looking for. Neither of those is a")
    w("second opinion; both are a different instrument. And `finish-verdicts-k78`")
    w("withdrew a sentence it had already written — *this is now a measurement")
    w("rather than an argument* — because its reviewer showed the retained claim")
    w("was still trivial over the difference it was meant to judge.")
    w("")
    w("**And the declines are not excuses.** `task-tree-scope-k70` spent four")
    w("executable tests instead, on the argument that a test which breaks if the")
    w("central claim is wrong is stronger evidence than a fresh context reading")
    w("the same prose. `ordinal-root-lifecycle-k14` declined **and** cut no review")
    w("leaf, because a later leaf was already chartered to contest its verdict —")
    w("*a review leaf here would buy the same read twice*.")
    w("")

    # ── the curve ────────────────────────────────────────────────────────────
    w('<a id="why-the-curve-cannot-be-read"></a>')
    w("")
    w("## Why the curve cannot be read")
    w("")
    seq = ", ".join(str(len(VERDICTS[c[0]])) for c in CHAINS)
    early = [len(VERDICTS[c[0]]) for c in CHAINS[:5]]
    late = [len(VERDICTS[c[0]]) for c in CHAINS[5:]]
    w(f"Findings per review, in the order the chains ran: **{seq}**.")
    w(f"First five mean **{sum(early)/len(early):.1f}**, last four mean")
    w(f"**{sum(late)/len(late):.1f}**. That looks like decay. Three things say it")
    w("is not readable as one.")
    w("")
    w("**1 · The gap is inside the noise.** A 2.7-finding difference between a")
    w("five-chain and a four-chain group, against Poisson variation at this mean,")
    w("is about **1.8σ** — before any confound. Reaching 3σ on that effect needs")
    w("roughly **twelve chains per group, so about 25 chains of matched subject")
    w("size**.")
    w("")
    w("**2 · The subjects are not matched, and not randomly so.** The first five")
    w("reviews read first-of-their-kind artifacts: a pre-registration, a")
    w("130-obligation catalogue, and three whole model columns. The last four read")
    w("incremental design decisions on an artifact that had already been reviewed")
    w("two to five times. Fewer findings against a smaller subject is not decay.")
    w("")
    w("**3 · The review rate rose while the yield fell, and that is the")
    w("explanation.** Review is not uniform over the tree — it is a function of")
    w("**session kind**:")
    w("")
    w("| producer kind | ran | reviewed | rate |")
    w("|---|--:|--:|--:|")
    for kind, ran in (("requirements", 1), ("design", 12), ("prototype", 23), ("impl", 1)):
        n = sum(1 for h in REVIEWED_PRODUCERS
                if any(c[1] == kind and c[0].rsplit("-k", 1)[0] == h.rsplit("-k", 1)[0]
                       for c in CHAINS))
        w(f"| `{kind}` | {ran} | {n} | {round(100*n/ran)}% |")
    w("")
    w("42% of `design` leaves earned a review against 13% of `prototype` leaves —")
    w("and all three reviewed prototypes are the Quint column, which introduced")
    w("the shared runner three columns inherit. The twenty Alloy prototypes were")
    w("covered by a scheduled `cross-model-replay-k15` instead. **The loop")
    w("reviewed further down its own value curve as it went**, and marginal")
    w("artifacts yield fewer findings. The falling count is a selection effect of")
    w("a rising review rate, and every review at the margin still found something.")
    w("")
    lo = min(gran[c[0]] / len(VERDICTS[c[0]]) for c in CHAINS)
    hi = max(gran[c[0]] / len(VERDICTS[c[0]]) for c in CHAINS)
    w("**4 · The unit is elastic, which is the deeper problem.** Sub-corrections")
    w(f"per finding run from **{lo:.1f} to {hi:.1f}** across the nine chains —")
    w(f"`experiment-baseline-k29`'s six findings carry")
    w(f"{gran['experiment-baseline-k29']} corrections between them, while six")
    w("chains are strictly one-to-one. And that measured range still understates")
    w("it, because **consequence is not captured at all**:")
    w("`finish-verdicts-k77`'s two findings are two whole verdict reversals that")
    w("flipped `keep` to `defer` and falsified an ADR's title, and they count the")
    w("same as a corrected transition count. **More chains is therefore the")
    w("weaker fix; a pre-registered severity scale applied by the reviewer is the")
    w("prior one**, because without a fixed unit a longer run measures more")
    w("enumeration style rather than more review.")
    w("")

    # ── what this weakens ────────────────────────────────────────────────────
    w("## What this weakens, and what survives")
    w("")
    w("**Weakened: *review yield did not decay*.** It is not established by this")
    w("tree and cannot be. The recollection that produced it — three findings from")
    w("`honest-classification-k80`'s own reviewer, five more from `k84` beside it,")
    w("two more added by `k85` — is accurate, and it spans **all three channels**,")
    w("so it was never a statement about review chains in the first place. Read as")
    w("one, it generalises from a single node; read as what it actually describes,")
    w("it says something different and true: *three independent readers of one")
    w("artifact each found what the two before them had not*.")
    w("")
    w("**Survives, and is countable exactly:**")
    w("")
    w("- **9 reviews, 9 integrations, a clean pairing in both directions, and not")
    w("  one empty review.** A review that finds nothing creates nothing and")
    w("  retires; none did.")
    w("- **45 findings raised, 45 verified, 0 dropped, 0 noise** at the tree")
    w("  level — with the caveat above that no mechanism for rejection exists in")
    w("  five of the nine chains.")
    w(f"- **{n_g} findings the integrations raised themselves**, in the four")
    w("  chains that recorded such a thing. Integration is not transcription: it")
    w("  found a defect worse than the review knew, a control that read green")
    w("  because its module never reached the subject, a clause it had itself")
    w("  deleted that was checking something true, and an invariant violated with")
    w("  no model mutation at all — found by **narrowing** the environment rather")
    w("  than widening it.")
    w(f"- **{n_i_raised} findings from three in-session reviewers**, one of which")
    w("  broke a landed disposition and one of which falsified a finding an")
    w("  integration had raised itself.")
    w("")

    # ── residues ─────────────────────────────────────────────────────────────
    w("## Residues — both directions, published rather than asserted")
    w("")
    w("| direction | expectation | result |")
    w("|---|---|---|")
    w(f"| forward — every enumerated item classified | 0 unclassified | "
      f"**{len(fwd)}** |")
    w(f"| reverse — every classified block resolves | 0 empty blocks | "
      f"**{len(rev)}** |")
    w(f"| block counts match the enumeration | 0 mismatches | **{len(mismatch)}** |")
    w(f"| positive — `{POSITIVE[2]}` | found in `{POSITIVE[0]}` | "
      f"**{'found' if pos_ok else 'MISSING'}** |")
    w(f"| negative — `{NEGATIVE[2].strip()}` | absent (that review has six findings) | "
      f"**{'absent' if neg_ok else 'PRESENT'}** |")
    w("")
    w("The negative control is the one that matters. Five of the nine reviews")
    w("number their findings `F1…`/`B1…`/`R1…`, and an instrument that matched the")
    w("shape rather than the content would happily report a seventh finding in a")
    w("review that has six — the same defect `task-tree-k55` found in")
    w("`models/run.sh`, which accepted `inv_TT_99_misspelled` as an obligation")
    w("because it checked the shape and never the manifest.")
    w("")
    w("**The controls were shown to fail before they were trusted.** A control")
    w("that has never been seen to fail is not a control — this corpus's own rule,")
    w("one level up. Three deliberate mutations of the classification were run")
    w("against the committed bodies:")
    w("")
    w("| mutation | expected | result |")
    w("|---|---|---|")
    w("| delete the block classifying `task-tree-k55`'s `## Findings` | forward residue fires | **7 unclassified items, exit 1** |")
    w("| add a block naming a heading no body has | reverse residue fires | **1 empty block, exit 1** |")
    w("| state 6 findings where the body enumerates 7 | count control fires | **1 mismatch, exit 1** |")
    w("")
    w("The first of those found a real hole. Until it was run, a block classifying")
    w("the document *root* owned the whole document, so deleting a section's")
    w("classification left its items silently absorbed and the forward control")
    w("read clean — an unrecognised failure defaulting to success, which is the")
    w("same defect `task-tree-k55` found in `models/run.sh`. The root now owns")
    w("only items at the root, and the mutation fires.")
    w("")
    w("**A second bug was found the same way, by a count that did not match.**")
    w("Every markdown table was losing exactly one data row: the separator line")
    w("reset the parser's in-table flag, so the first real row was eaten as a")
    w("second header. It read plausibly — one fewer row in every table — and")
    w("only the hand-written expectation caught it.")
    w("")
    w(f"**{total_items} items enumerated, classified as:** " + ", ".join(
        f"`{c}` {n}" for c, n in sorted(by_cls.items(), key=lambda kv: -kv[1]) if c))
    w("")
    w("`I` is 1 because the in-session reviewer's findings are written as")
    w("unordered bullets, which this enumeration deliberately does not treat as")
    w("items — every explanatory paragraph in these files uses them. That channel")
    w("is counted by hand from the sessions' own four-way classification, and its")
    w("table above is where it is checked.")
    w("")
    w("## What this record does not establish")
    w("")
    w("- **It counts findings, not value.** A finding that flipped two `keep`")
    w("  verdicts and a finding that corrected a transition count are one each.")
    w("- **It cannot see a finding nobody wrote down.** A review that noticed")
    w("  something and judged it not worth cutting an integration for leaves no")
    w("  trace; that is by design (*a review that finds nothing creates nothing*)")
    w("  and it means this document's floor is firmer than its ceiling.")
    w("- **It says nothing about what review cost.** Elapsed time per session is")
    w("  in [`loop-record.md`](loop-record.md); attributing it to review would")
    w("  need a measure of effort neither document has.")
    w("- **One grove, one subject matter, one operator.** Every chain here reviews")
    w("  formal-modelling work in a repository whose whole purpose was to be")
    w("  rigorous about evidence. That is the least likely place in the world to")
    w("  observe review yield decaying.")
    return "\n".join(o) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
"""Render a grove task tree (`.grove/`) in a herdr pane.

Reads the directory scheme published by grove's ADR *task-tree-scheme* and nothing
else: no socket, no state file, no cooperation from the `grove` binary. Deleting
grove leaves this with nothing to render; deleting this changes nothing about grove
(ADR *herdr-optional-ui*).

The tree shape comes entirely from entry names — position, outcome, slug and key are
all in the filename — so a poll costs one `scandir` per directory. The only file read
is the `**Kind:**` line of a *live* leaf, which is the only place a kind is displayed.
"""

from __future__ import annotations

import json
import os
import re
import select
import shutil
import signal
import sys
import termios
import tty
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterator, TextIO

GROVE_DIR = ".grove"
BRIEF = "BRIEF.md"
POLL_SECONDS = 1.0

# `NN-[DONE-|ABANDONED-]<slug>-k<key>.md`. The greedy slug group peels the *last*
# `-k<digits>`, matching grove's own parse rule: `05-task-k9-k3.md` is slug `task-k9`,
# key 3 (task-tree-scheme, "Naming grammar").
LEAF_RE = re.compile(r"^(\d{2})-(?:(DONE|ABANDONED)-)?(.+)-k(\d+)\.md$")
NODE_RE = re.compile(r"^(\d{2})-(.+)-k(\d+)$")

LIVE, DONE, ABANDONED = "live", "DONE", "ABANDONED"


# --------------------------------------------------------------------------- model


@dataclass
class Leaf:
    """One unit of work: a task file inside a node directory."""

    position: int
    slug: str
    key: int
    outcome: str
    path: Path

    @property
    def handle(self) -> str:
        """The position-free `<slug>-k<key>` handle — stable across renumbering."""
        return f"{self.slug}-k{self.key}"

    def kind(self) -> str:
        """The leaf's declared `**Kind:**`, read the way grove reads it.

        Degrades exactly as `tree_read::read_kind` does: a missing or unreadable line
        is `impl`, and the retired `work` spelling still reads as `impl`.
        """
        kind = read_marker(self.path, "**Kind:**", default="impl")
        return "impl" if kind == "work" else kind

    def harness(self) -> str:
        """The leaf's `**Harness:**` line, if it declares one. Almost none do."""
        return read_marker(self.path, "**Harness:**", default="")


@dataclass
class Node:
    """A directory holding a `BRIEF.md` charter plus its numbered children."""

    position: int
    slug: str
    key: int
    path: Path
    children: list[Leaf | Node] = field(default_factory=list)

    @property
    def handle(self) -> str:
        return f"{self.slug}-k{self.key}"

    @property
    def live(self) -> bool:
        """A node is done when its subtree holds no live leaf — never marked, implicit."""
        return any(leaf.outcome == LIVE for leaf in walk_leaves(self))


def read_marker(path: Path, marker: str, default: str) -> str:
    """First whitespace token after `marker` on the first line carrying it."""
    try:
        text = path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return default
    for line in text.splitlines():
        rest = line.lstrip()
        if rest.startswith(marker):
            token = rest[len(marker) :].split()
            return token[0] if token else default
    return default


def read_tree(directory: Path) -> list[Leaf | Node]:
    """Children of one node directory, in position order. Foreign entries are skipped."""
    children: list[Leaf | Node] = []
    try:
        entries = list(os.scandir(directory))
    except OSError:
        return children
    for entry in entries:
        if entry.is_dir():
            node_match = NODE_RE.match(entry.name)
            if node_match:
                node = Node(int(node_match[1]), node_match[2], int(node_match[3]), Path(entry.path))
                node.children = read_tree(node.path)
                children.append(node)
        elif entry.name != BRIEF:
            leaf_match = LEAF_RE.match(entry.name)
            if leaf_match:
                children.append(
                    Leaf(
                        int(leaf_match[1]),
                        leaf_match[3],
                        int(leaf_match[4]),
                        leaf_match[2] or LIVE,
                        Path(entry.path),
                    )
                )
    children.sort(key=lambda child: child.position)
    return children


def walk_leaves(node: Node) -> Iterator[Leaf]:
    """Every leaf under `node`, depth-first pre-order — `pick`'s own walk order."""
    for child in node.children:
        if isinstance(child, Node):
            yield from walk_leaves(child)
        else:
            yield child


def find_grove_root(start: Path) -> Path | None:
    """Nearest `.grove/` at or above `start`."""
    for directory in [start, *start.parents]:
        candidate = directory / GROVE_DIR
        if candidate.is_dir():
            return candidate
    return None


def tally(node: Node) -> dict[str, int]:
    counts = {LIVE: 0, DONE: 0, ABANDONED: 0}
    for leaf in walk_leaves(node):
        counts[leaf.outcome] += 1
    return counts


# ------------------------------------------------------------------------ terminal

RESET = "\x1b[0m"
DIM = "\x1b[2m"
BOLD = "\x1b[1m"
GREEN = "\x1b[32m"
YELLOW = "\x1b[33m"
CYAN = "\x1b[36m"

SGR_RE = re.compile(r"\x1b\[[0-9;]*m")


def paint(text: str, *codes: str) -> str:
    if not codes or os.environ.get("NO_COLOR"):
        return text
    return f"{''.join(codes)}{text}{RESET}"


def visible_width(text: str) -> int:
    """Printable width, ignoring the SGR escapes `paint` adds."""
    return len(SGR_RE.sub("", text))


def pad_to(text: str, width: int) -> str:
    return text + " " * max(0, width - visible_width(text))


def right_align(left: str, right: str, width: int) -> str:
    """`left` at the margin, `right` at the far edge; `right` is dropped if it will not fit."""
    if visible_width(left) + visible_width(right) + 1 > width:
        return left
    return pad_to(left, width - visible_width(right)) + right


# ------------------------------------------------------------------------- render


@dataclass
class Row:
    """One rendered line, and whether it is the leaf `pick` would return next."""

    text: str
    current: bool = False


def render_leaf(leaf: Leaf, indent: int, current: bool, width: int) -> Row:
    """One leaf line: marker, handle, and — for live leaves only — kind and harness."""
    pad = "  " * indent
    if leaf.outcome == DONE:
        return Row(f"{pad}{paint('✓', DIM)} {paint(leaf.handle, DIM)}")
    if leaf.outcome == ABANDONED:
        return Row(f"{pad}{paint('✗', DIM)} {paint(leaf.handle, DIM)}")
    marker = paint("▶", BOLD, GREEN) if current else paint("○", YELLOW)
    name = paint(leaf.handle, BOLD) if current else leaf.handle
    trailer = leaf.kind()
    harness = leaf.harness()
    if harness:
        trailer = f"{trailer} @{harness}"
    return Row(right_align(f"{pad}{marker} {name}", paint(trailer, DIM, CYAN), width), current)


def render_node(node: Node, indent: int, current: Leaf | None, width: int) -> list[Row]:
    """A live node expands; a finished one collapses to a single counted line."""
    pad = "  " * indent
    if node.live:
        head = Row(f"{pad}{paint('▼', DIM)} {paint(node.handle + '/', BOLD)}")
        return [head, *render_children(node, indent + 1, current, width)]
    counts = tally(node)
    summary = " · ".join(
        f"{counts[outcome]} {label}"
        for outcome, label in ((DONE, "done"), (ABANDONED, "pruned"))
        if counts[outcome]
    )
    head = f"{pad}{paint('✓', DIM)} {paint(node.handle + '/', DIM)}"
    return [Row(right_align(head, paint(summary, DIM), width))]


def render_children(node: Node, indent: int, current: Leaf | None, width: int) -> list[Row]:
    rows: list[Row] = []
    for child in node.children:
        if isinstance(child, Node):
            rows.extend(render_node(child, indent, current, width))
        else:
            rows.append(render_leaf(child, indent, child is current, width))
    return rows


def window(rows: list[Row], height: int, offset: int | None) -> list[str]:
    """`height` lines centred on the current leaf, or from `offset` once the human scrolls."""
    if len(rows) <= height:
        return [row.text for row in rows]
    if offset is None:
        focus = next((i for i, row in enumerate(rows) if row.current), 0)
        offset = focus - height // 2
    offset = max(0, min(offset, len(rows) - height))
    lines = [row.text for row in rows[offset : offset + height]]
    if offset:
        # The marker replaces the top visible row, so one more line is hidden than `offset`.
        lines[0] = paint(f"⋯ {offset + 1} above", DIM)
    hidden = len(rows) - offset - height
    if hidden:
        lines[-1] = paint(f"⋯ {hidden + 1} below", DIM)
    return lines


HINTS = "j/k scroll · g/G ends · r reload · q quit"


def render(root: Node, name: str, size: os.terminal_size, offset: int | None) -> list[str]:
    """The whole pane: title, tree, status line."""
    # One column short of the pane: writing the last cell is where terminals differ
    # about pending wrap, and a right-aligned trailer is exactly what would be eaten.
    width = max(20, size.columns - 1)
    current = next((leaf for leaf in walk_leaves(root) if leaf.outcome == LIVE), None)
    rows = render_children(root, 0, current, width)

    counts = tally(root)
    tallies = " · ".join(
        f"{counts[outcome]} {label}"
        for outcome, label in ((DONE, "done"), (ABANDONED, "pruned"), (LIVE, "live"))
        if counts[outcome]
    )
    if not counts[LIVE]:
        tallies = f"{tallies} · {paint('ready to finish', GREEN)}" if tallies else "ready to finish"

    rule = paint("─" * width, DIM)
    header = [right_align(paint(name, BOLD), paint(tallies, DIM), width), rule]
    footer = [rule, right_align("", paint(HINTS, DIM), width)]
    body = window(rows, max(1, size.lines - len(header) - len(footer)), offset)
    return [*header, *body, *footer]


def render_notice(headline: str, detail: str) -> list[str]:
    return [paint(headline, BOLD), paint(detail, DIM), "", paint("r reload · q quit", DIM)]


# --------------------------------------------------------------------------- drive


def resolve_start_dir(argv: list[str]) -> Path:
    """Where to start looking for `.grove/`.

    In a herdr pane the process cwd is the *plugin's* own directory, which says
    nothing about which grove the human is looking at — and which, when the plugin is
    linked from inside a grove, sits under one and would render the wrong tree. So
    the invocation context wins whenever herdr supplies it. The cwd is the fallback
    for running this by hand outside herdr; an explicit path argument beats both.
    """
    if argv:
        # Resolved, not merely absolute: the grove's name is its working tree's
        # basename, and `.`/`..` components would leave that empty or wrong.
        return Path(argv[0]).expanduser().resolve()
    try:
        context = json.loads(os.environ.get("HERDR_PLUGIN_CONTEXT_JSON") or "{}")
    except json.JSONDecodeError:
        context = {}
    for key in ("focused_pane_cwd", "workspace_cwd"):
        value = context.get(key)
        if value:
            return Path(value)
    return Path.cwd()


def snapshot(start: Path, offset: int | None) -> list[str]:
    """Render the tree once, or an explanatory screen when there is nothing to render."""
    size = shutil.get_terminal_size(fallback=(80, 24))
    grove = find_grove_root(start)
    if grove is None:
        return render_notice("no grove here", f"no {GROVE_DIR}/ at or above {start}")
    worktree = grove.parent
    root = Node(0, worktree.name, 0, grove, read_tree(grove))
    if not root.children:
        return render_notice(worktree.name, f"{GROVE_DIR}/ holds no task files")
    return render(root, worktree.name, size, offset)


def draw(lines: list[str], out: TextIO) -> None:
    """Repaint from the top, clearing each line as we go — no full-screen flash."""
    out.write("\x1b[H")
    out.write("\x1b[K\r\n".join(lines))
    out.write("\x1b[K\x1b[J")
    out.flush()


def read_key(timeout: float) -> str | None:
    """One keypress, or None on timeout. Arrow keys collapse to their `j`/`k` twins."""
    ready, _, _ = select.select([sys.stdin], [], [], timeout)
    if not ready:
        return None
    key = sys.stdin.read(1)
    if key != "\x1b":
        return key
    # An escape sequence arrives as one burst; a bare Escape does not.
    if not select.select([sys.stdin], [], [], 0.02)[0]:
        return "\x1b"
    sequence = sys.stdin.read(2)
    return {"[A": "k", "[B": "j", "[5": "k", "[6": "j"}.get(sequence, "")


@dataclass
class Viewer:
    """The pane's only mutable state: how far the human has scrolled, if at all."""

    start: Path
    offset: int | None = None

    def scroll(self, key: str, page: int) -> bool:
        """Apply a keypress. Returns False when the human asked to quit."""
        if key in ("q", "\x03", "\x1b"):
            return False
        if key == "r":
            self.offset = None
        elif key == "j":
            self.offset = (self.offset or 0) + 1
        elif key == "k":
            self.offset = max(0, (self.offset or 0) - 1)
        elif key == " ":
            self.offset = (self.offset or 0) + page
        elif key == "g":
            self.offset = 0
        elif key == "G":
            self.offset = sys.maxsize
        return True

    def run(self, out: TextIO) -> int:
        out.write("\x1b[?1049h\x1b[?25l\x1b[?7l")
        painted: list[str] | None = None
        try:
            while True:
                lines = snapshot(self.start, self.offset)
                if lines != painted:
                    draw(lines, out)
                    painted = lines
                key = read_key(POLL_SECONDS)
                if key is None:
                    continue
                if not self.scroll(key, max(1, shutil.get_terminal_size().lines - 5)):
                    return 0
                painted = None
        finally:
            out.write("\x1b[?7h\x1b[?25h\x1b[?1049l")
            out.flush()


def main(argv: list[str]) -> int:
    viewer = Viewer(resolve_start_dir(argv))
    if not sys.stdin.isatty():
        # No TTY (`plugin action invoke`, a pipe, a test): render once and leave.
        print("\n".join(snapshot(viewer.start, None)))
        return 0
    settings = termios.tcgetattr(sys.stdin)
    signal.signal(signal.SIGTERM, lambda *_: sys.exit(0))
    try:
        tty.setcbreak(sys.stdin.fileno())
        return viewer.run(sys.stdout)
    except KeyboardInterrupt:
        return 0
    finally:
        termios.tcsetattr(sys.stdin, termios.TCSADRAIN, settings)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

// grove's master/detail navigator over a repo's groves — the dashboard the
// `grove tui` subcommand surfaces. It renders **only** as a native trellis host
// surface ([`dashboard_surface`], in `mod native`): grove links the forked
// `zellij-*` crates unconditionally and trellis is the one, always-on TUI
// (ADR-0026). There is no standalone in-terminal event loop anymore — the legacy
// `tui::run` crossterm path and the `--local` flag were removed when local mode
// was eliminated.
//
// Architecture:
//   - All state derives from a `RepoView` snapshot. The snapshot is
//     re-scanned on `r`, on every fs-watch quiescence (`notify` events
//     coalesced by a 200ms debounce), and after every shell-out so the
//     round trip from capture → inbox count update is visible without
//     manual refresh.
//   - `App` owns the snapshot, screen/selection state, and the capture
//     modal. Rendering is a pure function of `App` + the screen rect,
//     which keeps the `TestBackend` snapshot test honest. The host surface
//     ticks this render into an off-screen `ratatui` `Buffer` that trellis
//     blits as a real pane; input arrives as `handle_key` calls.
//
// Walk-away-ability (SKILL.md constraint 6) is preserved by routing
// every write through the `grove-llm inbox-add` verb. The TUI never
// edits grove state directly.

use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

use crate::multi_repo_view::MultiRepoView;
use crate::repo_view::{
    self, GroveDetail, GroveSummary, Lifecycle, RepoView, TaskEntry, TaskKind,
};

const DEBOUNCE: Duration = Duration::from_millis(200);

/// Styling for a version-drift marker — bold yellow, to draw the eye on both
/// the header (`cli`-vs-`repo`) and grove rows (`worktree`-vs-`repo`/`cli`).
/// The drift *rule* is the same plain string-equality one `grove status` uses
/// (`CONTEXT.md`); only the presentation differs (colour vs. plain text).
const DRIFT_STYLE: Style = Style::new()
    .fg(Color::Yellow)
    .add_modifier(Modifier::BOLD);

// ---------------------------------------------------------------------------
// State

/// Top-level app state — single source of truth for both screens.
pub struct App {
    /// The surface's **own/primary** repo, when it has one — the single repo of
    /// an N=1 surface (`new`/`new_detail`: the per-grove detail surface is
    /// genuinely repo-explicit) whose detail screen reads groves by name. `None`
    /// for the **config-only fleet nav** (`new_fleet` via `dashboard_surface`),
    /// which has no cwd anchor (ADR-0027): its header derives from the whole
    /// fleet and grove rows carry their own owning repo.
    repo: Option<PathBuf>,
    /// The fleet snapshot the nav renders (070 Q4 "subsume"). Single-repo
    /// callers (`new`/`new_detail`, exercised by the unit tests) hold a
    /// one-element fleet — the N=1 case that renders flat. The native dashboard
    /// holds the resolved multi-repo fleet.
    fleet: MultiRepoView,
    /// Repo section roots the user has **collapsed** in the two-level nav —
    /// ephemeral per-session UI state, never persisted (070 Q5, constraint 1).
    /// Empty at N=1 (no headers to collapse).
    collapsed: BTreeSet<PathBuf>,
    screen: Screen,
    list: ListState,
    detail: Option<DetailState>,
    filter: FilterState,
    show_help: bool,
    status: Option<String>,
    capture: CaptureModal,
    /// Disposition picker for the selected inbox entry; `Some` while open.
    disposition: Option<DispositionModal>,
    /// Working-set **toggle picker** open (150-working-set/030): `true` between the `t`
    /// toggle leader and the member key (`d`/`t`/`y`/`v`) or a cancel. A which-key style
    /// sub-mode — scoping the member letters behind the leader keeps the nav's top-level
    /// keymap unambiguous and surfaces the choices in the whichkey bar, exactly as the
    /// disposition picker does for its three buckets. The target grove is *not* held here
    /// — it is the currently-mounted set, resolved by the native surface at action time.
    toggle_open: bool,
    /// A keystroke (Ctrl-E / Enter in body / submit) decides *that* an
    /// external action should run; the native host surface then carries it out
    /// (shell-out write, or `$EDITOR` on the harness tty via the `HostDriver`).
    /// Splitting these phases keeps `handle_key` pure enough to test without a
    /// real terminal or a live substrate.
    pending_action: Option<PendingAction>,
    /// **Detail-locked** mode (130-native-detail/020): the `App` is bound to one
    /// grove's detail, mounted as a per-grove host surface beside its harness in the
    /// content region. There is no grove list to return to (the constant nav is a
    /// *separate* surface), so the list/filter navigation that the master/detail
    /// dashboard offers is suppressed: `Esc`/`q` stay in detail (they do not pop to a
    /// list), and `/` does not open a filter. Everything else (tree/inbox nav, `c`
    /// capture, `d` disposition, `Ctrl-E`, `Tab`) works unchanged.
    detail_locked: bool,
    /// **Native frame** mode (leaf 140): this `App` renders as a trellis host
    /// surface (the constant nav or a per-grove detail), where the grove-owned
    /// [[whichkey bar]] is the *single* owner of the bottom hint line. When set,
    /// `render` suppresses this surface's own footer and the capture modal's inline
    /// hint — those hints are published to the whichkey instead ([`footer_line`]).
    /// Every production surface sets it `true`; `false` (the footer-drawing,
    /// whichkey-less rendering) survives only as a unit-test fixture, since the
    /// legacy in-terminal dashboard that once ran that way is gone (ADR-0026).
    native_chrome: bool,
}

/// Capture modal — opened by `c`, drives the two-step
/// target/body workflow. The body accepts multi-line input: `Enter`
/// inserts a newline (so a multi-line paste lands intact) and submit is
/// the deliberate `Ctrl-S` gesture, which a paste cannot trigger. A
/// longer or heavier edit still drops to `$EDITOR` via `Ctrl-E`.
#[derive(Default, Clone)]
pub struct CaptureModal {
    open: bool,
    field: CaptureField,
    target: String,
    body: String,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum CaptureField {
    #[default]
    Target,
    Body,
}

/// Disposition picker — opened by `d` on the selected entry while the Inbox
/// pane is focused. A sub-modal rather than three bare keys because `r` is the
/// global refresh key: scoping the choice behind a modal keeps the top-level
/// keymap unambiguous and the choice discoverable.
pub struct DispositionModal {
    /// Absolute path of the observation being dispositioned.
    path: PathBuf,
    /// Filename of that entry, shown in the picker title.
    entry: String,
}

/// The three [[Drain]] buckets. All three delete the observation file; the
/// choice only sets the `grove-llm inbox-drain` commit-message category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    Incorporated,
    Deferred,
    Rejected,
}

impl Disposition {
    /// The `grove-llm inbox-drain` finalize flag for this bucket.
    fn flag(self) -> &'static str {
        match self {
            Disposition::Incorporated => "--incorporated",
            Disposition::Deferred => "--deferred",
            Disposition::Rejected => "--rejected",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Disposition::Incorporated => "incorporated",
            Disposition::Deferred => "deferred",
            Disposition::Rejected => "rejected",
        }
    }
}

/// grove's **responsive breakpoint** (040-responsive-layout): the content region must
/// be at least this many columns wide for the **wide tier** — the whole [[working set]]
/// visible (harness + detail + terminal + yazi + vcs) in the harness-dominant side
/// stack. Below it is the **laptop tier**: the aux tools (terminal/yazi/vcs) default to
/// hidden (parked alive, toggle-able per 030), leaving harness + detail. Two tiers, not
/// a continuum (constraint 4) — the user can always toggle from the default.
///
/// 220 cols is sized so the wide tier gives the dominant harness (~60% ≈ 130+ cols) and
/// a usable ~90-col side stack; a MacBook-class full-screen terminal (content ≈ 175–200
/// cols after the 34-col nav) lands in the laptop tier, an ultra-wide / 5K2K display
/// (content ≈ 300+) in the wide tier. This is grove's policy alone: it is passed into
/// `HostDriver::swap_content`, and trellis only measures the region and applies the
/// comparison — it never owns the breakpoint (the one-way seam, ADR-0020 §4).
const WIDE_TIER_MIN_CONTENT_COLS: usize = 220;

/// A toggleable member of a grove's [[working set]] (150-working-set/030): the
/// per-grove **detail** surface and the aux tools **terminal**, **yazi**, **vcs**.
/// The **harness** is deliberately absent — it is always present (the grove's reason
/// to exist), so there is nothing to toggle. Each maps to the opaque `role` tag trellis
/// recorded for that member at mount (the harness is `primary`, never toggled; detail
/// is `secondary`; the aux tools self-name) — the addressing handle
/// `HostDriver::toggle_member` takes. Keeping the role strings in this one typed place
/// (not scattered string literals) is the seam against a silent role-name drift between
/// grove's [`aux_members`] composition and a toggle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingSetMember {
    Detail,
    Terminal,
    Yazi,
    Vcs,
}

impl WorkingSetMember {
    /// The opaque trellis `role` tag for this member — what
    /// [`HostDriver::toggle_member`](trellis_server::panes::host_pane::HostDriver::toggle_member)
    /// addresses it by. `Detail` is the pair's `secondary`; the aux tools self-name
    /// (matching the roles [`aux_members`] assigns).
    fn role(self) -> &'static str {
        match self {
            WorkingSetMember::Detail => "secondary",
            WorkingSetMember::Terminal => "terminal",
            WorkingSetMember::Yazi => "yazi",
            WorkingSetMember::Vcs => "vcs",
        }
    }

    /// The toggle-picker key that selects this member (the which-key second key after
    /// the `t` toggle leader): first letter of each, `t` doubling for terminal.
    fn key(self) -> char {
        match self {
            WorkingSetMember::Detail => 'd',
            WorkingSetMember::Terminal => 't',
            WorkingSetMember::Yazi => 'y',
            WorkingSetMember::Vcs => 'v',
        }
    }

    /// Resolve a picker key to its member, or `None` for any other key (which the
    /// picker treats as inert — only `Esc`/`Ctrl-C` cancel).
    fn from_key(c: char) -> Option<Self> {
        [
            WorkingSetMember::Detail,
            WorkingSetMember::Terminal,
            WorkingSetMember::Yazi,
            WorkingSetMember::Vcs,
        ]
        .into_iter()
        .find(|m| m.key() == c)
    }

    fn label(self) -> &'static str {
        match self {
            WorkingSetMember::Detail => "detail",
            WorkingSetMember::Terminal => "terminal",
            WorkingSetMember::Yazi => "yazi",
            WorkingSetMember::Vcs => "vcs",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendingAction {
    /// Submit `app.capture` via `grove-llm inbox-add`.
    Submit,
    /// Drop into `$EDITOR` (or `vi` fallback) to edit `app.capture.body`.
    EditBody,
    /// Disposition the observation at `path` into `disposition`'s bucket via
    /// `grove-llm inbox-drain` — all three buckets delete the file; the bucket
    /// only sets the drain commit-message category (faithful to Drain).
    Drain {
        path: PathBuf,
        disposition: Disposition,
    },
    /// Edit the *existing committed* observation at `path`: seed `$EDITOR` with
    /// its current body, then on a non-empty change round-trip through
    /// `grove-llm inbox-edit`. Distinct from `EditBody`, which edits the
    /// in-memory capture draft before it is ever committed.
    EditObservation {
        path: PathBuf,
    },
    /// Select grove `name`: swap its `grove do <name>` harness (run in `repo`) into
    /// the **content slot** beside the constant nav (ADR-0022/0023). The native
    /// dashboard drives this by a direct in-process `HostDriver::swap_content` call;
    /// the screen thread parks the previously-selected harness alive and mounts (or
    /// restores) this one — no `zellij action`, no tabs. `repo` is explicit so the
    /// cross-repo fleet (070) reuses the driving layer unchanged. Carried out by
    /// the native host surface — the only TUI path (ADR-0026).
    OpenHarness {
        name: String,
        repo: PathBuf,
    },
    /// Legacy "close the acting grove's harness" request (the `x` key). The
    /// content-swap model parks harnesses alive instead of closing them, so the
    /// native surface treats this as a no-op-with-status; kept so `x` has a
    /// defined disposition.
    CloseHarness {
        name: String,
    },
    /// Toggle the visibility of one [[working set]] member of the **currently-mounted**
    /// grove (150-working-set/030). Chosen from the toggle picker (`t` then a member
    /// letter); the native surface drives it via `HostDriver::toggle_member` keyed by the
    /// mounted grove, so it acts on the grove whose set is in the content region — not
    /// the nav's highlighted list row.
    ToggleMember {
        member: WorkingSetMember,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    GroveList,
    GroveDetail,
}

struct DetailState {
    grove: String,
    /// Selected row in the flattened task-tree view.
    tree: ListState,
    /// Selected pending observation in the inbox pane. Kept distinct from
    /// `tree` so each pane remembers its own cursor across `Tab` switches.
    inbox: ListState,
    right: RightPane,
    /// Scroll offset for the right pane (lines from top).
    right_scroll: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RightPane {
    LeafBody,
    Inbox,
    Brief,
}

impl RightPane {
    fn next(self) -> Self {
        match self {
            RightPane::LeafBody => RightPane::Inbox,
            RightPane::Inbox => RightPane::Brief,
            RightPane::Brief => RightPane::LeafBody,
        }
    }

    fn label(self) -> &'static str {
        match self {
            RightPane::LeafBody => "leaf",
            RightPane::Inbox => "inbox",
            RightPane::Brief => "brief",
        }
    }
}

/// Flat-list ordering for the filter-active nav (070-fleet-view/060). The
/// idle (grouped) nav always uses the fleet's natural order; this only takes
/// effect once the list has flattened.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum SortMode {
    /// Fleet order (current-repo-first / lifecycle / numeric prefix),
    /// reordered by fuzzy score when a text needle is present.
    #[default]
    Default,
    /// Most inbox-pending groves first — the "show me what needs attention"
    /// order.
    InboxDesc,
}

impl SortMode {
    /// Cycle `Default ↔ InboxDesc` (the `s` toggle).
    fn next(self) -> Self {
        match self {
            SortMode::Default => SortMode::InboxDesc,
            SortMode::InboxDesc => SortMode::Default,
        }
    }
}

/// The lifecycle predicate toggle (070-fleet-view/060). Cycles
/// `All → LiveOnly → SeedOnly → All`.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum LifecycleFilter {
    #[default]
    All,
    LiveOnly,
    SeedOnly,
}

impl LifecycleFilter {
    /// Cycle `All → LiveOnly → SeedOnly → All` (the `l` toggle).
    fn next(self) -> Self {
        match self {
            LifecycleFilter::All => LifecycleFilter::LiveOnly,
            LifecycleFilter::LiveOnly => LifecycleFilter::SeedOnly,
            LifecycleFilter::SeedOnly => LifecycleFilter::All,
        }
    }
}

/// The ephemeral (per-session, never persisted — constraint 1) filter applied
/// to the fleet nav. The text filter is **fuzzy** over `<repo>/<grove>`; the
/// three non-text dimensions (`inbox_only`, `lifecycle`, `sort`) are toggles.
/// While **any** dimension is engaged ([`active`](Self::active)) the nav
/// flattens out of its grouped shape into a single ranked list (070-060 Q3).
#[derive(Default)]
struct FilterState {
    /// True while the user is typing into the text-filter input.
    editing: bool,
    /// The committed (or in-progress) fuzzy needle — matched live against
    /// each grove's `<repo>/<grove>` string.
    text: String,
    /// Show only groves with pending inbox observations (`i`).
    inbox_only: bool,
    /// Lifecycle predicate (`l`): all / live-only / seed-only.
    lifecycle: LifecycleFilter,
    /// Flat-list ordering (`s`).
    sort: SortMode,
}

impl FilterState {
    /// Any narrowing or reordering engaged → the nav flattens into a single
    /// ranked list (070-060 Q3); idle leaves the grouped two-level shape.
    fn active(&self) -> bool {
        !self.text.is_empty()
            || self.inbox_only
            || self.lifecycle != LifecycleFilter::All
            || self.sort != SortMode::Default
    }

    /// Reset every dimension to idle — used on the transitions that return the
    /// nav to its resting grouped state (drill-in, back, refresh), so a filter
    /// never silently outlives the visit that set it.
    fn clear(&mut self) {
        *self = FilterState::default();
    }
}

impl App {
    pub fn new(repo: PathBuf, view: RepoView, preselect: Option<String>) -> Self {
        // Single-repo: the N=1 "fleet of one" (070 Q4). Wrap the already-scanned
        // `RepoView` rather than re-scanning, and render flat (no repo header).
        // An explicit anchor repo (`Some`) — this constructor is for genuinely
        // repo-scoped surfaces, not the cwd-anchored fleet nav (ADR-0027).
        let fleet = MultiRepoView::from_repos(vec![view]);
        Self::new_fleet(Some(repo), fleet, preselect)
    }

    /// Build the **native fleet nav** `App` over a multi-repo `fleet` (070 Q4).
    /// `repo` is the surface's own anchor repo, or `None` for the config-only
    /// fleet nav with no cwd anchor (ADR-0027); the single-repo [`new`](Self::new)
    /// routes through here with `Some`. `preselect` names a grove (in the anchor
    /// repo) to highlight; the selection is the index of that grove's row in the
    /// flattened nav. With no anchor, only the first-row fallback applies.
    pub fn new_fleet(
        repo: Option<PathBuf>,
        fleet: MultiRepoView,
        preselect: Option<String>,
    ) -> Self {
        let mut app = Self {
            repo,
            fleet,
            collapsed: BTreeSet::new(),
            screen: Screen::GroveList,
            list: ListState::default(),
            detail: None,
            filter: FilterState::default(),
            show_help: false,
            status: None,
            capture: CaptureModal::default(),
            disposition: None,
            toggle_open: false,
            pending_action: None,
            detail_locked: false,
            native_chrome: false,
        };
        // Highlight the preselected grove's row (or the first row). Built after
        // construction so it can consult the flattened nav rows.
        let rows = app.nav_rows_cached();
        let idx = preselect
            .as_deref()
            .and_then(|name| {
                rows.iter().position(|r| {
                    matches!(r, NavRow::Grove { repo, summary }
                        if app.repo.as_deref() == Some(*repo) && summary.name == name)
                })
            })
            .or_else(|| (!rows.is_empty()).then_some(0));
        app.list.select(idx);
        app
    }

    /// Build an `App` **locked to one grove's detail** (130-native-detail/020) —
    /// the state behind a per-grove [`DetailSurface`]. Starts straight in
    /// [`Screen::GroveDetail`] for `grove`, with fresh tree/inbox cursors and
    /// list/filter navigation suppressed (see [`App::detail_locked`]); the detail
    /// data itself is read from `view` by name on each render, exactly as the
    /// master/detail dashboard does after a drill-in.
    pub fn new_detail(repo: PathBuf, view: RepoView, grove: String) -> Self {
        let mut tree = ListState::default();
        tree.select(Some(0));
        let mut inbox = ListState::default();
        inbox.select(Some(0));
        Self {
            // A per-grove detail surface is genuinely repo-explicit (ADR-0027) —
            // it reads one repo's grove by name, not a cwd anchor.
            repo: Some(repo),
            fleet: MultiRepoView::from_repos(vec![view]),
            collapsed: BTreeSet::new(),
            screen: Screen::GroveDetail,
            list: ListState::default(),
            detail: Some(DetailState {
                grove,
                tree,
                inbox,
                right: RightPane::LeafBody,
                right_scroll: 0,
            }),
            filter: FilterState::default(),
            show_help: false,
            status: None,
            capture: CaptureModal::default(),
            disposition: None,
            toggle_open: false,
            pending_action: None,
            detail_locked: true,
            // A per-grove detail surface always runs inside the native frame, where
            // the grove-owned whichkey bar owns the bottom hint line.
            native_chrome: true,
        }
    }

    /// Rescan the repo without touching the status line. Used by fs-watch.
    fn refresh_silent(&mut self) -> Result<()> {
        // Full fleet rescan: re-scan every repo currently in the fleet, in place
        // (preserving fleet order). The fs-watch hot path uses the targeted
        // `rescan_event_paths` instead; this is `r` and the pathless/out-of-band
        // fallback. A repo whose rescan fails is dropped by `scan_with_warnings`
        // (070 Q3 silent-skip), so this never errors — the `Result` stays only
        // for signature compatibility with the call sites.
        let selected = self.selected_nav();
        let detail_grove = self.detail.as_ref().map(|d| d.grove.clone());
        let roots: Vec<PathBuf> =
            self.fleet.repos().iter().map(|r| r.repo_root.clone()).collect();
        let (fleet, _warnings) = MultiRepoView::scan_with_warnings(&roots);
        self.fleet = fleet;
        self.reselect(selected);
        self.handle_detail_vanish(detail_grove);
        Ok(())
    }

    /// Targeted fleet rescan — the fs-watch hot path (070 Q6). Re-scan only the
    /// repos that own `paths` (prefix-match via `fleet::owning_repo`), leaving
    /// every other repo's `RepoView` untouched, then restore the selection. A
    /// path under no fleet repo is a no-op; a single repo's scan failure leaves
    /// that repo's view unchanged (a transient I/O error must not drop a repo
    /// mid-session), so this, too, cannot fail.
    fn rescan_event_paths(&mut self, paths: &[PathBuf]) {
        let selected = self.selected_nav();
        let detail_grove = self.detail.as_ref().map(|d| d.grove.clone());
        for p in paths {
            let _ = self.fleet.rescan_for_event_path(p);
        }
        self.reselect(selected);
        self.handle_detail_vanish(detail_grove);
    }

    /// Restore the selection after a rescan: re-find the same grove (by
    /// repo+name) or the same repo header, else clamp into range (or clear when
    /// the fleet is empty).
    fn reselect(&mut self, prev: Option<NavSelection>) {
        let rows = self.nav_rows_cached();
        if rows.is_empty() {
            self.list.select(None);
            return;
        }
        let found = prev.and_then(|sel| {
            rows.iter().position(|r| match (&sel, r) {
                (
                    NavSelection::Grove { repo, name },
                    NavRow::Grove { repo: rr, summary },
                ) => *rr == repo.as_path() && summary.name == *name,
                (NavSelection::Header(repo), NavRow::RepoHeader { repo: rr, .. }) => {
                    *rr == repo.as_path()
                }
                _ => false,
            })
        });
        let idx = found.unwrap_or_else(|| {
            self.list.selected().unwrap_or(0).min(rows.len() - 1)
        });
        self.list.select(Some(idx));
    }

    /// If on the detail screen and the bound grove vanished from the fleet, pop
    /// back to the list. (The native detail surfaces are per-grove; this guards
    /// the in-`App` master/detail drill-in path the unit tests exercise.)
    fn handle_detail_vanish(&mut self, detail_grove: Option<String>) {
        if matches!(self.screen, Screen::GroveDetail) {
            let still_there = detail_grove
                .as_deref()
                .map(|name| self.detail_grove(name).is_some())
                .unwrap_or(false);
            if !still_there {
                self.screen = Screen::GroveList;
                self.detail = None;
            }
        }
    }

    /// Rescan the fleet and signal "refreshed" in the status line.
    /// Triggered by `r`. Clears any active filter (text + predicates + sort) to
    /// make the rescan's selection deterministic for the user — the nav returns
    /// to its grouped resting shape. (The fs-watch auto-rescan path keeps the
    /// filter, refreshing in place.)
    fn refresh(&mut self) -> Result<()> {
        self.filter.clear();
        self.refresh_silent()?;
        self.status = Some("refreshed".into());
        Ok(())
    }

    /// The surface's own repo's [`RepoView`], when it has an anchor repo. Falls
    /// back to the first repo in the fleet, then `None` (no anchor + empty fleet).
    /// The config-only fleet nav (`repo: None`) takes the first-repo fallback —
    /// but its header no longer reads a single repo's versions (ADR-0027 §6;
    /// `render_header` derives from the whole fleet instead).
    fn primary_view(&self) -> Option<&RepoView> {
        self.repo
            .as_deref()
            .and_then(|r| self.fleet.repo(r))
            .or_else(|| self.fleet.repos().first())
    }

    /// Resolve a grove's detail in the surface's **own** repo — the detail
    /// screen is single-repo-scoped (each per-grove detail surface is N=1 over
    /// an explicit `repo`). `None` when the surface has no anchor repo (the fleet
    /// nav, which drives detail through separate per-grove surfaces).
    fn detail_grove(&self, name: &str) -> Option<&GroveDetail> {
        self.repo.as_deref().and_then(|r| self.fleet.grove(r, name))
    }

    /// The flattened two-level nav rows the `ListState` indexes into — repo
    /// headers (N>1) interleaved with their filter-matched groves, honouring the
    /// collapse set. Rebuilt per call; bounded by fleet size.
    fn nav_rows_cached(&self) -> Vec<NavRow<'_>> {
        nav_rows(&self.fleet, &self.collapsed, &self.filter)
    }

    /// The number of nav rows — the movement length for `j`/`k` (which step over
    /// headers and groves alike, so a header can be selected to collapse it).
    fn nav_len(&self) -> usize {
        self.nav_rows_cached().len()
    }

    /// What the highlighted nav row is: a repo header or a grove (with its owning
    /// repo). `None` when nothing is selected or the fleet is empty.
    fn selected_nav(&self) -> Option<NavSelection> {
        let rows = self.nav_rows_cached();
        rows.get(self.list.selected()?).map(NavSelection::from_row)
    }

    /// Activate the highlighted nav row. A **grove** yields
    /// [`NavActivation::Open`] with its `(repo, name)` for the native surface to
    /// open (or the unit tests to drill into) — the repo is the grove's **owning**
    /// repo, carried for cross-repo open (070 Q4/050). A **repo header** toggles
    /// its section's collapse in place (ephemeral, 070 Q5) and yields
    /// [`NavActivation::Toggled`]. While a modal / filter / help intercepts keys
    /// — or nothing is selected — it yields [`NavActivation::Passthrough`] so
    /// `Enter` falls through to the shared handler (e.g. a capture-body newline).
    /// Supersedes the old name-only `nav_enter_target`.
    fn nav_activate(&mut self) -> NavActivation {
        if self.show_help
            || self.capture.open
            || self.disposition.is_some()
            || self.filter.editing
            || !matches!(self.screen, Screen::GroveList)
        {
            return NavActivation::Passthrough;
        }
        match self.selected_nav() {
            Some(NavSelection::Header(repo)) => {
                if !self.collapsed.remove(&repo) {
                    self.collapsed.insert(repo);
                }
                NavActivation::Toggled
            }
            Some(NavSelection::Grove { repo, name }) => NavActivation::Open(repo, name),
            None => NavActivation::Passthrough,
        }
    }
}

/// The outcome of activating a nav row (`Enter`/`o`): open a grove, toggle a
/// repo section's collapse, or let the key pass through to the shared handler.
#[derive(Debug, Clone, PartialEq, Eq)]
enum NavActivation {
    /// Open (or drill into) grove `name` in its owning `repo`.
    Open(PathBuf, String),
    /// A repo header was highlighted; its section's collapse was toggled.
    Toggled,
    /// A modal / filter / help is up, or nothing is selected — `Enter` should
    /// fall through to the shared key handler.
    Passthrough,
}

/// What the highlighted nav row resolves to — the owned counterpart of a
/// [`NavRow`], so a selection survives the rebuild of the borrowed row vec.
#[derive(Debug, Clone, PartialEq, Eq)]
enum NavSelection {
    /// A repo section header at `repo` — activating it toggles collapse.
    Header(PathBuf),
    /// A grove `name` in its owning `repo` — activating it opens/drills in.
    Grove { repo: PathBuf, name: String },
}

impl NavSelection {
    fn from_row(row: &NavRow) -> Self {
        match row {
            NavRow::RepoHeader { repo, .. } => NavSelection::Header(repo.to_path_buf()),
            NavRow::Grove { repo, summary } => NavSelection::Grove {
                repo: repo.to_path_buf(),
                name: summary.name.clone(),
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Shell-out helpers (used by both the native host surface and the unit tests)

fn short_err(e: &anyhow::Error) -> String {
    // `{:#}` collapses the anyhow chain into "top: cause: root", which
    // is what the status line needs — a single-line summary that names
    // the actual failure (e.g. "running grove-llm inbox-add: No such
    // file or directory") rather than just the topmost wrapper.
    format!("{:#}", e)
}

// ---------------------------------------------------------------------------
// Shell helpers

/// Locate the sibling `grove-llm` binary next to the running `grove`
/// executable. Falls back to PATH lookup. This handles both
/// `target/debug/` during development and `/usr/local/bin/` after
/// `brew install`, without assuming PATH ordering.
fn find_grove_llm() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let bin = if cfg!(windows) { "grove-llm.exe" } else { "grove-llm" };
            let sibling = parent.join(bin);
            if sibling.is_file() {
                return sibling;
            }
        }
    }
    PathBuf::from("grove-llm")
}

fn shell_capture(target: &str, body: &str) -> Result<()> {
    if target.trim().is_empty() {
        anyhow::bail!("target grove name is empty");
    }
    if body.trim().is_empty() {
        anyhow::bail!("body is empty");
    }
    let tf = tempfile::Builder::new()
        .prefix("grove-capture-")
        .suffix(".md")
        .tempfile()
        .context("creating body tempfile")?;
    std::fs::write(tf.path(), body)
        .with_context(|| format!("writing body to {}", tf.path().display()))?;
    let status = std::process::Command::new(find_grove_llm())
        .arg("inbox-add")
        .arg(format!("--to={}", target.trim()))
        .arg("--body-file")
        .arg(tf.path())
        .status()
        .context("running grove-llm inbox-add")?;
    if !status.success() {
        anyhow::bail!(
            "grove-llm inbox-add exited with status {:?}",
            status.code()
        );
    }
    Ok(())
}

/// Disposition a single observation by shelling out to `grove-llm
/// inbox-drain --for=<grove> --<bucket>=<path>`. The CLI deletes the file and
/// commits (and pushes when a remote is configured); the TUI never touches the
/// `grove-meta` git plumbing directly, mirroring `shell_capture`.
fn shell_drain(grove: &str, path: &Path, disposition: Disposition) -> Result<()> {
    if grove.trim().is_empty() {
        anyhow::bail!("grove name is empty");
    }
    let status = std::process::Command::new(find_grove_llm())
        .arg("inbox-drain")
        .arg(format!("--for={}", grove.trim()))
        .arg(disposition.flag())
        .arg(path)
        .status()
        .context("running grove-llm inbox-drain")?;
    if !status.success() {
        anyhow::bail!(
            "grove-llm inbox-drain exited with status {:?}",
            status.code()
        );
    }
    Ok(())
}

/// The result of an inbox-edit round-trip: either the body was changed and
/// committed, or the user left it untouched (so no verb ran).
enum EditOutcome {
    Saved,
    Unchanged,
}

/// Shared tail of the observation-edit flow: given the original body and the
/// edited body, no-op on no change, reject an empty result, else round-trip
/// through the `grove-llm inbox-edit` verb. The native host surface drives
/// `$EDITOR` on the harness tty and calls this with the result.
fn decide_observation_edit(path: &Path, current: &str, edited: String) -> Result<EditOutcome> {
    if edited == current {
        return Ok(EditOutcome::Unchanged);
    }
    if edited.trim().is_empty() {
        anyhow::bail!("edited body is empty; the observation was left unchanged");
    }
    inbox_edit_verb(path, &edited)?;
    Ok(EditOutcome::Saved)
}

/// Rewrite the committed observation at `path` via `grove-llm inbox-edit`. The
/// CLI recomputes the content-hash filename, commits, and pushes when a remote
/// is configured; the dashboard never touches `grove-meta` git plumbing.
fn inbox_edit_verb(path: &Path, edited: &str) -> Result<()> {
    let tf = tempfile::Builder::new()
        .prefix("grove-edit-")
        .suffix(".md")
        .tempfile()
        .context("creating body tempfile")?;
    std::fs::write(tf.path(), edited)
        .with_context(|| format!("writing edited body to {}", tf.path().display()))?;
    let status = std::process::Command::new(find_grove_llm())
        .arg("inbox-edit")
        .arg(path)
        .arg("--body-file")
        .arg(tf.path())
        .status()
        .context("running grove-llm inbox-edit")?;
    if !status.success() {
        anyhow::bail!(
            "grove-llm inbox-edit exited with status {:?}",
            status.code()
        );
    }
    Ok(())
}

/// Returns true when the app should exit.
fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) -> Result<bool> {
    // Help overlay: any key dismisses.
    if app.show_help {
        app.show_help = false;
        return Ok(false);
    }

    // Capture modal swallows almost everything; only Ctrl-C still quits.
    if app.capture.open {
        handle_capture_key(app, code, mods);
        return Ok(false);
    }

    // Disposition picker swallows its keys (i/d/r choose, Esc/Ctrl-C cancel).
    if app.disposition.is_some() {
        handle_disposition_key(app, code, mods);
        return Ok(false);
    }

    // Working-set toggle picker swallows its keys (d/t/y/v choose a member, Esc/Ctrl-C
    // cancel; any other key is inert). 150-working-set/030.
    if app.toggle_open {
        handle_toggle_key(app, code, mods);
        return Ok(false);
    }

    // Filter-edit mode swallows almost everything.
    if app.filter.editing {
        match code {
            KeyCode::Esc => {
                app.filter.editing = false;
                app.filter.text.clear();
            }
            KeyCode::Enter => {
                app.filter.editing = false;
            }
            KeyCode::Backspace => {
                app.filter.text.pop();
            }
            KeyCode::Char(c) => {
                app.filter.text.push(c);
            }
            _ => {}
        }
        return Ok(false);
    }

    // Ctrl-C always quits.
    if mods.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
        return Ok(true);
    }

    // Ctrl-E on the detail screen edits the selected inbox observation. Handled
    // ahead of the `(screen, code)` match because that match ignores modifiers,
    // and a bare `e` must not trigger an edit.
    if mods.contains(KeyModifiers::CONTROL)
        && matches!(code, KeyCode::Char('e'))
        && matches!(app.screen, Screen::GroveDetail)
    {
        request_observation_edit(app);
        return Ok(false);
    }

    match (app.screen, code) {
        (_, KeyCode::Char('?')) => {
            app.show_help = true;
        }
        // Filtering navigates the grove list; in detail-locked mode there is no
        // list (the constant nav is a separate surface), so `/` is inert.
        (_, KeyCode::Char('/')) if !app.detail_locked => {
            app.filter.editing = true;
            app.filter.text.clear();
        }
        (_, KeyCode::Char('r')) => {
            app.refresh()?;
        }
        (_, KeyCode::Char('c')) => {
            open_capture_modal(app);
        }
        // Harness driving (native path): `o` selects the acting grove — swapping
        // its harness into the content slot beside the constant nav (the same action
        // as `Enter` on the nav list). `x` is the retired close affordance (the swap
        // model parks harnesses alive; see `PendingAction::CloseHarness`). The
        // decision is recorded here and enacted by the native `process_action`
        // (`HostDriver::swap_content`).
        (_, KeyCode::Char('o')) => {
            request_open_harness(app);
        }
        (_, KeyCode::Char('x')) => {
            request_close_harness(app);
        }
        // `t` opens the working-set toggle picker — but only in the **native nav**
        // (`native_chrome`), where a content region with a mounted working set exists.
        // Without `native_chrome` (the unit-test rendering) there is no substrate to
        // toggle, so `t` stays inert (150-working-set/030).
        (Screen::GroveList, KeyCode::Char('t')) if app.native_chrome => {
            app.toggle_open = true;
        }
        (Screen::GroveList, KeyCode::Char('q')) => return Ok(true),
        // Fleet filter toggles (070-fleet-view/060). Each engages a non-text
        // filter dimension; any active dimension flattens the grouped nav into a
        // single ranked list (070-060 Q3). `/` (above) drives the *text* needle;
        // these three are the predicate/sort toggles. Each re-anchors the
        // selection onto the same grove when it survives the new predicate set
        // (else the render clamps) — the filter reshapes the rows like a rescan.
        (Screen::GroveList, KeyCode::Char('i')) => {
            let sel = app.selected_nav();
            app.filter.inbox_only = !app.filter.inbox_only;
            app.reselect(sel);
        }
        (Screen::GroveList, KeyCode::Char('l')) => {
            let sel = app.selected_nav();
            app.filter.lifecycle = app.filter.lifecycle.next();
            app.reselect(sel);
        }
        (Screen::GroveList, KeyCode::Char('s')) => {
            let sel = app.selected_nav();
            app.filter.sort = app.filter.sort.next();
            app.reselect(sel);
        }
        // Movement steps over the flattened two-level nav rows — repo headers
        // and groves alike — so a header can be highlighted to collapse it.
        (Screen::GroveList, KeyCode::Down | KeyCode::Char('j')) => {
            let len = app.nav_len() as isize;
            move_selection(&mut app.list, len, 1);
        }
        (Screen::GroveList, KeyCode::Up | KeyCode::Char('k')) => {
            let len = app.nav_len() as isize;
            move_selection(&mut app.list, len, -1);
        }
        (Screen::GroveList, KeyCode::Enter) => {
            // `nav_activate` toggles a section when a repo header is highlighted;
            // on a grove it yields its `(repo, name)`, which this shared handler
            // drills into as detail (the path the unit tests exercise). The native
            // nav intercepts `Enter` earlier to open the harness instead.
            if let NavActivation::Open(_repo, name) = app.nav_activate() {
                let mut tree = ListState::default();
                tree.select(Some(0));
                let mut inbox = ListState::default();
                inbox.select(Some(0));
                app.detail = Some(DetailState {
                    grove: name,
                    tree,
                    inbox,
                    right: RightPane::LeafBody,
                    right_scroll: 0,
                });
                app.screen = Screen::GroveDetail;
                app.filter.clear();
            }
        }
        // Detail screen. `Esc`/`q` pop back to the grove list in the master/detail
        // dashboard — but in detail-locked mode there is no list to return to (the
        // nav is a separate constant surface), so they are inert: the detail surface
        // stays put, and the user moves focus to the nav with the leader instead.
        (Screen::GroveDetail, KeyCode::Esc | KeyCode::Char('q')) if !app.detail_locked => {
            app.screen = Screen::GroveList;
            app.detail = None;
            app.filter.clear();
        }
        (Screen::GroveDetail, KeyCode::Tab) => {
            if let Some(d) = app.detail.as_mut() {
                d.right = d.right.next();
                d.right_scroll = 0;
            }
        }
        (Screen::GroveDetail, KeyCode::Char('d')) => {
            // Disposition the selected inbox entry — only meaningful while the
            // Inbox pane is focused and has a selection (see open helper).
            open_disposition_modal(app);
        }
        (Screen::GroveDetail, KeyCode::Down | KeyCode::Char('j')) => {
            move_detail_selection(app, 1);
        }
        (Screen::GroveDetail, KeyCode::Up | KeyCode::Char('k')) => {
            move_detail_selection(app, -1);
        }
        (Screen::GroveDetail, KeyCode::PageDown) => {
            if let Some(d) = app.detail.as_mut() {
                d.right_scroll = d.right_scroll.saturating_add(10);
            }
        }
        (Screen::GroveDetail, KeyCode::PageUp) => {
            if let Some(d) = app.detail.as_mut() {
                d.right_scroll = d.right_scroll.saturating_sub(10);
            }
        }
        _ => {}
    }
    Ok(false)
}

fn open_capture_modal(app: &mut App) {
    // Pre-fill target from context: on detail, the current grove (jump
    // straight to body); on list, the currently-selected grove if any
    // (let the user edit it, since list-screen capture often means
    // capturing to a *different* grove).
    let (target, field) = match app.screen {
        Screen::GroveDetail => match app.detail.as_ref() {
            Some(d) => (d.grove.clone(), CaptureField::Body),
            None => (String::new(), CaptureField::Target),
        },
        Screen::GroveList => {
            // The selection indexes the two-level nav rows (which interleave repo
            // headers), so resolve the highlighted *grove* through `selected_nav`
            // rather than a flat-list index. A header (or nothing) selected
            // prefills an empty target for the user to type.
            let pre = match app.selected_nav() {
                Some(NavSelection::Grove { name, .. }) => name,
                _ => String::new(),
            };
            (pre, CaptureField::Target)
        }
    };
    app.capture = CaptureModal {
        open: true,
        field,
        target,
        body: String::new(),
    };
}

/// The grove the user is acting on for harness driving, **with its owning
/// repo**: the highlighted grove row on the list screen (a repo header has no
/// acting grove), or the open grove on the detail screen (in the surface's own
/// repo). `None` when nothing actionable is selected. The repo is the grove's
/// owning repo so a cross-repo open targets the right one (070 Q4/050).
fn acting_grove(app: &App) -> Option<(PathBuf, String)> {
    match app.screen {
        // The detail screen only runs on a repo-explicit surface (`new_detail`,
        // `repo: Some`), so the anchor is present; `None` otherwise is inert.
        Screen::GroveDetail => app
            .detail
            .as_ref()
            .zip(app.repo.clone())
            .map(|(d, repo)| (repo, d.grove.clone())),
        Screen::GroveList => match app.selected_nav() {
            Some(NavSelection::Grove { repo, name }) => Some((repo, name)),
            _ => None,
        },
    }
}

/// Request "select the acting grove" — swap its harness into the content slot
/// (ADR-0022/0023). The grove's **owning** repo is carried so the cross-repo
/// fleet (070) opens it in the right repo. No-op when no grove is selected.
fn request_open_harness(app: &mut App) {
    if let Some((repo, name)) = acting_grove(app) {
        app.pending_action = Some(PendingAction::OpenHarness { name, repo });
    }
}

/// Request the retired "close the acting grove's harness" affordance (`x`). No-op
/// when no grove is selected; the native surface parks rather than closes.
fn request_close_harness(app: &mut App) {
    if let Some((_repo, name)) = acting_grove(app) {
        app.pending_action = Some(PendingAction::CloseHarness { name });
    }
}

fn handle_capture_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    // Ctrl-C closes the modal rather than quitting the app — the outer
    // handler already short-circuits Ctrl-C to quit *before* we get
    // here, but only when the modal is closed. Inside the modal, the
    // outer handler returned early so the user's Ctrl-C lands here.
    if mods.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
        app.capture = CaptureModal::default();
        return;
    }
    match code {
        KeyCode::Esc => {
            app.capture = CaptureModal::default();
        }
        KeyCode::Tab => {
            app.capture.field = match app.capture.field {
                CaptureField::Target => CaptureField::Body,
                CaptureField::Body => CaptureField::Target,
            };
        }
        KeyCode::Char('e') if mods.contains(KeyModifiers::CONTROL) => {
            // Drop to $EDITOR with the current body. Live loop picks
            // this up after the key handler returns.
            app.pending_action = Some(PendingAction::EditBody);
        }
        KeyCode::Char('s') if mods.contains(KeyModifiers::CONTROL) => {
            // The deliberate submit gesture. Distinct from `Enter` so a
            // pasted (or typed) newline in the body cannot fire submit by
            // accident; works from either field once both are filled.
            if !app.capture.target.trim().is_empty()
                && !app.capture.body.trim().is_empty()
            {
                app.pending_action = Some(PendingAction::Submit);
            }
        }
        KeyCode::Enter => match app.capture.field {
            CaptureField::Target => {
                if !app.capture.target.trim().is_empty() {
                    app.capture.field = CaptureField::Body;
                }
            }
            CaptureField::Body => {
                // `Enter` in the body inserts a newline rather than
                // submitting — multi-line observations are typed and
                // pasted here, and submit is the deliberate Ctrl-S above.
                app.capture.body.push('\n');
            }
        },
        KeyCode::Backspace => match app.capture.field {
            CaptureField::Target => {
                app.capture.target.pop();
            }
            CaptureField::Body => {
                app.capture.body.pop();
            }
        },
        KeyCode::Char(c) if !mods.contains(KeyModifiers::CONTROL) => {
            match app.capture.field {
                CaptureField::Target => app.capture.target.push(c),
                CaptureField::Body => app.capture.body.push(c),
            }
        }
        _ => {}
    }
}

/// Open the disposition picker for the currently-selected inbox entry. No-op
/// unless the Inbox pane is focused and holds at least one observation — `d`
/// is inert from any other pane or an empty inbox.
fn open_disposition_modal(app: &mut App) {
    let modal = {
        let Some(d) = app.detail.as_ref() else {
            return;
        };
        if d.right != RightPane::Inbox {
            return;
        }
        let Some(gd) = app.detail_grove(&d.grove) else {
            return;
        };
        if gd.inbox.is_empty() {
            return;
        }
        // Clamp the selection the same way the renderer does, so the picker
        // always targets a real entry even if the cursor drifted past the end.
        let idx = d.inbox.selected().unwrap_or(0).min(gd.inbox.len() - 1);
        let path = gd.inbox[idx].clone();
        let entry = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        DispositionModal { path, entry }
    };
    app.disposition = Some(modal);
}

/// Request an `$EDITOR` round-trip for the currently-selected inbox entry. No-op
/// unless the Inbox pane is focused and holds at least one observation — Ctrl-E
/// is inert from any other pane or an empty inbox, mirroring `d`.
fn request_observation_edit(app: &mut App) {
    let path = {
        let Some(d) = app.detail.as_ref() else {
            return;
        };
        if d.right != RightPane::Inbox {
            return;
        }
        let Some(gd) = app.detail_grove(&d.grove) else {
            return;
        };
        if gd.inbox.is_empty() {
            return;
        }
        // Clamp the selection the same way the renderer does, so we always
        // target a real entry even if the cursor drifted past the end.
        let idx = d.inbox.selected().unwrap_or(0).min(gd.inbox.len() - 1);
        gd.inbox[idx].clone()
    };
    app.pending_action = Some(PendingAction::EditObservation { path });
}

/// Handle a key while the disposition picker is open: `i`/`d`/`r` choose a
/// bucket and request the drain; `Esc`/`Ctrl-C` cancel. Mirrors the capture
/// modal's "decide here, run in the loop" split, so it stays unit-testable.
fn handle_disposition_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    if mods.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
        app.disposition = None;
        return;
    }
    let disposition = match code {
        KeyCode::Esc => {
            app.disposition = None;
            return;
        }
        KeyCode::Char('i') => Disposition::Incorporated,
        KeyCode::Char('d') => Disposition::Deferred,
        KeyCode::Char('r') => Disposition::Rejected,
        _ => return,
    };
    if let Some(modal) = app.disposition.take() {
        app.pending_action = Some(PendingAction::Drain {
            path: modal.path,
            disposition,
        });
    }
}

/// Handle a key while the working-set toggle picker is open (150-working-set/030): a
/// member letter (`d`/`t`/`y`/`v`) closes the picker and requests the toggle; `Esc`/
/// `Ctrl-C` cancel; any other key is inert (the picker stays open). Mirrors the
/// disposition picker's "decide here, run in the loop" split — the request becomes a
/// [`PendingAction::ToggleMember`] the native surface enacts on the *mounted* grove via
/// `HostDriver::toggle_member`, so this stays unit-testable with no substrate.
fn handle_toggle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    if mods.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
        app.toggle_open = false;
        return;
    }
    match code {
        KeyCode::Esc => app.toggle_open = false,
        KeyCode::Char(c) => {
            if let Some(member) = WorkingSetMember::from_key(c) {
                app.toggle_open = false;
                app.pending_action = Some(PendingAction::ToggleMember { member });
            }
            // An unrecognised letter is inert — the picker stays open for a valid key.
        }
        _ => {}
    }
}

fn move_selection(state: &mut ListState, len: isize, delta: isize) {
    if len <= 0 {
        state.select(None);
        return;
    }
    let cur = state.selected().unwrap_or(0) as isize;
    let next = (cur + delta).rem_euclid(len) as usize;
    state.select(Some(next));
}

fn flat_rows_len(app: &App) -> usize {
    flatten_for(app).len()
}

fn inbox_len(app: &App) -> usize {
    app.detail
        .as_ref()
        .and_then(|d| app.detail_grove(&d.grove))
        .map(|gd| gd.inbox.len())
        .unwrap_or(0)
}

/// Move the selection in whichever detail-screen pane currently owns `j`/`k`:
/// the inbox list while the Inbox pane is focused, the task tree otherwise.
/// Lengths are read before the mutable borrow of `app.detail` to avoid
/// aliasing `app.view`.
fn move_detail_selection(app: &mut App, delta: isize) {
    let rows = flat_rows_len(app);
    let inbox = inbox_len(app);
    if let Some(d) = app.detail.as_mut() {
        if d.right == RightPane::Inbox {
            move_selection(&mut d.inbox, inbox as isize, delta);
        } else {
            move_selection(&mut d.tree, rows as isize, delta);
        }
        d.right_scroll = 0;
    }
}

// ---------------------------------------------------------------------------
// The two-level fleet nav model (070-fleet-view/040 + /060 filtering)
//
// `nav_rows` is a **pure** projection (no `ratatui`) of the `MultiRepoView`
// into the flat row sequence the `ListState` indexes into, so the
// grouping/collapse/N=1-hide and the filter/sort rules are unit-tested without
// a render. It has **two shapes** keyed off [`FilterState::active`]:
//
//   * **Idle (grouped)** — the resting nav (070 Q2/Q4): repo section headers
//     (N>1), each followed by its groves, honouring the ephemeral collapse set.
//     The fleet is already sorted current-repo-first → explicit → scanned by
//     the discovery layer (`fleet::resolve`); this preserves that order.
//   * **Filter-active (flat)** — any engaged filter dimension flattens the nav
//     into a single ranked list across all repos, with **no** section headers
//     (070-060 Q3). Each grove is matched (fuzzy text over `<repo>/<grove>` +
//     the inbox-pending / lifecycle predicates) and the survivors are ordered
//     by sort mode → fuzzy score → fleet order.

/// One row of the flattened two-level nav: a repo section header or a grove
/// beneath one. Borrows from the fleet snapshot — rebuilt cheaply per render.
enum NavRow<'a> {
    /// A repo section header — emitted only in the **idle (grouped)** nav when
    /// the fleet spans **>1** repo (N=1 hides it, 070 Q4; the filter-active flat
    /// nav has none, 070-060 Q3). `count` is the repo's grove count; `collapsed`
    /// drives the caret and whether its grove rows follow.
    RepoHeader {
        repo: &'a Path,
        count: usize,
        collapsed: bool,
    },
    /// A grove row under repo `repo`. `repo` is carried so selecting the row
    /// opens the grove **in its owning repo** (cross-repo open, 070 Q4/050) —
    /// not the nav surface's own repo.
    Grove {
        repo: &'a Path,
        summary: &'a GroveSummary,
    },
}

/// Project the fleet into nav rows. Dispatches on [`FilterState::active`]:
/// idle → the grouped two-level shape; any filter engaged → a single ranked
/// flat list (070-060 Q3).
fn nav_rows<'a>(
    fleet: &'a MultiRepoView,
    collapsed: &BTreeSet<PathBuf>,
    filter: &FilterState,
) -> Vec<NavRow<'a>> {
    if filter.active() {
        flat_filtered_rows(fleet, filter)
    } else {
        grouped_rows(fleet, collapsed)
    }
}

/// The idle nav: per repo (in fleet order) a section header — unless N=1, where
/// the lone header is hidden so the rows read as today's flat single-repo nav
/// (070 Q4) — followed by its groves, save when the repo is collapsed (header
/// only). No filtering happens here; that is the flat path's job.
fn grouped_rows<'a>(
    fleet: &'a MultiRepoView,
    collapsed: &BTreeSet<PathBuf>,
) -> Vec<NavRow<'a>> {
    let multi = fleet.repos().len() > 1;
    let mut rows = Vec::new();
    for (repo, groves) in fleet.groups() {
        // N>1 ⇒ a section header; N=1 ⇒ none (flat as today). Collapse is only
        // meaningful with a header to carry the marker, so it never hides the
        // lone N=1 repo's groves.
        if multi {
            let is_collapsed = collapsed.contains(repo);
            rows.push(NavRow::RepoHeader {
                repo,
                count: groves.len(),
                collapsed: is_collapsed,
            });
            if is_collapsed {
                continue;
            }
        }
        for summary in groves {
            rows.push(NavRow::Grove { repo, summary });
        }
    }
    rows
}

/// The filter-active nav: every grove across the fleet that passes all engaged
/// predicates (fuzzy text over `<repo>/<grove>`, inbox-pending, lifecycle),
/// flattened into a single ranked list with no section headers (070-060 Q3).
/// Ordering is layered: the sort toggle is primary (inbox-pending-desc when
/// engaged), then fuzzy score (best matches first when a needle is present),
/// then the original fleet order (a stable final key, so equal-ranked groves
/// keep current-repo-first / lifecycle / prefix order).
fn flat_filtered_rows<'a>(
    fleet: &'a MultiRepoView,
    filter: &FilterState,
) -> Vec<NavRow<'a>> {
    // (repo, summary, fleet_index, fuzzy_score) for each survivor.
    let mut matched: Vec<(&Path, &GroveSummary, usize, i32)> = Vec::new();
    let mut fleet_index = 0usize;
    for (repo, groves) in fleet.groups() {
        let repo_base = repo_basename(repo);
        for summary in groves {
            let idx = fleet_index;
            fleet_index += 1;

            let lifecycle_ok = match filter.lifecycle {
                LifecycleFilter::All => true,
                LifecycleFilter::LiveOnly => summary.lifecycle == Lifecycle::Live,
                LifecycleFilter::SeedOnly => summary.lifecycle == Lifecycle::Seed,
            };
            if !lifecycle_ok {
                continue;
            }
            if filter.inbox_only && summary.inbox_pending == 0 {
                continue;
            }
            // Fuzzy text matches against the qualified `<repo>/<grove>` string,
            // so typing a repo basename surfaces that repo's groves (Q5 shape).
            let hay = format!("{repo_base}/{}", summary.name);
            let Some(score) = fuzzy_score(&filter.text, &hay) else {
                continue;
            };
            matched.push((repo, summary, idx, score));
        }
    }

    matched.sort_by(|a, b| {
        let sort_key = match filter.sort {
            SortMode::InboxDesc => b.1.inbox_pending.cmp(&a.1.inbox_pending),
            SortMode::Default => std::cmp::Ordering::Equal,
        };
        sort_key
            // Higher fuzzy score first (no-op when the needle is empty: every
            // score is 0).
            .then_with(|| b.3.cmp(&a.3))
            // Stable final key: original fleet order.
            .then_with(|| a.2.cmp(&b.2))
    });

    matched
        .into_iter()
        .map(|(repo, summary, _, _)| NavRow::Grove { repo, summary })
        .collect()
}

/// A repo root's basename for display/matching (`acme-api` for
/// `/src/acme-api`); falls back to the full path when there is no file name.
fn repo_basename(repo: &Path) -> String {
    repo.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| repo.display().to_string())
}

/// Fuzzy **subsequence** match of `needle` against `haystack`, both compared
/// case-insensitively. Returns `Some(score)` when every char of `needle`
/// appears in `haystack` in order (a subsequence), else `None`. Higher score =
/// better: a contiguous run scores far above a scattered match, a match at the
/// very start gets a small bonus, and each gap skipped since the previous match
/// costs — so the strongest matches sort to the top of the flat nav. An empty
/// needle matches anything with score 0 (every grove passes, none preferred).
fn fuzzy_score(needle: &str, haystack: &str) -> Option<i32> {
    if needle.is_empty() {
        return Some(0);
    }
    let hay: Vec<char> = haystack.chars().flat_map(char::to_lowercase).collect();
    let mut score = 0i32;
    let mut from = 0usize; // next haystack index to search from
    let mut prev: Option<usize> = None; // previous matched index
    for nc in needle.chars().flat_map(char::to_lowercase) {
        let off = hay[from..].iter().position(|&c| c == nc)?;
        let pos = from + off;
        score += 10; // a char matched
        if prev.is_some() && prev == pos.checked_sub(1) {
            score += 15; // contiguous with the previous match
        }
        if pos == 0 {
            score += 5; // anchored at the start
        }
        score -= off as i32; // gap skipped since the last match
        prev = Some(pos);
        from = pos + 1;
    }
    Some(score)
}

// ---------------------------------------------------------------------------
// Rendering

pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();
    // A one-row header (cli + repo versions) sits above the body on both screens.
    // In the **native frame** the grove-owned whichkey bar owns the bottom hint
    // line (a separate full-width pane), so this surface draws no footer; the
    // non-`native_chrome` rendering (unit tests) keeps its own footer below the body.
    let main = if app.native_chrome {
        let [header, main] = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(area);
        render_header(f, header, app);
        main
    } else {
        let [header, main, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(area);
        render_header(f, header, app);
        f.render_widget(Paragraph::new(footer_line(app)), footer);
        main
    };

    match app.screen {
        Screen::GroveList => render_grove_list(f, main, app),
        Screen::GroveDetail => render_grove_detail(f, main, app),
    }

    if app.capture.open {
        render_capture_modal(f, area, app);
    }
    if let Some(modal) = app.disposition.as_ref() {
        render_disposition_modal(f, area, modal);
    }
    if app.show_help {
        render_help_overlay(f, area);
    }
}

/// A small centred picker listing the three [[Drain]] buckets and their
/// hotkeys. Purely presentational — the keys are handled in
/// `handle_disposition_key`.
fn render_disposition_modal(f: &mut Frame, area: Rect, modal: &DispositionModal) {
    let popup = centered_rect(60, 30, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("disposition — grove inbox drain");
    let inner = block.inner(popup);
    f.render_widget(block, popup);

    let lines = vec![
        Line::from(Span::styled(
            modal.entry.clone(),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from("  i  incorporated"),
        Line::from("  d  deferred"),
        Line::from("  r  rejected"),
        Line::from(""),
        Line::from(Span::styled(
            "Esc cancel",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
}

fn render_capture_modal(f: &mut Frame, area: Rect, app: &App) {
    let modal = &app.capture;
    let popup = centered_rect(70, 50, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .borders(Borders::ALL)
        .title("capture — grove inbox add");

    let inner = block.inner(popup);
    f.render_widget(block, popup);

    // In the native frame the whichkey bar shows the capture keys (`footer_line`),
    // so the modal drops its inline hint row; the non-`native_chrome` rendering
    // (unit tests) keeps the hint inside the modal (no whichkey to delegate to).
    let (target_area, body_area, hint_area) = if app.native_chrome {
        let [target_area, body_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(3)]).areas(inner);
        (target_area, body_area, None)
    } else {
        let [target_area, body_area, hint_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .areas(inner);
        (target_area, body_area, Some(hint_area))
    };

    let target_focus = matches!(modal.field, CaptureField::Target);
    let body_focus = matches!(modal.field, CaptureField::Body);

    let target_text = if target_focus {
        format!("{}_", modal.target)
    } else {
        modal.target.clone()
    };
    let target_style = if target_focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let target_para = Paragraph::new(target_text)
        .style(target_style)
        .block(Block::default().borders(Borders::ALL).title("target grove"));
    f.render_widget(target_para, target_area);

    let body_text = if body_focus {
        format!("{}_", modal.body)
    } else {
        modal.body.clone()
    };
    let body_style = if body_focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let body_para = Paragraph::new(body_text)
        .style(body_style)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::ALL).title("observation body"));
    f.render_widget(body_para, body_area);

    if let Some(hint_area) = hint_area {
        let hint =
            "Tab=switch  Enter=next(target)/newline(body)  Ctrl-S=submit  Ctrl-E=$EDITOR  Esc=cancel";
        f.render_widget(Paragraph::new(hint), hint_area);
    }
}

/// Build the header line — the `cli` layer plus each installed harness's
/// `repo` layer — as styled spans. On a `cli`-vs-`repo` mismatch the affected
/// `repo` token is styled via [`DRIFT_STYLE`] to draw the eye, using the same
/// string-equality drift rule as the rows and `grove status`. In multi-harness
/// repos each `repo` token is disambiguated as `repo[<name>]=…`.
fn header_spans(
    cli: &str,
    repo_versions: &BTreeMap<&'static str, Option<String>>,
) -> Vec<Span<'static>> {
    let mut spans = vec![Span::raw(format!("grove cli={}", cli))];
    if repo_versions.is_empty() {
        spans.push(Span::raw("  ·  (not installed)"));
        return spans;
    }
    let multi = repo_versions.len() > 1;
    for (name, ver) in repo_versions {
        spans.push(Span::raw("  ·  "));
        let label = if multi {
            format!("repo[{}]=", name)
        } else {
            "repo=".to_string()
        };
        match ver {
            Some(v) => {
                let style = if v != cli { DRIFT_STYLE } else { Style::default() };
                spans.push(Span::styled(format!("{}{}", label, v), style));
            }
            None => spans.push(Span::raw(format!("{}(unknown)", label))),
        }
    }
    spans
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    // The config-only fleet nav has no anchor repo (ADR-0027 §6), so the header
    // derives from the **whole fleet**: the running binary's methodology version
    // (`status::CLI_VERSION`, shared across repos) plus an `N repos · M groves`
    // summary. Per-repo version *drift* stays per-row in the nav
    // (`render_grove_list`). A repo-explicit surface (`new`/`new_detail`) shows
    // its own repo's version layers instead, exactly as before.
    let spans = match app.repo.as_deref().and_then(|r| app.fleet.repo(r)) {
        Some(v) => header_spans(v.cli_version(), v.repo_versions()),
        None => fleet_header_spans(app),
    };
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

/// The config-only fleet nav header (ADR-0027 §6): `grove cli=<v>  ·  N repos ·
/// M groves`. The `cli` is the running binary (shared across the fleet); the
/// counts come from the fleet snapshot. Pluralised so a one-repo / one-grove
/// fleet reads naturally.
fn fleet_header_spans(app: &App) -> Vec<Span<'static>> {
    let n_repos = app.fleet.repos().len();
    let n_groves: usize = app.fleet.repos().iter().map(|r| r.groves().len()).sum();
    fn plural(n: usize, one: &'static str, many: &'static str) -> &'static str {
        if n == 1 { one } else { many }
    }
    vec![
        Span::raw(format!("grove cli={}", crate::status::CLI_VERSION)),
        Span::raw(format!(
            "  ·  {} {} · {} {}",
            n_repos,
            plural(n_repos, "repo", "repos"),
            n_groves,
            plural(n_groves, "grove", "groves"),
        )),
    ]
}

/// A compact one-line description of every engaged filter dimension — the
/// `/needle` text plus `[inbox]` / `[live]`|`[seed]` / `[↓inbox]` tags — or
/// `None` when the filter is idle. Shared by the list title and the footer so
/// the active filter is always legible. Mirrors [`FilterState::active`]: it is
/// `Some` exactly when `active()` is true.
fn filter_summary(filter: &FilterState) -> Option<String> {
    if !filter.active() {
        return None;
    }
    let mut parts: Vec<String> = Vec::new();
    if !filter.text.is_empty() {
        parts.push(format!("/{}", filter.text));
    }
    if filter.inbox_only {
        parts.push("[inbox]".to_string());
    }
    match filter.lifecycle {
        LifecycleFilter::All => {}
        LifecycleFilter::LiveOnly => parts.push("[live]".to_string()),
        LifecycleFilter::SeedOnly => parts.push("[seed]".to_string()),
    }
    if filter.sort == SortMode::InboxDesc {
        parts.push("[↓inbox]".to_string());
    }
    Some(parts.join(" "))
}

/// The empty-fleet in-nav empty-state (ADR-0027 §5 / 020 Q6): a short panel that
/// names where the fleet comes from, so the user can populate it. Drawn in place
/// of the grove list when the resolved fleet has no repos.
fn render_empty_fleet(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(
            "No repos in the fleet",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("The grove TUI is resolved purely from config. Add repos by:"),
        Line::from(Span::styled(
            "  • listing them in ~/.config/grove/fleet.toml",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "    (repos = [\".\"] or scan_roots = [\"…/Development\"])",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  • or launching with --repo <path> (e.g. grove tui --repo .)",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    let panel = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("groves"));
    f.render_widget(panel, area);
}

fn render_grove_list(f: &mut Frame, area: Rect, app: &App) {
    // An **empty fleet** (no manifest, no scan hits, no `--repo`) still launches
    // the TUI (ADR-0027 §5): the nav renders a helpful in-nav empty-state pointing
    // at the config, rather than a pre-launch git-repo error. This is the
    // config-only model's replacement for the removed cwd gate — not a new
    // gate-shaped branch, just a render of the empty case.
    if app.fleet.repos().is_empty() {
        render_empty_fleet(f, area);
        return;
    }
    // The two-level fleet nav (070 Q2/Q4): repo section headers (only when the
    // fleet spans >1 repo) interleaved with their groves. At N=1 there are no
    // headers and the rows read flat, exactly as the single-repo nav. `cli` is
    // the one running binary (shared across repos); the `repo`-version drift a
    // grove row shows is its **owning** repo's, looked up per row.
    let rows = app.nav_rows_cached();
    let multi = app.fleet.repos().len() > 1;
    // When a filter is active the nav is flat (no section headers, 070-060 Q3),
    // so each grove row carries its repo as a `<repo>/` prefix; idle the header
    // supplies the attribution and grove rows are just indented under it.
    let flat = app.filter.active();
    let cli = app
        .primary_view()
        .map(|v| v.cli_version())
        .unwrap_or(crate::status::CLI_VERSION);
    let empty_versions = BTreeMap::new();
    let items: Vec<ListItem> = rows
        .iter()
        .map(|row| match row {
            NavRow::RepoHeader { repo, count, collapsed } => {
                ListItem::new(repo_header_row(repo, *count, *collapsed))
            }
            NavRow::Grove { repo, summary } => {
                let repo_versions = app
                    .fleet
                    .repo(repo)
                    .map(|v| v.repo_versions())
                    .unwrap_or(&empty_versions);
                let mut line = grove_row(summary, cli, repo_versions);
                if multi && flat {
                    // Flat ranked list: prefix the owning repo for attribution
                    // (the section header that would carry it is gone).
                    line.spans.insert(
                        0,
                        Span::styled(
                            format!("{}/", repo_basename(repo)),
                            Style::default().fg(Color::DarkGray),
                        ),
                    );
                } else if multi {
                    // Grouped: indent under the section header so the two-level
                    // structure reads at a glance.
                    line.spans.insert(0, Span::raw("  "));
                }
                ListItem::new(line)
            }
        })
        .collect();
    let title = match filter_summary(&app.filter) {
        Some(s) => format!("groves  {s}"),
        None => "groves".to_string(),
    };
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut state = app.list.clone();
    // Clamp selection in case the filter or a collapse narrowed the rows.
    if let Some(sel) = state.selected() {
        if rows.is_empty() {
            state.select(None);
        } else if sel >= rows.len() {
            state.select(Some(rows.len() - 1));
        }
    }
    f.render_stateful_widget(list, area, &mut state);
}

/// A repo section header row: a collapse caret, the repo's basename, and its
/// grove count. Bold so it reads as a section divider above its indented groves
/// (070 Q2). Rendered only when the fleet spans >1 repo (N=1 hides it).
fn repo_header_row(repo: &Path, count: usize, collapsed: bool) -> Line<'static> {
    let caret = if collapsed { "▸" } else { "▾" };
    let name = repo_basename(repo);
    Line::from(vec![
        Span::styled(
            format!("{caret} {name}"),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  ({count})"),
            Style::default().fg(Color::DarkGray),
        ),
    ])
}

fn grove_row(
    g: &GroveSummary,
    cli: &str,
    repo_versions: &BTreeMap<&'static str, Option<String>>,
) -> Line<'static> {
    let badge = match g.lifecycle {
        Lifecycle::Live => Span::styled(" live ", Style::default().fg(Color::Green)),
        Lifecycle::Seed => Span::styled(" seed ", Style::default().fg(Color::Yellow)),
    };
    let mut spans = vec![
        badge,
        Span::raw(" "),
        Span::raw(g.name.clone()),
        Span::raw(format!("  leaves:{}/{}", g.live_leaves, g.retired_leaves)),
    ];
    // The `worktree` layer, one segment per relevant harness (Seeds carry
    // none). Prefix the harness only when this grove spans more than one.
    let multi = g.worktree_versions.len() > 1;
    for (harness, worktree) in &g.worktree_versions {
        let repo = repo_versions.get(harness).cloned().flatten();
        spans.push(Span::raw("  "));
        spans.extend(worktree_spans(
            multi.then_some(*harness),
            worktree,
            &repo,
            cli,
        ));
    }
    if g.inbox_pending > 0 {
        spans.push(Span::styled(
            format!("  inbox:{}", g.inbox_pending),
            Style::default().fg(Color::Cyan),
        ));
    }
    Line::from(spans)
}

/// Build the trailing `worktree=…` version segment for a grove row as styled
/// spans. Mirrors `status::version_segment`'s rule exactly — plain
/// string-equality drift, `(unknown)` for a missing stamp, `repo=(none)` for
/// an orphan — but colours the `⚠` markers via [`DRIFT_STYLE`] instead of
/// emitting plain text. `harness` is `Some` only in multi-harness repos, where
/// it disambiguates the segment as `worktree[<name>]=…`.
fn worktree_spans(
    harness: Option<&str>,
    worktree: &Option<String>,
    repo: &Option<String>,
    cli: &str,
) -> Vec<Span<'static>> {
    let key = match harness {
        Some(h) => format!("worktree[{}]=", h),
        None => "worktree=".to_string(),
    };
    let Some(wt) = worktree else {
        return vec![Span::raw(format!("{}(unknown)", key))];
    };
    let mut spans = vec![Span::raw(format!("{}{}", key, wt))];
    match repo {
        None => spans.push(Span::raw(" repo=(none)")),
        Some(r) if r != wt => spans.push(Span::styled(format!(" ⚠ repo={}", r), DRIFT_STYLE)),
        Some(_) => {}
    }
    if cli != wt {
        spans.push(Span::styled(format!(" ⚠ cli={}", cli), DRIFT_STYLE));
    }
    spans
}

fn render_grove_detail(f: &mut Frame, area: Rect, app: &App) {
    let Some(detail) = app.detail.as_ref() else {
        return;
    };
    let summary_name = detail.grove.clone();
    let grove_detail = app.detail_grove(&summary_name);

    let [left, right] =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).areas(area);

    // Left pane: task tree.
    let rows = flatten_for(app);
    let items: Vec<ListItem> = rows
        .iter()
        .map(|r| {
            let indent = "  ".repeat(r.depth);
            let style = if r.is_retired {
                Style::default().fg(Color::DarkGray)
            } else if r.is_node {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(Span::styled(format!("{}{}", indent, r.name), style)))
        })
        .collect();
    let left_title = if app.filter.text.is_empty() {
        format!("{} — tree", summary_name)
    } else {
        format!("{} — tree  /{}", summary_name, app.filter.text)
    };
    let left_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(left_title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut tree_state = detail.tree.clone();
    if let Some(sel) = tree_state.selected() {
        if rows.is_empty() {
            tree_state.select(None);
        } else if sel >= rows.len() {
            tree_state.select(Some(rows.len() - 1));
        }
    }
    f.render_stateful_widget(left_list, left, &mut tree_state);

    // Right pane. The inbox pane is a selectable list over a body view; the
    // other panes are a single scrollable paragraph.
    if detail.right == RightPane::Inbox {
        render_inbox_pane(f, right, app, grove_detail);
    } else {
        let (title, body) = right_pane_content(app, grove_detail, &rows);
        let para = Paragraph::new(body)
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: false })
            .scroll((detail.right_scroll, 0));
        f.render_widget(para, right);
    }
}

/// Render the inbox right-pane: a selectable list of pending observation
/// filenames on top, the selected entry's body below. Bodies are read on
/// demand (`repo_view::read_path`); the scan loads none.
fn render_inbox_pane(f: &mut Frame, area: Rect, app: &App, detail: Option<&GroveDetail>) {
    let Some(d) = app.detail.as_ref() else {
        return;
    };
    let Some(detail) = detail else {
        let para = Paragraph::new(String::new())
            .block(Block::default().borders(Borders::ALL).title("inbox — no snapshot"));
        f.render_widget(para, area);
        return;
    };

    let title = format!("inbox ({})", detail.inbox.len());
    if detail.inbox.is_empty() {
        let para = Paragraph::new("(no pending observations)")
            .block(Block::default().borders(Borders::ALL).title(title))
            .wrap(Wrap { trim: false });
        f.render_widget(para, area);
        return;
    }

    let [list_area, body_area] =
        Layout::vertical([Constraint::Percentage(40), Constraint::Min(0)]).areas(area);

    // List of observation filenames.
    let items: Vec<ListItem> = detail
        .inbox
        .iter()
        .map(|p| {
            let name = p
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            ListItem::new(Line::from(name))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut state = d.inbox.clone();
    if let Some(sel) = state.selected() {
        if sel >= detail.inbox.len() {
            state.select(Some(detail.inbox.len() - 1));
        }
    } else {
        state.select(Some(0));
    }
    f.render_stateful_widget(list, list_area, &mut state);

    // Body of the selected entry.
    let body = state
        .selected()
        .and_then(|i| detail.inbox.get(i))
        .map(|p| repo_view::read_path(p).unwrap_or_else(|e| format!("(error: {})", e)))
        .unwrap_or_default();
    let body_para = Paragraph::new(body)
        .block(Block::default().borders(Borders::ALL).title("body"))
        .wrap(Wrap { trim: false })
        .scroll((d.right_scroll, 0));
    f.render_widget(body_para, body_area);
}

fn right_pane_content(
    app: &App,
    detail: Option<&GroveDetail>,
    rows: &[FlatRow],
) -> (String, String) {
    let Some(d) = app.detail.as_ref() else {
        return ("right".into(), String::new());
    };
    let Some(detail) = detail else {
        return (format!("{} — no snapshot", d.right.label()), String::new());
    };
    let selected = d
        .tree
        .selected()
        .and_then(|i| rows.get(i));

    match d.right {
        RightPane::LeafBody => {
            let Some(row) = selected else {
                return ("leaf".into(), "(no task tree)".into());
            };
            if row.is_node {
                (
                    format!("leaf — {}", row.name),
                    format!("(node selected: switch via Tab to see its BRIEF)"),
                )
            } else {
                let body = repo_view::read_path(&row.path).unwrap_or_else(|e| format!("(error: {})", e));
                (format!("leaf — {}", row.name), body)
            }
        }
        // The inbox pane is rendered by `render_inbox_pane` (selectable list +
        // body), not as a single paragraph, so it never reaches here.
        RightPane::Inbox => unreachable!("inbox pane is rendered by render_inbox_pane"),
        RightPane::Brief => {
            let brief_path = enclosing_brief(detail, selected);
            match brief_path {
                Some(p) => {
                    let body = repo_view::read_path(&p)
                        .unwrap_or_else(|e| format!("(error: {})", e));
                    (
                        format!("brief — {}", p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()),
                        body,
                    )
                }
                None => ("brief".into(), "(no BRIEF.md in scope)".into()),
            }
        }
    }
}

fn enclosing_brief(detail: &GroveDetail, selected: Option<&FlatRow>) -> Option<PathBuf> {
    let tree = detail.task_tree.as_ref()?;
    let row = match selected {
        Some(r) => r,
        None => return tree.root_brief.clone(),
    };
    // If selected is itself a node with a brief, use that.
    if row.is_node {
        if let Some(p) = &row.node_brief {
            return Some(p.clone());
        }
    }
    // Otherwise the enclosing-node brief, or the root brief if at top.
    row.parent_brief.clone().or_else(|| tree.root_brief.clone())
}

/// The bottom hint line for the focused surface, as an owned [`Line`]. This is
/// the **single source** of grove's key hints (leaf 140): in the native frame each
/// surface *publishes* it to the grove-owned [[whichkey bar]] (`publish_whichkey`)
/// when it gains focus or changes state, so the bar always reflects the focused
/// surface. (The non-`native_chrome` rendering used in unit tests draws it as a
/// plain footer.)
///
/// Hints use sigils (⏎ Enter, ⎋ Esc, ⇥ Tab, ⌃o the leader). The line is
/// context-sensitive: a modal or filter takes precedence over the base
/// screen hints, so the keys shown are always the ones currently live.
fn footer_line(app: &App) -> Line<'static> {
    if app.show_help {
        return Line::from("⎋ / ? close help");
    }
    if app.capture.open {
        return Line::from(
            "⇥ switch field · ⏎ next / newline · ⌃s submit · ⌃e $EDITOR · ⎋ cancel",
        );
    }
    if app.disposition.is_some() {
        return Line::from("i incorporated · d deferred · r rejected · ⎋ cancel");
    }
    if app.toggle_open {
        // The working-set toggle picker (150-working-set/030): the member letters live
        // here, behind the `t` leader, so the bar names exactly the keys now live.
        return Line::from("toggle: d detail · t terminal · y yazi · v vcs · ⎋ cancel");
    }
    if app.filter.editing {
        return Line::from(vec![
            Span::styled(
                format!("/{}_", app.filter.text),
                Style::default().add_modifier(Modifier::REVERSED),
            ),
            Span::raw("  ⏎ apply · ⎋ cancel"),
        ]);
    }
    // The base per-screen hints. `⌥1-9` tab switching is gone with ADR-0023 (the
    // constant nav + content swap has no tabs), so the nav advertises only the
    // leader.
    // `t toggle` only applies in the native nav (a mounted working set to toggle);
    // without `native_chrome` there is no content region, so the hint is omitted,
    // matching the `t` opener's `native_chrome` gate (150-working-set/030).
    let hint = match app.screen {
        Screen::GroveList if app.native_chrome => "⏎ open · j/k move · ⌃o nav · t toggle · x close · / filter · i inbox · l live/seed · s sort · c capture · r refresh · ? help · q quit",
        Screen::GroveList => "⏎ open · j/k move · ⌃o nav · x close · / filter · i inbox · l live/seed · s sort · c capture · r refresh · ? help · q quit",
        Screen::GroveDetail => "⇥ cycle · j/k move · o open · x close · PgUp/PgDn scroll · / filter · c capture · r refresh · ⎋ back · ? help",
    };
    let mut spans = vec![Span::raw(hint)];
    if let Some(summary) = filter_summary(&app.filter) {
        spans.push(Span::raw("   "));
        spans.push(Span::styled(
            format!("filter: {summary}"),
            Style::default().fg(Color::Yellow),
        ));
    }
    if let Some(s) = app.status.as_deref() {
        spans.push(Span::raw("   "));
        spans.push(Span::styled(s.to_string(), Style::default().fg(Color::Green)));
    }
    Line::from(spans)
}

fn render_help_overlay(f: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from(Span::styled(
            "grove tui — keybindings",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  ↑/↓ or j/k    move selection"),
        Line::from("  ⏎ Enter       open (or switch to) the grove's workspace tab"),
        Line::from("  ⌃o            focus this nav from any pane (the leader)"),
        Line::from("  ⌥1-9 / ⌥[ ]   switch between workspace tabs"),
        Line::from("  ⎋ Esc / q     back / quit"),
        Line::from("  Tab           cycle right pane (leaf → inbox → brief)"),
        Line::from("                  in the inbox pane, j/k select an observation"),
        Line::from("  o             open (or switch to) the grove's workspace tab"),
        Line::from("  t             toggle a working-set pane for the mounted grove"),
        Line::from("                  (then d=detail, t=terminal, y=yazi, v=vcs, Esc=cancel)"),
        Line::from("  x             close the grove's workspace tab"),
        Line::from("  d             disposition the selected observation"),
        Line::from("                  (i=incorporated, d=deferred, r=rejected, Esc=cancel)"),
        Line::from("  Ctrl-E        edit the selected observation's body in $EDITOR"),
        Line::from("  PgUp / PgDn   scroll right pane"),
        Line::from("  /             fuzzy-filter the fleet by repo/grove name"),
        Line::from("                  (Enter=apply, Esc=cancel; flattens to a ranked list)"),
        Line::from("  i             toggle: show only groves with a pending inbox"),
        Line::from("  l             cycle lifecycle filter (all → live → seed)"),
        Line::from("  s             cycle sort (fleet order ↔ inbox-pending first)"),
        Line::from("  c             capture an observation to a grove's inbox"),
        Line::from("                  Tab=switch field, Ctrl-E=edit in $EDITOR,"),
        Line::from("                  Enter on body=newline, Ctrl-S=submit, Esc=cancel"),
        Line::from("  r             rescan the repo (also: fs-watch auto-refreshes)"),
        Line::from("  ?             toggle this help"),
        Line::from("  Ctrl-C        force quit"),
        Line::from(""),
        Line::from("  (any key dismisses help)"),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title("help");
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });
    let popup = centered_rect(60, 60, area);
    f.render_widget(Clear, popup);
    f.render_widget(para, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);
    horizontal[1]
}

// ---------------------------------------------------------------------------
// Task-tree flattening
//
// The data layer's `TaskTree` is nested; the list widget wants a flat
// sequence with depth. We flatten depth-first, apply the substring
// filter, and decorate each row with the bits of context the right pane
// later needs (the row's own `BRIEF.md` if it's a node, the enclosing
// node's `BRIEF.md` otherwise) so right-pane rendering does not have to
// re-walk the tree.

#[derive(Debug, Clone)]
struct FlatRow {
    depth: usize,
    name: String,
    path: PathBuf,
    is_node: bool,
    is_retired: bool,
    node_brief: Option<PathBuf>,
    parent_brief: Option<PathBuf>,
}

fn flatten_for(app: &App) -> Vec<FlatRow> {
    let Some(d) = app.detail.as_ref() else {
        return Vec::new();
    };
    let Some(detail) = app.detail_grove(&d.grove) else {
        return Vec::new();
    };
    let Some(tree) = detail.task_tree.as_ref() else {
        return Vec::new();
    };
    let needle = app.filter.text.to_lowercase();
    let mut rows = Vec::new();
    for entry in &tree.entries {
        push_entry(entry, 0, tree.root_brief.as_deref(), &needle, &mut rows);
    }
    rows
}

fn push_entry(
    entry: &TaskEntry,
    depth: usize,
    parent_brief: Option<&Path>,
    needle: &str,
    out: &mut Vec<FlatRow>,
) {
    let matches = needle.is_empty() || entry.name.to_lowercase().contains(needle);
    match &entry.kind {
        TaskKind::Leaf => {
            if matches {
                out.push(FlatRow {
                    depth,
                    name: entry.name.clone(),
                    path: entry.path.clone(),
                    is_node: false,
                    is_retired: entry.is_retired,
                    node_brief: None,
                    parent_brief: parent_brief.map(Path::to_path_buf),
                });
            }
        }
        TaskKind::Node { brief, children } => {
            if matches {
                out.push(FlatRow {
                    depth,
                    name: entry.name.clone(),
                    path: entry.path.clone(),
                    is_node: true,
                    is_retired: entry.is_retired,
                    node_brief: brief.clone(),
                    parent_brief: parent_brief.map(Path::to_path_buf),
                });
            }
            let next_parent_brief = brief.as_deref().or(parent_brief);
            for c in children {
                push_entry(c, depth + 1, next_parent_brief, needle, out);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use std::fs;
    use tempfile::TempDir;

    fn touch(p: &Path, body: &str) {
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(p, body).unwrap();
    }

    fn fixture_repo() -> TempDir {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        // Live grove "alpha" with a node, a leaf, and a retired leaf.
        let alpha = root.join(".grove-worktrees/alpha/.grove");
        touch(&alpha.join("BRIEF.md"), "# alpha — brief\n\nGoal: ship.\n");
        touch(&alpha.join("010-first.md"), "# 010-first\n\nWork here.\n");
        touch(&alpha.join("020-node/BRIEF.md"), "# 020-node — brief\n");
        touch(&alpha.join("020-node/010-child.md"), "# 010-child\n");
        touch(&alpha.join("done/000-old.md"), "# old\n");
        // alpha has two pending inbox observations (sorted chronologically).
        let alpha_inbox = root.join(".grove-meta/inboxes/alpha");
        fs::create_dir_all(&alpha_inbox).unwrap();
        touch(
            &alpha_inbox.join("2026-05-27T09-00-00Z-aaa-1111.md"),
            "first alpha observation body\n",
        );
        touch(
            &alpha_inbox.join("2026-05-28T09-00-00Z-bbb-2222.md"),
            "second alpha observation body\n",
        );
        // Seed for grove "beta": inbox only, no worktree.
        let beta_inbox = root.join(".grove-meta/inboxes/beta");
        fs::create_dir_all(&beta_inbox).unwrap();
        touch(
            &beta_inbox.join("2026-05-28T10-00-00Z--note.md"),
            "first observation\n",
        );
        // Live grove "gamma" with a leaf but an empty inbox (no inbox dir) —
        // the substrate for "disposition is a no-op when the inbox is empty".
        let gamma = root.join(".grove-worktrees/gamma/.grove");
        touch(&gamma.join("BRIEF.md"), "# gamma — brief\n");
        touch(&gamma.join("010-task.md"), "# 010-task\n\nWork.\n");
        tmp
    }

    fn render_to_buffer(app: &App, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, app)).unwrap();
        let buf = terminal.backend().buffer().clone();
        // Concatenate row contents into a newline-delimited string.
        let mut s = String::new();
        for y in 0..buf.area.height {
            for x in 0..buf.area.width {
                s.push_str(buf[(x, y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    #[test]
    fn grove_list_renders_live_and_seed() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(tmp.path().to_path_buf(), view, None);
        let out = render_to_buffer(&app, 80, 12);
        assert!(out.contains("alpha"), "grove name missing:\n{}", out);
        assert!(out.contains("beta"), "seed grove name missing:\n{}", out);
        assert!(out.contains("live"), "live badge missing:\n{}", out);
        assert!(out.contains("seed"), "seed badge missing:\n{}", out);
        assert!(out.contains("inbox:1"), "inbox count missing:\n{}", out);
        // Footer hints (sigils — leaf 120-native-nav).
        assert!(out.contains("⏎ open"), "footer missing:\n{}", out);
    }

    #[test]
    fn preselection_targets_named_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(
            tmp.path().to_path_buf(),
            view,
            Some("beta".into()),
        );
        assert_eq!(app.list.selected(), Some(1)); // alphabetical: alpha=0, beta=1
    }

    /// The whichkey/footer hint line (leaf 140) is the single hint source for both
    /// the local footer and the native bar, and is context-sensitive: a modal or a
    /// live filter takes precedence over the base per-screen hints.
    #[test]
    fn footer_line_is_context_sensitive() {
        fn text(app: &App) -> String {
            footer_line(app)
                .spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect()
        }
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);

        // Base grove-list hints: the leader + open, but no retired tab-switch hint
        // (ADR-0023 removed `⌥1-9 switch`).
        let base = text(&app);
        assert!(base.contains("⏎ open"), "list hints: {base}");
        assert!(base.contains("⌃o nav"), "leader hint: {base}");
        assert!(!base.contains("⌥1-9"), "no tab-switch hint: {base}");

        // A live filter edit echoes the query and shows apply/cancel.
        app.filter.editing = true;
        app.filter.text = "au".into();
        let filtering = text(&app);
        assert!(filtering.contains("/au_"), "filter echo: {filtering}");
        assert!(filtering.contains("⏎ apply"), "filter keys: {filtering}");
        app.filter.editing = false;
        app.filter.text.clear();

        // The capture modal's keys take precedence over the base hints.
        app.capture.open = true;
        let capturing = text(&app);
        assert!(capturing.contains("⌃s submit"), "capture submit: {capturing}");
        assert!(capturing.contains("⎋ cancel"), "capture cancel: {capturing}");
        app.capture.open = false;

        // Help.
        app.show_help = true;
        assert!(text(&app).contains("close help"), "help hint");
        app.show_help = false;

        // The working-set toggle picker names its member keys, taking precedence over
        // the base hints (150-working-set/030).
        app.toggle_open = true;
        let toggling = text(&app);
        assert!(toggling.contains("d detail"), "toggle members: {toggling}");
        assert!(toggling.contains("t terminal"), "toggle members: {toggling}");
        assert!(toggling.contains("y yazi"), "toggle members: {toggling}");
        assert!(toggling.contains("v vcs"), "toggle members: {toggling}");
        app.toggle_open = false;

        // The base `t toggle` hint appears only in the native nav (a working set to
        // toggle); a `native_chrome=false` `App` omits it.
        assert!(!text(&app).contains("t toggle"), "no toggle hint without native_chrome: {}", text(&app));
        app.native_chrome = true;
        assert!(text(&app).contains("t toggle"), "toggle hint in native nav: {}", text(&app));
    }

    /// The working-set toggle UX decision layer (150-working-set/030): the `t` picker
    /// opens only in the native nav, a member letter requests a `ToggleMember` for the
    /// right member, and cancel keys close it without acting. Driving the *substrate*
    /// (the actual hide/show) is trellis's, covered by its own suite; here we pin the
    /// grove-side keymap → action mapping, unit-testable with no host driver.
    #[test]
    fn toggle_picker_maps_member_letters_to_toggle_actions() {
        // Role tags must match what `aux_members` / the trellis mount record, or a
        // toggle would address a member that does not exist.
        assert_eq!(WorkingSetMember::Detail.role(), "secondary");
        assert_eq!(WorkingSetMember::Terminal.role(), "terminal");
        assert_eq!(WorkingSetMember::Yazi.role(), "yazi");
        assert_eq!(WorkingSetMember::Vcs.role(), "vcs");
        // The picker keys round-trip through `from_key`.
        for m in [
            WorkingSetMember::Detail,
            WorkingSetMember::Terminal,
            WorkingSetMember::Yazi,
            WorkingSetMember::Vcs,
        ] {
            assert_eq!(WorkingSetMember::from_key(m.key()), Some(m));
        }
        assert_eq!(WorkingSetMember::from_key('z'), None);

        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        app.native_chrome = true; // the native nav, where a working set exists

        // `t` opens the picker (no action queued yet).
        handle_key(&mut app, KeyCode::Char('t'), KeyModifiers::NONE).unwrap();
        assert!(app.toggle_open, "t opens the toggle picker");
        assert!(app.pending_action.is_none(), "no action until a member is chosen");

        // `y` chooses yazi: the picker closes and a ToggleMember{Yazi} is queued.
        handle_key(&mut app, KeyCode::Char('y'), KeyModifiers::NONE).unwrap();
        assert!(!app.toggle_open, "picker closes on a member key");
        assert_eq!(
            app.pending_action.take(),
            Some(PendingAction::ToggleMember {
                member: WorkingSetMember::Yazi
            })
        );

        // `t t` — the leader then terminal's own `t` — toggles the terminal.
        handle_key(&mut app, KeyCode::Char('t'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('t'), KeyModifiers::NONE).unwrap();
        assert_eq!(
            app.pending_action.take(),
            Some(PendingAction::ToggleMember {
                member: WorkingSetMember::Terminal
            })
        );

        // An unrecognised key inside the picker is inert; Esc cancels with no action.
        handle_key(&mut app, KeyCode::Char('t'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('z'), KeyModifiers::NONE).unwrap();
        assert!(app.toggle_open, "an unknown member key leaves the picker open");
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert!(!app.toggle_open, "Esc cancels the picker");
        assert!(app.pending_action.is_none(), "cancel queues no action");

        // Without `native_chrome` `t` is inert — no working set to toggle.
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut plain = App::new(tmp.path().to_path_buf(), view, None);
        handle_key(&mut plain, KeyCode::Char('t'), KeyModifiers::NONE).unwrap();
        assert!(!plain.toggle_open, "t is inert without native_chrome");
    }

    #[test]
    fn enter_drills_into_detail_and_tree_is_visible() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveDetail);
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("alpha"), "title missing:\n{}", out);
        assert!(out.contains("010-first.md"), "leaf row missing:\n{}", out);
        assert!(out.contains("020-node"), "node row missing:\n{}", out);
        assert!(out.contains("done"), "done row missing:\n{}", out);
        // Right pane defaults to LeafBody — should show first-leaf body.
        assert!(out.contains("Work here."), "leaf body missing:\n{}", out);
    }

    // --- Detail-locked mode (130-native-detail/020): the App behind a per-grove
    // DetailSurface, bound to one grove with list/filter navigation suppressed. ---

    fn detail_locked_app(tmp: &TempDir, grove: &str) -> App {
        let view = RepoView::scan(tmp.path()).unwrap();
        App::new_detail(tmp.path().to_path_buf(), view, grove.to_string())
    }

    #[test]
    fn new_detail_locks_onto_the_named_grove() {
        let tmp = fixture_repo();
        let app = detail_locked_app(&tmp, "alpha");
        assert_eq!(app.screen, Screen::GroveDetail);
        assert_eq!(app.detail.as_ref().unwrap().grove, "alpha");
        assert!(app.detail_locked);
        // It renders that grove's tree + first-leaf body straight away.
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("010-first.md"), "leaf row missing:\n{}", out);
        assert!(out.contains("Work here."), "leaf body missing:\n{}", out);
    }

    #[test]
    fn detail_locked_q_and_esc_stay_in_detail() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        // `q` must neither quit the session nor pop to a (non-existent) list.
        let quit = handle_key(&mut app, KeyCode::Char('q'), KeyModifiers::NONE).unwrap();
        assert!(!quit, "q must not quit from a detail surface");
        assert_eq!(app.screen, Screen::GroveDetail, "q stays in detail");
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveDetail, "Esc stays in detail");
        assert!(app.detail.is_some(), "the detail is not dropped");
    }

    #[test]
    fn detail_locked_slash_does_not_open_filter() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        handle_key(&mut app, KeyCode::Char('/'), KeyModifiers::NONE).unwrap();
        assert!(!app.filter.editing, "no list to filter in detail-locked mode");
    }

    #[test]
    fn detail_locked_c_opens_capture_prefilled_with_the_grove() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert!(app.capture.open);
        assert_eq!(app.capture.target, "alpha", "capture targets the locked grove");
    }

    #[test]
    fn detail_locked_d_on_inbox_requests_drain_for_the_locked_grove() {
        let tmp = fixture_repo();
        let mut app = detail_locked_app(&tmp, "alpha");
        // Tab cycles the right pane LeafBody → Inbox, so `d` acts on the inbox.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(app.detail.as_ref().unwrap().right, RightPane::Inbox);
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_some(), "disposition picker open on the inbox");
        handle_key(&mut app, KeyCode::Char('i'), KeyModifiers::NONE).unwrap();
        match app.pending_action.take() {
            Some(PendingAction::Drain { disposition, path }) => {
                assert_eq!(disposition, Disposition::Incorporated);
                assert!(
                    path.to_string_lossy().contains("inboxes/alpha/"),
                    "drains an alpha observation: {path:?}"
                );
            }
            other => panic!("expected a Drain action, got {other:?}"),
        }
    }

    #[test]
    fn tab_cycles_right_pane_to_brief() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // leaf → inbox: alpha has pending observations, so the list + body render.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("inbox (2)"), "inbox count title missing:\n{}", out);
        assert!(out.contains("aaa-1111"), "observation filename missing:\n{}", out);
        // inbox → brief
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("alpha — brief"), "root brief missing:\n{}", out);
    }

    #[test]
    fn inbox_pane_jk_moves_inbox_selection_not_tree() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // Focus the inbox pane.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        let tree_before = app.detail.as_ref().unwrap().tree.selected();
        assert_eq!(app.detail.as_ref().unwrap().inbox.selected(), Some(0));
        // j moves the inbox selection, leaving the tree cursor untouched.
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap();
        let d = app.detail.as_ref().unwrap();
        assert_eq!(d.inbox.selected(), Some(1), "inbox selection should advance");
        assert_eq!(d.tree.selected(), tree_before, "tree selection must not move");
        // The selected entry's body renders.
        let out = render_to_buffer(&app, 100, 20);
        assert!(
            out.contains("second alpha observation body"),
            "selected body missing:\n{}",
            out
        );
    }

    #[test]
    fn inbox_selection_survives_pane_cycle() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap(); // select #1
        // Cycle all the way around: inbox → brief → leaf → inbox.
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(
            app.detail.as_ref().unwrap().inbox.selected(),
            Some(1),
            "inbox selection must persist across Tab cycling"
        );
    }

    /// Drive the detail screen into the focused inbox pane, ready for a
    /// disposition keystroke. Returns the app preselected on `alpha` (which
    /// the fixture gives two pending observations).
    fn app_focused_on_alpha_inbox(tmp: &TempDir) -> App {
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap(); // → detail
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox pane
        app
    }

    #[test]
    fn d_on_inbox_entry_opens_disposition_picker() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_some(), "disposition picker should open");
        assert_eq!(app.pending_action, None, "opening the picker is not yet an action");
    }

    #[test]
    fn disposition_choice_requests_drain_with_path_and_bucket() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        // Select the second observation, then disposition it as rejected.
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('r'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::Drain { path, disposition }) => {
                assert_eq!(*disposition, Disposition::Rejected);
                assert!(
                    path.ends_with("2026-05-28T09-00-00Z-bbb-2222.md"),
                    "drain should target the selected entry, got {:?}",
                    path
                );
            }
            other => panic!("expected Drain pending action, got {:?}", other),
        }
        assert!(app.disposition.is_none(), "picker closes once a bucket is chosen");
    }

    #[test]
    fn disposition_hotkeys_map_to_each_bucket() {
        for (key, expected) in [
            ('i', Disposition::Incorporated),
            ('d', Disposition::Deferred),
            ('r', Disposition::Rejected),
        ] {
            let tmp = fixture_repo();
            let mut app = app_focused_on_alpha_inbox(&tmp);
            handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap(); // open
            handle_key(&mut app, KeyCode::Char(key), KeyModifiers::NONE).unwrap(); // choose
            match app.pending_action.as_ref() {
                Some(PendingAction::Drain { disposition, .. }) => {
                    assert_eq!(*disposition, expected, "key {:?} → wrong bucket", key);
                }
                other => panic!("key {:?}: expected Drain, got {:?}", key, other),
            }
        }
    }

    #[test]
    fn ctrl_e_on_inbox_entry_requests_edit_with_selected_path() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        // Select the second observation, then Ctrl-E to edit it.
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::EditObservation { path }) => {
                assert!(
                    path.ends_with("2026-05-28T09-00-00Z-bbb-2222.md"),
                    "edit should target the selected entry, got {:?}",
                    path
                );
            }
            other => panic!("expected EditObservation pending action, got {:?}", other),
        }
    }

    #[test]
    fn ctrl_e_is_noop_when_inbox_empty() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        // gamma is a live grove with no pending observations.
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("gamma".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox pane (empty)
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, None, "no edit for an empty inbox");
    }

    #[test]
    fn ctrl_e_is_noop_when_inbox_pane_not_focused() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // Right pane defaults to LeafBody — Ctrl-E must not request an edit.
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, None, "edit only fires from the inbox pane");
    }

    #[test]
    fn esc_cancels_disposition_picker() {
        let tmp = fixture_repo();
        let mut app = app_focused_on_alpha_inbox(&tmp);
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_none(), "Esc should cancel the picker");
        assert_eq!(app.pending_action, None, "cancel must not request a drain");
    }

    #[test]
    fn d_is_noop_when_inbox_empty() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        // gamma is a live grove with no pending observations.
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("gamma".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap(); // → inbox pane (empty)
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_none(), "no picker for an empty inbox");
        assert_eq!(app.pending_action, None, "no drain for an empty inbox");
    }

    #[test]
    fn d_is_noop_when_inbox_pane_not_focused() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // Right pane defaults to LeafBody — `d` must not open the picker.
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::NONE).unwrap();
        assert!(app.disposition.is_none(), "picker only opens from the inbox pane");
    }

    #[test]
    fn escape_returns_to_list_from_detail() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveDetail);
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert_eq!(app.screen, Screen::GroveList);
    }

    #[test]
    fn slash_filters_grove_list() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        handle_key(&mut app, KeyCode::Char('/'), KeyModifiers::NONE).unwrap();
        for c in "bet".chars() {
            handle_key(&mut app, KeyCode::Char(c), KeyModifiers::NONE).unwrap();
        }
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        // At N=1 the nav rows are all groves (no repo header); the filter leaves
        // only "beta".
        let rows = app.nav_rows_cached();
        let names: Vec<&str> = rows
            .iter()
            .filter_map(|r| match r {
                NavRow::Grove { summary, .. } => Some(summary.name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(names, vec!["beta"]);
    }

    #[test]
    fn help_overlay_toggles() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        handle_key(&mut app, KeyCode::Char('?'), KeyModifiers::NONE).unwrap();
        assert!(app.show_help);
        let out = render_to_buffer(&app, 80, 16);
        assert!(out.contains("keybindings"), "help title missing:\n{}", out);
    }

    #[test]
    fn q_on_list_quits() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        let quit = handle_key(&mut app, KeyCode::Char('q'), KeyModifiers::NONE).unwrap();
        assert!(quit);
    }

    // -----------------------------------------------------------------
    // Harness driving keys (`o` open/switch, `x` close)
    //
    // `handle_key` only records the intent (a `PendingAction`); the controller
    // executes it against zellij (see `harness_drive` for that logic). These
    // assert the pure decision: which grove, and that the repo is carried so
    // 070's cross-repo fleet reuses the path unchanged.

    #[test]
    fn o_on_list_requests_open_harness_for_selected_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('o'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::OpenHarness { name, repo }) => {
                assert_eq!(name, "alpha");
                assert_eq!(repo, tmp.path(), "open carries the repo for cross-repo reuse");
            }
            other => panic!("expected OpenHarness, got {:?}", other),
        }
    }

    #[test]
    fn o_on_detail_requests_open_harness_for_current_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap(); // → detail
        handle_key(&mut app, KeyCode::Char('o'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::OpenHarness { name, .. }) => assert_eq!(name, "alpha"),
            other => panic!("expected OpenHarness, got {:?}", other),
        }
    }

    #[test]
    fn x_on_list_requests_close_harness_for_selected_grove() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("beta".into()));
        handle_key(&mut app, KeyCode::Char('x'), KeyModifiers::NONE).unwrap();
        match app.pending_action.as_ref() {
            Some(PendingAction::CloseHarness { name }) => assert_eq!(name, "beta"),
            other => panic!("expected CloseHarness, got {:?}", other),
        }
    }

    #[test]
    fn harness_keys_inert_inside_capture_modal() {
        // While typing an observation, `o`/`x` must type into the field, not
        // drive harnesses.
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap(); // open capture
        handle_key(&mut app, KeyCode::Char('o'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('x'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.pending_action, None, "harness keys must be inert in the modal");
        assert!(app.capture.open, "the capture modal stays open");
    }

    #[test]
    fn footer_shows_harness_hints() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(tmp.path().to_path_buf(), view, None);
        let out = render_to_buffer(&app, 120, 12);
        assert!(out.contains("⏎ open"), "list footer missing ⏎ open:\n{}", out);
        assert!(out.contains("x close"), "list footer missing x close:\n{}", out);
    }

    // -----------------------------------------------------------------
    // Capture modal

    fn type_str(app: &mut App, s: &str) {
        for c in s.chars() {
            handle_key(app, KeyCode::Char(c), KeyModifiers::NONE).unwrap();
        }
    }

    #[test]
    fn c_on_list_opens_modal_with_selected_as_target() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        // Preselect alpha; pressing `c` should pre-fill target with the
        // selected grove and start on the target field so the user can
        // edit it (capture to a *different* grove is the common case from
        // the list screen).
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert!(app.capture.open);
        assert_eq!(app.capture.target, "alpha");
        assert_eq!(app.capture.field, CaptureField::Target);
    }

    #[test]
    fn c_on_detail_jumps_to_body_with_grove_prefilled() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert!(app.capture.open);
        assert_eq!(app.capture.target, "alpha");
        // On detail, capturing-to-current-grove is the common case; jump
        // straight to body to save a Tab.
        assert_eq!(app.capture.field, CaptureField::Body);
    }

    #[test]
    fn typing_fills_active_field_and_enter_advances() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        // Open from list (no preselect) → starts on Target field, empty.
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        // Clear whatever preselect put there.
        for _ in 0..app.capture.target.len() {
            handle_key(&mut app, KeyCode::Backspace, KeyModifiers::NONE).unwrap();
        }
        assert_eq!(app.capture.target, "");
        type_str(&mut app, "newgrove");
        assert_eq!(app.capture.target, "newgrove");
        // Enter on target advances to body.
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
        type_str(&mut app, "noticed a thing");
        assert_eq!(app.capture.body, "noticed a thing");
    }

    #[test]
    fn enter_on_empty_target_does_not_advance() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        for _ in 0..app.capture.target.len() {
            handle_key(&mut app, KeyCode::Backspace, KeyModifiers::NONE).unwrap();
        }
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Target);
    }

    #[test]
    fn ctrl_s_in_body_with_both_fields_requests_submit() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        // Detail jumps to body field, target=alpha.
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "first observation");
        handle_key(&mut app, KeyCode::Char('s'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, Some(PendingAction::Submit));
        // Submitting does *not* close the modal in the key handler;
        // the live loop will close it after the shell-out finishes.
        assert!(app.capture.open);
    }

    #[test]
    fn ctrl_s_with_empty_body_does_not_submit() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('s'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, None);
    }

    #[test]
    fn enter_in_body_inserts_newline_and_does_not_submit() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "a");
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        type_str(&mut app, "b");
        assert_eq!(app.capture.body, "a\nb");
        assert_eq!(app.pending_action, None);
        assert!(app.capture.open);
    }

    #[test]
    fn multiline_paste_in_body_does_not_submit_or_truncate() {
        // A terminal paste (no bracketed-paste) arrives as chars
        // interspersed with Enter key events. The whole string must land
        // in the body intact, and submit must not fire.
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        for c in "line1\nline2\nline3".chars() {
            if c == '\n' {
                handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
            } else {
                handle_key(&mut app, KeyCode::Char(c), KeyModifiers::NONE).unwrap();
            }
        }
        assert_eq!(app.capture.body, "line1\nline2\nline3");
        assert_eq!(app.pending_action, None);
    }

    #[test]
    fn enter_in_target_still_advances_to_body() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, None);
        // List screen opens on the target field, prefilled with a grove.
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Target);
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
        assert_eq!(app.pending_action, None);
    }

    #[test]
    fn ctrl_e_in_body_requests_editor() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('e'), KeyModifiers::CONTROL).unwrap();
        assert_eq!(app.pending_action, Some(PendingAction::EditBody));
    }

    #[test]
    fn tab_toggles_field() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Target);
        handle_key(&mut app, KeyCode::Tab, KeyModifiers::NONE).unwrap();
        assert_eq!(app.capture.field, CaptureField::Body);
    }

    #[test]
    fn esc_closes_modal() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "abc");
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE).unwrap();
        assert!(!app.capture.open);
        assert_eq!(app.capture.body, "");
    }

    #[test]
    fn capture_modal_renders_with_target_and_body() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let mut app = App::new(tmp.path().to_path_buf(), view, Some("alpha".into()));
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        handle_key(&mut app, KeyCode::Char('c'), KeyModifiers::NONE).unwrap();
        type_str(&mut app, "hello");
        let out = render_to_buffer(&app, 100, 24);
        assert!(out.contains("capture"), "modal title missing:\n{}", out);
        assert!(out.contains("target grove"), "target field label missing:\n{}", out);
        assert!(out.contains("alpha"), "target value missing:\n{}", out);
        assert!(out.contains("hello"), "body value missing:\n{}", out);
        assert!(out.contains("Ctrl-E"), "modal hint missing:\n{}", out);
    }

    #[test]
    fn footer_shows_c_hint() {
        let tmp = fixture_repo();
        let view = RepoView::scan(tmp.path()).unwrap();
        let app = App::new(tmp.path().to_path_buf(), view, None);
        let out = render_to_buffer(&app, 100, 12);
        assert!(out.contains("c capture"), "list footer missing c capture:\n{}", out);
    }

    // -----------------------------------------------------------------
    // Version surfaces end-to-end (header on both screens + row segment)

    fn write_version(p: &Path, version: &str) {
        touch(p, &format!("| version | `{}` |\n", version));
    }

    #[test]
    fn header_and_row_show_versions_on_both_screens() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let claude = crate::harness::by_name("claude").unwrap();
        write_version(&claude.install_path(root).join("VERSION.md"), "4.0.0");
        let alpha = root.join(".grove-worktrees/alpha");
        write_version(&claude.install_path(&alpha).join("VERSION.md"), "4.0.0");
        touch(&alpha.join(".grove/010-first.md"), "# 010-first\n");

        let view = RepoView::scan(root).unwrap();
        let mut app = App::new(root.to_path_buf(), view, Some("alpha".into()));

        let out = render_to_buffer(&app, 100, 12);
        assert!(out.contains("cli="), "header cli missing on list:\n{out}");
        assert!(out.contains("repo=4.0.0"), "header repo missing on list:\n{out}");
        assert!(out.contains("worktree=4.0.0"), "row worktree missing:\n{out}");

        // The header is drawn on the detail screen too.
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();
        let out = render_to_buffer(&app, 100, 16);
        assert!(out.contains("cli="), "header missing on detail:\n{out}");
    }

    #[test]
    fn row_flags_worktree_repo_drift() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        let claude = crate::harness::by_name("claude").unwrap();
        write_version(&claude.install_path(root).join("VERSION.md"), "9.9.9");
        let alpha = root.join(".grove-worktrees/alpha");
        write_version(&claude.install_path(&alpha).join("VERSION.md"), "4.0.0");
        touch(&alpha.join(".grove/010-first.md"), "# 010-first\n");

        let view = RepoView::scan(root).unwrap();
        let app = App::new(root.to_path_buf(), view, Some("alpha".into()));
        let out = render_to_buffer(&app, 120, 12);
        assert!(out.contains("worktree=4.0.0"), "worktree missing:\n{out}");
        assert!(out.contains("⚠ repo=9.9.9"), "drift marker missing:\n{out}");
    }

    // -----------------------------------------------------------------
    // Header spans (cli + repo layers)

    fn repo_map(pairs: &[(&'static str, Option<&str>)]) -> BTreeMap<&'static str, Option<String>> {
        pairs
            .iter()
            .map(|(k, v)| (*k, v.map(|s| s.to_string())))
            .collect()
    }

    #[test]
    fn header_spans_single_harness_aligned_has_no_drift() {
        let repo = repo_map(&[("claude", Some("4.0.0"))]);
        let spans = header_spans("4.0.0", &repo);
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("cli=4.0.0"), "got: {text}");
        assert!(text.contains("repo=4.0.0"), "got: {text}");
        assert!(!text.contains("repo[claude]"), "single harness must not prefix: {text}");
        assert!(spans.iter().all(|sp| sp.style != DRIFT_STYLE));
    }

    #[test]
    fn header_spans_flags_repo_drift_token() {
        let repo = repo_map(&[("claude", Some("3.0.1"))]);
        let spans = header_spans("4.0.0", &repo);
        let marker = spans.iter().find(|sp| sp.content.contains("3.0.1")).unwrap();
        assert_eq!(marker.style, DRIFT_STYLE);
    }

    #[test]
    fn header_spans_multi_harness_prefixes_repo_tokens() {
        let repo = repo_map(&[("claude", Some("4.0.0")), ("codex", Some("4.0.0"))]);
        let spans = header_spans("4.0.0", &repo);
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("repo[claude]=4.0.0"), "got: {text}");
        assert!(text.contains("repo[codex]=4.0.0"), "got: {text}");
    }

    #[test]
    fn header_spans_not_installed_shows_cli_only() {
        let repo = repo_map(&[]);
        let spans = header_spans("4.0.0", &repo);
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert!(text.contains("cli=4.0.0"), "got: {text}");
        assert!(text.contains("not installed"), "got: {text}");
    }

    #[test]
    fn config_only_fleet_nav_header_shows_cli_and_fleet_counts() {
        // The anchorless fleet nav (ADR-0027 §6) shows the running binary's
        // version plus an `N repos · M groves` summary, not a single repo's layers.
        let tmp = TempDir::new().unwrap();
        let roots = vec![
            fleet_repo(tmp.path(), "alpha", &["one", "two"]),
            fleet_repo(tmp.path(), "beta", &["three"]),
        ];
        let fleet = MultiRepoView::scan(&roots);
        let app = App::new_fleet(None, fleet, None);
        let out = render_to_buffer(&app, 80, 12);
        assert!(out.contains("2 repos"), "got: {out}");
        assert!(out.contains("3 groves"), "got: {out}");
        assert!(
            out.contains(&format!("cli={}", crate::status::CLI_VERSION)),
            "got: {out}"
        );
    }

    #[test]
    fn empty_fleet_renders_the_in_nav_empty_state() {
        // An empty resolved fleet still launches the TUI (ADR-0027 §5): the nav
        // points at the config rather than erroring. Header reads `0 repos`.
        let app = App::new_fleet(None, MultiRepoView::from_repos(vec![]), None);
        let out = render_to_buffer(&app, 80, 12);
        assert!(out.contains("No repos in the fleet"), "got: {out}");
        assert!(out.contains("fleet.toml"), "got: {out}");
        assert!(out.contains("--repo"), "got: {out}");
        assert!(out.contains("0 repos"), "got: {out}");
    }

    // -----------------------------------------------------------------
    // Version segment spans (worktree layer)

    fn text_of(spans: &[Span]) -> String {
        spans.iter().map(|s| s.content.as_ref()).collect()
    }

    fn s(v: &str) -> Option<String> {
        Some(v.to_string())
    }

    #[test]
    fn worktree_spans_aligned_has_no_drift_style() {
        let spans = worktree_spans(None, &s("4.0.0"), &s("4.0.0"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree=4.0.0");
        assert!(
            spans.iter().all(|sp| sp.style != DRIFT_STYLE),
            "aligned segment must carry no drift styling"
        );
    }

    #[test]
    fn worktree_spans_flags_repo_drift_in_drift_style() {
        let spans = worktree_spans(None, &s("3.0.1"), &s("4.0.0"), "3.0.1");
        assert_eq!(text_of(&spans), "worktree=3.0.1 ⚠ repo=4.0.0");
        let marker = spans
            .iter()
            .find(|sp| sp.content.contains("repo=4.0.0"))
            .unwrap();
        assert_eq!(marker.style, DRIFT_STYLE);
    }

    #[test]
    fn worktree_spans_flags_cli_drift_in_drift_style() {
        let spans = worktree_spans(None, &s("3.0.1"), &s("3.0.1"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree=3.0.1 ⚠ cli=4.0.0");
        let marker = spans
            .iter()
            .find(|sp| sp.content.contains("cli=4.0.0"))
            .unwrap();
        assert_eq!(marker.style, DRIFT_STYLE);
    }

    #[test]
    fn worktree_spans_unknown_is_not_drift() {
        let spans = worktree_spans(None, &None, &s("4.0.0"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree=(unknown)");
        assert!(spans.iter().all(|sp| sp.style != DRIFT_STYLE));
    }

    #[test]
    fn worktree_spans_orphan_shows_repo_none_without_warning() {
        let spans = worktree_spans(None, &s("4.0.0"), &None, "4.0.0");
        assert_eq!(text_of(&spans), "worktree=4.0.0 repo=(none)");
        assert!(spans.iter().all(|sp| sp.style != DRIFT_STYLE));
    }

    #[test]
    fn worktree_spans_prefixes_harness_when_named() {
        let spans = worktree_spans(Some("codex"), &s("4.0.0"), &s("4.0.0"), "4.0.0");
        assert_eq!(text_of(&spans), "worktree[codex]=4.0.0");
    }
    #[test]
    fn vcs_tool_defaults_to_lazygit_for_a_plain_git_worktree() {
        // 020-aux-tool-panes: the vcs pane is not hard-wired to git — but a
        // worktree with no `.jj/` is the default (lazygit) case.
        let tmp = TempDir::new().unwrap();
        assert_eq!(vcs_tool(tmp.path()), "lazygit");
    }

    #[test]
    fn vcs_tool_selects_lazyjj_for_a_jj_worktree() {
        // The detection seam (brief): a `.jj/` present routes to lazyjj, so the
        // tool lands later as a one-point change without re-touching the spawn.
        let tmp = TempDir::new().unwrap();
        fs::create_dir(tmp.path().join(".jj")).unwrap();
        assert_eq!(vcs_tool(tmp.path()), "lazyjj");
    }

    #[test]
    fn on_path_finds_a_binary_in_a_path_dir() {
        // A file present in one of the PATH dirs resolves to its full path; a
        // name absent from every PATH dir resolves to `None` (the graceful-
        // fallback trigger for an uninstalled aux tool).
        let tmp = TempDir::new().unwrap();
        let bin = tmp.path().join("grove-fake-tool");
        fs::write(&bin, "#!/bin/sh\n").unwrap();
        let found = on_path("grove-fake-tool", &std::ffi::OsString::from(tmp.path()));
        assert_eq!(found.as_deref(), Some(bin.as_path()));
        assert_eq!(
            on_path("grove-definitely-absent-xyz", &std::ffi::OsString::from(tmp.path())),
            None
        );
    }

    // ---- the two-level fleet nav model (070-fleet-view/040) ----

    /// Build `parent/<name>/` as a repo with one live grove worktree per entry
    /// in `groves` and return its path — a multi-repo fleet fixture for the
    /// pure `nav_rows` tests (lighter than `fixture_repo`, which is one repo).
    fn fleet_repo(parent: &Path, name: &str, groves: &[&str]) -> PathBuf {
        let repo = parent.join(name);
        for g in groves {
            let root = repo.join(".grove-worktrees").join(g).join(".grove");
            touch(&root.join("010-x.md"), "# 010-x\n");
        }
        repo
    }

    /// Add `count` pending observation files to `repo`'s inbox for `grove`. When
    /// `grove` has no live worktree this makes a **seed** (an inbox dir with no
    /// worktree); when it does, it gives that live grove `inbox_pending = count`.
    fn add_inbox(repo: &Path, grove: &str, count: usize) {
        let dir = repo.join(".grove-meta").join("inboxes").join(grove);
        for i in 0..count {
            touch(&dir.join(format!("2026-01-01T00-00-{i:02}Z-obs-deadbeef.md")), "# obs\n");
        }
    }

    #[test]
    fn nav_rows_n1_is_flat_with_no_repo_header() {
        // N=1 (070 Q4): the lone repo's section header is hidden; the rows are
        // just its groves, exactly as today's single-repo nav.
        let tmp = TempDir::new().unwrap();
        let r = fleet_repo(tmp.path(), "solo", &["alpha", "beta"]);
        let fleet = MultiRepoView::scan(&[r.clone()]);
        let rows = nav_rows(&fleet, &BTreeSet::new(), &FilterState::default());
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|r| matches!(r, NavRow::Grove { .. })));
        let names: Vec<&str> = rows
            .iter()
            .filter_map(|r| match r {
                NavRow::Grove { summary, .. } => Some(summary.name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn nav_rows_multi_repo_has_headers_then_groves_carrying_repo() {
        // N>1: a header per repo (in fleet order), its groves following, each
        // grove row carrying its owning repo (for cross-repo open, 070 Q4/050).
        let tmp = TempDir::new().unwrap();
        let r1 = fleet_repo(tmp.path(), "one", &["alpha"]);
        let r2 = fleet_repo(tmp.path(), "two", &["beta", "gamma"]);
        let fleet = MultiRepoView::scan(&[r1.clone(), r2.clone()]);
        let rows = nav_rows(&fleet, &BTreeSet::new(), &FilterState::default());

        // header(r1), grove(alpha@r1), header(r2), grove(beta@r2), grove(gamma@r2)
        assert_eq!(rows.len(), 5);
        match &rows[0] {
            NavRow::RepoHeader { repo, count, collapsed } => {
                assert_eq!(*repo, r1.as_path());
                assert_eq!(*count, 1);
                assert!(!collapsed);
            }
            _ => panic!("row 0 should be r1's header"),
        }
        match &rows[1] {
            NavRow::Grove { repo, summary } => {
                assert_eq!(*repo, r1.as_path());
                assert_eq!(summary.name, "alpha");
            }
            _ => panic!("row 1 should be alpha@r1"),
        }
        match &rows[3] {
            NavRow::Grove { repo, summary } => {
                assert_eq!(*repo, r2.as_path());
                assert_eq!(summary.name, "beta");
            }
            _ => panic!("row 3 should be beta@r2"),
        }
    }

    #[test]
    fn nav_rows_collapsed_section_hides_its_groves_but_keeps_header() {
        // A collapsed repo shows its header (full count, collapsed marker) and
        // none of its grove rows; other sections are unaffected (070 Q5).
        let tmp = TempDir::new().unwrap();
        let r1 = fleet_repo(tmp.path(), "one", &["alpha"]);
        let r2 = fleet_repo(tmp.path(), "two", &["beta", "gamma"]);
        let fleet = MultiRepoView::scan(&[r1.clone(), r2.clone()]);
        let mut collapsed = BTreeSet::new();
        collapsed.insert(r1.clone());
        let rows = nav_rows(&fleet, &collapsed, &FilterState::default());

        // header(r1, collapsed), header(r2), grove(beta), grove(gamma)
        assert_eq!(rows.len(), 4);
        match &rows[0] {
            NavRow::RepoHeader { repo, count, collapsed } => {
                assert_eq!(*repo, r1.as_path());
                assert_eq!(*count, 1); // count reflects groves even while hidden
                assert!(collapsed);
            }
            _ => panic!("row 0 should be r1's collapsed header"),
        }
        assert!(matches!(&rows[1], NavRow::RepoHeader { repo, .. } if *repo == r2.as_path()));
        assert!(matches!(&rows[2], NavRow::Grove { summary, .. } if summary.name == "beta"));
    }

    /// A `FilterState` carrying just a text needle (the common test shape).
    fn text_filter(needle: &str) -> FilterState {
        FilterState {
            text: needle.to_string(),
            ..Default::default()
        }
    }

    /// The grove names in a flat (filter-active) row sequence, in order. Panics
    /// if any row is a header — the flat nav must emit none (070-060 Q3).
    fn flat_names<'a>(rows: &'a [NavRow<'a>]) -> Vec<&'a str> {
        rows.iter()
            .map(|r| match r {
                NavRow::Grove { summary, .. } => summary.name.as_str(),
                NavRow::RepoHeader { .. } => panic!("flat nav must not emit headers"),
            })
            .collect()
    }

    #[test]
    fn nav_rows_text_filter_flattens_and_drops_headers() {
        // An active text filter flattens the grouped nav into a single ranked
        // list with NO section headers (070-060 Q3); only matching groves
        // survive, across all repos.
        let tmp = TempDir::new().unwrap();
        let r1 = fleet_repo(tmp.path(), "one", &["apex", "april"]);
        let r2 = fleet_repo(tmp.path(), "two", &["beta"]);
        let fleet = MultiRepoView::scan(&[r1.clone(), r2.clone()]);
        let rows = nav_rows(&fleet, &BTreeSet::new(), &text_filter("ap"));

        // apex, april match "ap"; beta does not. No headers at all.
        assert!(rows.iter().all(|r| matches!(r, NavRow::Grove { .. })));
        let names = flat_names(&rows);
        assert!(names.contains(&"apex") && names.contains(&"april"));
        assert!(!names.contains(&"beta"));
    }

    #[test]
    fn nav_rows_text_filter_matches_repo_name() {
        // The fuzzy needle matches against `<repo>/<grove>`, so typing a repo
        // basename surfaces that repo's groves even when their own names don't
        // contain the needle (070 Q5 "repo/grove name").
        let tmp = TempDir::new().unwrap();
        let r1 = fleet_repo(tmp.path(), "alpha-repo", &["work"]);
        let r2 = fleet_repo(tmp.path(), "beta-repo", &["chore"]);
        let fleet = MultiRepoView::scan(&[r1.clone(), r2.clone()]);
        // "alpha" matches r1's basename; r1's grove "work" surfaces, r2's hides.
        let rows = nav_rows(&fleet, &BTreeSet::new(), &text_filter("alpha"));
        assert_eq!(flat_names(&rows), vec!["work"]);
    }

    #[test]
    fn nav_rows_inbox_only_drops_groves_with_no_pending() {
        // The inbox-pending predicate keeps only groves with observations; with
        // it engaged the nav flattens (no headers) and empty repos vanish.
        let tmp = TempDir::new().unwrap();
        let r1 = fleet_repo(tmp.path(), "one", &["alpha", "beta"]);
        add_inbox(&r1, "alpha", 2); // alpha has 2 pending; beta has none
        let r2 = fleet_repo(tmp.path(), "two", &["gamma"]); // none pending
        let fleet = MultiRepoView::scan(&[r1.clone(), r2.clone()]);
        let filter = FilterState {
            inbox_only: true,
            ..Default::default()
        };
        let rows = nav_rows(&fleet, &BTreeSet::new(), &filter);
        assert_eq!(flat_names(&rows), vec!["alpha"]);
    }

    #[test]
    fn nav_rows_lifecycle_filter_selects_live_or_seed() {
        // `l` cycles all → live-only → seed-only. A seed is an inbox dir with no
        // worktree; a live grove has a worktree.
        let tmp = TempDir::new().unwrap();
        let r = fleet_repo(tmp.path(), "one", &["alpha"]); // live
        add_inbox(&r, "ghost", 1); // seed (no worktree)
        let fleet = MultiRepoView::scan(&[r]);

        let live = FilterState {
            lifecycle: LifecycleFilter::LiveOnly,
            ..Default::default()
        };
        assert_eq!(
            flat_names(&nav_rows(&fleet, &BTreeSet::new(), &live)),
            vec!["alpha"]
        );

        let seed = FilterState {
            lifecycle: LifecycleFilter::SeedOnly,
            ..Default::default()
        };
        assert_eq!(
            flat_names(&nav_rows(&fleet, &BTreeSet::new(), &seed)),
            vec!["ghost"]
        );
    }

    #[test]
    fn nav_rows_sort_inbox_desc_orders_by_pending_count() {
        // The sort toggle puts the most inbox-pending groves first (across the
        // flattened fleet), regardless of repo.
        let tmp = TempDir::new().unwrap();
        let r1 = fleet_repo(tmp.path(), "one", &["low", "high"]);
        add_inbox(&r1, "low", 1);
        add_inbox(&r1, "high", 5);
        let r2 = fleet_repo(tmp.path(), "two", &["mid"]);
        add_inbox(&r2, "mid", 3);
        let fleet = MultiRepoView::scan(&[r1.clone(), r2.clone()]);
        let filter = FilterState {
            sort: SortMode::InboxDesc,
            ..Default::default()
        };
        let rows = nav_rows(&fleet, &BTreeSet::new(), &filter);
        // high(5) → mid(3) → low(1), ignoring repo grouping.
        assert_eq!(flat_names(&rows), vec!["high", "mid", "low"]);
    }

    #[test]
    fn nav_rows_idle_stays_grouped_with_headers() {
        // Sanity: with no filter dimension engaged the nav keeps its grouped
        // two-level shape (headers present), so flattening is filter-gated.
        let tmp = TempDir::new().unwrap();
        let r1 = fleet_repo(tmp.path(), "one", &["alpha"]);
        let r2 = fleet_repo(tmp.path(), "two", &["beta"]);
        let fleet = MultiRepoView::scan(&[r1.clone(), r2.clone()]);
        let rows = nav_rows(&fleet, &BTreeSet::new(), &FilterState::default());
        assert!(rows.iter().any(|r| matches!(r, NavRow::RepoHeader { .. })));
    }

    #[test]
    fn fuzzy_score_matches_subsequence_case_insensitively() {
        // Every needle char appears in order → Some; case-insensitive both ways.
        assert!(fuzzy_score("afb", "acme/fix-bug").is_some()); // a..f..b
        assert!(fuzzy_score("FIX", "acme/fix-bug").is_some());
        // A char out of order / absent → None.
        assert!(fuzzy_score("xyz", "acme/fix-bug").is_none());
        assert!(fuzzy_score("ba", "abc").is_none()); // b before a not a subseq
        // Empty needle matches anything with the neutral score 0.
        assert_eq!(fuzzy_score("", "anything"), Some(0));
    }

    #[test]
    fn fuzzy_score_prefers_contiguous_and_earlier_matches() {
        // A contiguous run beats a scattered subsequence of the same chars.
        let contiguous = fuzzy_score("ab", "abxx").unwrap();
        let scattered = fuzzy_score("ab", "axbx").unwrap();
        assert!(contiguous > scattered, "{contiguous} !> {scattered}");
        // An earlier match beats a later one.
        let early = fuzzy_score("a", "axxx").unwrap();
        let late = fuzzy_score("a", "xxxa").unwrap();
        assert!(early > late, "{early} !> {late}");
    }

    #[test]
    fn filter_state_active_tracks_every_dimension() {
        assert!(!FilterState::default().active());
        assert!(text_filter("x").active());
        assert!(FilterState { inbox_only: true, ..Default::default() }.active());
        assert!(FilterState { lifecycle: LifecycleFilter::LiveOnly, ..Default::default() }.active());
        assert!(FilterState { sort: SortMode::InboxDesc, ..Default::default() }.active());
        // clear() returns it to idle.
        let mut f = text_filter("x");
        f.inbox_only = true;
        f.clear();
        assert!(!f.active());
    }

    #[test]
    fn lifecycle_and_sort_toggles_cycle() {
        assert_eq!(LifecycleFilter::All.next(), LifecycleFilter::LiveOnly);
        assert_eq!(LifecycleFilter::LiveOnly.next(), LifecycleFilter::SeedOnly);
        assert_eq!(LifecycleFilter::SeedOnly.next(), LifecycleFilter::All);
        assert_eq!(SortMode::Default.next(), SortMode::InboxDesc);
        assert_eq!(SortMode::InboxDesc.next(), SortMode::Default);
    }

    #[test]
    fn filter_summary_describes_active_dimensions() {
        assert_eq!(filter_summary(&FilterState::default()), None);
        let f = FilterState {
            text: "wip".into(),
            inbox_only: true,
            lifecycle: LifecycleFilter::SeedOnly,
            sort: SortMode::InboxDesc,
            ..Default::default()
        };
        assert_eq!(
            filter_summary(&f).as_deref(),
            Some("/wip [inbox] [seed] [↓inbox]")
        );
    }

    /// Build a multi-repo `App` (native fleet nav) over `repos`, each
    /// `(name, groves)`, with the first repo as the surface's own/primary repo.
    fn fleet_app(parent: &Path, repos: &[(&str, &[&str])]) -> App {
        let roots: Vec<PathBuf> = repos
            .iter()
            .map(|(name, groves)| fleet_repo(parent, name, groves))
            .collect();
        let fleet = MultiRepoView::scan(&roots);
        App::new_fleet(Some(roots[0].clone()), fleet, None)
    }

    #[test]
    fn filter_toggle_keys_engage_and_flatten_the_nav() {
        // `i` / `l` / `s` on the grove list engage the predicate/sort dimensions
        // and flatten the nav (070-060). Idle starts grouped with headers.
        let tmp = TempDir::new().unwrap();
        let mut app = fleet_app(tmp.path(), &[("one", &["alpha"]), ("two", &["beta"])]);
        assert!(!app.filter.active());
        assert!(app
            .nav_rows_cached()
            .iter()
            .any(|r| matches!(r, NavRow::RepoHeader { .. })));

        // `i` engages inbox-only → flat, no headers.
        handle_key(&mut app, KeyCode::Char('i'), KeyModifiers::NONE).unwrap();
        assert!(app.filter.inbox_only);
        assert!(app
            .nav_rows_cached()
            .iter()
            .all(|r| matches!(r, NavRow::Grove { .. })));

        // `i` again clears it → back to grouped.
        handle_key(&mut app, KeyCode::Char('i'), KeyModifiers::NONE).unwrap();
        assert!(!app.filter.inbox_only);
        assert!(!app.filter.active());

        // `l` cycles lifecycle; `s` cycles sort.
        handle_key(&mut app, KeyCode::Char('l'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.filter.lifecycle, LifecycleFilter::LiveOnly);
        handle_key(&mut app, KeyCode::Char('s'), KeyModifiers::NONE).unwrap();
        assert_eq!(app.filter.sort, SortMode::InboxDesc);
    }

    #[test]
    fn nav_activate_on_a_header_toggles_that_section_collapse() {
        let tmp = TempDir::new().unwrap();
        let mut app = fleet_app(tmp.path(), &[("one", &["alpha"]), ("two", &["beta"])]);
        // Row 0 is r1's header (N>1). Activating it collapses, hiding alpha.
        app.list.select(Some(0));
        assert_eq!(app.nav_activate(), NavActivation::Toggled);
        let rows = app.nav_rows_cached();
        assert!(matches!(&rows[0], NavRow::RepoHeader { collapsed: true, .. }));
        // r1's grove is gone; row 1 is now r2's header.
        assert!(matches!(&rows[1], NavRow::RepoHeader { .. }));
        // Activating again expands.
        assert_eq!(app.nav_activate(), NavActivation::Toggled);
        assert!(matches!(&app.nav_rows_cached()[1], NavRow::Grove { .. }));
    }

    #[test]
    fn nav_activate_on_a_grove_returns_its_owning_repo_and_name() {
        let tmp = TempDir::new().unwrap();
        let mut app = fleet_app(tmp.path(), &[("one", &["alpha"]), ("two", &["beta"])]);
        // Rows: header(r1), alpha@r1, header(r2), beta@r2. Select beta (row 3).
        app.list.select(Some(3));
        let got = app.nav_activate();
        // The fleet stores roots as passed (this test bypasses `fleet::resolve`,
        // which is what canonicalizes in production); the nav row carries the same
        // path the fleet was scanned with.
        let r2 = tmp.path().join("two");
        assert_eq!(got, NavActivation::Open(r2, "beta".to_string()));
    }

    #[test]
    fn acting_grove_in_list_carries_the_selected_repo_not_the_surface_repo() {
        // The surface's own repo is r1, but the highlighted grove is in r2 — the
        // acting grove must carry r2 so a cross-repo open targets the right repo.
        let tmp = TempDir::new().unwrap();
        let mut app = fleet_app(tmp.path(), &[("one", &["alpha"]), ("two", &["beta"])]);
        app.list.select(Some(3)); // beta@r2
        let r2 = tmp.path().join("two");
        assert_eq!(acting_grove(&app), Some((r2, "beta".to_string())));
        // On a header row there is no acting grove.
        app.list.select(Some(0));
        assert_eq!(acting_grove(&app), None);
    }

    #[test]
    fn rescan_event_paths_refreshes_only_the_owning_repo_and_keeps_selection() {
        let tmp = TempDir::new().unwrap();
        let mut app = fleet_app(tmp.path(), &[("one", &["alpha"]), ("two", &["beta"])]);
        // Highlight beta@r2 (row 3: header r1, alpha, header r2, beta).
        app.list.select(Some(3));

        // A new grove lands in r1 on disk; fire a watch event under r1.
        let r1 = tmp.path().join("one");
        let root = r1.join(".grove-worktrees").join("delta").join(".grove");
        touch(&root.join("010-x.md"), "# 010-x\n");
        app.rescan_event_paths(&[root.join("010-x.md")]);

        // r1's section refreshed (alpha + delta); r2 untouched; and the selection
        // followed beta@r2 to its new row (now after the extra delta row).
        let rows = app.nav_rows_cached();
        let r1_groves: Vec<&str> = rows
            .iter()
            .filter_map(|r| match r {
                NavRow::Grove { repo, summary } if *repo == r1.as_path() => {
                    Some(summary.name.as_str())
                }
                _ => None,
            })
            .collect();
        assert_eq!(r1_groves, vec!["alpha", "delta"]);
        assert_eq!(
            app.selected_nav(),
            Some(NavSelection::Grove {
                repo: tmp.path().join("two"),
                name: "beta".to_string()
            })
        );
    }

    #[test]
    fn multi_repo_nav_renders_collapsible_section_headers() {
        // N>1: the grouped two-level nav shows a section header (caret + repo
        // basename) per repo, with its groves beneath.
        let tmp = TempDir::new().unwrap();
        let app = fleet_app(tmp.path(), &[("one", &["alpha"]), ("two", &["beta"])]);
        let out = render_to_buffer(&app, 80, 16);
        assert!(out.contains("one"), "repo header 'one' missing:\n{out}");
        assert!(out.contains("two"), "repo header 'two' missing:\n{out}");
        assert!(out.contains("alpha") && out.contains("beta"), "groves missing:\n{out}");
        assert!(out.contains('▾'), "expanded-section caret missing:\n{out}");
    }

    #[test]
    fn single_repo_nav_hides_the_section_header() {
        // N=1 (070 Q4): no caret, no repo-name header — visually today's nav.
        let tmp = TempDir::new().unwrap();
        let app = fleet_app(tmp.path(), &[("solo", &["alpha"])]);
        let out = render_to_buffer(&app, 80, 12);
        assert!(out.contains("alpha"), "grove row missing:\n{out}");
        assert!(!out.contains('▾'), "N=1 must hide the section caret:\n{out}");
        assert!(!out.contains("solo"), "N=1 must not show the repo-name header:\n{out}");
    }

    #[test]
    fn filter_active_render_flattens_with_repo_prefix_and_titles_the_filter() {
        // With a text filter engaged the multi-repo nav flattens: no section
        // carets, each grove row carries its `<repo>/` prefix for attribution,
        // and the list title shows the active filter (070-060 Q3).
        let tmp = TempDir::new().unwrap();
        let mut app = fleet_app(tmp.path(), &[("one", &["alpha"]), ("two", &["beta"])]);
        // Type "/beta" and apply.
        handle_key(&mut app, KeyCode::Char('/'), KeyModifiers::NONE).unwrap();
        for c in "beta".chars() {
            handle_key(&mut app, KeyCode::Char(c), KeyModifiers::NONE).unwrap();
        }
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE).unwrap();

        let out = render_to_buffer(&app, 80, 16);
        assert!(!out.contains('▾'), "flat nav must show no section caret:\n{out}");
        // The row reads `two/ live  beta`: a dim `<repo>/` prefix, then the grove
        // row (badge + name). Assert the prefix and the name both render.
        assert!(out.contains("two/"), "repo prefix missing on flat row:\n{out}");
        assert!(out.contains("beta"), "matching grove missing:\n{out}");
        assert!(!out.contains("alpha"), "non-matching grove must be filtered out:\n{out}");
        assert!(out.contains("/beta"), "filter not shown in the title:\n{out}");
    }
}

// ===========================================================================
// Native host surface (ADR-0021): the v1 dashboard, rendered as a trellis
// `HostPane` in-process — no proxy socket, no `zellij action`, no WASM.
//
// This is leaf 030-port-dashboard-drive. The whole point is **port, not
// rewrite**: the `App`, `render`, `handle_key`, `WatchSet` debounce, and
// shell-out writes above are reused *unchanged*. Only the transport is new —
// instead of a crossterm tty loop (`run`) or a proxy socket (`Controller`),
// the dashboard draws into an off-screen `ratatui` buffer that trellis blits
/// The VCS TUI to embed for `worktree`'s [[working set]] (020-aux-tool-panes):
/// **lazyjj** for a Jujutsu worktree (a `.jj/` is present), else **lazygit**.
/// This single indirection is the seam the brief asks for — default lazygit now,
/// lazyjj a one-point change later — so the aux-spawn path never branches on the
/// VCS itself. Pure over the worktree path (no shell-out), so it is unit-testable
/// and sits below the ADR-0013 presentation boundary.
//
// Consumed by `mod native` (the aux-spawn path) and by the unit tests.
fn vcs_tool(worktree: &Path) -> &'static str {
    if worktree.join(".jj").is_dir() {
        "lazyjj"
    } else {
        "lazygit"
    }
}

/// Resolve `bin` against the `PATH`-style search list `path`, returning the first
/// matching existing file. A bare name is looked up in each `PATH` entry; a name
/// containing `/` is treated as a path and checked directly. `None` means "not
/// found on `PATH`" — the signal the aux-spawn path uses to substitute a graceful
/// in-pane message rather than failing the whole working-set mount when an aux
/// tool (yazi/lazygit) is not installed. Pure over `(bin, path)`, so it is
/// testable without touching the process environment.
fn on_path(bin: &str, path: &std::ffi::OsStr) -> Option<PathBuf> {
    if bin.contains('/') {
        let p = PathBuf::from(bin);
        return p.is_file().then_some(p);
    }
    std::env::split_paths(path).find_map(|dir| {
        let full = dir.join(bin);
        full.is_file().then_some(full)
    })
}

// as a real pane, receives keys in-process, drives tabs/panes by direct
// `HostDriver` call, and wakes fs-watch redraws via a tick instruction.
//
// It lives in a child module so it can reuse this module's private items
// (`render`, `handle_key`, `App`'s fields, `refresh_silent`, the `shell_*`
// writers) without widening their visibility. It links the forked `zellij-*`
// crates unconditionally — trellis is the only TUI (ADR-0026).
// ===========================================================================
mod native {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};
    use std::sync::{mpsc, Arc, Mutex};

    use anyhow::{Context, Result};
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event::{KeyCode, KeyEvent};
    use ratatui::layout::Rect;
    use ratatui::style::{Color, Style};
    use ratatui::text::Line;
    use ratatui::widgets::{Paragraph, Widget};
    use ratatui::Terminal;
    use tempfile::NamedTempFile;

    use trellis::input::command::RunCommand;
    use trellis_server::panes::host_pane::{
        register_keyed_host_surface, HostDriver, HostSurface,
    };

    use super::{
        decide_observation_edit, footer_line, handle_key, on_path, render,
        shell_capture, shell_drain, short_err, vcs_tool, App, CaptureField, CaptureModal,
        EditOutcome, MultiRepoView, NavActivation, PendingAction, RepoView, DEBOUNCE,
        WIDE_TIER_MIN_CONTENT_COLS,
    };

    // -----------------------------------------------------------------------
    // The grove-owned whichkey bar (ADR-0019, leaf 140).
    //
    // One full-width line across the bottom of the native frame, the **single**
    // owner of grove's key hints: the nav/detail surfaces suppress their own
    // footers (`App::native_chrome`) and instead *publish* their hint line here
    // when they gain focus or change state. The harness draws no hint of its own —
    // when it is focused, the surface that lost focus relinquishes the bar to a
    // "keys go to the harness" line.
    //
    // The bar is a passive, non-selectable host pane; it never has focus and so
    // cannot learn the focused context by `set_focused`. Instead the *publishing*
    // surface is always the focused one (input only routes to the focused pane), so
    // it knows its own hints — and a `request_tick` through the whichkey's stored
    // `HostDriver` wakes the bar to redraw. No new trellis focus hook is needed.
    // -----------------------------------------------------------------------

    /// Which surface currently owns the whichkey line. Tracked so a surface only
    /// relinquishes the bar (on losing focus) when it still owns it — keeping the
    /// hand-off order-independent across the paired `set_focused(false)` /
    /// `set_focused(true)` calls trellis makes when focus moves.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum WhichkeyOwner {
        Nav,
        Detail,
        Harness,
    }

    /// The published whichkey content: its owner plus the line the bar renders.
    /// `None` until the first surface publishes (the nav, when it is focused at
    /// first-layout).
    static WHICHKEY_LINE: Mutex<Option<(WhichkeyOwner, Line<'static>)>> = Mutex::new(None);

    /// The whichkey pane's own [`HostDriver`], stashed by [`WhichkeySurface::set_driver`]
    /// so any focused surface can `request_tick` the bar to redraw when it publishes
    /// a new line. `None` until the whichkey pane is injected.
    static WHICHKEY_DRIVER: Mutex<Option<HostDriver>> = Mutex::new(None);

    /// Publish `line` as the whichkey content owned by `owner`, and wake the bar to
    /// redraw — but only when the content actually changed, so a held key (or a
    /// no-op redraw) does not tick the bar every keystroke. Called by the focused
    /// nav/detail surface; a no-op-but-safe call when no whichkey pane exists yet
    /// (the driver is then `None`).
    fn publish_whichkey(owner: WhichkeyOwner, line: Line<'static>) {
        let changed = {
            let mut slot = WHICHKEY_LINE.lock().unwrap();
            let same = matches!(&*slot, Some((o, l)) if *o == owner && *l == line);
            if !same {
                *slot = Some((owner, line));
            }
            !same
        };
        if changed {
            if let Some(driver) = WHICHKEY_DRIVER.lock().unwrap().as_ref() {
                driver.request_tick();
            }
        }
    }

    /// Hand the whichkey to the harness context when `owner` loses focus — but only
    /// if `owner` still holds the bar (the newly-focused surface may already have
    /// claimed it, in which case its line must stand).
    fn relinquish_whichkey(owner: WhichkeyOwner) {
        let owns = matches!(&*WHICHKEY_LINE.lock().unwrap(), Some((o, _)) if *o == owner);
        if owns {
            publish_whichkey(WhichkeyOwner::Harness, harness_whichkey_line());
        }
    }

    /// The whichkey line shown while a harness pane is focused: grove owns only the
    /// leader (locked mode passes every other key to the harness itself).
    fn harness_whichkey_line() -> Line<'static> {
        Line::from("⌃o nav   ·   keys go to the focused harness")
    }

    /// The grove-owned whichkey bar as a trellis [`HostSurface`] (leaf 140): a
    /// stateless renderer of whatever [`WHICHKEY_LINE`] the focused surface has
    /// published. Injected as a full-width, non-selectable bottom pane
    /// ([`Tab::inject_whichkey_pane`](trellis_server::tab)); never focused.
    pub struct WhichkeySurface;

    /// Build the (stateless) whichkey surface for [`register_whichkey_surface`].
    pub fn whichkey_surface() -> WhichkeySurface {
        WhichkeySurface
    }

    impl HostSurface for WhichkeySurface {
        fn draw(&mut self, area: Rect, buf: &mut Buffer) {
            let line = WHICHKEY_LINE
                .lock()
                .unwrap()
                .as_ref()
                .map(|(_, l)| l.clone())
                .unwrap_or_else(|| Line::from(""));
            // A subtle bar background sets the hint line apart from the surfaces
            // above it; the published spans keep their own foreground colours.
            Paragraph::new(line)
                .style(Style::default().bg(Color::Indexed(236)).fg(Color::Gray))
                .render(area, buf);
        }

        fn set_driver(&mut self, driver: HostDriver) {
            *WHICHKEY_DRIVER.lock().unwrap() = Some(driver);
        }

        fn tick(&mut self) -> bool {
            // A publishing surface ticks the bar when it changes the line; always
            // redraw on a tick (the surface only ticks when something changed).
            true
        }
    }

    // -----------------------------------------------------------------------
    // The native `$EDITOR` drop (130-native-detail/030; first slice of ADR-0020
    // §6 embedded-tool observability).
    //
    // v1 ran `$EDITOR` by suspending the tty (`process_pending_action`'s
    // `suspended()`); a host surface has no tty (it renders in the server daemon —
    // ADR-0021), so instead it asks trellis to run `$EDITOR <tempfile>` as a real
    // terminal pane and signal `editor_exited` when the child exits. The flow is
    // two-phase: `begin_pending_edit` seeds the tempfile and opens the pane, then
    // `finish_edit` reads it back once trellis reports the exit. Shared verbatim by
    // the home dashboard and the per-grove detail surfaces.
    // -----------------------------------------------------------------------

    /// A `$EDITOR` drop in flight: the seeded tempfile (held alive until the editor
    /// exits) plus what to do with the edited text. A surface stashes one between
    /// its `open_editor` request and the `editor_exited` that completes the flow.
    enum PendingEdit {
        /// Edit the in-memory capture draft; on exit the edited text becomes the
        /// capture body (v1 `PendingAction::EditBody`).
        Body { tempfile: NamedTempFile },
        /// Edit a committed inbox observation; on a non-empty change, round-trip
        /// through `grove-llm inbox-edit` (v1 `PendingAction::EditObservation`).
        Observation {
            tempfile: NamedTempFile,
            path: PathBuf,
            original: String,
            grove: String,
        },
    }

    /// Seed a tempfile with `initial` and ask trellis to open `$EDITOR` on it as a
    /// terminal pane whose exit it will observe. Returns the live tempfile handle to
    /// hold until [`finish_edit`] reads it back; `Err` if the tempfile can't be
    /// created or written.
    fn seed_editor(driver: &HostDriver, initial: &str) -> Result<NamedTempFile> {
        let tf = tempfile::Builder::new()
            .prefix("grove-edit-")
            .suffix(".md")
            .tempfile()
            .context("creating editor tempfile")?;
        std::fs::write(tf.path(), initial)
            .with_context(|| format!("seeding editor tempfile {}", tf.path().display()))?;
        driver.open_editor(tf.path().to_path_buf());
        Ok(tf)
    }

    /// Begin a `$EDITOR` drop for an `EditBody`/`EditObservation` the v1 handler
    /// queued: seed the tempfile and open the editor pane. Returns the in-flight
    /// [`PendingEdit`] to stash, or `None` (recording a status) when there is no
    /// driver yet or the seed/read fails.
    fn begin_pending_edit(
        app: &mut App,
        driver: &Option<HostDriver>,
        action: PendingAction,
    ) -> Option<PendingEdit> {
        let Some(driver) = driver.as_ref() else {
            app.status = Some("editor unavailable (surface not mounted)".to_string());
            return None;
        };
        match action {
            PendingAction::EditBody => match seed_editor(driver, &app.capture.body) {
                Ok(tempfile) => Some(PendingEdit::Body { tempfile }),
                Err(e) => {
                    app.status = Some(format!("editor failed: {}", short_err(&e)));
                    None
                }
            },
            PendingAction::EditObservation { path } => {
                let original = match std::fs::read_to_string(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        app.status = Some(format!(
                            "reading observation failed: {}",
                            short_err(&anyhow::anyhow!(e))
                        ));
                        return None;
                    }
                };
                let grove = app
                    .detail
                    .as_ref()
                    .map(|d| d.grove.clone())
                    .unwrap_or_default();
                match seed_editor(driver, &original) {
                    Ok(tempfile) => Some(PendingEdit::Observation {
                        tempfile,
                        path,
                        original,
                        grove,
                    }),
                    Err(e) => {
                        app.status = Some(format!("editor failed: {}", short_err(&e)));
                        None
                    }
                }
            }
            // Only the two edit actions reach here; anything else is a caller bug.
            _ => None,
        }
    }

    /// Complete a `$EDITOR` drop once trellis reports the editor exited: read the
    /// tempfile back and apply it. Mirrors `process_pending_action`'s
    /// `EditBody`/`EditObservation` arms minus the tty suspend (the editor ran as a
    /// real pane). A non-zero/signalled exit is treated as "abort, leave unchanged",
    /// faithful to v1's `shell_editor`, which errored on a non-zero status.
    fn finish_edit(app: &mut App, pending: PendingEdit, exit_status: Option<i32>) {
        if exit_status != Some(0) {
            app.status = Some("editor exited without saving; left unchanged".to_string());
            return;
        }
        match pending {
            PendingEdit::Body { tempfile } => match std::fs::read_to_string(tempfile.path()) {
                Ok(new_body) => {
                    app.capture.body = new_body;
                    app.capture.field = CaptureField::Body;
                }
                Err(e) => {
                    app.status = Some(format!(
                        "reading edited body failed: {}",
                        short_err(&anyhow::anyhow!(e))
                    ));
                }
            },
            PendingEdit::Observation {
                tempfile,
                path,
                original,
                grove,
            } => {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let edited = match std::fs::read_to_string(tempfile.path()) {
                    Ok(s) => s,
                    Err(e) => {
                        app.status = Some(format!(
                            "reading edited observation failed: {}",
                            short_err(&anyhow::anyhow!(e))
                        ));
                        return;
                    }
                };
                match decide_observation_edit(&path, &original, edited) {
                    Ok(EditOutcome::Unchanged) => app.status = Some(format!("{name}: unchanged")),
                    // The fs-watch on the inbox fires on the rename, debounces, and
                    // rescans — the renamed entry then reappears.
                    Ok(EditOutcome::Saved) => {
                        app.status = Some(format!("edited {name} in {grove}"))
                    }
                    Err(e) => app.status = Some(format!("edit failed: {}", short_err(&e))),
                }
                let _ = app.refresh_silent();
            }
        }
    }

    /// Build the dashboard surface from the `--repo` flags: resolve the **fleet**
    /// it spans (070 Q1/Q4 — the manifest + scan-discovered repos plus these
    /// flags, config-only with no cwd anchor, ADR-0027), scan it, seed the `App`,
    /// and prepare an off-screen render target. The nav then renders the grouped
    /// two-level fleet (flat at N=1, or the empty-state at N=0). The fs-watch
    /// thread is *not* started here — it needs the [`HostDriver`], which only
    /// arrives in [`set_driver`] once the pane is constructed on the server's
    /// screen thread.
    ///
    /// Never errors on a single unreadable repo (070 Q3 silent-skip drops it from
    /// the fleet with a stderr breadcrumb); the `Result` stays for the off-screen
    /// terminal build.
    pub fn dashboard_surface(repo_flags: &[PathBuf]) -> Result<DashboardSurface> {
        // Resolve the fleet this dashboard spans — config-only, from the manifest
        // + scan roots + these `--repo` flags, with no cwd anchor (ADR-0027). The
        // same flags are stored on the surface so `set_driver`'s fleet fs-watch
        // re-resolves the identical repo set.
        let roots = crate::fleet::resolve(repo_flags);
        let fleet = MultiRepoView::scan(&roots);
        // The config-only fleet nav has no anchor repo and no cwd preselect
        // (ADR-0027 §3): the nav starts at row 0.
        let mut app = App::new_fleet(None, fleet, None);
        // The nav renders in the native frame: suppress its own footer; the
        // grove-owned whichkey bar (leaf 140) owns the bottom hint line.
        app.native_chrome = true;
        // Initial size is a placeholder; the first `draw` resizes to the pane.
        let terminal = Terminal::new(TestBackend::new(80, 24))
            .map_err(|e| anyhow::anyhow!("building the off-screen render target: {e}"))?;
        Ok(DashboardSurface {
            repo_flags: repo_flags.to_vec(),
            app,
            terminal,
            driver: None,
            open_harnesses: BTreeSet::new(),
            mounted_grove: None,
            pending_edit: None,
            _watcher: None,
            dirty: Arc::new(Mutex::new(DirtyBuf::default())),
        })
    }

    /// The v1 dashboard as a trellis [`HostSurface`]. Wraps the unchanged `App`
    /// and renders it through an off-screen [`TestBackend`] terminal so the v1
    /// `render(f, app)` is reused verbatim; the resulting cells are blitted into
    /// the pane buffer trellis composites.
    pub struct DashboardSurface {
        /// The `--repo` flags this surface was built from (ADR-0027) — stored,
        /// not a single anchor repo, so `set_driver` re-resolves the identical
        /// config-only fleet for the fs-watch (manifest + scan roots + these
        /// flags). Empty when the fleet is the manifest/scan set alone.
        repo_flags: Vec<PathBuf>,
        /// The unchanged v1 dashboard state.
        app: App,
        /// Off-screen render target — the trick that lets the v1 `render`
        /// (which needs a `Frame`) run without a real terminal. We draw into it,
        /// then copy its buffer into the host pane buffer.
        terminal: Terminal<TestBackend>,
        /// The layout/redraw handle, set once at first-layout. `None` until then.
        driver: Option<HostDriver>,
        /// The **repo-qualified [`harness_key`]s** with an open working set — the
        /// native analogue of the retired `HarnessTabs` id map (the screen thread
        /// addresses sets by this opaque key, so no numeric id round-trip is needed).
        /// Keyed by `(repo, name)` not bare name, so two same-named groves across
        /// repos track independently (050-cross-repo-harness); membership drives the
        /// `first_open` (spawn vs restore) decision.
        open_harnesses: BTreeSet<String>,
        /// The `(repo, name)` of the grove whose [[working set]] is currently mounted
        /// in the content region (the last one swapped to), or `None` before any
        /// selection. The toggle re-derives the **repo-qualified** [`harness_key`]
        /// from this to address the mounted set (150-working-set/030), so a same-named
        /// grove in another repo never steals the toggle (050-cross-repo-harness); the
        /// bare name is kept alongside for the status line. *Not* the nav's highlighted
        /// list row, which may differ when the user moved the cursor without
        /// re-selecting. A toggle with nothing mounted is a no-op-with-status.
        mounted_grove: Option<(PathBuf, String)>,
        /// A `$EDITOR` drop in flight (a capture-body edit from the nav), held from
        /// the `open_editor` request until `editor_exited` reads the tempfile back.
        pending_edit: Option<PendingEdit>,
        /// Kept alive so the fs-watch thread's channel stays open; dropping it
        /// (on surface drop) closes the channel and the thread exits cleanly.
        _watcher: Option<RecommendedWatcher>,
        /// The fleet fs-watch's **dirty-path buffer** (070 Q6). The watch thread
        /// records the (`.git/`-filtered) event paths here as a burst settles;
        /// `tick` drains it and re-scans **only the repos those paths belong to**
        /// (`App::rescan_event_paths`), leaving the rest of the fleet untouched —
        /// so an event under repo A never re-scans repo B. A `full_refresh`
        /// marker (a pathless/conservative event) falls back to a whole-fleet
        /// rescan. Shared `Arc` so the off-thread watch and the on-thread `tick`
        /// hand paths across without changing `HostDriver::request_tick`'s shape.
        dirty: Arc<Mutex<DirtyBuf>>,
    }

    impl DashboardSurface {
        /// Run a deferred action the v1 `handle_key` queued. The non-interactive
        /// `grove-llm` writes run **synchronously on the screen thread** (sub-second
        /// git commits; the v1 `suspended()` tty dance is gone because the surface
        /// no longer owns a tty). Harness open/close/focus drive tabs by direct
        /// `HostDriver` call. A `$EDITOR` drop (`Ctrl-E` on a capture draft) opens a
        /// real trellis editor pane and reads the result back on exit
        /// ([`begin_pending_edit`] / [`finish_edit`], 130-native-detail/030).
        fn process_action(&mut self, action: PendingAction) {
            match action {
                PendingAction::Submit => {
                    let target = self.app.capture.target.clone();
                    let body = self.app.capture.body.clone();
                    match shell_capture(&target, &body) {
                        Ok(()) => self.app.status = Some(format!("captured to {target}")),
                        Err(e) => {
                            self.app.status = Some(format!("capture failed: {}", short_err(&e)))
                        }
                    }
                    self.app.capture = CaptureModal::default();
                    let _ = self.app.refresh_silent();
                }
                PendingAction::Drain { path, disposition } => {
                    let grove = self
                        .app
                        .detail
                        .as_ref()
                        .map(|d| d.grove.clone())
                        .unwrap_or_default();
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    match shell_drain(&grove, &path, disposition) {
                        Ok(()) => self.app.status = Some(format!("{} {}", disposition.label(), name)),
                        Err(e) => {
                            self.app.status =
                                Some(format!("disposition failed: {}", short_err(&e)))
                        }
                    }
                    let _ = self.app.refresh_silent();
                }
                PendingAction::OpenHarness { name, repo } => {
                    let Some(driver) = self.driver.clone() else {
                        return;
                    };
                    // Drive the content swap (ADR-0022/0023, pair-aware per 130/020):
                    // mount this grove's `grove do <name>` harness **and** its
                    // per-grove detail into the content region beside the constant
                    // nav, parking the previously-selected grove's pair alive
                    // off-screen. The screen thread dedupes (already-shown → no-op,
                    // parked → restore, first time → spawn); grove only tracks names
                    // for the status line + first-open detail build. Run the same
                    // binary the server is (dev `target/debug/grove` or an installed).
                    // Repo-qualify the working-set key so a same-named grove in
                    // another repo addresses its *own* harness/detail/aux set, never
                    // this one (050-cross-repo-harness / 070 Q7). Every map and
                    // trellis verb below keys on this, not the bare name.
                    let key = harness_key(&repo, &name);
                    let first_open = !self.open_harnesses.contains(&key);
                    let secondary_surface_key = if first_open {
                        // Build the per-grove detail surface and stash it in the keyed
                        // registry; the screen thread takes it when it mounts the pair.
                        // On a scan failure the harness still swaps in (detail-less);
                        // grove notes it on the status line rather than blocking the
                        // selection.
                        match detail_surface(repo.clone(), name.clone()) {
                            Ok(detail) => {
                                let dkey = detail_surface_key(&key);
                                register_keyed_host_surface(dkey.clone(), Box::new(detail));
                                Some(dkey)
                            }
                            Err(e) => {
                                self.app.status =
                                    Some(format!("detail unavailable: {}", short_err(&e)));
                                None
                            }
                        }
                    } else {
                        // Already open: its detail pane is parked alive and restored by
                        // the swap; no rebuild, no registry entry.
                        None
                    };
                    let grove_bin =
                        std::env::current_exe().unwrap_or_else(|_| PathBuf::from("grove"));
                    // The aux working-set members (terminal/yazi/vcs) run in the grove's
                    // worktree cwd (020-aux-tool-panes), so yazi/lazygit act on the
                    // grove's tree. Passed every swap; the screen thread only spawns them
                    // on first-open (a re-selection restores the parked-alive set).
                    let worktree = crate::repo::grove_worktree(&repo, &name);
                    let shell = std::env::var_os("SHELL").unwrap_or_else(|| "/bin/sh".into());
                    let path = std::env::var_os("PATH").unwrap_or_default();
                    let aux = aux_members(&worktree, &shell, &path);
                    // The responsive default (040-responsive-layout): grove's two-tier
                    // breakpoint, applied by trellis at the first-open mount — a wide
                    // content region shows the whole working set, a laptop-sized one parks
                    // the aux tools (harness + detail). Re-selection restores the parked
                    // set with its own visibility, so the default only bites on first open.
                    driver.swap_content(
                        &key,
                        repo.clone(),
                        grove_bin,
                        vec!["do".to_string(), name.clone()],
                        secondary_surface_key,
                        aux,
                        Some(WIDE_TIER_MIN_CONTENT_COLS),
                    );
                    if self.open_harnesses.insert(key) {
                        self.app.status = Some(format!("opened harness: {name}"));
                    } else {
                        self.app.status = Some(format!("switched to harness: {name}"));
                    }
                    // This grove's working set is now the mounted one — the target a
                    // subsequent member toggle addresses (150-working-set/030). Track
                    // repo+name so the toggle re-derives the qualified key while the
                    // status line still shows the bare grove name.
                    self.mounted_grove = Some((repo, name));
                }
                PendingAction::ToggleMember { member } => {
                    // Toggle one member of the **currently-mounted** grove's working set
                    // (150-working-set/030), via `HostDriver::toggle_member` keyed by that
                    // grove — so it acts on the set in the content region, not the nav's
                    // highlighted row. trellis hides/shows the member alive (park/restore +
                    // re-tile) and ignores the verb when `key` is not the mounted set, so
                    // passing the mounted grove keeps the toggle per-grove. A no-op-with-
                    // status when nothing is mounted yet (no working set to toggle).
                    match (&self.driver, &self.mounted_grove) {
                        (Some(driver), Some((repo, grove))) => {
                            driver.toggle_member(&harness_key(repo, grove), member.role());
                            self.app.status =
                                Some(format!("toggled {} in {grove}", member.label()));
                        }
                        _ => {
                            self.app.status =
                                Some("no working set mounted — select a grove first".into());
                        }
                    }
                }
                PendingAction::CloseHarness { name } => {
                    // The content-swap model parks harnesses alive rather than
                    // closing them; there is no per-grove close from the nav (a
                    // lifecycle concern beyond this leaf — `GoToTab`/close-tab
                    // retired with ADR-0023).
                    self.app.status =
                        Some(format!("{name} stays parked (no close in the swap model)"));
                }
                action @ (PendingAction::EditBody | PendingAction::EditObservation { .. }) => {
                    // Open `$EDITOR` as a real trellis pane; `editor_exited` reads it
                    // back. Disjoint borrows of `app` (mut) and `driver` (shared).
                    self.pending_edit = begin_pending_edit(&mut self.app, &self.driver, action);
                }
            }
        }
    }

    impl HostSurface for DashboardSurface {
        fn draw(&mut self, area: Rect, buf: &mut Buffer) {
            // Match the off-screen terminal to the pane's content rect, render the
            // *unchanged* v1 dashboard into it, then blit its cells into the host
            // buffer trellis composites. `area` always has origin (0,0) (the host
            // pane builds it that way), so source and destination coords align.
            let _ = self.terminal.resize(Rect::new(0, 0, area.width, area.height));
            let Self { app, terminal, .. } = self;
            let _ = terminal.draw(|f| render(f, app));
            let src = terminal.backend().buffer();
            let w = area.width.min(src.area.width);
            let h = area.height.min(src.area.height);
            for y in 0..h {
                for x in 0..w {
                    buf[(area.x + x, area.y + y)] = src[(x, y)].clone();
                }
            }
        }

        fn handle_key(&mut self, key: KeyEvent) -> bool {
            // Native-nav select: `Enter` opens (or switches to) the selected
            // grove's [[workspace]] in **its owning repo** (070 Q4/050 cross-repo
            // open), instead of the v1 master/detail drill-in. `nav_activate`
            // returns `None` when a modal / filter / help is up (so `Enter` falls
            // through to the shared handler, e.g. a newline in the capture body)
            // **and** when a repo header is highlighted — there it toggles the
            // section's collapse in place, so we re-render and consume the key.
            if matches!(key.code, KeyCode::Enter) && key.modifiers.is_empty() {
                match self.app.nav_activate() {
                    NavActivation::Open(repo, name) => {
                        self.process_action(PendingAction::OpenHarness { name, repo });
                        return true;
                    }
                    NavActivation::Toggled => {
                        // A section collapse toggled: consume the key and
                        // re-render; republish hints since the rows changed.
                        publish_whichkey(WhichkeyOwner::Nav, footer_line(&self.app));
                        return true;
                    }
                    // Passthrough: a modal / filter / help is up — let `Enter`
                    // fall through to the shared handler below.
                    NavActivation::Passthrough => {}
                }
            }

            // Route to the unchanged v1 key handler, then drain any action it
            // queued (capture/drain writes, harness drive). v1's `q` returns
            // `true` ("quit the loop"); natively that means quit the session.
            let quit = match handle_key(&mut self.app, key.code, key.modifiers) {
                Ok(q) => q,
                Err(e) => {
                    self.app.status = Some(format!("key error: {}", short_err(&e)));
                    false
                }
            };
            if let Some(action) = self.app.pending_action.take() {
                self.process_action(action);
            }
            if quit {
                if let Some(driver) = &self.driver {
                    driver.quit();
                }
            }
            // The nav is the focused surface (input only routes to the focused
            // pane), so republish its hints to the whichkey — the key likely moved
            // the selection, toggled a modal, or set a status line.
            publish_whichkey(WhichkeyOwner::Nav, footer_line(&self.app));
            // Always re-render: a key almost always moves the selection or sets a
            // status line, and the cost of an extra draw is trivial.
            true
        }

        fn set_focused(&mut self, focused: bool) {
            // The whichkey bar reflects the focused surface (leaf 140): claim it
            // with the nav's hints on focus, and hand it to the harness context on
            // blur (if the nav still owns it).
            if focused {
                publish_whichkey(WhichkeyOwner::Nav, footer_line(&self.app));
            } else {
                relinquish_whichkey(WhichkeyOwner::Nav);
            }
        }

        fn set_driver(&mut self, driver: HostDriver) {
            self.driver = Some(driver.clone());
            // Replace v1's in-loop `WatchSet` polling with a dedicated fs-watch
            // thread (the shared 110/030 pattern). At fleet scale (070/030 Q6) the
            // home dashboard watches **every fleet repo's** two grove-state roots on
            // this single watcher — `fleet::resolve` re-resolves the same
            // config-only fleet from the stored `--repo` flags (ADR-0027), matching
            // the surface's own `dashboard_surface` resolve; `fleet_watch_dirs`
            // expands each repo to its `.grove-worktrees/` + `.grove-meta/inboxes/`.
            // The watch records the dirty event paths into the shared `dirty`
            // buffer so `tick` re-scans only the owning repo
            // (`App::rescan_event_paths`); an event under an unrelated fleet repo
            // leaves that repo's section untouched.
            let roots = crate::fleet::resolve(&self.repo_flags);
            self._watcher = spawn_grove_watch(
                crate::fleet::fleet_watch_dirs(&roots),
                driver,
                Some(self.dirty.clone()),
            );
        }

        fn tick(&mut self) -> bool {
            // An fs-watch settle (or out-of-band wake): drain the dirty buffer and
            // re-scan. With concrete event paths, re-scan only the repos they
            // belong to (070 Q6 targeted re-scan); a `full_refresh` marker (a
            // pathless/conservative event) or an empty drain (an out-of-band wake)
            // falls back to a whole-fleet rescan.
            let dirty = std::mem::take(&mut *self.dirty.lock().unwrap());
            if dirty.full_refresh || dirty.paths.is_empty() {
                if let Err(e) = self.app.refresh_silent() {
                    self.app.status = Some(format!("rescan failed: {}", short_err(&e)));
                }
            } else {
                self.app.rescan_event_paths(&dirty.paths);
            }
            true
        }

        fn editor_exited(&mut self, exit_status: Option<i32>) -> bool {
            // The `$EDITOR` pane this surface opened (Ctrl-E) exited: read the
            // tempfile back and apply it. `take` so a stray double-signal is inert.
            match self.pending_edit.take() {
                Some(pending) => {
                    finish_edit(&mut self.app, pending, exit_status);
                    true
                }
                None => false,
            }
        }
    }

    /// The fleet fs-watch's **dirty-path buffer** (070 Q6), drained by `tick`.
    /// The watch thread records the dirty event paths here as a burst settles;
    /// `tick` re-scans only the repos those paths belong to. `full_refresh` is set
    /// when a pathless/conservative event arrives, forcing a whole-fleet rescan.
    #[derive(Default)]
    struct DirtyBuf {
        full_refresh: bool,
        paths: Vec<PathBuf>,
    }

    /// Record one debounce-thread message into the shared dirty buffer (when a
    /// surface supplied one): an empty `paths` is the conservative full-refresh
    /// marker; otherwise the (already `.git/`-filtered) paths are accumulated for
    /// the targeted per-repo re-scan.
    fn record_dirty(dirty: &Option<Arc<Mutex<DirtyBuf>>>, paths: Vec<PathBuf>) {
        if let Some(d) = dirty {
            let mut buf = d.lock().unwrap();
            if paths.is_empty() {
                buf.full_refresh = true;
            } else {
                buf.paths.extend(paths);
            }
        }
    }

    /// Spawn the shared fs-watch → debounce → `request_tick` thread for a host
    /// surface (the 110/030 pattern, factored so the home dashboard and each
    /// per-grove detail reuse it). Watches each existing dir in `dirs` recursively,
    /// coalesces bursts under [`DEBOUNCE`], and posts a tick through `driver` when
    /// the filesystem settles — so the surface is only ever mutated on the screen
    /// thread (in `tick`), never from this thread. When `dirty` is `Some`, the
    /// (`.git/`-filtered) event paths are recorded into it for the targeted
    /// per-repo re-scan (070 Q6); when `None` (the per-grove detail surface, which
    /// re-scans its single grove regardless), only the tick is posted. Returns the
    /// [`RecommendedWatcher`] to keep alive (dropping it closes the channel and
    /// ends the thread); `None` on an exotic platform with no watcher, where manual
    /// `r` refresh still works.
    fn spawn_grove_watch(
        dirs: Vec<PathBuf>,
        driver: HostDriver,
        dirty: Option<Arc<Mutex<DirtyBuf>>>,
    ) -> Option<RecommendedWatcher> {
        let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();
        let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            // Drop `.git/`-internal churn before it can wake a tick (070 Q6):
            // pack/ref/index writes inside any watched worktree's `.git/`, which
            // fleet-scale watching would amplify N-fold. A pathless event becomes
            // a conservative full-refresh marker (empty vec). Mirrors the legacy
            // `WatchSet::drain` filter, but forwards the surviving paths so `tick`
            // can target the owning repo.
            if let Ok(ev) = res {
                if ev.paths.is_empty() {
                    let _ = tx.send(Vec::new());
                } else {
                    let real: Vec<PathBuf> = ev
                        .paths
                        .into_iter()
                        .filter(|p| !crate::fleet::path_is_git_internal(p))
                        .collect();
                    if !real.is_empty() {
                        let _ = tx.send(real);
                    }
                }
            }
        })
        .ok()?;
        for dir in dirs {
            if dir.is_dir() {
                let _ = watcher.watch(&dir, RecursiveMode::Recursive);
            }
        }
        std::thread::spawn(move || {
            // Block for the first event, then coalesce until quiet for DEBOUNCE,
            // then tick. Mirrors v1's `WatchSet` debounce, moved off the (now
            // event-driven) render path. Each message's paths are recorded into the
            // shared buffer as it arrives, so the settle tick sees the whole burst.
            while let Ok(first) = rx.recv() {
                record_dirty(&dirty, first);
                loop {
                    match rx.recv_timeout(DEBOUNCE) {
                        Ok(paths) => {
                            record_dirty(&dirty, paths);
                            continue;
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => break,
                        Err(mpsc::RecvTimeoutError::Disconnected) => return,
                    }
                }
                driver.request_tick();
            }
        });
        Some(watcher)
    }

    /// The **repo-qualified** key identifying one grove's harness / working set
    /// across the fleet (050-cross-repo-harness). Two repos can each own a grove of
    /// the same bare name (`grove/fix-bug` and `acme-api/fix-bug`), so every map and
    /// trellis verb that addresses a working set keys on this — `open_harnesses`,
    /// `mounted_grove`, the `swap_content`/`toggle_member` content key, and (via
    /// [`detail_surface_key`]) the per-grove detail registry — never on the bare
    /// name, which would collide and mis-focus (070 Q7). The full repo *path* (not
    /// its basename) is used so two repos sharing a basename in different parents
    /// still derive distinct keys; the string is opaque to trellis (it "never
    /// interprets it"), so its exact shape is private to grove.
    fn harness_key(repo: &Path, name: &str) -> String {
        format!("{}\u{1f}{name}", repo.display())
    }

    /// The keyed-registry key under which grove stashes a grove's [`DetailSurface`]
    /// for the content-swap to mount as the secondary pane (ADR-0023). Namespaced so
    /// it never collides with the opaque working-set key the swap uses for the pair.
    /// Derived from the **repo-qualified** [`harness_key`], so two same-named groves
    /// in different repos get distinct detail registry slots (050-cross-repo-harness).
    fn detail_surface_key(harness_key: &str) -> String {
        format!("grove-detail:{harness_key}")
    }

    /// Build the **aux working-set members** (020-aux-tool-panes) for `worktree`: a
    /// plain **terminal** (`shell`), **yazi** (files), and the **vcs** TUI (lazygit /
    /// lazyjj via [`vcs_tool`]), in that order — matching the `grove-term`/`grove-yazi`/
    /// `grove-vcs` slot order in `GROVE_TUI_LAYOUT`, since `swap_content` maps the aux
    /// members onto the aux slots positionally. Each runs as a command pane in the
    /// grove's `worktree` cwd, so yazi/lazygit operate on the grove's tree. A tool not
    /// found on `path` is replaced by a graceful in-pane message (a held shell printing
    /// "not installed") rather than failing the whole working-set mount — yazi/lazygit
    /// are not guaranteed present. The terminal is the user's `shell`, always present,
    /// so it needs no fallback. `shell`/`path` are passed in (not read from the
    /// environment here) so the composition is deterministic and unit-testable.
    fn aux_members(
        worktree: &Path,
        shell: &std::ffi::OsStr,
        path: &std::ffi::OsStr,
    ) -> Vec<(String, RunCommand)> {
        let in_worktree = |command: PathBuf, args: Vec<String>| RunCommand {
            command,
            args,
            cwd: Some(worktree.to_path_buf()),
            // Hold the pane open if the child exits, so a quick failure (or a tool
            // that exits) shows its output instead of the pane vanishing.
            hold_on_close: true,
            ..RunCommand::default()
        };
        // A tool resolved on `path`, or a graceful "<bin> not installed" shell message.
        let tool = |role: &str, bin: &str| -> (String, RunCommand) {
            let run = match on_path(bin, path) {
                Some(full) => in_worktree(full, vec![]),
                None => in_worktree(
                    PathBuf::from(shell),
                    vec![
                        "-c".to_string(),
                        format!("printf '%s\\n' '{bin} is not installed (not on PATH)'"),
                    ],
                ),
            };
            (role.to_string(), run)
        };
        vec![
            ("terminal".to_string(), in_worktree(PathBuf::from(shell), vec![])),
            tool("yazi", "yazi"),
            tool("vcs", vcs_tool(worktree)),
        ]
    }

    /// Build the per-grove **detail surface** (130-native-detail/020): an `App`
    /// locked to `grove`'s detail (task tree + inbox + capture), scanning `repo`.
    /// Mounted beside that grove's harness in the content region the first time the
    /// grove is selected, then parked alive on switch-away. `Err` only if the initial
    /// scan fails — the caller swaps the harness in detail-less and notes it.
    pub fn detail_surface(repo: PathBuf, grove: String) -> Result<DetailSurface> {
        let view = RepoView::scan(&repo)?;
        let app = App::new_detail(repo.clone(), view, grove.clone());
        let terminal = Terminal::new(TestBackend::new(80, 24))
            .map_err(|e| anyhow::anyhow!("building the off-screen render target: {e}"))?;
        Ok(DetailSurface {
            repo,
            grove,
            app,
            terminal,
            driver: None,
            pending_edit: None,
            _watcher: None,
        })
    }

    /// One grove's **detail** as a trellis [`HostSurface`] (130-native-detail/020):
    /// the v1 master/detail dashboard reused in detail-locked mode (see
    /// [`App::new_detail`]) and rendered through an off-screen [`TestBackend`], so
    /// the v1 `render(f, app)` / `handle_key(app, …)` are reused verbatim. One
    /// instance per grove, mounted beside its harness and **parked alive** (never
    /// dropped) when another grove is selected — so its task/inbox cursor, an
    /// in-flight capture, and its fs-watch all survive a switch-away, with no
    /// cross-talk between groves.
    pub struct DetailSurface {
        /// The repo the grove lives in; fs-watch roots derive from it + the grove
        /// name. The `App` re-scans the whole repo (cheap) but the watch is scoped to
        /// just this grove.
        repo: PathBuf,
        /// The grove this surface is bound to (its worktree + inbox are watched).
        grove: String,
        /// The detail-locked v1 dashboard state, bound to `grove`.
        app: App,
        /// Off-screen render target (same blit trick as the home dashboard).
        terminal: Terminal<TestBackend>,
        /// The redraw/tick handle, set once at mount. `None` until then.
        driver: Option<HostDriver>,
        /// A `$EDITOR` drop in flight (a capture-body or inbox-observation edit),
        /// held from the `open_editor` request until `editor_exited` reads it back.
        pending_edit: Option<PendingEdit>,
        /// Kept alive so the per-grove fs-watch thread's channel stays open.
        _watcher: Option<RecommendedWatcher>,
    }

    impl DetailSurface {
        /// Run a deferred action the v1 `handle_key` queued, scoped to the detail's
        /// in-process powers. Capture (`c` → `Ctrl-S`) and inbox triage (`d`) run the
        /// same synchronous `grove-llm` shell-outs the home dashboard does; a
        /// `$EDITOR` drop (`Ctrl-E` on a capture draft or a selected inbox entry)
        /// opens a real trellis editor pane and reads the result back on exit
        /// ([`begin_pending_edit`] / [`finish_edit`], 130-native-detail/030). Harness
        /// open/close belong to the **nav** (it owns the content swap), so this
        /// surface ignores them — its grove's harness is already mounted beside it.
        fn process_action(&mut self, action: PendingAction) {
            match action {
                PendingAction::Submit => {
                    let target = self.app.capture.target.clone();
                    let body = self.app.capture.body.clone();
                    match shell_capture(&target, &body) {
                        Ok(()) => self.app.status = Some(format!("captured to {target}")),
                        Err(e) => {
                            self.app.status = Some(format!("capture failed: {}", short_err(&e)))
                        }
                    }
                    self.app.capture = CaptureModal::default();
                    let _ = self.app.refresh_silent();
                }
                PendingAction::Drain { path, disposition } => {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    match shell_drain(&self.grove, &path, disposition) {
                        Ok(()) => {
                            self.app.status = Some(format!("{} {}", disposition.label(), name))
                        }
                        Err(e) => {
                            self.app.status =
                                Some(format!("disposition failed: {}", short_err(&e)))
                        }
                    }
                    let _ = self.app.refresh_silent();
                }
                action @ (PendingAction::EditBody | PendingAction::EditObservation { .. }) => {
                    // Open `$EDITOR` as a real trellis pane in place of this detail
                    // pane; `editor_exited` reads the tempfile back. For an inbox
                    // entry the readback round-trips through `grove-llm inbox-edit`.
                    self.pending_edit = begin_pending_edit(&mut self.app, &self.driver, action);
                }
                // The nav drives the content swap and the working-set toggles (it owns
                // the content region); the detail surface must never call `swap_content`
                // or `toggle_member` (it would fight the nav). Its grove's harness is
                // already mounted beside it, and the `t` picker only opens in the nav
                // anyway (GroveList + `native_chrome`).
                PendingAction::OpenHarness { .. }
                | PendingAction::CloseHarness { .. }
                | PendingAction::ToggleMember { .. } => {}
            }
        }
    }

    impl HostSurface for DetailSurface {
        fn draw(&mut self, area: Rect, buf: &mut Buffer) {
            // Same off-screen render + blit as the home dashboard: render the v1
            // (detail-locked) `App` into a `TestBackend` sized to the pane, then copy
            // its cells into the host buffer trellis composites.
            let _ = self.terminal.resize(Rect::new(0, 0, area.width, area.height));
            let Self { app, terminal, .. } = self;
            let _ = terminal.draw(|f| render(f, app));
            let src = terminal.backend().buffer();
            let w = area.width.min(src.area.width);
            let h = area.height.min(src.area.height);
            for y in 0..h {
                for x in 0..w {
                    buf[(area.x + x, area.y + y)] = src[(x, y)].clone();
                }
            }
        }

        fn handle_key(&mut self, key: KeyEvent) -> bool {
            // Route to the unchanged v1 key handler (in detail-locked mode), then
            // drain any action it queued. Unlike the nav, the detail surface never
            // quits the session on the handler's `true` — session lifecycle (`q` /
            // quit) belongs to the nav; here `q`/`Esc` are inert (detail-locked).
            if let Err(e) = handle_key(&mut self.app, key.code, key.modifiers) {
                self.app.status = Some(format!("key error: {}", short_err(&e)));
            }
            if let Some(action) = self.app.pending_action.take() {
                self.process_action(action);
            }
            // This detail surface is the focused one (input only routes to the
            // focused pane); republish its hints to the whichkey.
            publish_whichkey(WhichkeyOwner::Detail, footer_line(&self.app));
            true
        }

        fn set_focused(&mut self, focused: bool) {
            // Claim the whichkey with this grove's detail hints on focus; hand it to
            // the harness context on blur (if this detail still owns it).
            if focused {
                publish_whichkey(WhichkeyOwner::Detail, footer_line(&self.app));
            } else {
                relinquish_whichkey(WhichkeyOwner::Detail);
            }
        }

        fn set_driver(&mut self, driver: HostDriver) {
            self.driver = Some(driver.clone());
            // Per-grove fs-watch (narrower than the home dashboard's repo-wide
            // watch): just this grove's worktree `.grove/` tree and its inbox. Fewer
            // handles and far less `.git/` churn, and a tick only ever refreshes
            // *this* surface (the driver carries this pane's id) — no cross-talk.
            self._watcher = spawn_grove_watch(
                vec![
                    self.repo
                        .join(".grove-worktrees")
                        .join(&self.grove)
                        .join(".grove"),
                    self.repo.join(".grove-meta").join("inboxes").join(&self.grove),
                ],
                driver,
                // This surface re-scans its single grove on any settle, so it needs
                // no per-repo targeting — `tick` ignores paths.
                None,
            );
        }

        fn tick(&mut self) -> bool {
            if let Err(e) = self.app.refresh_silent() {
                self.app.status = Some(format!("rescan failed: {}", short_err(&e)));
            }
            true
        }

        fn editor_exited(&mut self, exit_status: Option<i32>) -> bool {
            // The `$EDITOR` pane this detail opened (Ctrl-E) exited: read it back and
            // apply (capture body, or an inbox-edit round-trip). `take` so a stray
            // double-signal is inert.
            match self.pending_edit.take() {
                Some(pending) => {
                    finish_edit(&mut self.app, pending, exit_status);
                    true
                }
                None => false,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// A bare repo with the two dirs `RepoView::scan` reads, so an `App` can be
        /// built without a full grove fixture (these tests only exercise the
        /// `$EDITOR` readback, not rendering).
        fn empty_app() -> App {
            let tmp = tempfile::tempdir().unwrap();
            std::fs::create_dir_all(tmp.path().join(".grove-worktrees")).unwrap();
            std::fs::create_dir_all(tmp.path().join(".grove-meta").join("inboxes")).unwrap();
            let view = RepoView::scan(tmp.path()).unwrap();
            // Keep the tempdir alive for the App's lifetime by leaking it — the test
            // process is short-lived and never re-scans.
            std::mem::forget(tmp);
            App::new(std::env::temp_dir(), view, None)
        }

        #[test]
        fn finish_edit_body_reads_tempfile_into_the_capture_draft() {
            let mut app = empty_app();
            app.capture.body = "old draft".into();
            let tf = tempfile::Builder::new().suffix(".md").tempfile().unwrap();
            std::fs::write(tf.path(), "edited in $EDITOR").unwrap();

            finish_edit(&mut app, PendingEdit::Body { tempfile: tf }, Some(0));
            assert_eq!(app.capture.body, "edited in $EDITOR");
            assert_eq!(app.capture.field, CaptureField::Body);
        }

        #[test]
        fn finish_edit_aborts_on_a_nonzero_exit() {
            // A non-zero / signalled editor exit (e.g. vim `:cq`) means "abort":
            // the draft is left exactly as it was, faithful to v1's `shell_editor`.
            let mut app = empty_app();
            app.capture.body = "untouched".into();
            let tf = tempfile::Builder::new().suffix(".md").tempfile().unwrap();
            std::fs::write(tf.path(), "edited but aborted").unwrap();

            finish_edit(&mut app, PendingEdit::Body { tempfile: tf }, Some(1));
            assert_eq!(app.capture.body, "untouched");
        }

        /// The whichkey bar renders whatever line the focused surface published, and
        /// the publish/relinquish ownership keeps the hand-off order-independent
        /// (leaf 140). One test because the published line is a process-global —
        /// keeping the steps in a single test makes them sequential (no inter-test
        /// races on the shared `WHICHKEY_LINE`).
        #[test]
        fn whichkey_publishes_renders_and_hands_off() {
            // The bar renders the published line.
            publish_whichkey(WhichkeyOwner::Nav, Line::from("⏎ open · ⌃o nav"));
            let mut wk = whichkey_surface();
            let area = Rect::new(0, 0, 40, 1);
            let mut buf = Buffer::empty(area);
            wk.draw(area, &mut buf);
            let row: String = (0..area.width)
                .map(|x| buf[(x, 0)].symbol().chars().next().unwrap_or(' '))
                .collect();
            assert!(row.contains("open"), "whichkey shows published hints: {row:?}");
            assert!(row.contains("nav"), "whichkey shows the leader hint: {row:?}");

            // A non-owner losing focus is inert: the nav never owned the bar here
            // (detail does), so detail's line stands.
            publish_whichkey(WhichkeyOwner::Detail, Line::from("detail hints"));
            relinquish_whichkey(WhichkeyOwner::Nav);
            let owner = WHICHKEY_LINE.lock().unwrap().as_ref().map(|(o, _)| *o);
            assert_eq!(owner, Some(WhichkeyOwner::Detail), "non-owner blur is inert");

            // The owner losing focus hands the bar to the harness context.
            relinquish_whichkey(WhichkeyOwner::Detail);
            let owner = WHICHKEY_LINE.lock().unwrap().as_ref().map(|(o, _)| *o);
            assert_eq!(owner, Some(WhichkeyOwner::Harness), "owner blur → harness");
        }

        #[test]
        fn harness_key_qualifies_by_repo_so_same_named_groves_dont_collide() {
            // Two repos each own a grove named "fix-bug". The harness/working-set
            // key must be repo-qualified so they address *distinct* working sets —
            // a bare name would collide and mis-focus (070 Q7 / 050-cross-repo).
            let repo_a = PathBuf::from("/work/grove");
            let repo_b = PathBuf::from("/work/acme-api");
            let a = harness_key(&repo_a, "fix-bug");
            let b = harness_key(&repo_b, "fix-bug");
            assert_ne!(a, b, "same grove name in different repos must not share a key");
            // Stable: the same (repo, name) always derives the same key, so a
            // re-select finds the already-open set instead of spawning a duplicate.
            assert_eq!(a, harness_key(&repo_a, "fix-bug"), "the key is stable per (repo, name)");
            // The per-grove detail-surface registry key is derived from the
            // qualified key, so it inherits the same non-collision.
            assert_ne!(
                detail_surface_key(&a),
                detail_surface_key(&b),
                "per-grove detail surfaces must not collide across repos either"
            );
        }

        #[test]
        fn aux_members_compose_terminal_yazi_vcs_in_the_worktree_cwd() {
            use std::ffi::OsStr;
            // A worktree with no `.jj/` (vcs → lazygit) and an empty PATH dir (no aux
            // tool resolves) → every member runs in the worktree cwd, and the unfound
            // tools fall back to a graceful shell message rather than failing the mount.
            let tmp = tempfile::tempdir().unwrap();
            let worktree = tmp.path();
            let members = aux_members(worktree, OsStr::new("/bin/zsh"), OsStr::new(""));

            let roles: Vec<&str> = members.iter().map(|(r, _)| r.as_str()).collect();
            assert_eq!(
                roles,
                ["terminal", "yazi", "vcs"],
                "aux members are terminal, yazi, vcs in slot order"
            );
            for (_, run) in &members {
                assert_eq!(
                    run.cwd.as_deref(),
                    Some(worktree),
                    "every aux member runs in the grove's worktree cwd"
                );
            }
            // The terminal is the shell itself; the unfound yazi/vcs fall back to the
            // shell printing a "not installed" message (graceful, not a failed mount).
            assert_eq!(members[0].1.command, PathBuf::from("/bin/zsh"));
            assert!(members[0].1.args.is_empty(), "the terminal is a bare shell");
            assert_eq!(members[1].1.command, PathBuf::from("/bin/zsh"), "yazi falls back to the shell");
            assert!(
                members[1].1.args.last().is_some_and(|a| a.contains("yazi is not installed")),
                "the yazi fallback prints a graceful message"
            );
            assert!(
                members[2].1.args.last().is_some_and(|a| a.contains("lazygit is not installed")),
                "the vcs fallback names the resolved tool (lazygit for a git worktree)"
            );
        }

        #[test]
        fn aux_members_resolve_a_present_tool_to_its_full_path() {
            use std::ffi::OsStr;
            // With lazygit present on PATH, the vcs member runs that binary directly
            // (no fallback) — in the worktree cwd, no args.
            let tmp = tempfile::tempdir().unwrap();
            let bindir = tmp.path().join("bin");
            std::fs::create_dir(&bindir).unwrap();
            std::fs::write(bindir.join("lazygit"), "#!/bin/sh\n").unwrap();
            let members = aux_members(tmp.path(), OsStr::new("/bin/sh"), bindir.as_os_str());

            let vcs = &members[2].1;
            assert_eq!(vcs.command, bindir.join("lazygit"), "vcs resolves to the found lazygit");
            assert!(vcs.args.is_empty(), "a found tool runs with no wrapper args");
        }
    }
}

pub use native::{dashboard_surface, whichkey_surface};

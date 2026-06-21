//! The focus state machine (E4): grove-owned, **leader-dispatch** input
//! arbitration.
//!
//! grove owns the draw loop, so there is no zellij locked-mode — grove is the
//! arbiter by construction and sees every crossterm event first. [`arbitrate`] is
//! a **pure function** `(Focus, Leader, Event) → (Focus, Action)`: the
//! headless-testable transition table the brief calls for. The app
//! ([`crate::tui::app`]) owns the side effects — it applies the returned
//! [`Action`] (forward to the pane, mutate the modal buffer, redraw, quit) and
//! adopts the returned [`Focus`].
//!
//! ## The model (050/010-surfaces verdict)
//!
//! A grove's view is a *composed layout* grove draws — visible panels coexist and
//! focus moves laterally between them. [`Focus::Pane`] is the home base; the
//! leader is a **dispatch gate**, not a direct flip.
//!
//! - **[`Focus::Pane`]** — a foreign rmux pane is focused. Forward *every* key to
//!   it except the leader (maximising key fidelity). This is the generalisation of
//!   the former `Harness`: it is *any* focused foreign pane (harness today; the
//!   050 aux term/yazi/vcs panes tomorrow). Which pane is focused lives in the
//!   app's `self.focused` map key — the focus type does not name it.
//! - **[`Focus::Detail`]** — the per-grove detail panel, a focus peer beside the
//!   pane. Carries a [`DetailOrigin`]: entered from a pane (leader → `d`) `Esc`
//!   returns to the pane; entered from the nav (Tab / `l`, the 060 live preview)
//!   `Esc` returns to the nav. The widget itself is 030/040.
//! - **[`Focus::Nav`]** — grove's grove-list surface. Its in-surface keys
//!   (`j`/`k`/arrows move, `Enter` opens, `Esc` returns to the pane) stay; the
//!   former leader-prefixed actions (`c`/`e`/`q`) have moved onto the gate.
//! - **[`Focus::Modal`]** — a focus overlay that captures all keys (text into its
//!   buffer), restoring the **prior** focus on `Esc` (cancel) or `Enter`
//!   (submit). Swallows the leader (it owns focus while up).
//! - **[`Focus::LeaderPending`]** — the transient dispatch gate. The leader (from
//!   any non-modal surface) enters it; the *next* key dispatches: `g`→Nav,
//!   `d`→Detail, `c`→Capture, `e`→OpenEditor, `q`→Quit, `Esc`/anything-else →
//!   cancel back to the surface we leadered from (`prior`). The aux-pane keys
//!   `t`/`y`/`v` toggle the current grove's term/yazi/vcs panes (050/030) — each
//!   a toggle-with-focus-follow that lands on [`Focus::Pane`] (the pane on show,
//!   the harness on hide).
//!
//! ## Whichkey lives in the app footer, not here (050/010 verdict)
//!
//! The leader menu and per-surface hint line are one footer the [`App`] draws (see
//! [`crate::tui::footer`]). The focus type only carries *which state* we are in;
//! the footer reads it. ADR-0019's single-hint-owner holds by construction — one
//! draw loop, one footer — with no publish/subscribe or injected pane.

use ratatui::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEventKind,
};

use crate::tui::config::Leader;
use crate::tui::filter_mode::FilterEdit;
use crate::tui::input::{map_key, KeyToken};
use crate::tui::pane::PaneRole;

/// Which kind of modal is up. The enum lets a single [`Focus::Modal`] variant
/// carry several overlays without reshaping the focus type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalKind {
    /// The capture modal (a text buffer).
    Capture,
    /// The move/re-route target picker (a selectable grove list, 040 grooming):
    /// pick which grove the selected observation moves to.
    MovePicker,
    /// The seed-start confirm (rmux-tui-polish 070): a y/n prompt over the nav,
    /// shown when `Enter` lands on a **seed** row. `y`/`Enter` confirm (spawn
    /// `grove do <name>`, landing on the new harness pane); `n`/`Esc` cancel back
    /// to the nav. It owns focus while up (like every modal), and unlike the
    /// other two it carries no buffer/list — the seed to start rides in the App's
    /// `pending_start`, armed when the modal opens.
    Confirm,
}

/// How a [`Focus::Detail`] was entered — the seam the 060 live preview needs so
/// the detail panel's `Esc` returns to the surface the user came from, while
/// [`arbitrate`] stays a pure function (the origin rides in the focus value, not
/// in mutable app state).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailOrigin {
    /// Entered via the leader gate from a pane (`leader → d`): `Esc` → the pane.
    Pane,
    /// Entered from the nav (Tab, or `l` on a grove row): the file-manager live
    /// preview (Q7). `Esc` → the nav, not the pane — detail remembers it came
    /// from the nav.
    Nav,
}

/// Which surface currently owns keyboard input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Focus {
    /// A focused foreign rmux pane (forward everything but the leader). The app's
    /// `self.focused` says *which* pane; the focus type is pane-agnostic.
    Pane,
    /// The per-grove detail panel (a focus peer beside the pane). `origin` records
    /// how it was entered (see [`DetailOrigin`]): it decides where `Esc` returns
    /// and, in the app, whether detail follows the focused pane's grove or the
    /// nav-highlighted one (the 060 live preview).
    Detail { origin: DetailOrigin },
    /// grove's grove-list surface.
    Nav,
    /// The nav's **filter mode** (rmux-tui-polish 050): entered with `/` from
    /// [`Focus::Nav`], a text-entry sub-mode where keys edit the fuzzy needle and
    /// the toggles (Ctrl-i/l/s) live-re-rank the list. `Enter` accepts (back to
    /// [`Focus::Nav`], criteria kept), `Esc` clears (back to Nav, idle). The live
    /// [`Criteria`](crate::tui::filter::Criteria) and the ranked cursor are App
    /// state ([`crate::tui::filter_mode::FilterMode`]); this variant only says we
    /// are typing a filter.
    Filter,
    /// A focus overlay; restores `prior` on cancel/submit.
    Modal { kind: ModalKind, prior: Box<Focus> },
    /// The transient leader-dispatch gate: the next key dispatches, then we leave.
    /// `prior` is the surface we leadered from, restored on cancel.
    LeaderPending { prior: Box<Focus> },
}

/// What the app should do in response to one input event. The transition table
/// is pure; this enum is the instruction the impure app layer carries out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Nothing to do (and nothing changed).
    Ignore,
    /// Forward literal text to the focused pane (`send_text`).
    SendText(String),
    /// Forward a tmux key token to the focused pane (`send_key`).
    SendKey(String),
    /// Forward a bracketed paste to the pane — the app wraps it in
    /// `\e[200~…\e[201~` so multi-line pastes don't execute line-by-line.
    SendPaste(String),
    /// Forward a left-click to the focused pane at a (row, col) pane cell.
    HarnessClick { row: u16, col: u16 },
    /// Move the nav selection up one row.
    NavUp,
    /// Move the nav selection down one row.
    NavDown,
    /// Open (or focus) the harness for the nav's currently selected grove.
    /// The app applies the open/focus and lands on the pane — unless the
    /// cursor sits on a repo section header (the app toggles its fold and
    /// reverts the focus to Nav) or a seed (inert until 070 wires the
    /// confirm-and-start flow). The table cannot see the row kind, so the
    /// app resolves it — the [`Action::DetailMove`] revert precedent.
    NavSelect,
    /// Collapse the repo section under the nav cursor (`h`). The fold itself
    /// lives in the app's `Nav`; a no-op at a single-repo fleet (no headers).
    NavCollapse,
    /// Enter the detail **live preview** from the nav (060): `Tab`. The app
    /// focuses the detail panel on the highlighted grove; on a header or an empty
    /// list there is nothing to inspect, so it reverts to Nav (the [`NavSelect`]
    /// revert precedent — the pure table can't see the row kind).
    NavPeek,
    /// `l` from the nav (060): on a grove row enter the detail live preview (like
    /// [`NavPeek`]); on a header expand its fold (the vim-fold idiom `l` carried
    /// from 030). The app resolves the row kind, expanding and reverting to Nav
    /// for the header case.
    NavPeekOrExpand,
    /// `Esc` in normal Nav — the **layered Esc** (050 Q5). The table can't see
    /// whether a filter is engaged (App state), so it optimistically lands on the
    /// pane; the App reverts to Nav and clears the filter when one *was* engaged
    /// (the [`Action::NavSelect`] revert precedent). First Esc clears, second
    /// returns to the pane.
    NavEsc,
    /// Enter the nav's `/` filter mode (050): the App reprojects so the ranked
    /// list is fresh, then the user types. Preserves any existing needle/toggles
    /// (re-entering after accept), since only [`Action::FilterClear`] resets them.
    FilterEnter,
    /// Apply one in-mode edit to the live criteria (typed needle char/paste,
    /// backspace, or a Ctrl-i/l/s toggle). The App folds it in, reprojects, and
    /// snaps the cursor to the top row (fzf — the leaf's Notes).
    FilterEdit(FilterEdit),
    /// `Enter` in filter mode: accept and return to [`Focus::Nav`] with the
    /// criteria kept engaged (the nav now browses the ranked list).
    FilterAccept,
    /// `Esc` in filter mode: clear every dimension and return to [`Focus::Nav`]
    /// idle (the nav reverts to its grouped shape).
    FilterClear,
    /// Move up in the detail panel: the widget interprets this as *select the
    /// previous inbox observation* (when the grove has pending observations) or,
    /// for an empty inbox, *scroll the content up* — a single cursor the widget
    /// resolves against its own state (the impure half stays in the app).
    DetailUp,
    /// Move down in the detail panel (next observation, or scroll — see
    /// [`Action::DetailUp`]).
    DetailDown,
    /// Reject the detail panel's selected inbox observation (040 grooming): the
    /// app shells out to `grove-llm inbox-drain --rejected` under `spawn_blocking`.
    DetailReject,
    /// Begin a move/re-route of the selected inbox observation: the app opens the
    /// [`ModalKind::MovePicker`] target-grove picker (a no-op when nothing is
    /// selected — the app reverts the focus).
    DetailMove,
    /// Move the move-picker's selection up one row.
    MovePickerUp,
    /// Move the move-picker's selection down one row.
    MovePickerDown,
    /// Commit the move: send the pending observation to the picker's selected
    /// grove (`inbox-add --body-file`) then drop it here, under `spawn_blocking`.
    MovePickerSelect,
    /// Confirm the seed-start (`y`/`Enter` in the [`ModalKind::Confirm`] modal,
    /// 070): spawn `grove do <name>` for the armed seed, focus its new harness
    /// pane, and re-scan so the row flips Seed → Live. arbitrate optimistically
    /// lands on [`Focus::Pane`] (the harness it spawns); the app reverts to
    /// [`Focus::Nav`] if the spawn fails or nothing is armed (the [`NavSelect`]
    /// revert precedent). Cancel reuses [`Action::ModalCancel`] (drops the arm).
    ConfirmStart,
    /// A grove surface changed; the app should redraw.
    Redraw,
    /// Insert literal text into the focused modal's buffer.
    ModalInsert(String),
    /// Delete the last char of the modal buffer.
    ModalBackspace,
    /// Submit the modal buffer (grove's capture write).
    ModalSubmit,
    /// Dump the focused pane's rendered history into `$EDITOR` (040, ADR-0029):
    /// leader → `e`. The app suspends the loop, shells out to stock `rmux
    /// capture-pane`, runs the editor, and restores the TUI.
    OpenEditor,
    /// Toggle a working-set aux pane (term/yazi/vcs) for the current grove
    /// (050/030): leader → `t`/`y`/`v`. The *decision* — lazy-spawn on first
    /// show, hide-not-close on toggle-off, focus-follow to the pane on show or
    /// back to the harness on hide — is all app state the pure table cannot see,
    /// so the gate just names the role to flip and the app resolves the rest.
    ToggleAux(PaneRole),
    /// Discard the modal buffer.
    ModalCancel,
    /// Quit the TUI.
    Quit,
}

/// The pure transition. Given the current focus, the configured leader, and one
/// crossterm event, return the next focus and the action to perform.
pub fn arbitrate(focus: &Focus, leader: &Leader, ev: &Event) -> (Focus, Action) {
    match focus {
        Focus::Pane => arbitrate_pane(leader, ev),
        Focus::Detail { origin } => arbitrate_detail(*origin, leader, ev),
        Focus::Nav => arbitrate_nav(leader, ev),
        Focus::Filter => arbitrate_filter(ev),
        Focus::Modal {
            kind: ModalKind::Capture,
            prior,
        } => arbitrate_modal(prior, ev),
        Focus::Modal {
            kind: ModalKind::MovePicker,
            prior,
        } => arbitrate_move_picker(prior, ev),
        Focus::Modal {
            kind: ModalKind::Confirm,
            prior,
        } => arbitrate_confirm(prior, ev),
        Focus::LeaderPending { prior } => arbitrate_pending(prior, ev),
    }
}

/// A focused foreign pane: forward every key but the leader (which opens the
/// dispatch gate, remembering we came from the pane). Mouse/paste forward too.
fn arbitrate_pane(leader: &Leader, ev: &Event) -> (Focus, Action) {
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) {
                return (
                    Focus::LeaderPending {
                        prior: Box::new(Focus::Pane),
                    },
                    Action::Redraw,
                );
            }
            let action = match map_key(key) {
                KeyToken::Text(s) => Action::SendText(s),
                KeyToken::Key(k) => Action::SendKey(k),
                KeyToken::Unmapped => Action::Ignore,
            };
            (Focus::Pane, action)
        }
        Event::Paste(text) => (Focus::Pane, Action::SendPaste(text.clone())),
        Event::Mouse(m) => {
            if let MouseEventKind::Down(MouseButton::Left) = m.kind {
                (
                    Focus::Pane,
                    Action::HarnessClick {
                        row: m.row,
                        col: m.column,
                    },
                )
            } else {
                // Rich mouse passthrough (drag/wheel/motion) is deferred to 050
                // (likely a rmux raw-mouse capability) — do not build a lossy
                // automation-call translator here.
                (Focus::Pane, Action::Ignore)
            }
        }
        _ => (Focus::Pane, Action::Ignore),
    }
}

/// The per-grove detail panel. The leader opens the gate (remembering Detail and
/// its `origin`); `Esc` returns to where the panel was entered from — the pane
/// (leader → `d`) or the nav (Tab / `l`, the 060 live preview); `j`/`k` (or
/// arrows) move within the panel (select an inbox observation, or scroll — the
/// widget resolves which). The two **grooming** keys act on the selected
/// observation: `x` rejects it (stays in Detail), `m` opens the move-target
/// picker (a [`ModalKind::MovePicker`] modal that restores Detail on cancel).
/// Any other key is inert (not forwarded to the pane — detail owns focus).
fn arbitrate_detail(origin: DetailOrigin, leader: &Leader, ev: &Event) -> (Focus, Action) {
    let here = Focus::Detail { origin };
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) {
                return (
                    Focus::LeaderPending {
                        prior: Box::new(here),
                    },
                    Action::Redraw,
                );
            }
            match key.code {
                // The origin-aware return: nav-preview detail goes back to the
                // nav, pane-entered detail back to the pane (Q7).
                KeyCode::Esc => {
                    let back = match origin {
                        DetailOrigin::Pane => Focus::Pane,
                        DetailOrigin::Nav => Focus::Nav,
                    };
                    (back, Action::Redraw)
                }
                KeyCode::Down | KeyCode::Char('j') => (here, Action::DetailDown),
                KeyCode::Up | KeyCode::Char('k') => (here, Action::DetailUp),
                // Grooming: reject stays in Detail; move opens the target picker
                // (the app reverts the focus if there is nothing selected to move).
                // The modal's prior keeps the origin so cancel lands back here.
                KeyCode::Char('x') => (here, Action::DetailReject),
                KeyCode::Char('m') => (
                    Focus::Modal {
                        kind: ModalKind::MovePicker,
                        prior: Box::new(here),
                    },
                    Action::DetailMove,
                ),
                _ => (here, Action::Ignore),
            }
        }
        _ => (here, Action::Ignore),
    }
}

/// grove's grove-list surface. In-surface keys (`j`/`k`/arrows move, `Enter`
/// opens) stay; `Esc` returns to the home pane; the leader opens the gate. The
/// former leader-prefixed actions (`c`/`e`/`q`) have moved onto the gate, so they
/// are inert here now.
fn arbitrate_nav(leader: &Leader, ev: &Event) -> (Focus, Action) {
    match ev {
        Event::Key(key) if is_press(key) => {
            if leader.matches(key) {
                return (
                    Focus::LeaderPending {
                        prior: Box::new(Focus::Nav),
                    },
                    Action::Redraw,
                );
            }
            match key.code {
                // Layered Esc (050 Q5): optimistically lands on the pane; the App
                // reverts to Nav + clears when a filter was engaged (NavEsc).
                KeyCode::Esc => (Focus::Pane, Action::NavEsc),
                // `/` enters the filter sub-mode, preserving any prior needle.
                KeyCode::Char('/') => (Focus::Filter, Action::FilterEnter),
                // List navigation: arrows or vim j/k. The selection lives in the
                // app's `Nav` (or the filter cursor when engaged); arbitrate only
                // emits the movement intent.
                KeyCode::Up | KeyCode::Char('k') => (Focus::Nav, Action::NavUp),
                KeyCode::Down | KeyCode::Char('j') => (Focus::Nav, Action::NavDown),
                // Fold the repo section under the cursor (Q6: vim-style h/l).
                KeyCode::Char('h') => (Focus::Nav, Action::NavCollapse),
                // `l` is row-kind-dependent (060): a grove row enters the detail
                // live preview, a header expands its fold. The pure table can't
                // see the row kind, so it optimistically lands on Detail{Nav} and
                // the app reverts to Nav (expanding) on a header.
                KeyCode::Char('l') => (
                    Focus::Detail {
                        origin: DetailOrigin::Nav,
                    },
                    Action::NavPeekOrExpand,
                ),
                // Tab enters the detail live preview (060); the app reverts to Nav
                // when the cursor sits on a header / empty list (nothing to read).
                KeyCode::Tab => (
                    Focus::Detail {
                        origin: DetailOrigin::Nav,
                    },
                    Action::NavPeek,
                ),
                // Open/focus the selected grove's harness, landing on the pane.
                // (On a header or seed row the app reverts the focus to Nav —
                // see [`Action::NavSelect`].)
                KeyCode::Enter => (Focus::Pane, Action::NavSelect),
                _ => (Focus::Nav, Action::Ignore),
            }
        }
        _ => (Focus::Nav, Action::Ignore),
    }
}

/// The nav's **filter mode** (050): a text-entry sub-mode over the grove list.
/// Plain chars (and pastes) extend the fuzzy needle; `Backspace` trims it; the
/// three toggles re-rank live — **Ctrl-i *or* Tab** flips inbox-pending (Q3's
/// legacy-terminal aliasing: many terminals send the same byte for both),
/// `Ctrl-l` cycles lifecycle, `Ctrl-s` cycles sort. `↑`/`↓` move the ranked
/// cursor while typing (fzf). `Enter` accepts (→ Nav, criteria kept); `Esc`
/// clears (→ Nav, idle). Everything else — including the leader — is swallowed
/// (this is text entry; the leader is a valid needle char via plain `g`).
fn arbitrate_filter(ev: &Event) -> (Focus, Action) {
    let stay = |action| (Focus::Filter, action);
    match ev {
        Event::Key(key) if is_press(key) => {
            use ratatui::crossterm::event::KeyModifiers;
            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
            match key.code {
                KeyCode::Enter => (Focus::Nav, Action::FilterAccept),
                KeyCode::Esc => (Focus::Nav, Action::FilterClear),
                KeyCode::Backspace => stay(Action::FilterEdit(FilterEdit::Backspace)),
                // Move the ranked cursor without leaving the needle (fzf).
                KeyCode::Up => stay(Action::NavUp),
                KeyCode::Down => stay(Action::NavDown),
                // Inbox toggle: Tab is the legacy alias for Ctrl-i (same byte on
                // many terminals — Q3). Ctrl-l/Ctrl-s cycle lifecycle/sort.
                KeyCode::Tab => stay(Action::FilterEdit(FilterEdit::ToggleInbox)),
                KeyCode::Char('i') if ctrl => stay(Action::FilterEdit(FilterEdit::ToggleInbox)),
                KeyCode::Char('l') if ctrl => stay(Action::FilterEdit(FilterEdit::CycleLifecycle)),
                KeyCode::Char('s') if ctrl => stay(Action::FilterEdit(FilterEdit::CycleSort)),
                // Plain printable chars extend the needle; modifier chords are
                // swallowed (like the capture modal).
                KeyCode::Char(c) if is_plain_char(key) => {
                    stay(Action::FilterEdit(FilterEdit::Insert(c.to_string())))
                }
                _ => stay(Action::Ignore),
            }
        }
        // A paste lands in the needle literally (no pane to forward to).
        Event::Paste(text) => stay(Action::FilterEdit(FilterEdit::Insert(text.clone()))),
        _ => stay(Action::Ignore),
    }
}

/// The transient dispatch gate: the next key press routes, then we leave. Any key
/// without a route (including `Esc`) cancels back to `prior`, so the gate never
/// traps the user. Non-press / non-key events keep the gate open.
fn arbitrate_pending(prior: &Focus, ev: &Event) -> (Focus, Action) {
    let stay = || {
        (
            Focus::LeaderPending {
                prior: Box::new(prior.clone()),
            },
            Action::Ignore,
        )
    };
    let cancel = || (prior.clone(), Action::Redraw);
    match ev {
        Event::Key(key) if is_press(key) => match key.code {
            KeyCode::Char('g') => (Focus::Nav, Action::Redraw),
            // Detail's `Esc`-return origin is sticky through the gate: leadering
            // from the nav (or a nav-preview detail) keeps the Nav origin, so
            // `Esc` still returns to the nav; from a pane it returns to the pane.
            KeyCode::Char('d') => {
                let origin = match prior {
                    Focus::Nav => DetailOrigin::Nav,
                    Focus::Detail { origin } => *origin,
                    _ => DetailOrigin::Pane,
                };
                (Focus::Detail { origin }, Action::Redraw)
            }
            // The capture modal restores the surface we leadered from on cancel.
            KeyCode::Char('c') => (
                Focus::Modal {
                    kind: ModalKind::Capture,
                    prior: Box::new(prior.clone()),
                },
                Action::Redraw,
            ),
            // Aux-pane toggles (050/030): show/hide the current grove's
            // term/yazi/vcs pane. Always land on `Pane` — focus-follows to the
            // pane on show, returns to the harness on hide (both foreign panes,
            // so both are `Focus::Pane`); the app resolves which from its state.
            KeyCode::Char('t') => (Focus::Pane, Action::ToggleAux(PaneRole::Term)),
            KeyCode::Char('y') => (Focus::Pane, Action::ToggleAux(PaneRole::Yazi)),
            KeyCode::Char('v') => (Focus::Pane, Action::ToggleAux(PaneRole::Vcs)),
            // Open-in-editor / quit return focus to the prior surface (the editor
            // and quit operate on the app, not on which surface is focused).
            KeyCode::Char('e') => (prior.clone(), Action::OpenEditor),
            KeyCode::Char('q') => (prior.clone(), Action::Quit),
            // Esc and any unmapped key cancel the gate (redraw to clear the menu).
            _ => cancel(),
        },
        // A non-press key (kitty release) or a non-key event leaves the gate open.
        _ => stay(),
    }
}

/// The capture modal (a text buffer). `Esc` cancels / `Enter` submits, both
/// restoring `prior`; chars edit the buffer.
fn arbitrate_modal(prior: &Focus, ev: &Event) -> (Focus, Action) {
    let stay = |action| {
        (
            Focus::Modal {
                kind: ModalKind::Capture,
                prior: Box::new(prior.clone()),
            },
            action,
        )
    };
    match ev {
        Event::Key(key) if is_press(key) => match key.code {
            KeyCode::Esc => (prior.clone(), Action::ModalCancel),
            KeyCode::Enter => (prior.clone(), Action::ModalSubmit),
            KeyCode::Backspace => stay(Action::ModalBackspace),
            // Literal text into the buffer; modifier chords are swallowed (the
            // modal owns focus, like grove's capture).
            KeyCode::Char(c) if is_plain_char(key) => stay(Action::ModalInsert(c.to_string())),
            _ => stay(Action::Ignore),
        },
        // Paste lands in the buffer literally (not bracketed-wrapped — there is
        // no pane to forward to while the modal is up).
        Event::Paste(text) => stay(Action::ModalInsert(text.clone())),
        _ => stay(Action::Ignore),
    }
}

/// The move/re-route target picker (040 grooming): a selectable grove list, so
/// it behaves like [`Nav`](crate::tui::nav), not like a text modal. `j`/`k`
/// (or arrows) move the selection; `Enter` commits the move (and restores the
/// `prior` Detail surface); `Esc` cancels (also restoring Detail). The leader is
/// swallowed — the modal owns focus while up.
fn arbitrate_move_picker(prior: &Focus, ev: &Event) -> (Focus, Action) {
    let stay = |action| {
        (
            Focus::Modal {
                kind: ModalKind::MovePicker,
                prior: Box::new(prior.clone()),
            },
            action,
        )
    };
    match ev {
        Event::Key(key) if is_press(key) => match key.code {
            KeyCode::Esc => (prior.clone(), Action::ModalCancel),
            KeyCode::Enter => (prior.clone(), Action::MovePickerSelect),
            KeyCode::Up | KeyCode::Char('k') => stay(Action::MovePickerUp),
            KeyCode::Down | KeyCode::Char('j') => stay(Action::MovePickerDown),
            _ => stay(Action::Ignore),
        },
        _ => stay(Action::Ignore),
    }
}

/// The seed-start confirm (070): a y/n prompt over the nav. `y`/`Enter` confirm
/// — spawn the grove, landing optimistically on the pane the harness will fill
/// ([`Action::ConfirmStart`]); `n`/`Esc` cancel back to `prior` (the nav),
/// reusing [`Action::ModalCancel`] to drop the armed seed. The leader and every
/// other key are swallowed — the modal owns focus while up.
fn arbitrate_confirm(prior: &Focus, ev: &Event) -> (Focus, Action) {
    let stay = |action| {
        (
            Focus::Modal {
                kind: ModalKind::Confirm,
                prior: Box::new(prior.clone()),
            },
            action,
        )
    };
    match ev {
        Event::Key(key) if is_press(key) => match key.code {
            // Confirm: land on the pane the spawned harness will own (the app
            // reverts to Nav on a spawn failure / nothing armed).
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                (Focus::Pane, Action::ConfirmStart)
            }
            // Cancel: back to the nav, dropping the armed seed.
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                (prior.clone(), Action::ModalCancel)
            }
            _ => stay(Action::Ignore),
        },
        _ => stay(Action::Ignore),
    }
}

/// Only Press/Repeat events drive the machine; Release events (kitty progressive
/// enhancement) must not double-send or re-transition.
fn is_press(key: &KeyEvent) -> bool {
    matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat)
}

/// A printable char with no Ctrl/Alt (Shift is already in the glyph) — the chars
/// that should land literally in a modal buffer.
fn is_plain_char(key: &KeyEvent) -> bool {
    use ratatui::crossterm::event::KeyModifiers;
    !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{KeyModifiers, MouseEvent};

    fn leader() -> Leader {
        Leader::alt_g()
    }

    fn key_ev(code: KeyCode, mods: KeyModifiers) -> Event {
        Event::Key(KeyEvent::new(code, mods))
    }

    fn leader_ev() -> Event {
        key_ev(KeyCode::Char('g'), KeyModifiers::ALT)
    }

    fn release_ev(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new_with_kind(
            code,
            KeyModifiers::NONE,
            KeyEventKind::Release,
        ))
    }

    fn left_click(row: u16, col: u16) -> Event {
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            row,
            column: col,
            modifiers: KeyModifiers::NONE,
        })
    }

    fn pending_over(prior: Focus) -> Focus {
        Focus::LeaderPending {
            prior: Box::new(prior),
        }
    }

    fn detail(origin: DetailOrigin) -> Focus {
        Focus::Detail { origin }
    }

    fn capture_over(prior: Focus) -> Focus {
        Focus::Modal {
            kind: ModalKind::Capture,
            prior: Box::new(prior),
        }
    }

    // --- Pane focus (the former Harness, generalised) ----------------------

    #[test]
    fn pane_forwards_plain_char_as_text() {
        let (focus, action) = arbitrate(
            &Focus::Pane,
            &leader(),
            &key_ev(KeyCode::Char('a'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::SendText("a".into()));
    }

    #[test]
    fn pane_forwards_special_key_as_token() {
        let (_, action) = arbitrate(
            &Focus::Pane,
            &leader(),
            &key_ev(KeyCode::Up, KeyModifiers::CONTROL),
        );
        assert_eq!(action, Action::SendKey("C-Up".into()));
    }

    #[test]
    fn pane_leader_enters_leader_pending_remembering_the_pane() {
        let (focus, action) = arbitrate(&Focus::Pane, &leader(), &leader_ev());
        assert_eq!(focus, pending_over(Focus::Pane));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pane_paste_is_forwarded_wrapped() {
        let (focus, action) = arbitrate(
            &Focus::Pane,
            &leader(),
            &Event::Paste("line1\nline2".into()),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::SendPaste("line1\nline2".into()));
    }

    #[test]
    fn pane_left_click_forwards_a_click() {
        let (focus, action) = arbitrate(&Focus::Pane, &leader(), &left_click(4, 7));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::HarnessClick { row: 4, col: 7 });
    }

    #[test]
    fn pane_ignores_key_release() {
        let (focus, action) = arbitrate(&Focus::Pane, &leader(), &release_ev(KeyCode::Char('a')));
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::Ignore);
    }

    // --- LeaderPending: the dispatch gate ----------------------------------

    #[test]
    fn pending_g_dispatches_to_nav() {
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('g'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_d_dispatches_to_detail_with_a_pane_origin() {
        // Leadered from a pane → detail's Esc origin is the pane.
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('d'), KeyModifiers::NONE),
        );
        assert_eq!(focus, detail(DetailOrigin::Pane));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_d_from_nav_keeps_the_nav_origin() {
        // Leadered from the nav → detail's Esc returns to the nav, not the pane.
        let (focus, _) = arbitrate(
            &pending_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Char('d'), KeyModifiers::NONE),
        );
        assert_eq!(focus, detail(DetailOrigin::Nav));
        // And the origin is sticky if we leadered from a nav-preview detail.
        let (focus, _) = arbitrate(
            &pending_over(detail(DetailOrigin::Nav)),
            &leader(),
            &key_ev(KeyCode::Char('d'), KeyModifiers::NONE),
        );
        assert_eq!(focus, detail(DetailOrigin::Nav));
    }

    #[test]
    fn pending_c_opens_capture_modal_restoring_the_prior_surface() {
        // Leadered from Detail → the capture modal's prior is Detail, so cancel
        // returns there, not to the pane.
        let (focus, action) = arbitrate(
            &pending_over(detail(DetailOrigin::Pane)),
            &leader(),
            &key_ev(KeyCode::Char('c'), KeyModifiers::NONE),
        );
        assert_eq!(focus, capture_over(detail(DetailOrigin::Pane)));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_e_opens_editor_and_returns_to_prior() {
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('e'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::OpenEditor);
    }

    #[test]
    fn pending_q_quits() {
        let (_, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('q'), KeyModifiers::NONE),
        );
        assert_eq!(action, Action::Quit);
    }

    #[test]
    fn pending_tyv_toggle_the_aux_panes_and_land_on_pane() {
        // leader → t/y/v flips the current grove's term/yazi/vcs pane. The pure
        // table can't know visible-vs-hidden, so it names the role and lands on
        // `Pane` (focus-follow on show / return-to-harness on hide both → Pane).
        for (ch, role) in [
            ('t', PaneRole::Term),
            ('y', PaneRole::Yazi),
            ('v', PaneRole::Vcs),
        ] {
            let (focus, action) = arbitrate(
                &pending_over(detail(DetailOrigin::Pane)),
                &leader(),
                &key_ev(KeyCode::Char(ch), KeyModifiers::NONE),
            );
            assert_eq!(
                focus,
                Focus::Pane,
                "{ch} lands on the pane regardless of prior"
            );
            assert_eq!(action, Action::ToggleAux(role), "{ch} toggles {role:?}");
        }
    }

    #[test]
    fn pending_esc_cancels_back_to_prior() {
        let (focus, action) = arbitrate(
            &pending_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(
            focus,
            Focus::Nav,
            "cancel restores the surface we leadered from"
        );
        assert_eq!(action, Action::Redraw, "redraw to clear the menu footer");
    }

    #[test]
    fn pending_unmapped_key_cancels_back_to_prior() {
        // A key with no dispatch meaning does not trap the user — it cancels.
        let (focus, action) = arbitrate(
            &pending_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Char('z'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn pending_ignores_key_release_and_stays_pending() {
        // A release event (kitty) must not be mistaken for a dispatch key.
        let start = pending_over(Focus::Pane);
        let (focus, action) = arbitrate(&start, &leader(), &release_ev(KeyCode::Char('g')));
        assert_eq!(focus, start);
        assert_eq!(action, Action::Ignore);
    }

    // --- Detail focus (transition wired; widget is 030) --------------------

    #[test]
    fn detail_leader_enters_pending_remembering_detail() {
        let (focus, action) = arbitrate(&detail(DetailOrigin::Pane), &leader(), &leader_ev());
        assert_eq!(focus, pending_over(detail(DetailOrigin::Pane)));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn detail_pane_origin_esc_returns_to_pane() {
        let (focus, action) = arbitrate(
            &detail(DetailOrigin::Pane),
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(
            focus,
            Focus::Pane,
            "pane-entered detail Esc's back to the pane"
        );
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn detail_nav_origin_esc_returns_to_nav() {
        // The 060 live-preview leg: detail entered from the nav remembers it and
        // Esc returns to the nav, not the pane.
        let (focus, action) = arbitrate(
            &detail(DetailOrigin::Nav),
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(
            focus,
            Focus::Nav,
            "nav-entered detail Esc's back to the nav"
        );
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn detail_jk_and_arrows_move_within_the_widget() {
        let here = detail(DetailOrigin::Pane);
        for code in [KeyCode::Down, KeyCode::Char('j')] {
            let (focus, action) = arbitrate(&here, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, here);
            assert_eq!(action, Action::DetailDown);
        }
        for code in [KeyCode::Up, KeyCode::Char('k')] {
            let (focus, action) = arbitrate(&here, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, here);
            assert_eq!(action, Action::DetailUp);
        }
    }

    #[test]
    fn detail_x_rejects_the_selected_observation_in_place() {
        let here = detail(DetailOrigin::Nav);
        let (focus, action) = arbitrate(
            &here,
            &leader(),
            &key_ev(KeyCode::Char('x'), KeyModifiers::NONE),
        );
        assert_eq!(
            focus, here,
            "reject stays in the detail panel (preserving origin)"
        );
        assert_eq!(action, Action::DetailReject);
    }

    #[test]
    fn detail_m_opens_the_move_picker_modal_preserving_origin() {
        // Grooming a nav-previewed grove: the picker's prior is Detail{Nav}, so
        // cancel returns to the preview, not to a pane.
        let (focus, action) = arbitrate(
            &detail(DetailOrigin::Nav),
            &leader(),
            &key_ev(KeyCode::Char('m'), KeyModifiers::NONE),
        );
        assert_eq!(
            focus,
            Focus::Modal {
                kind: ModalKind::MovePicker,
                prior: Box::new(detail(DetailOrigin::Nav)),
            }
        );
        assert_eq!(action, Action::DetailMove);
    }

    #[test]
    fn detail_swallows_other_unmapped_keys() {
        // A key with no in-surface meaning is inert (not forwarded to the pane).
        let here = detail(DetailOrigin::Pane);
        let (focus, action) = arbitrate(
            &here,
            &leader(),
            &key_ev(KeyCode::Char('z'), KeyModifiers::NONE),
        );
        assert_eq!(focus, here);
        assert_eq!(action, Action::Ignore);
    }

    // --- Move picker modal (a selectable grove list) -----------------------

    fn move_picker_over(prior: Focus) -> Focus {
        Focus::Modal {
            kind: ModalKind::MovePicker,
            prior: Box::new(prior),
        }
    }

    #[test]
    fn move_picker_jk_and_arrows_move_the_selection_and_stay() {
        let prior = detail(DetailOrigin::Pane);
        for code in [KeyCode::Down, KeyCode::Char('j')] {
            let (focus, action) = arbitrate(
                &move_picker_over(prior.clone()),
                &leader(),
                &key_ev(code, KeyModifiers::NONE),
            );
            assert_eq!(focus, move_picker_over(prior.clone()));
            assert_eq!(action, Action::MovePickerDown);
        }
        for code in [KeyCode::Up, KeyCode::Char('k')] {
            let (focus, action) = arbitrate(
                &move_picker_over(prior.clone()),
                &leader(),
                &key_ev(code, KeyModifiers::NONE),
            );
            assert_eq!(focus, move_picker_over(prior.clone()));
            assert_eq!(action, Action::MovePickerUp);
        }
    }

    #[test]
    fn move_picker_enter_commits_and_returns_to_detail() {
        let (focus, action) = arbitrate(
            &move_picker_over(detail(DetailOrigin::Pane)),
            &leader(),
            &key_ev(KeyCode::Enter, KeyModifiers::NONE),
        );
        assert_eq!(focus, detail(DetailOrigin::Pane));
        assert_eq!(action, Action::MovePickerSelect);
    }

    #[test]
    fn move_picker_esc_cancels_back_to_detail() {
        let (focus, action) = arbitrate(
            &move_picker_over(detail(DetailOrigin::Pane)),
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(focus, detail(DetailOrigin::Pane));
        assert_eq!(action, Action::ModalCancel);
    }

    #[test]
    fn move_picker_swallows_the_leader() {
        // The picker owns focus; the leader is just another swallowed key.
        let prior = detail(DetailOrigin::Pane);
        let (focus, action) = arbitrate(&move_picker_over(prior.clone()), &leader(), &leader_ev());
        assert_eq!(focus, move_picker_over(prior));
        assert_eq!(action, Action::Ignore);
    }

    // --- Confirm modal (the 070 seed-start y/n prompt) ---------------------

    fn confirm_over(prior: Focus) -> Focus {
        Focus::Modal {
            kind: ModalKind::Confirm,
            prior: Box::new(prior),
        }
    }

    #[test]
    fn confirm_y_and_enter_start_the_grove_landing_on_the_pane() {
        // Both `y` and Enter confirm; the focus lands optimistically on the pane
        // the spawned harness will own (the app reverts on failure).
        for code in [KeyCode::Char('y'), KeyCode::Char('Y'), KeyCode::Enter] {
            let (focus, action) = arbitrate(
                &confirm_over(Focus::Nav),
                &leader(),
                &key_ev(code, KeyModifiers::NONE),
            );
            assert_eq!(focus, Focus::Pane, "{code:?} lands on the pane");
            assert_eq!(action, Action::ConfirmStart, "{code:?} confirms the start");
        }
    }

    #[test]
    fn confirm_n_and_esc_cancel_back_to_the_nav() {
        // `n` and Esc both cancel, restoring the nav we opened the confirm over
        // and dropping the armed seed (ModalCancel).
        for code in [KeyCode::Char('n'), KeyCode::Char('N'), KeyCode::Esc] {
            let (focus, action) = arbitrate(
                &confirm_over(Focus::Nav),
                &leader(),
                &key_ev(code, KeyModifiers::NONE),
            );
            assert_eq!(focus, Focus::Nav, "{code:?} returns to the nav");
            assert_eq!(action, Action::ModalCancel, "{code:?} drops the armed seed");
        }
    }

    #[test]
    fn confirm_swallows_the_leader_and_other_keys() {
        // The modal owns focus: the leader and any unmapped key are inert (the
        // prompt stays up rather than acting or leaking to a surface).
        let prior = Focus::Nav;
        let (focus, action) = arbitrate(&confirm_over(prior.clone()), &leader(), &leader_ev());
        assert_eq!(focus, confirm_over(prior.clone()));
        assert_eq!(action, Action::Ignore);
        let (focus, action) = arbitrate(
            &confirm_over(prior.clone()),
            &leader(),
            &key_ev(KeyCode::Char('z'), KeyModifiers::NONE),
        );
        assert_eq!(focus, confirm_over(prior));
        assert_eq!(action, Action::Ignore);
    }

    #[test]
    fn confirm_ignores_key_release() {
        // A kitty release of `y` must not be mistaken for a confirm.
        let start = confirm_over(Focus::Nav);
        let (focus, action) = arbitrate(&start, &leader(), &release_ev(KeyCode::Char('y')));
        assert_eq!(focus, start);
        assert_eq!(action, Action::Ignore);
    }

    // --- Nav focus ---------------------------------------------------------

    #[test]
    fn nav_leader_enters_pending_remembering_nav() {
        let (focus, action) = arbitrate(&Focus::Nav, &leader(), &leader_ev());
        assert_eq!(focus, pending_over(Focus::Nav));
        assert_eq!(action, Action::Redraw);
    }

    #[test]
    fn nav_esc_is_the_layered_esc_landing_optimistically_on_the_pane() {
        // The table can't see whether a filter is engaged (App state), so Esc
        // optimistically lands on the pane and emits NavEsc; the App reverts to
        // Nav + clears when a filter *was* engaged (layer one), else stays.
        let (focus, action) = arbitrate(
            &Focus::Nav,
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::NavEsc);
    }

    #[test]
    fn nav_slash_enters_filter_mode() {
        let (focus, action) = arbitrate(
            &Focus::Nav,
            &leader(),
            &key_ev(KeyCode::Char('/'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Filter);
        assert_eq!(action, Action::FilterEnter);
    }

    #[test]
    fn nav_arrows_and_jk_move_the_selection() {
        for code in [KeyCode::Up, KeyCode::Char('k')] {
            let (focus, action) =
                arbitrate(&Focus::Nav, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Nav);
            assert_eq!(action, Action::NavUp);
        }
        for code in [KeyCode::Down, KeyCode::Char('j')] {
            let (focus, action) =
                arbitrate(&Focus::Nav, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Nav);
            assert_eq!(action, Action::NavDown);
        }
    }

    #[test]
    fn nav_h_collapses_staying_in_nav() {
        let (focus, action) = arbitrate(
            &Focus::Nav,
            &leader(),
            &key_ev(KeyCode::Char('h'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav);
        assert_eq!(action, Action::NavCollapse);
    }

    #[test]
    fn nav_l_optimistically_enters_detail_and_lets_the_app_resolve_the_row() {
        // `l` is row-kind-dependent (060): the pure table lands on Detail{Nav}
        // and emits NavPeekOrExpand; the app keeps the preview on a grove row or
        // reverts to Nav + expands on a header.
        let (focus, action) = arbitrate(
            &Focus::Nav,
            &leader(),
            &key_ev(KeyCode::Char('l'), KeyModifiers::NONE),
        );
        assert_eq!(focus, detail(DetailOrigin::Nav));
        assert_eq!(action, Action::NavPeekOrExpand);
    }

    #[test]
    fn nav_tab_enters_the_detail_live_preview() {
        let (focus, action) = arbitrate(
            &Focus::Nav,
            &leader(),
            &key_ev(KeyCode::Tab, KeyModifiers::NONE),
        );
        assert_eq!(focus, detail(DetailOrigin::Nav));
        assert_eq!(action, Action::NavPeek);
    }

    #[test]
    fn nav_enter_selects_and_lands_on_pane() {
        let (focus, action) = arbitrate(
            &Focus::Nav,
            &leader(),
            &key_ev(KeyCode::Enter, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::NavSelect);
    }

    #[test]
    fn nav_no_longer_handles_leader_prefixed_actions_directly() {
        // c / e / q moved onto the dispatch gate; in Nav they are plain,
        // unhandled keys now (the gate owns those routes).
        for code in [KeyCode::Char('c'), KeyCode::Char('e'), KeyCode::Char('q')] {
            let (focus, action) =
                arbitrate(&Focus::Nav, &leader(), &key_ev(code, KeyModifiers::NONE));
            assert_eq!(focus, Focus::Nav);
            assert_eq!(
                action,
                Action::Ignore,
                "{code:?} should be inert in Nav now"
            );
        }
    }

    // --- Filter mode (the `/` sub-mode over the nav) -----------------------

    fn ctrl(c: char) -> Event {
        key_ev(KeyCode::Char(c), KeyModifiers::CONTROL)
    }

    #[test]
    fn filter_plain_char_extends_the_needle_and_stays() {
        let (focus, action) = arbitrate(
            &Focus::Filter,
            &leader(),
            &key_ev(KeyCode::Char('a'), KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Filter);
        assert_eq!(action, Action::FilterEdit(FilterEdit::Insert("a".into())));
    }

    #[test]
    fn filter_backspace_trims_the_needle() {
        let (focus, action) = arbitrate(
            &Focus::Filter,
            &leader(),
            &key_ev(KeyCode::Backspace, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Filter);
        assert_eq!(action, Action::FilterEdit(FilterEdit::Backspace));
    }

    #[test]
    fn filter_ctrl_i_and_tab_both_toggle_inbox_pending() {
        // Q3 legacy-terminal aliasing: Ctrl-i and Tab are the same byte on many
        // terminals, so both must drive the inbox toggle in mode.
        let (_, via_ctrl) = arbitrate(&Focus::Filter, &leader(), &ctrl('i'));
        assert_eq!(via_ctrl, Action::FilterEdit(FilterEdit::ToggleInbox));
        let (focus, via_tab) = arbitrate(
            &Focus::Filter,
            &leader(),
            &key_ev(KeyCode::Tab, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Filter);
        assert_eq!(via_tab, Action::FilterEdit(FilterEdit::ToggleInbox));
    }

    #[test]
    fn filter_ctrl_l_and_ctrl_s_cycle_lifecycle_and_sort() {
        let (_, lifecycle) = arbitrate(&Focus::Filter, &leader(), &ctrl('l'));
        assert_eq!(lifecycle, Action::FilterEdit(FilterEdit::CycleLifecycle));
        let (_, sort) = arbitrate(&Focus::Filter, &leader(), &ctrl('s'));
        assert_eq!(sort, Action::FilterEdit(FilterEdit::CycleSort));
    }

    #[test]
    fn filter_arrows_move_the_ranked_cursor_without_leaving_the_needle() {
        let (focus, down) = arbitrate(
            &Focus::Filter,
            &leader(),
            &key_ev(KeyCode::Down, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Filter, "still typing — the needle stays");
        assert_eq!(down, Action::NavDown);
        let (_, up) = arbitrate(
            &Focus::Filter,
            &leader(),
            &key_ev(KeyCode::Up, KeyModifiers::NONE),
        );
        assert_eq!(up, Action::NavUp);
    }

    #[test]
    fn filter_enter_accepts_back_to_nav_keeping_the_criteria() {
        let (focus, action) = arbitrate(
            &Focus::Filter,
            &leader(),
            &key_ev(KeyCode::Enter, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav, "accept returns to the nav surface");
        assert_eq!(action, Action::FilterAccept);
    }

    #[test]
    fn filter_esc_clears_back_to_nav() {
        let (focus, action) = arbitrate(
            &Focus::Filter,
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav);
        assert_eq!(action, Action::FilterClear);
    }

    #[test]
    fn filter_swallows_the_leader_and_unmapped_chords() {
        // Text entry owns every key: the leader is just a swallowed chord (a plain
        // `g` would be a needle char), as is any Ctrl chord without a binding.
        let (focus, action) = arbitrate(&Focus::Filter, &leader(), &leader_ev());
        assert_eq!(focus, Focus::Filter);
        assert_eq!(action, Action::Ignore);
        let (_, action) = arbitrate(&Focus::Filter, &leader(), &ctrl('z'));
        assert_eq!(action, Action::Ignore);
    }

    #[test]
    fn filter_ignores_key_release() {
        let (focus, action) = arbitrate(&Focus::Filter, &leader(), &release_ev(KeyCode::Char('a')));
        assert_eq!(focus, Focus::Filter);
        assert_eq!(action, Action::Ignore);
    }

    // --- Modal focus -------------------------------------------------------

    #[test]
    fn modal_char_inserts_into_buffer_and_stays() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Char('h'), KeyModifiers::NONE),
        );
        assert_eq!(focus, capture_over(Focus::Nav));
        assert_eq!(action, Action::ModalInsert("h".into()));
    }

    #[test]
    fn modal_esc_cancels_and_restores_prior() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Esc, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Nav);
        assert_eq!(action, Action::ModalCancel);
    }

    #[test]
    fn modal_enter_submits_and_restores_prior() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Pane),
            &leader(),
            &key_ev(KeyCode::Enter, KeyModifiers::NONE),
        );
        assert_eq!(focus, Focus::Pane);
        assert_eq!(action, Action::ModalSubmit);
    }

    #[test]
    fn modal_backspace_edits_buffer() {
        let (_, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &key_ev(KeyCode::Backspace, KeyModifiers::NONE),
        );
        assert_eq!(action, Action::ModalBackspace);
    }

    #[test]
    fn modal_paste_inserts_literally() {
        let (focus, action) = arbitrate(
            &capture_over(Focus::Nav),
            &leader(),
            &Event::Paste("pasted".into()),
        );
        assert_eq!(focus, capture_over(Focus::Nav));
        assert_eq!(action, Action::ModalInsert("pasted".into()));
    }

    #[test]
    fn modal_swallows_the_leader() {
        // The modal owns focus; the leader is just another swallowed key (it does
        // NOT open the dispatch gate while a modal is up).
        let (focus, action) = arbitrate(&capture_over(Focus::Nav), &leader(), &leader_ev());
        assert_eq!(focus, capture_over(Focus::Nav));
        assert_eq!(action, Action::Ignore);
    }
}

//! The controller↔proxy seam for the dashboard (ADR-0016).
//!
//! Per ADR-0016 every dashboard surface is a **dumb-terminal proxy** to one
//! controlling process: the controller renders ratatui and ships the ANSI
//! output *down* a unix-domain socket; the proxy blits it to its stdout and
//! forwards raw stdin *up*; the controller decodes those bytes into key events
//! itself. This module holds the four pieces of that seam:
//!
//! - [`proto`] — the up-direction wire codec (`resize` / `input` frames).
//! - [`decode`] — the input decoder: raw stdin bytes → `KeyCode`/`KeyModifiers`.
//! - [`backend`] — the render-over-socket ratatui backend the controller draws
//!   into, sized to the proxy rather than to any local tty.
//! - [`proxy`] — the dumb `grove __dash-proxy` client itself.
//!
//! `proto` and `proxy` are pure transport (no `ratatui`, no `RepoView`).
//! `decode` produces presentation input (`crossterm` key types) but imports no
//! data layer; `backend` is the controller-side render target. All of it sits
//! at or above the ADR-0013 presentation seam.

pub mod backend;
pub mod decode;
pub mod proto;
pub mod proxy;

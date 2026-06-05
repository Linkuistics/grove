//! Host-supplied branding for user-facing framework strings.
//!
//! trellis is a reusable TUI framework with a **one-way crate seam** (ADR-0021):
//! the framework must never depend on its consumer. So user-facing identity text
//! (the exit banner, etc.) cannot be hardcoded to either "Zellij" (a leaked impl
//! detail) or the consumer's name. Instead the framework *declares* this
//! [`Branding`] trait; a host app registers an implementation via
//! [`register_branding`], and the framework reads it through [`branding`].
//!
//! The default (no host registered) is deliberately **neutral** — every method
//! has a do-nothing default, so standalone trellis never names "Zellij" in
//! user-facing text. This mirrors the [`crate::consts::DEBUG_MODE`] `OnceLock`
//! handshake: a process-wide cell the host sets once during framework boot. The
//! client and the re-exec'd server each register their own (a trait object
//! cannot cross the re-exec), so registration belongs in each process's boot.

use std::sync::OnceLock;

/// User-facing branding the framework consults for identity strings. Every
/// method defaults to the neutral framework behaviour, so a host overrides only
/// the surfaces it cares about. New branded surfaces are added here as methods
/// with neutral defaults — the single injection point for the whole framework.
pub trait Branding: Send + Sync {
    /// The message printed as the terminal tears down on a **normal** exit.
    /// `None` (the default) prints no banner — a clean return to the shell,
    /// which is what a host presenting as its own UI wants.
    fn exit_message(&self) -> Option<String> {
        None
    }
}

/// The neutral default used when no host has registered branding — every method
/// falls through to the trait defaults (no "Zellij" anywhere).
struct DefaultBranding;
impl Branding for DefaultBranding {}

static BRANDING: OnceLock<Box<dyn Branding>> = OnceLock::new();

/// Register the host's [`Branding`]. Call once per process during framework
/// boot — the foreground client and the re-exec'd server each register their
/// own, since a `Box<dyn Branding>` cannot survive the re-exec. A second call is
/// ignored (mirrors [`crate::consts::DEBUG_MODE`]'s set-once semantics).
pub fn register_branding(branding: Box<dyn Branding>) {
    let _ = BRANDING.set(branding);
}

/// The registered host branding, or a neutral default when none was registered.
pub fn branding() -> &'static dyn Branding {
    static DEFAULT: DefaultBranding = DefaultBranding;
    BRANDING.get().map(|b| b.as_ref()).unwrap_or(&DEFAULT)
}

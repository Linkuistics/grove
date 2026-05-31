//! The controller-side render target: a ratatui `Backend` that draws *into a
//! proxy* over the socket (ADR-0016).
//!
//! The down direction of the seam is raw output bytes (`super::proto`), and
//! ratatui already knows how to render to any `io::Write` via `CrosstermBackend`
//! — its ANSI escape stream *is* the down-wire payload, and its cell-diffing
//! already minimises the bytes. So `ProxyBackend` is a thin wrapper around a
//! `CrosstermBackend<W>` writing to the socket, with two deliberate overrides:
//!
//! - **`size`/`window_size` report the *proxy's* size, not a local tty.** The
//!   controller is a persistent process with no usable terminal of its own;
//!   `CrosstermBackend::size()` would query the controller's tty (wrong, or an
//!   error). The proxy tells us its size over the seam, and that is what ratatui
//!   must lay out against. `set_size` updates it on a SIGWINCH-driven resize.
//! - **`get_cursor_position` never touches a tty.** `CrosstermBackend` answers
//!   it by writing a DSR query and reading the report back from *its* input —
//!   impossible over a one-way down socket (it would hang). We track the last
//!   position we were told to set and return that.
//!
//! Crucially the controller builds its `Terminal` with [`terminal`] / a plain
//! `Terminal::new`, **not** `ratatui::init()`: the *proxy* owns the real tty
//! (raw mode + alternate screen), so the controller must not also toggle them.

use std::io::{self, Write};

use ratatui::backend::{Backend, CrosstermBackend, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};
use ratatui::Terminal;

/// A ratatui backend that renders into a dumb proxy over the socket, sized to
/// that proxy rather than to any local terminal.
pub struct ProxyBackend<W: Write> {
    inner: CrosstermBackend<W>,
    size: Size,
    cursor: Position,
}

impl<W: Write> ProxyBackend<W> {
    /// Wrap a down-socket writer, sized to the proxy's reported `size`.
    pub fn new(writer: W, size: Size) -> Self {
        Self {
            inner: CrosstermBackend::new(writer),
            size,
            cursor: Position::ORIGIN,
        }
    }

    /// Update the layout size after the proxy reports a resize (SIGWINCH).
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }
}

/// Build a ratatui `Terminal` over a `ProxyBackend`. Uses `Terminal::new` — not
/// `ratatui::init()` — because the proxy, not the controller, owns the real tty
/// modes; the controller must not also enter raw mode / the alternate screen.
pub fn terminal<W: Write>(writer: W, size: Size) -> io::Result<Terminal<ProxyBackend<W>>> {
    Terminal::new(ProxyBackend::new(writer, size))
}

impl<W: Write> Backend for ProxyBackend<W> {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        self.inner.draw(content)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.inner.hide_cursor()
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.inner.show_cursor()
    }

    /// Tracked, never queried from a tty — see the module note.
    fn get_cursor_position(&mut self) -> io::Result<Position> {
        Ok(self.cursor)
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        let pos = position.into();
        self.cursor = pos;
        self.inner.set_cursor_position(pos)
    }

    fn clear(&mut self) -> io::Result<()> {
        self.inner.clear()
    }

    /// The proxy's size, not the controller's tty.
    fn size(&self) -> io::Result<Size> {
        Ok(self.size)
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        Ok(WindowSize {
            columns_rows: self.size,
            // Pixel size is only used by image protocols; we don't carry it
            // over the seam, and zero is the documented "unknown" value.
            pixels: Size::ZERO,
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        Backend::flush(&mut self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use ratatui::widgets::Paragraph;

    /// A `Write` that records everything into a shared buffer, so a test can
    /// render and then inspect the bytes that would have gone down the socket.
    #[derive(Clone)]
    struct SharedBuf(Rc<RefCell<Vec<u8>>>);

    impl Write for SharedBuf {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn reports_proxy_size_not_local_tty() {
        let backend = ProxyBackend::new(io::sink(), Size::new(120, 40));
        assert_eq!(backend.size().unwrap(), Size::new(120, 40));
    }

    #[test]
    fn set_size_updates_reported_size() {
        let mut backend = ProxyBackend::new(io::sink(), Size::new(80, 24));
        backend.set_size(Size::new(100, 30));
        assert_eq!(backend.size().unwrap(), Size::new(100, 30));
    }

    #[test]
    fn rendering_emits_ansi_to_the_writer() {
        let sink = Rc::new(RefCell::new(Vec::new()));
        let writer = SharedBuf(sink.clone());
        let mut term = terminal(writer, Size::new(20, 5)).unwrap();
        // The frame's area must match the proxy size we configured.
        let area = term.get_frame().area();
        assert_eq!((area.width, area.height), (20, 5));
        term.draw(|f| f.render_widget(Paragraph::new("hi"), f.area()))
            .unwrap();
        assert!(
            !sink.borrow().is_empty(),
            "rendering should have written ANSI bytes down the socket"
        );
    }

    #[test]
    fn cursor_position_is_tracked_without_a_tty() {
        // The whole point: this must NOT call crossterm::cursor::position(),
        // which would block/fail with no real terminal.
        let mut backend = ProxyBackend::new(io::sink(), Size::new(10, 10));
        backend.set_cursor_position(Position::new(3, 7)).unwrap();
        assert_eq!(backend.get_cursor_position().unwrap(), Position::new(3, 7));
    }
}

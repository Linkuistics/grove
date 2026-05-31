//! The controller↔proxy wire protocol (ADR-0016).
//!
//! The seam is one bidirectional unix-domain socket. Both directions are
//! tag-prefixed, length-delimited frames so the decoder reassembles cleanly
//! across partial socket reads:
//!
//! - **Down** ([`DownFrame`], controller→proxy): an [`DownFrame::Output`] frame
//!   carries the raw ANSI escape stream ratatui's `CrosstermBackend` produces
//!   (the hot path — the proxy blits its payload to stdout), and a rare
//!   [`DownFrame::RunEditor`] control frame tells the proxy to run the user's
//!   `$EDITOR` on its *own* tty over a shared-filesystem path (ADR-0017).
//! - **Up** ([`UpFrame`], proxy→controller): [`UpFrame::Resize`] and
//!   [`UpFrame::Input`] (the dumb proxy's size and forwarded keystrokes), plus
//!   [`UpFrame::EditorDone`] — the reply that a `RunEditor` child has exited.
//!
//! 010 left the down direction *unframed* ("proposed; refine in build"); the
//! `RunEditor` control verb is the build-time refinement that the interactive
//! `$EDITOR` drop needs (only the proxy owns a real tty), so the down direction
//! gains the same tiny tag+length framing the up direction already had.
//!
//! This is transport only — no `ratatui`, no `RepoView` (ADR-0013 seam). The
//! codec is a tiny hand-rolled thing; a handful of variants does not earn
//! `serde`.

use std::ffi::OsString;
use std::io::{self, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

/// A frame travelling *up* the socket, from the dumb proxy to the controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpFrame {
    /// The proxy's terminal size, sent on connect and on every SIGWINCH.
    Resize { cols: u16, rows: u16 },
    /// A run of raw stdin bytes the proxy forwarded up verbatim. The controller
    /// feeds these to its own input decoder (`super::decode`).
    Input(Vec<u8>),
    /// The reply to a [`DownFrame::RunEditor`]: the `$EDITOR` child the proxy
    /// ran on its tty has exited. `ok` is whether it exited successfully, so the
    /// controller can mirror the old `shell_editor`'s status check (ADR-0017).
    EditorDone { ok: bool },
}

const TAG_RESIZE: u8 = b'S';
const TAG_INPUT: u8 = b'I';
const TAG_EDITOR_DONE: u8 = b'D';

impl UpFrame {
    /// Serialise into the wire bytes. `Resize` is a fixed 5 bytes
    /// (`S` + cols:u16 + rows:u16, big-endian); `Input` is `I` + len:u32 + bytes;
    /// `EditorDone` is `D` + a single 0/1 ok byte.
    pub fn encode(&self) -> Vec<u8> {
        match self {
            UpFrame::Resize { cols, rows } => {
                let mut out = Vec::with_capacity(5);
                out.push(TAG_RESIZE);
                out.extend_from_slice(&cols.to_be_bytes());
                out.extend_from_slice(&rows.to_be_bytes());
                out
            }
            UpFrame::Input(bytes) => {
                let mut out = Vec::with_capacity(5 + bytes.len());
                out.push(TAG_INPUT);
                out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
                out.extend_from_slice(bytes);
                out
            }
            UpFrame::EditorDone { ok } => vec![TAG_EDITOR_DONE, u8::from(*ok)],
        }
    }
}

/// A streaming reassembler for the up direction. Socket reads arrive in
/// arbitrary chunks; `extend` appends them and `next` pops whole frames,
/// returning `None` while the buffer holds only a partial frame.
#[derive(Default)]
pub struct UpDecoder {
    buf: Vec<u8>,
}

impl UpDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append freshly-read socket bytes.
    pub fn extend(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    /// Pop the next complete frame, or `None` if the buffer does not yet hold
    /// one. A leading byte that is neither tag is a desync; we drop it and retry
    /// so a single corrupt byte cannot wedge the stream forever.
    pub fn next_frame(&mut self) -> Option<UpFrame> {
        loop {
            match self.buf.first().copied() {
                None => return None,
                Some(TAG_RESIZE) => {
                    if self.buf.len() < 5 {
                        return None;
                    }
                    let cols = u16::from_be_bytes([self.buf[1], self.buf[2]]);
                    let rows = u16::from_be_bytes([self.buf[3], self.buf[4]]);
                    self.buf.drain(..5);
                    return Some(UpFrame::Resize { cols, rows });
                }
                Some(TAG_INPUT) => {
                    if self.buf.len() < 5 {
                        return None;
                    }
                    let len =
                        u32::from_be_bytes([self.buf[1], self.buf[2], self.buf[3], self.buf[4]])
                            as usize;
                    if self.buf.len() < 5 + len {
                        return None;
                    }
                    let bytes = self.buf[5..5 + len].to_vec();
                    self.buf.drain(..5 + len);
                    return Some(UpFrame::Input(bytes));
                }
                Some(TAG_EDITOR_DONE) => {
                    if self.buf.len() < 2 {
                        return None;
                    }
                    let ok = self.buf[1] != 0;
                    self.buf.drain(..2);
                    return Some(UpFrame::EditorDone { ok });
                }
                Some(_) => {
                    // Unknown tag: drop one byte and resync.
                    self.buf.drain(..1);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Down direction (controller→proxy)

/// A frame travelling *down* the socket, from the controller to the dumb proxy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownFrame {
    /// Raw output bytes — the ANSI escape stream ratatui's `CrosstermBackend`
    /// produced. The proxy blits the payload straight to its stdout. This is the
    /// hot path; one frame per rendered draw (see [`FrameWriter`]).
    Output(Vec<u8>),
    /// Run the user's `$EDITOR` on the proxy's *own* tty against `path`, then
    /// reply [`UpFrame::EditorDone`] (ADR-0017). The path is a controller-created
    /// tempfile reachable by the proxy over the shared filesystem (local-proxy
    /// assumption); the proxy — not the controller — resolves `$EDITOR`, since
    /// that is the user's terminal-session environment.
    RunEditor { path: PathBuf },
}

const TAG_OUTPUT: u8 = b'O';
const TAG_RUN_EDITOR: u8 = b'E';

impl DownFrame {
    /// Serialise into the wire bytes: a tag, a `u32` big-endian length, then the
    /// payload (`Output`'s raw bytes, or `RunEditor`'s UTF-8 path).
    pub fn encode(&self) -> Vec<u8> {
        let (tag, payload): (u8, &[u8]) = match self {
            DownFrame::Output(bytes) => (TAG_OUTPUT, bytes),
            // Unix raw `OsStr` bytes — lossless over this unix-socket-only seam.
            DownFrame::RunEditor { path } => (TAG_RUN_EDITOR, path.as_os_str().as_bytes()),
        };
        let mut out = Vec::with_capacity(5 + payload.len());
        out.push(tag);
        out.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        out.extend_from_slice(payload);
        out
    }
}

/// A streaming reassembler for the down direction — the proxy's counterpart to
/// [`UpDecoder`]. Both frame kinds share the `tag + u32 len + payload` shape.
#[derive(Default)]
pub struct DownDecoder {
    buf: Vec<u8>,
}

impl DownDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append freshly-read socket bytes.
    pub fn extend(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    /// Pop the next complete frame, or `None` while the buffer holds only a
    /// partial one. A leading byte that is neither tag is a desync; drop it and
    /// retry so one corrupt byte cannot wedge the stream (mirrors [`UpDecoder`]).
    pub fn next_frame(&mut self) -> Option<DownFrame> {
        loop {
            let tag = match self.buf.first().copied() {
                None => return None,
                Some(t @ (TAG_OUTPUT | TAG_RUN_EDITOR)) => t,
                Some(_) => {
                    self.buf.drain(..1);
                    continue;
                }
            };
            if self.buf.len() < 5 {
                return None;
            }
            let len = u32::from_be_bytes([self.buf[1], self.buf[2], self.buf[3], self.buf[4]])
                as usize;
            if self.buf.len() < 5 + len {
                return None;
            }
            let payload = self.buf[5..5 + len].to_vec();
            self.buf.drain(..5 + len);
            return Some(match tag {
                TAG_OUTPUT => DownFrame::Output(payload),
                // Lossless inverse of `encode`'s unix `OsStr` bytes.
                _ => DownFrame::RunEditor {
                    path: PathBuf::from(OsString::from_vec(payload)),
                },
            });
        }
    }
}

/// An `io::Write` that frames everything written to it into a single
/// [`DownFrame::Output`] frame **per flush**, then forwards it to the inner
/// socket writer. ratatui writes a draw's escapes incrementally and calls
/// `flush` once at the end, so each rendered frame becomes exactly one `Output`
/// frame on the wire (and the proxy stays a pure blit). The controller wraps its
/// socket in this and hands it to `ProxyBackend` as the backend writer.
pub struct FrameWriter<W: Write> {
    inner: W,
    buf: Vec<u8>,
}

impl<W: Write> FrameWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            buf: Vec::new(),
        }
    }
}

impl<W: Write> Write for FrameWriter<W> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // An empty draw (no escapes) emits no frame — nothing to blit.
        if !self.buf.is_empty() {
            let frame = DownFrame::Output(std::mem::take(&mut self.buf)).encode();
            self.inner.write_all(&frame)?;
        }
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drain_all(dec: &mut UpDecoder) -> Vec<UpFrame> {
        let mut out = Vec::new();
        while let Some(f) = dec.next_frame() {
            out.push(f);
        }
        out
    }

    #[test]
    fn resize_round_trips() {
        let frame = UpFrame::Resize {
            cols: 120,
            rows: 40,
        };
        let mut dec = UpDecoder::new();
        dec.extend(&frame.encode());
        assert_eq!(dec.next_frame(), Some(frame));
        assert_eq!(dec.next_frame(), None);
    }

    #[test]
    fn input_round_trips() {
        let frame = UpFrame::Input(b"hello\x1b[A".to_vec());
        let mut dec = UpDecoder::new();
        dec.extend(&frame.encode());
        assert_eq!(dec.next_frame(), Some(frame));
        assert_eq!(dec.next_frame(), None);
    }

    #[test]
    fn empty_input_round_trips() {
        let frame = UpFrame::Input(Vec::new());
        let mut dec = UpDecoder::new();
        dec.extend(&frame.encode());
        assert_eq!(dec.next_frame(), Some(frame));
    }

    #[test]
    fn two_frames_back_to_back() {
        let a = UpFrame::Resize { cols: 80, rows: 24 };
        let b = UpFrame::Input(b"x".to_vec());
        let mut wire = a.encode();
        wire.extend(b.encode());
        let mut dec = UpDecoder::new();
        dec.extend(&wire);
        assert_eq!(drain_all(&mut dec), vec![a, b]);
    }

    #[test]
    fn split_read_reassembles_byte_by_byte() {
        let frame = UpFrame::Input(b"abc".to_vec());
        let wire = frame.encode();
        let mut dec = UpDecoder::new();
        // Feed every byte but the last: still incomplete.
        for byte in &wire[..wire.len() - 1] {
            dec.extend(&[*byte]);
            assert_eq!(dec.next_frame(), None, "frame completed too early");
        }
        dec.extend(&[*wire.last().unwrap()]);
        assert_eq!(dec.next_frame(), Some(frame));
    }

    #[test]
    fn partial_header_then_rest() {
        let frame = UpFrame::Resize { cols: 200, rows: 50 };
        let wire = frame.encode();
        let mut dec = UpDecoder::new();
        dec.extend(&wire[..3]);
        assert_eq!(dec.next_frame(), None);
        dec.extend(&wire[3..]);
        assert_eq!(dec.next_frame(), Some(frame));
    }

    #[test]
    fn unknown_leading_byte_is_resynced() {
        let frame = UpFrame::Resize { cols: 10, rows: 10 };
        let mut wire = vec![b'Z']; // garbage tag
        wire.extend(frame.encode());
        let mut dec = UpDecoder::new();
        dec.extend(&wire);
        assert_eq!(dec.next_frame(), Some(frame));
    }

    #[test]
    fn editor_done_round_trips_both_polarities() {
        for ok in [true, false] {
            let frame = UpFrame::EditorDone { ok };
            let mut dec = UpDecoder::new();
            dec.extend(&frame.encode());
            assert_eq!(dec.next_frame(), Some(frame));
            assert_eq!(dec.next_frame(), None);
        }
    }

    #[test]
    fn editor_done_interleaves_with_other_up_frames() {
        // A resize can arrive mid-edit (the user resized while the editor ran);
        // the decoder must keep both kinds straight back-to-back.
        let a = UpFrame::Resize { cols: 90, rows: 25 };
        let b = UpFrame::EditorDone { ok: true };
        let mut wire = a.encode();
        wire.extend(b.encode());
        let mut dec = UpDecoder::new();
        dec.extend(&wire);
        assert_eq!(drain_all(&mut dec), vec![a, b]);
    }

    // -- Down direction --------------------------------------------------

    fn drain_down(dec: &mut DownDecoder) -> Vec<DownFrame> {
        let mut out = Vec::new();
        while let Some(f) = dec.next_frame() {
            out.push(f);
        }
        out
    }

    #[test]
    fn output_frame_round_trips() {
        let frame = DownFrame::Output(b"\x1b[2J\x1b[Hhello".to_vec());
        let mut dec = DownDecoder::new();
        dec.extend(&frame.encode());
        assert_eq!(dec.next_frame(), Some(frame));
        assert_eq!(dec.next_frame(), None);
    }

    #[test]
    fn run_editor_frame_round_trips() {
        let frame = DownFrame::RunEditor {
            path: PathBuf::from("/tmp/grove-capture-abc123.md"),
        };
        let mut dec = DownDecoder::new();
        dec.extend(&frame.encode());
        assert_eq!(dec.next_frame(), Some(frame));
    }

    #[test]
    fn down_split_read_reassembles_byte_by_byte() {
        let frame = DownFrame::Output(b"abc".to_vec());
        let wire = frame.encode();
        let mut dec = DownDecoder::new();
        for byte in &wire[..wire.len() - 1] {
            dec.extend(&[*byte]);
            assert_eq!(dec.next_frame(), None, "frame completed too early");
        }
        dec.extend(&[*wire.last().unwrap()]);
        assert_eq!(dec.next_frame(), Some(frame));
    }

    #[test]
    fn down_two_frames_back_to_back() {
        let a = DownFrame::Output(b"x".to_vec());
        let b = DownFrame::RunEditor {
            path: PathBuf::from("/tmp/e.md"),
        };
        let mut wire = a.encode();
        wire.extend(b.encode());
        let mut dec = DownDecoder::new();
        dec.extend(&wire);
        assert_eq!(drain_down(&mut dec), vec![a, b]);
    }

    #[test]
    fn down_unknown_leading_byte_is_resynced() {
        let frame = DownFrame::Output(b"ok".to_vec());
        let mut wire = vec![0xFF]; // garbage tag
        wire.extend(frame.encode());
        let mut dec = DownDecoder::new();
        dec.extend(&wire);
        assert_eq!(dec.next_frame(), Some(frame));
    }

    // -- FrameWriter -----------------------------------------------------

    #[test]
    fn frame_writer_emits_one_output_frame_per_flush() {
        let mut fw = FrameWriter::new(Vec::new());
        // ratatui-style incremental writes, then a single flush per draw.
        fw.write_all(b"\x1b[H").unwrap();
        fw.write_all(b"frame-one").unwrap();
        fw.flush().unwrap();
        fw.write_all(b"frame-two").unwrap();
        fw.flush().unwrap();

        let mut dec = DownDecoder::new();
        dec.extend(&fw.inner);
        assert_eq!(
            drain_down(&mut dec),
            vec![
                DownFrame::Output(b"\x1b[Hframe-one".to_vec()),
                DownFrame::Output(b"frame-two".to_vec()),
            ]
        );
    }

    #[test]
    fn frame_writer_empty_flush_emits_nothing() {
        let mut fw = FrameWriter::new(Vec::new());
        fw.flush().unwrap();
        assert!(fw.inner.is_empty(), "an empty draw must put no frame on the wire");
    }
}

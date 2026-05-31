//! The controller↔proxy wire protocol (ADR-0016).
//!
//! The seam is one bidirectional unix-domain socket. The two directions are
//! deliberately asymmetric:
//!
//! - **Down** (controller→proxy) is *raw output bytes* — the ANSI escape stream
//!   ratatui's `CrosstermBackend` already produces. There is no framing: the
//!   proxy copies whatever arrives straight to its stdout. So this module models
//!   only the up direction.
//! - **Up** (proxy→controller) needs framing to tell a **resize** from a chunk
//!   of forwarded **input**. Frames are tag-prefixed and length-delimited so the
//!   decoder reassembles cleanly across partial socket reads.
//!
//! This is transport only — no `ratatui`, no `RepoView` (ADR-0013 seam). The
//! codec is a tiny hand-rolled thing; a 2-variant frame does not earn `serde`.

/// A frame travelling *up* the socket, from the dumb proxy to the controller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpFrame {
    /// The proxy's terminal size, sent on connect and on every SIGWINCH.
    Resize { cols: u16, rows: u16 },
    /// A run of raw stdin bytes the proxy forwarded up verbatim. The controller
    /// feeds these to its own input decoder (`super::decode`).
    Input(Vec<u8>),
}

const TAG_RESIZE: u8 = b'S';
const TAG_INPUT: u8 = b'I';

impl UpFrame {
    /// Serialise into the wire bytes. `Resize` is a fixed 5 bytes
    /// (`S` + cols:u16 + rows:u16, big-endian); `Input` is `I` + len:u32 + bytes.
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
                Some(_) => {
                    // Unknown tag: drop one byte and resync.
                    self.buf.drain(..1);
                }
            }
        }
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
}

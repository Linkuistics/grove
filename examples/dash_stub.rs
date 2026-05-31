//! A throwaway **stub controller** for exercising the dashboard seam (ADR-0016)
//! by hand. It is *not* the real dashboard loop (that is leaf 020) — it renders
//! a trivial frame and echoes the keys it decodes, just enough to prove the
//! controller↔proxy seam end to end against a real `grove __dash-proxy`.
//!
//! Usage (two terminals):
//!
//! ```text
//!   # terminal A — the stub controller (listens):
//!   cargo run --example dash_stub -- /tmp/grove-dash.sock
//!
//!   # terminal B — a real dumb proxy (connects, owns B's tty):
//!   cargo run -- __dash-proxy --socket /tmp/grove-dash.sock
//! ```
//!
//! Terminal B then shows the stub's rendered dashboard; keys typed in B are
//! forwarded up, decoded by the controller, and echoed back into the frame B
//! displays. Resize terminal B and the reported size updates. Ctrl-C the stub
//! (terminal A) to close the socket; the proxy restores B's terminal and exits.

use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

use grove::dash::decode::{InputDecoder, Key};
use grove::dash::proto::{FrameWriter, UpDecoder, UpFrame};

use ratatui::layout::Size;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

// The down direction is now framed (`DownFrame::Output` per draw), so the stub
// wraps its socket writer in a `FrameWriter` just like the real controller.
type Backend = grove::dash::backend::ProxyBackend<FrameWriter<UnixStream>>;

fn main() -> anyhow::Result<()> {
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp/grove-dash.sock"));

    // Fresh listener: a stale socket file would refuse to bind.
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path)?;
    eprintln!("stub controller listening on {}", path.display());
    eprintln!("connect a proxy:  grove __dash-proxy --socket {}", path.display());

    let (stream, _addr) = listener.accept()?;
    eprintln!("proxy connected");

    let mut reader = stream.try_clone()?;
    let writer = stream.try_clone()?;

    // Read up-frames until we learn the proxy's initial size.
    let mut frames = UpDecoder::new();
    let mut keys = InputDecoder::new();
    let mut rbuf = [0u8; 4096];

    let mut size = wait_for_size(&mut reader, &mut frames, &mut rbuf)?;
    let mut term = grove::dash::backend::terminal(FrameWriter::new(writer), size)?;

    let mut log: Vec<String> = Vec::new();
    draw(&mut term, size, &log)?;

    // Main loop: relay decoded input into the echoed frame; track resizes.
    loop {
        let n = reader.read(&mut rbuf)?;
        if n == 0 {
            break; // proxy closed.
        }
        frames.extend(&rbuf[..n]);
        let mut dirty = false;
        while let Some(frame) = frames.next_frame() {
            match frame {
                UpFrame::Resize { cols, rows } => {
                    size = Size::new(cols, rows);
                    term.backend_mut().set_size(size);
                    let _ = term.resize(ratatui::layout::Rect::new(0, 0, cols, rows));
                    dirty = true;
                }
                UpFrame::Input(bytes) => {
                    for key in keys.feed(&bytes) {
                        log.push(describe(&key));
                        if log.len() > 12 {
                            log.remove(0);
                        }
                        dirty = true;
                    }
                }
                // The stub never sends `RunEditor`, so this should not arrive;
                // log it if it does rather than crash the seam check.
                UpFrame::EditorDone { ok } => {
                    log.push(format!("EditorDone(ok={ok})"));
                    dirty = true;
                }
            }
        }
        if dirty {
            draw(&mut term, size, &log)?;
        }
    }

    eprintln!("proxy disconnected");
    Ok(())
}

/// Block reading frames until the first `Resize` tells us the proxy's size.
fn wait_for_size(
    reader: &mut UnixStream,
    frames: &mut UpDecoder,
    rbuf: &mut [u8],
) -> anyhow::Result<Size> {
    loop {
        let n = reader.read(rbuf)?;
        if n == 0 {
            anyhow::bail!("proxy closed before reporting its size");
        }
        frames.extend(&rbuf[..n]);
        while let Some(frame) = frames.next_frame() {
            if let UpFrame::Resize { cols, rows } = frame {
                return Ok(Size::new(cols, rows));
            }
        }
    }
}

fn draw(term: &mut Terminal<Backend>, size: Size, log: &[String]) -> anyhow::Result<()> {
    term.draw(|f| {
        let mut lines = vec![
            Line::from(Span::styled(
                " grove dashboard proxy — stub controller ",
                Style::default().add_modifier(Modifier::REVERSED),
            )),
            Line::from(""),
            Line::from(format!("proxy size: {} cols × {} rows", size.width, size.height)),
            Line::from("type keys in the proxy terminal — decoded controller-side below:"),
            Line::from(""),
        ];
        for entry in log {
            lines.push(Line::from(format!("  {entry}")));
        }
        let block = Block::default().borders(Borders::ALL).title("seam check");
        f.render_widget(Paragraph::new(lines).block(block), f.area());
    })?;
    Ok(())
}

fn describe(key: &Key) -> String {
    if key.mods.is_empty() {
        format!("{:?}", key.code)
    } else {
        format!("{:?} + {:?}", key.mods, key.code)
    }
}

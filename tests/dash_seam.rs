//! End-to-end test of the controller↔proxy seam (ADR-0016) over a real
//! `UnixStream` pair — no tty required.
//!
//! The unit tests cover the codec, decoder, and backend in isolation. This one
//! proves they compose across an actual socket: the controller renders a frame
//! whose ANSI bytes arrive at the proxy end (down), and a proxy's `Input` /
//! `Resize` frames travel back and decode into the right key and size (up).

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::time::Duration;

use grove::dash::decode::InputDecoder;
use grove::dash::proto::{UpDecoder, UpFrame};

use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Size;
use ratatui::widgets::Paragraph;

#[test]
fn rendered_frame_reaches_the_proxy_end() {
    let (controller_end, mut proxy_end) = UnixStream::pair().unwrap();
    proxy_end
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    // Controller side: render "HELLO" into a proxy-sized terminal over the wire.
    let mut term = grove::dash::backend::terminal(controller_end, Size::new(40, 10)).unwrap();
    // `Terminal::draw` flushes the backend (our socket writer) on completion.
    term.draw(|f| f.render_widget(Paragraph::new("HELLO"), f.area()))
        .unwrap();

    // Proxy side: the ANSI stream the proxy would blit must carry the text.
    let mut buf = [0u8; 8192];
    let n = proxy_end.read(&mut buf).unwrap();
    assert!(n > 0, "no bytes arrived at the proxy end");
    let stream = &buf[..n];
    assert!(
        stream.windows(5).any(|w| w == b"HELLO"),
        "rendered text did not reach the proxy end of the socket"
    );
}

#[test]
fn proxy_input_and_resize_decode_at_the_controller() {
    let (mut controller_end, mut proxy_end) = UnixStream::pair().unwrap();
    controller_end
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    // Proxy side: report a resize, then forward a Ctrl-Right arrow.
    proxy_end
        .write_all(&UpFrame::Resize { cols: 100, rows: 30 }.encode())
        .unwrap();
    proxy_end
        .write_all(&UpFrame::Input(b"\x1b[1;5C".to_vec()).encode())
        .unwrap();
    proxy_end.flush().unwrap();
    drop(proxy_end); // EOF so the controller read loop below terminates.

    // Controller side: reassemble frames and decode the input bytes to a key.
    let mut frames = UpDecoder::new();
    let mut keys = InputDecoder::new();
    let mut buf = [0u8; 4096];
    let mut got_resize = None;
    let mut decoded = Vec::new();
    loop {
        let n = controller_end.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        frames.extend(&buf[..n]);
        while let Some(frame) = frames.next_frame() {
            match frame {
                UpFrame::Resize { cols, rows } => got_resize = Some((cols, rows)),
                UpFrame::Input(bytes) => decoded.extend(keys.feed(&bytes)),
            }
        }
    }

    assert_eq!(got_resize, Some((100, 30)));
    assert_eq!(decoded.len(), 1);
    assert_eq!(decoded[0].code, KeyCode::Right);
    assert_eq!(decoded[0].mods, KeyModifiers::CONTROL);
}

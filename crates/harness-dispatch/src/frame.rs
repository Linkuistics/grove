//! The front's half of the private protocol channel: length-prefixed JSON.
//!
//! A frame is a four-byte big-endian length followed by that many bytes of
//! UTF-8 JSON, the same encoding `worker/src/channel.ts` writes. The channel is
//! a socket pair the front creates; the worker's end is its descriptor 3.

use std::io::{self, Read, Write};

use serde_json::Value;

/// The largest frame either side sends or accepts, matching the worker.
pub const MAX_FRAME_BYTES: usize = 1024 * 1024;

#[derive(Debug)]
pub enum FrameError {
    /// The peer closed the channel before a whole frame arrived.
    Closed,
    Io(io::Error),
    TooLarge(usize),
    NotJson(serde_json::Error),
}

impl std::fmt::Display for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameError::Closed => write!(f, "the channel closed before a whole frame arrived"),
            FrameError::Io(error) => write!(f, "{error}"),
            FrameError::TooLarge(length) => {
                write!(f, "a frame of {length} bytes exceeds {MAX_FRAME_BYTES}")
            }
            FrameError::NotJson(error) => write!(f, "a frame is not JSON: {error}"),
        }
    }
}

pub fn write_frame(mut writer: impl Write, message: &Value) -> Result<(), FrameError> {
    let body = serde_json::to_vec(message).map_err(FrameError::NotJson)?;
    if body.len() > MAX_FRAME_BYTES {
        return Err(FrameError::TooLarge(body.len()));
    }
    let length = u32::try_from(body.len()).map_err(|_| FrameError::TooLarge(body.len()))?;
    writer
        .write_all(&length.to_be_bytes())
        .and_then(|()| writer.write_all(&body))
        .map_err(FrameError::Io)
}

pub fn read_frame(mut reader: impl Read) -> Result<Value, FrameError> {
    let mut header = [0; 4];
    read_exact(&mut reader, &mut header)?;
    let length = u32::from_be_bytes(header) as usize;
    if length > MAX_FRAME_BYTES {
        return Err(FrameError::TooLarge(length));
    }
    let mut body = vec![0; length];
    read_exact(&mut reader, &mut body)?;
    serde_json::from_slice(&body).map_err(FrameError::NotJson)
}

fn read_exact(reader: &mut impl Read, buffer: &mut [u8]) -> Result<(), FrameError> {
    reader.read_exact(buffer).map_err(|error| {
        if error.kind() == io::ErrorKind::UnexpectedEof {
            FrameError::Closed
        } else {
            FrameError::Io(error)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_frame_round_trips_with_a_big_endian_length() {
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &json!({"type": "hello"})).unwrap();
        assert_eq!(&bytes[..4], &[0, 0, 0, 16]);
        assert_eq!(
            read_frame(bytes.as_slice()).unwrap(),
            json!({"type": "hello"})
        );
    }

    #[test]
    fn a_truncated_frame_reads_as_closed_and_an_oversized_one_is_refused() {
        assert!(matches!(
            read_frame([0, 0, 0, 9, b'{'].as_slice()),
            Err(FrameError::Closed)
        ));
        let oversized = u32::try_from(MAX_FRAME_BYTES + 1).unwrap().to_be_bytes();
        assert!(matches!(
            read_frame(oversized.as_slice()),
            Err(FrameError::TooLarge(_))
        ));
    }
}

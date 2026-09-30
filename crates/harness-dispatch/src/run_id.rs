//! Run identities (`docs/specs/harness-selection-and-execution.md`, *Identity
//! and original creator*).
//!
//! Every run gets a collision-resistant identity that needs no namespace: a
//! version-4 UUID, 122 random bits in the canonical lowercase
//! `8-4-4-4-12` form. The bits come from `/dev/urandom`, which macOS and every
//! Linux at the glibc 2.17 floor provide; glibc's `getrandom` wrapper arrived
//! only in 2.25. `inspect` shows one too, marked as proposed: it is never
//! recorded, and `run` allocates its own.

use std::fmt;
use std::fs::File;
use std::io::Read as _;

use crate::refusal::{Refusal, Stage, EXIT_MALFORMED, EXIT_RECORD};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunId(String);

impl RunId {
    /// A fresh identity. Failing to read the system's randomness refuses with
    /// exit 4: without an identity there is no run to record.
    pub fn allocate() -> Result<RunId, Refusal> {
        let mut bytes = [0_u8; 16];
        File::open("/dev/urandom")
            .and_then(|mut source| source.read_exact(&mut bytes))
            .map_err(|error| {
                Refusal::new(
                    "run_id_unavailable",
                    Stage::Record,
                    EXIT_RECORD,
                    format!("no run ID could be allocated: /dev/urandom cannot be read: {error}"),
                    "run harness-dispatch where /dev/urandom is readable; a run is never \
                     launched without its identity",
                )
                .source("/dev/urandom")
            })?;
        Ok(RunId::from_bytes(bytes))
    }

    fn from_bytes(mut bytes: [u8; 16]) -> RunId {
        bytes[6] = (bytes[6] & 0x0f) | 0x40; // version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80; // RFC 4122 variant
        let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        RunId(format!(
            "{}-{}-{}-{}-{}",
            &hex[0..8],
            &hex[8..12],
            &hex[12..16],
            &hex[16..20],
            &hex[20..32]
        ))
    }

    /// A caller's `--run` value, which must be a run ID in the canonical form
    /// this command writes. Nothing is normalized: an uppercase or braced
    /// spelling is refused rather than read as the ID it resembles.
    pub fn parse(given: &str, input: &str) -> Result<RunId, Refusal> {
        if is_canonical(given) {
            return Ok(RunId(given.to_owned()));
        }
        Err(Refusal::new(
            "malformed_input",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("{input} {given:?} is not a run ID"),
            "pass a run ID exactly as harness-dispatch reported it: 36 characters of lowercase \
             hexadecimal digits and hyphens, such as the harness's HARNESS_DISPATCH_RUN_ID",
        )
        .input(input))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RunId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Whether `given` has the canonical run ID shape: 36 characters of lowercase
/// hexadecimal digits with hyphens at 8, 13, 18 and 23.
pub fn is_canonical(given: &str) -> bool {
    given.len() == 36
        && given.bytes().enumerate().all(|(at, byte)| match at {
            8 | 13 | 18 | 23 => byte == b'-',
            _ => byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_allocated_id_is_a_canonical_version_4_uuid() {
        let id = RunId::allocate().unwrap();
        assert!(is_canonical(id.as_str()), "{id}");
        assert_eq!(&id.as_str()[14..15], "4", "{id}");
        assert!("89ab".contains(&id.as_str()[19..20]), "{id}");
        assert_ne!(id, RunId::allocate().unwrap());
    }

    #[test]
    fn the_version_and_variant_bits_are_forced() {
        let id = RunId::from_bytes([0xff; 16]);
        assert_eq!(id.as_str(), "ffffffff-ffff-4fff-bfff-ffffffffffff");
        let id = RunId::from_bytes([0; 16]);
        assert_eq!(id.as_str(), "00000000-0000-4000-8000-000000000000");
    }

    #[test]
    fn only_the_canonical_spelling_parses() {
        let good = "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e";
        assert_eq!(RunId::parse(good, "--run").unwrap().as_str(), good);
        for bad in [
            "",
            "0192F0C4-7A1E-4B2C-9D3E-4F5A6B7C8D9E",
            "{0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e}",
            "0192f0c47a1e4b2c9d3e4f5a6b7c8d9e",
            "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9",
            "0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9g",
            "0192f0c4_7a1e-4b2c-9d3e-4f5a6b7c8d9e",
        ] {
            let refusal = RunId::parse(bad, "--run").unwrap_err();
            assert_eq!(refusal.code, "malformed_input", "{bad:?}");
            assert_eq!(refusal.exit, EXIT_MALFORMED, "{bad:?}");
        }
    }
}

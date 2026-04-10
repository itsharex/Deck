use thiserror::Error;

/// Magic bytes: 0xDE 0xCC ("DeCc" → DeckClip)
pub const MAGIC: [u8; 2] = [0xDE, 0xCC];

/// Header size: 2 (magic) + 4 (length) = 6 bytes
const HEADER_SIZE: usize = 6;

/// Maximum payload size: 16 MB
const MAX_PAYLOAD_SIZE: u32 = 16 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("invalid magic bytes")]
    InvalidMagic,
    #[error("payload too large: {0} bytes (max {MAX_PAYLOAD_SIZE})")]
    PayloadTooLarge(u32),
    #[error("incomplete frame: need {need} bytes, got {got}")]
    Incomplete { need: usize, got: usize },
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Encode a JSON-serializable value into a framed message.
///
/// Frame format:
/// ```text
/// ┌──────────────┬──────────────┬────────────────────┐
/// │ Magic (2B)   │ Length (4B)  │  JSON Payload       │
/// │ 0xDE 0xCC    │ big-endian   │  UTF-8 encoded      │
/// └──────────────┴──────────────┴────────────────────┘
/// ```
pub fn encode_frame<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, CodecError> {
    let payload = serde_json::to_vec(value)?;
    let len = payload.len() as u32;
    if len > MAX_PAYLOAD_SIZE {
        return Err(CodecError::PayloadTooLarge(len));
    }

    let mut buf = Vec::with_capacity(HEADER_SIZE + payload.len());
    buf.extend_from_slice(&MAGIC);
    buf.extend_from_slice(&len.to_be_bytes());
    buf.extend_from_slice(&payload);
    Ok(buf)
}

/// Attempt to decode a frame from a byte buffer.
///
/// Returns `Ok((value, consumed))` on success, where `consumed` is the
/// total number of bytes consumed from the buffer.
///
/// Returns `Err(CodecError::Incomplete { .. })` if the buffer doesn't
/// contain a complete frame yet.
pub fn decode_frame<T: serde::de::DeserializeOwned>(buf: &[u8]) -> Result<(T, usize), CodecError> {
    if buf.len() < HEADER_SIZE {
        return Err(CodecError::Incomplete {
            need: HEADER_SIZE,
            got: buf.len(),
        });
    }

    if buf[0..2] != MAGIC {
        return Err(CodecError::InvalidMagic);
    }

    let len = u32::from_be_bytes([buf[2], buf[3], buf[4], buf[5]]);
    if len > MAX_PAYLOAD_SIZE {
        return Err(CodecError::PayloadTooLarge(len));
    }

    let total = HEADER_SIZE + len as usize;
    if buf.len() < total {
        return Err(CodecError::Incomplete {
            need: total,
            got: buf.len(),
        });
    }

    let value = serde_json::from_slice(&buf[HEADER_SIZE..total])?;
    Ok((value, total))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestMsg {
        hello: String,
    }

    #[test]
    fn roundtrip() {
        let msg = TestMsg {
            hello: "world".into(),
        };
        let encoded = encode_frame(&msg).unwrap();
        assert_eq!(&encoded[0..2], &MAGIC);

        let (decoded, consumed): (TestMsg, _) = decode_frame(&encoded).unwrap();
        assert_eq!(decoded, msg);
        assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn incomplete() {
        let msg = TestMsg {
            hello: "world".into(),
        };
        let encoded = encode_frame(&msg).unwrap();
        let partial = &encoded[..encoded.len() - 1];
        match decode_frame::<TestMsg>(partial) {
            Err(CodecError::Incomplete { .. }) => {}
            other => panic!("expected Incomplete, got {:?}", other),
        }
    }

    #[test]
    fn invalid_magic() {
        let buf = [0x00, 0x00, 0x00, 0x00, 0x00, 0x02, b'{', b'}'];
        match decode_frame::<serde_json::Value>(&buf) {
            Err(CodecError::InvalidMagic) => {}
            other => panic!("expected InvalidMagic, got {:?}", other),
        }
    }
}

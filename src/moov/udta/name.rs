use crate::*;

/// A track title in `trak/udta/name`, as written by ffmpeg.
///
/// Unlike the null-terminated `ilst/name`, this payload is plain UTF-8 bytes
/// with no full-box header, length, language or terminator. Keep it opaque on
/// decode so applications ignoring source metadata need not validate it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TrackName {
    pub value: Vec<u8>,
}

impl Atom for TrackName {
    const KIND: FourCC = FourCC::new(b"name");

    fn decode_body<B: Buf>(buf: &mut B) -> Result<Self> {
        Ok(Self {
            value: Vec::decode(buf)?,
        })
    }

    fn encode_body<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.value.encode(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn track_name_is_raw_utf8_without_terminator() {
        for label in ["", "Music", "Music: 音楽"] {
            let name = TrackName {
                value: label.as_bytes().to_vec(),
            };
            let mut bytes = Vec::new();
            name.encode(&mut bytes).unwrap();
            let mut expected = Vec::new();
            expected.extend_from_slice(&((8 + label.len()) as u32).to_be_bytes());
            expected.extend_from_slice(b"name");
            expected.extend_from_slice(label.as_bytes());
            assert_eq!(bytes, expected);
            assert_eq!(TrackName::decode(&mut bytes.as_slice()).unwrap(), name);
        }
    }

    #[test]
    fn track_name_decode_preserves_opaque_source_bytes() {
        let name = TrackName {
            value: vec![0xff, 0, 0xfe],
        };
        let mut bytes = Vec::new();
        name.encode(&mut bytes).unwrap();
        assert_eq!(TrackName::decode(&mut bytes.as_slice()).unwrap(), name);
    }
}

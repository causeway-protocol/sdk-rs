//! Derivation-path canonicalization (spec §6.5).
//!
//! Mirrored byte-for-byte from `causeway::derivation_path` so the SDK
//! and the on-chain program produce identical hashes for identical
//! inputs. The fixtures under
//! `programs/causeway/tests/fixtures/payload_hash_vectors.json` are
//! the cross-implementation source of truth.

use sha2::{Digest, Sha256};
use thiserror::Error;

/// Maximum length of the canonical encoding (1 byte count + 4 segments
/// of 1+32 each = 133).
pub const DERIVATION_PATH_MAX_LEN: usize = 133;

/// Hard caps on a single segment.
pub const DERIVATION_PATH_MAX_SEGMENTS: usize = 4;
pub const DERIVATION_PATH_MAX_SEGMENT_LEN: usize = 32;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PathError {
    #[error("derivation path exceeds the M1 segment count limit (max 4)")]
    TooLong,
    #[error("derivation path segment exceeds 32 bytes")]
    SegmentTooLong,
}

/// Output of `canonicalize_path`. Carries the active byte length so
/// downstream consumers don't accidentally hash the (zero) padding.
#[derive(Debug, Clone, Copy)]
pub struct Canonical {
    pub bytes: [u8; DERIVATION_PATH_MAX_LEN],
    pub len: u8,
}

impl Canonical {
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }
}

/// Encode a derivation path into the canonical form:
/// `len_u8 || (seg_len_u8 || seg_bytes)*`. Zero-padded to 133 bytes
/// for fixed-size storage; only the first `len` bytes are
/// semantically meaningful.
pub fn canonicalize_path(segments: &[&[u8]]) -> Result<Canonical, PathError> {
    if segments.len() > DERIVATION_PATH_MAX_SEGMENTS {
        return Err(PathError::TooLong);
    }
    let mut out = [0u8; DERIVATION_PATH_MAX_LEN];
    let mut cursor = 0usize;
    out[cursor] = segments.len() as u8;
    cursor += 1;
    for seg in segments {
        if seg.len() > DERIVATION_PATH_MAX_SEGMENT_LEN {
            return Err(PathError::SegmentTooLong);
        }
        out[cursor] = seg.len() as u8;
        cursor += 1;
        out[cursor..cursor + seg.len()].copy_from_slice(seg);
        cursor += seg.len();
    }
    Ok(Canonical {
        bytes: out,
        len: cursor as u8,
    })
}

/// SHA-256 over the active bytes of the canonical encoding. This is
/// the value bound into PDA seeds and into `round_id` /
/// `payload_hash`. Hashing the padded blob would change with the
/// padding constant and is wrong per spec.
pub fn derivation_path_hash(canonical: &Canonical) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_slice());
    let out = hasher.finalize();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&out);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_path_canonicalizes_to_single_zero_byte() {
        let c = canonicalize_path(&[]).unwrap();
        assert_eq!(c.len, 1);
        assert_eq!(c.as_slice(), &[0u8]);
    }

    #[test]
    fn one_segment_path() {
        let c = canonicalize_path(&[b"alice"]).unwrap();
        assert_eq!(c.len, 1 + 1 + 5);
        assert_eq!(c.as_slice(), b"\x01\x05alice");
    }

    #[test]
    fn four_segments_max_length() {
        let segs: Vec<&[u8]> = vec![&[0xAAu8; 32], &[0xBBu8; 32], &[0xCCu8; 32], &[0xDDu8; 32]];
        let c = canonicalize_path(&segs).unwrap();
        assert_eq!(c.len as usize, DERIVATION_PATH_MAX_LEN);
    }

    #[test]
    fn five_segments_rejected() {
        let segs: Vec<&[u8]> = vec![&[0u8; 4]; 5];
        assert_eq!(canonicalize_path(&segs).unwrap_err(), PathError::TooLong);
    }

    #[test]
    fn segment_over_32_bytes_rejected() {
        let segs: &[&[u8]] = &[&[0u8; 33]];
        assert_eq!(
            canonicalize_path(segs).unwrap_err(),
            PathError::SegmentTooLong
        );
    }

    #[test]
    fn distinct_paths_hash_distinctly() {
        let a = derivation_path_hash(&canonicalize_path(&[b"alice"]).unwrap());
        let b = derivation_path_hash(&canonicalize_path(&[b"bob"]).unwrap());
        assert_ne!(a, b);
    }

    #[test]
    fn hash_uses_only_active_bytes_not_padding() {
        let c1 = canonicalize_path(&[b"a"]).unwrap();
        let mut c2 = canonicalize_path(&[b"a"]).unwrap();
        c2.bytes[10] = 0xFF;
        c2.bytes[100] = 0xFF;
        assert_eq!(derivation_path_hash(&c1), derivation_path_hash(&c2));
    }

    #[test]
    fn segment_order_matters_for_hash() {
        let a = derivation_path_hash(&canonicalize_path(&[b"x", b"y"]).unwrap());
        let b = derivation_path_hash(&canonicalize_path(&[b"y", b"x"]).unwrap());
        assert_ne!(a, b);
    }
}

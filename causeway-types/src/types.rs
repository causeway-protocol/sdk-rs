//! Off-chain enum mirror for `programs/causeway/src/state/types.rs`.
//!
//! Borsh-only. Derives match the on-chain anchor versions byte-for-byte
//! (verified in `tests/anchor_parity.rs`).

use borsh::{BorshDeserialize, BorshSerialize};

/// Asset identifier. Discriminant byte is used in PDA seeds and the canonical
/// derivation-path tweak (causeway protocol §6.2, §7.1).
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AssetId {
    Btc = 0,
    Eth = 1,
    ZecT = 2,
}

impl AssetId {
    /// Canonical single-byte representation used in PDA seeds and tweak inputs.
    pub fn as_byte(self) -> u8 {
        self as u8
    }
}

/// Vault lifecycle status. Transitions are enforced on-chain by
/// `pause_vault`, `unpause_vault`, `begin_rotation`, `finalize_rotation`.
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum VaultStatus {
    Active = 0,
    Rotating = 1,
    Paused = 2,
}

/// Off-chain derivation mode (causeway protocol §6.3). BTC is locked to
/// `PathAStateless`; ECDSA assets pick A/B/C at the M1.1 gate.
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum DerivationMode {
    PathAStateless = 0,
    PathBPerPathRegistry = 1,
    PathCPerTenantRegistry = 2,
}

/// Why a vault is in `Paused` state. `RotationFinalized` is permanent;
/// the other two reasons can be lifted via `unpause_vault`.
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum PauseReason {
    EmergencyPause = 0,
    GovernanceHalt = 1,
    RotationFinalized = 2,
}

/// Lifecycle of a `SigningRequest` (causeway protocol §7.1).
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum RequestStatus {
    Pending = 0,
    Completed = 1,
    Failed = 2,
}

/// Per-asset sighash discriminator used in I8 compatibility checks.
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SighashKind {
    BtcTaprootKeySpend = 0,
    BtcSegwitV0 = 1,
    EthLegacy = 2,
    EthEip1559 = 3,
    ZecTransparentZip244 = 4,
}

/// Wire format of the final aggregated signature stored on-chain.
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SignatureFormat {
    Schnorr64 = 0,
    EcdsaDer = 1,
    EcdsaRecoverable65 = 2,
}

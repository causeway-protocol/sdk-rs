//! Byte-parity tests: every enum in `causeway-types` produces
//! byte-identical wire output vs an anchor-decorated mirror.
//!
//! Mirrors the Apr 25 spike-D harness (sandbox at
//! `causeway-research/sdk-spike/rust-extract/`) which proved this
//! extraction is mechanical.

use causeway_types::*;

mod anchor_ref {
    use anchor_lang::prelude::*;

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum AssetId { Btc = 0, Eth = 1, ZecT = 2 }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum VaultStatus { Active = 0, Rotating = 1, Paused = 2 }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum DerivationMode {
        PathAStateless = 0,
        PathBPerPathRegistry = 1,
        PathCPerTenantRegistry = 2,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum PauseReason {
        EmergencyPause = 0,
        GovernanceHalt = 1,
        RotationFinalized = 2,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum RequestStatus { Pending = 0, Completed = 1, Failed = 2 }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum SighashKind {
        BtcTaprootKeySpend = 0,
        BtcSegwitV0 = 1,
        EthLegacy = 2,
        EthEip1559 = 3,
        ZecTransparentZip244 = 4,
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
    #[repr(u8)]
    pub enum SignatureFormat {
        Schnorr64 = 0,
        EcdsaDer = 1,
        EcdsaRecoverable65 = 2,
    }
}

fn anchor_bytes<T: anchor_lang::AnchorSerialize>(v: &T) -> Vec<u8> {
    let mut out = Vec::new();
    v.serialize(&mut out).unwrap();
    out
}

fn borsh_bytes<T: borsh::BorshSerialize>(v: &T) -> Vec<u8> {
    let mut out = Vec::new();
    v.serialize(&mut out).unwrap();
    out
}

#[test]
fn asset_id_byte_parity() {
    for (a, b) in [
        (AssetId::Btc,  anchor_ref::AssetId::Btc),
        (AssetId::Eth,  anchor_ref::AssetId::Eth),
        (AssetId::ZecT, anchor_ref::AssetId::ZecT),
    ] {
        assert_eq!(borsh_bytes(&a), anchor_bytes(&b), "{a:?}");
    }
}

#[test]
fn vault_status_byte_parity() {
    for (a, b) in [
        (VaultStatus::Active,   anchor_ref::VaultStatus::Active),
        (VaultStatus::Rotating, anchor_ref::VaultStatus::Rotating),
        (VaultStatus::Paused,   anchor_ref::VaultStatus::Paused),
    ] {
        assert_eq!(borsh_bytes(&a), anchor_bytes(&b), "{a:?}");
    }
}

#[test]
fn derivation_mode_byte_parity() {
    for (a, b) in [
        (DerivationMode::PathAStateless,         anchor_ref::DerivationMode::PathAStateless),
        (DerivationMode::PathBPerPathRegistry,   anchor_ref::DerivationMode::PathBPerPathRegistry),
        (DerivationMode::PathCPerTenantRegistry, anchor_ref::DerivationMode::PathCPerTenantRegistry),
    ] {
        assert_eq!(borsh_bytes(&a), anchor_bytes(&b), "{a:?}");
    }
}

#[test]
fn pause_reason_byte_parity() {
    for (a, b) in [
        (PauseReason::EmergencyPause,    anchor_ref::PauseReason::EmergencyPause),
        (PauseReason::GovernanceHalt,    anchor_ref::PauseReason::GovernanceHalt),
        (PauseReason::RotationFinalized, anchor_ref::PauseReason::RotationFinalized),
    ] {
        assert_eq!(borsh_bytes(&a), anchor_bytes(&b), "{a:?}");
    }
}

#[test]
fn request_status_byte_parity() {
    for (a, b) in [
        (RequestStatus::Pending,   anchor_ref::RequestStatus::Pending),
        (RequestStatus::Completed, anchor_ref::RequestStatus::Completed),
        (RequestStatus::Failed,    anchor_ref::RequestStatus::Failed),
    ] {
        assert_eq!(borsh_bytes(&a), anchor_bytes(&b), "{a:?}");
    }
}

#[test]
fn sighash_kind_byte_parity() {
    for (a, b) in [
        (SighashKind::BtcTaprootKeySpend,    anchor_ref::SighashKind::BtcTaprootKeySpend),
        (SighashKind::BtcSegwitV0,           anchor_ref::SighashKind::BtcSegwitV0),
        (SighashKind::EthLegacy,             anchor_ref::SighashKind::EthLegacy),
        (SighashKind::EthEip1559,            anchor_ref::SighashKind::EthEip1559),
        (SighashKind::ZecTransparentZip244,  anchor_ref::SighashKind::ZecTransparentZip244),
    ] {
        assert_eq!(borsh_bytes(&a), anchor_bytes(&b), "{a:?}");
    }
}

#[test]
fn signature_format_byte_parity() {
    for (a, b) in [
        (SignatureFormat::Schnorr64,           anchor_ref::SignatureFormat::Schnorr64),
        (SignatureFormat::EcdsaDer,            anchor_ref::SignatureFormat::EcdsaDer),
        (SignatureFormat::EcdsaRecoverable65,  anchor_ref::SignatureFormat::EcdsaRecoverable65),
    ] {
        assert_eq!(borsh_bytes(&a), anchor_bytes(&b), "{a:?}");
    }
}

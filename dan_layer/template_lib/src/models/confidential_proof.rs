//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::mem::size_of;

use serde::{de::Error, Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

use crate::{
    crypto::{BalanceProofSignature, PedersonCommitmentBytes, RistrettoPublicKeyBytes, SchnorrSignatureBytes},
    models::Amount,
};

/// A statement for confidential and revealed outputs. A statement must contain either confidential outputs or non-zero
/// revealed funds or both.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "../../bindings/src/types/"))]
pub struct ConfidentialOutputStatement {
    /// Proof of the confidential resources that are going to be transferred to the receiver
    pub output_statement: Option<ConfidentialStatement>,
    /// Proof of the transaction change, which goes back to the sender's vault
    pub change_statement: Option<ConfidentialStatement>,
    /// Bulletproof range proof for the output and change commitments proving that values are in the range
    /// [minimum_value_promise, 2^64)
    pub range_proof: Vec<u8>,
    /// The amount of revealed funds to output
    pub output_revealed_amount: Amount,
    /// The amount of revealed funds to return to the sender
    pub change_revealed_amount: Amount,
}

impl ConfidentialOutputStatement {
    /// Creates an output proof for minting which only mints a revealed amount.
    pub fn mint_revealed<T: Into<Amount>>(amount: T) -> Self {
        Self {
            output_statement: None,
            change_statement: None,
            range_proof: vec![],
            output_revealed_amount: amount.into(),
            change_revealed_amount: Amount::zero(),
        }
    }
}

/// A zero-knowledge proof that a confidential resource amount is valid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "../../bindings/src/types/"))]
pub struct ConfidentialStatement {
    #[cfg_attr(feature = "ts", ts(type = "Array<number>"))]
    pub commitment: PedersonCommitmentBytes,
    /// Public nonce (R) that was used to generate the commitment mask
    #[cfg_attr(feature = "ts", ts(type = "Array<number>"))]
    pub sender_public_nonce: RistrettoPublicKeyBytes,
    /// Encrypted mask and value for the recipient.
    #[cfg_attr(feature = "ts", ts(type = "Array<number>"))]
    pub encrypted_data: EncryptedData,
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub minimum_value_promise: u64,
    /// If the view key is enabled for a given resource, this proof MUST be provided, otherwise it MUST NOT.
    pub viewable_balance_proof: Option<ViewableBalanceProof>,
}

/// ### Verifiable encryption
///
/// A verifiable ElGamal encryption proving system that asserts the value bound to a Pedersen
/// commitment matches the value encrypted to a given public key. This will be used to assert that the issuer can
/// decrypt account balances without knowing the opening to the account's balance commitment.
///
/// The proving relation is $\\{ (C, E, R, P); (v, m, r) | C = mG + vH, E = vG + rP, R = rG \\}$.
///
/// The prover samples $x_v, x_m, x_r$ uniformly at random.
/// It computes $C' = x_v H + x_m G$, $E' = x_v G + x_r P$, and $R' = x_r G$ and sends them to the verifier.
/// The verifier samples nonzero $e$ uniformly at random and sends it to the prover.
/// The prover computes $s_v = ev + x_v$, $s_m = em + x_m$, and $s_r = er + x_r$ and sends them to the verifier.
/// The verifier accepts the proof if and only if $eC + C' = s_v H + s_m G$, $eE + E' = s_v G + s_r P$, and $eR + R' =
/// s_r G$.
///
/// It is a sigma protocol for the relation that is complete, $2$-special sound, and special honest-verifier zero
/// knowledge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "../../bindings/src/types/"))]
pub struct ViewableBalanceProof {
    /// The encrypted value that takes the form: E = v.G + r.P
    /// where v is the value, G is the generator, r is the secret_nonce and P is the view key
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub elgamal_encrypted: RistrettoPublicKeyBytes,
    /// The public nonce used in the ElGamal encryption R = r.G
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub elgamal_public_nonce: RistrettoPublicKeyBytes,
    /// Part of the proof that the encrypted value is correctly constructed. C' = x_v.H + x_m.G
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub c_prime: RistrettoPublicKeyBytes,
    /// Part of the proof that the encrypted value is correctly constructed. E' = x_v.G + x_r.P
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub e_prime: RistrettoPublicKeyBytes,
    /// Part of the proof that the encrypted value is correctly constructed. R' = x_r.G
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub r_prime: RistrettoPublicKeyBytes,
    /// Part of the proof that the encrypted value is correctly constructed. s_v = x_v + e.v
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub s_v: SchnorrSignatureBytes,
    /// Part of the proof that the encrypted value is correctly constructed. s_m = x_m + e.m
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub s_m: SchnorrSignatureBytes,
    /// Part of the proof that the encrypted value is correctly constructed. s_r = x_r + e.r
    #[cfg_attr(feature = "ts", ts(type = "Uint8Array"))]
    pub s_r: SchnorrSignatureBytes,
}

impl ViewableBalanceProof {
    pub fn as_challenge_fields(&self) -> ViewableBalanceProofChallengeFields<'_> {
        ViewableBalanceProofChallengeFields {
            elgamal_encrypted: &self.elgamal_encrypted,
            elgamal_public_nonce: &self.elgamal_public_nonce,
            c_prime: &self.c_prime,
            e_prime: &self.e_prime,
            r_prime: &self.r_prime,
        }
    }
}

#[derive(Clone, Copy, Serialize)]
pub struct ViewableBalanceProofChallengeFields<'a> {
    pub elgamal_encrypted: &'a RistrettoPublicKeyBytes,
    pub elgamal_public_nonce: &'a RistrettoPublicKeyBytes,
    pub c_prime: &'a RistrettoPublicKeyBytes,
    pub e_prime: &'a RistrettoPublicKeyBytes,
    pub r_prime: &'a RistrettoPublicKeyBytes,
}

/// A zero-knowledge proof that a transfer of confidential resources is valid
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "../../bindings/src/types/"))]
pub struct ConfidentialWithdrawProof {
    #[cfg_attr(feature = "ts", ts(type = "Array<Uint8Array>"))]
    pub inputs: Vec<PedersonCommitmentBytes>,
    /// The amount to withdraw from revealed funds i.e. the revealed funds as inputs
    #[cfg_attr(feature = "ts", ts(type = "number"))]
    pub input_revealed_amount: Amount,
    pub output_proof: ConfidentialOutputStatement,
    /// Balance proof
    #[cfg_attr(feature = "ts", ts(type = "Array<number>"))]
    pub balance_proof: BalanceProofSignature,
}

impl ConfidentialWithdrawProof {
    /// Creates a withdrawal proof for revealed funds of a specific amount
    pub fn revealed_withdraw<T: Into<Amount>>(amount: T) -> Self {
        // There are no confidential inputs or outputs (this amounts to the same thing as a Fungible resource transfer)
        // So signature s = 0 + e.x where x is a 0 excess, is valid.
        let balance_proof = BalanceProofSignature::zero();

        let amount = amount.into();
        Self {
            inputs: vec![],
            input_revealed_amount: amount,
            output_proof: ConfidentialOutputStatement::mint_revealed(amount),
            balance_proof,
        }
    }

    pub fn revealed_to_confidential<T: Into<Amount>>(
        input_revealed_amount: T,
        output_proof: ConfidentialOutputStatement,
        balance_proof: BalanceProofSignature,
    ) -> Self {
        Self {
            inputs: vec![],
            input_revealed_amount: input_revealed_amount.into(),
            output_proof,
            balance_proof,
        }
    }

    /// Returns true if the withdraw proof is only transferring revealed funds, otherwise false
    /// The method for determining this is strict, as this can be used to determine whether to
    /// safely skip the balance proof check. To return true it requires:
    /// - Empty inputs
    /// - Output and Change outputs must be None
    /// - Empty range proof
    /// - Zero balance proof
    /// - Revealed funds > 0 in the inputs and outputs
    pub fn is_revealed_only(&self) -> bool {
        // Range proof must be empty
        self.output_proof.range_proof.is_empty() &&
        // Excess will be zero
        self.inputs.is_empty() &&
            self.output_proof.output_statement.is_none() &&
            self.output_proof.change_statement.is_none() &&
            // zero balance proof
            self.balance_proof == BalanceProofSignature::zero() &&
            // There are revealed funds
            self.input_revealed_amount > Amount::zero() &&
            self.output_proof.output_revealed_amount + self.output_proof.change_revealed_amount > Amount::zero()
    }

    pub fn revealed_input_amount(&self) -> Amount {
        self.input_revealed_amount
    }

    pub fn revealed_output_amount(&self) -> Amount {
        self.output_proof.output_revealed_amount
    }

    pub fn revealed_change_amount(&self) -> Amount {
        self.output_proof.change_revealed_amount
    }
}

/// Used by the receiver to determine the value component of the commitment, in both confidential transfers and Minotari
/// burns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncryptedData(Vec<u8>);

impl EncryptedData {
    pub const ENCRYPTED_DATA_SIZE_TOTAL: usize = Self::SIZE_NONCE + Self::SIZE_VALUE + Self::SIZE_MASK + Self::SIZE_TAG;
    pub const SIZE_MASK: usize = 32;
    pub const SIZE_NONCE: usize = 24;
    pub const SIZE_TAG: usize = 16;
    pub const SIZE_VALUE: usize = size_of::<u64>();

    pub const fn min_size() -> usize {
        Self::ENCRYPTED_DATA_SIZE_TOTAL
    }

    pub const fn max_size() -> usize {
        Self::min_size() + 256
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn tag_slice(&self) -> &[u8] {
        &self.0[..Self::SIZE_TAG]
    }

    pub fn nonce_slice(&self) -> &[u8] {
        &self.0[Self::SIZE_TAG..Self::SIZE_NONCE + Self::SIZE_TAG]
    }

    pub fn payload_slice(&self) -> &[u8] {
        &self.0[Self::payload_offset()..]
    }

    pub const fn payload_offset() -> usize {
        Self::SIZE_TAG + Self::SIZE_NONCE
    }
}

impl AsRef<[u8]> for EncryptedData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<Vec<u8>> for EncryptedData {
    type Error = usize;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < Self::min_size() {
            return Err(value.len());
        }
        if value.len() > Self::max_size() {
            return Err(value.len());
        }
        Ok(Self(value))
    }
}

impl Serialize for EncryptedData {
    fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serde_with::As::<serde_with::Bytes>::serialize(&self.0, __serializer)
    }
}

impl<'de> Deserialize<'de> for EncryptedData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        // TODO: implement a deserializer that only deserializes up to some MAX_BYTES
        serde_with::As::<serde_with::Bytes>::deserialize(deserializer).and_then(|v: Vec<u8>| {
            EncryptedData::try_from(v).map_err(|len| D::Error::custom(format!("EncryptedData invalid length {len}")))
        })
    }
}

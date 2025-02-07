use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Deps, Uint128};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Cosmos format (secp256k1 verification scheme).
    #[returns(VerifyResponse)]
    VerifyCosmosSignature {
        /// Message to verify.
        message: Binary,
        /// Serialized signature. Cosmos format (64 bytes).
        signature: Binary,
        /// Serialized compressed (33 bytes) or uncompressed (65 bytes) public key.
        public_key: Binary,
    },
    /// Cosmos format (secp256r1 verification scheme).
    #[returns(VerifyResponse)]
    VerifySecp256R1Signature {
        /// Message to verify.
        message: Binary,
        /// Serialized signature. Cosmos format (64 bytes).
        signature: Binary,
        /// Serialized compressed (33 bytes) or uncompressed (65 bytes) public key.
        public_key: Binary,
    },
    /// Ethereum text verification (compatible to the eth_sign RPC/web3 endpoint).
    /// This cannot be used to verify transaction.
    ///
    /// See https://web3js.readthedocs.io/en/v1.2.0/web3-eth.html#sign
    #[returns(VerifyResponse)]
    VerifyEthereumText {
        /// Message to verify. This will be wrapped in the standard container
        /// `"\x19Ethereum Signed Message:\n" + len(message) + message` before verification.
        message: String,
        /// Serialized signature. Fixed length format (64 bytes `r` and `s` plus the one byte `v`).
        signature: Binary,
        /// Signer address.
        /// This is matched case insensitive, so you can provide check-summed and non-check-summed addresses. Checksums are not validated.
        signer_address: String,
    },
    #[returns(VerifyResponse)]
    VerifyEthereumTransaction {
        /// Ethereum address in hex format (42 characters, starting with 0x)
        from: String,
        /// Ethereum address in hex format (42 characters, starting with 0x)
        to: String,
        nonce: u64,
        gas_limit: Uint128,
        gas_price: Uint128,
        value: Uint128,
        data: Binary,
        chain_id: u64,
        r: Binary,
        s: Binary,
        v: u64,
    },
    /// Tendermint format (ed25519 verification scheme).
    #[returns(VerifyResponse)]
    VerifyTendermintSignature {
        /// Message to verify.
        message: Binary,
        /// Serialized signature. Tendermint format (64 bytes).
        signature: Binary,
        /// Serialized public key. Tendermint format (32 bytes).
        public_key: Binary,
    },
    /// Tendermint format (batch ed25519 verification scheme).
    #[returns(VerifyResponse)]
    VerifyTendermintBatch {
        /// Messages to verify.
        messages: Vec<Binary>,
        /// Serialized signatures. Tendermint format (64 bytes).
        signatures: Vec<Binary>,
        /// Serialized public keys. Tendermint format (32 bytes).
        public_keys: Vec<Binary>,
    },
    /// Returns a list of supported verification schemes.
    /// No pagination - this is a short list.
    #[returns(ListVerificationsResponse)]
    ListVerificationSchemes {},
}

#[cw_serde]
pub struct VerifyResponse {
    pub verifies: bool,
}

#[cw_serde]
pub struct ListVerificationsResponse {
    pub verification_schemes: Vec<String>,
}

pub(crate) fn list_verifications(_deps: Deps) -> Vec<String> {
    vec![
        "secp256k1".into(),
        "secp256r1".into(),
        "ed25519".into(),
        "ed25519_batch".into(),
    ]
}

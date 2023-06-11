use crate::abi::{encode_packed, EncodePackedError, Tokenizable};
use crate::types::{Address, Bytes, U256};
use crate::utils::keccak256;
use ethabi::ethereum_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UserOp {
    pub sender: Address,
    pub nonce: U256,
    #[serde(rename = "initCode")]
    pub init_code: Bytes,
    #[serde(rename = "callData")]
    pub call_data: Bytes,
    #[serde(rename = "callGasLimit")]
    pub call_gas_limit: U256,
    #[serde(rename = "verificationGasLimit")]
    pub verification_gas_limit: U256,
    #[serde(rename = "preVerificationGas")]
    pub pre_verificaiton_gas: U256,
    #[serde(rename = "maxFeePerGas")]
    pub max_fee_per_gas: U256,
    #[serde(rename = "maxPriorityFeePerGas")]
    pub max_priority_fee_per_gas: U256,
    #[serde(rename = "paymasterAndData")]
    pub paymaster_and_data: Bytes,
    pub signature: Bytes,
}

impl UserOp {
    pub fn get_user_op_hash(
        &self,
        entry_point: Address,
        chain_id: U256,
    ) -> Result<H256, EncodePackedError> {
        let packed = encode_packed(&[
            self.sender.into_token(),
            self.nonce.into_token(),
            keccak256(&self.init_code).into_token(),
            keccak256(&self.call_data).into_token(),
            self.call_gas_limit.into_token(),
            self.verification_gas_limit.into_token(),
            self.pre_verificaiton_gas.into_token(),
            self.max_fee_per_gas.into_token(),
            self.max_priority_fee_per_gas.into_token(),
            keccak256(&self.paymaster_and_data).into_token(),
        ])?;

        Ok(keccak256(encode_packed(&[
            keccak256(packed).into_token(),
            entry_point.into_token(),
            chain_id.into_token(),
        ])?)
        .into())
    }
}

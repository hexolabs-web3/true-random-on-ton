use serde::Serialize;
use serde::Deserialize;
use strum::{Display, EnumString};
use num_bigint::BigUint;
use std::str::FromStr;
use sha2::{Sha256, Sha512, Digest};
use hex;

#[derive(Debug, PartialEq, Display, EnumString, Deserialize, Serialize)]
pub enum ConvertInputValueType {
    #[strum(serialize = "Be")]
    #[serde(rename = "Be")]
    Be,
    #[strum(serialize = "Le")]
    #[serde(rename = "Le")]
    Le,
}

#[derive(Debug)]
pub struct IntStringInvalid;

#[derive(Debug)]
pub struct HexStringInvalid;

pub enum ConvertInputError {
    IntStringInvalid(IntStringInvalid),
    HexStringInvalid(HexStringInvalid)
}

#[derive(Debug, Deserialize)]
pub struct ConvertInputs {
    value     : String,
    value_type: ConvertInputValueType
}

#[derive(Serialize)]
pub struct ConvertOutput {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ShaInputs {
    value: String,
}

#[derive(Serialize)]
pub struct ShaOutput {
    pub value: String,
}

pub fn convert_to_hex_string(int_string: String, value_type: ConvertInputValueType) -> Result<String, IntStringInvalid> {
    match BigUint::from_str(&int_string) {
        Ok(biguint) => {
            match value_type {
                ConvertInputValueType::Be => Ok(hex::encode(&biguint.to_bytes_be())),
                ConvertInputValueType::Le => Ok(hex::encode(&biguint.to_bytes_le())),
            }
        },
        Err(_err) => Err(IntStringInvalid)
    }
}

pub fn convert_to_int_string(hex_string: String, value_type: ConvertInputValueType) -> Result<String, HexStringInvalid> {
    // log::debug!("Hex: {}, Type: {:?}", hex_string, value_type);
    match hex::decode(hex_string) {
        Ok(vec) => {
            match value_type {
                ConvertInputValueType::Be => Ok(BigUint::from_bytes_be(&vec).to_string()),
                ConvertInputValueType::Le => Ok(BigUint::from_bytes_le(&vec).to_string()),
            }
        },
        Err(_err) => Err(HexStringInvalid)
    }
}

pub fn sha256(hex_string: String) -> Result<String, HexStringInvalid> {
    match hex::decode(hex_string) {
        Ok(vec) => {
            let mut hasher = Sha256::new();
            hasher.update(vec);
            let result = hasher.finalize();
            Ok(hex::encode(result))
        },
        Err(_err) => Err(HexStringInvalid)
    }
}

pub fn sha512(hex_string: String) -> Result<String, HexStringInvalid> {
    match hex::decode(hex_string) {
        Ok(vec) => {
            let mut hasher = Sha512::new();
            hasher.update(vec);
            let result = hasher.finalize();
            Ok(hex::encode(result))
        },
        Err(_err) => Err(HexStringInvalid)
    }
}

// Api endpoints.
pub fn api_convert_to_hex(convert_inputs: ConvertInputs) -> Result<ConvertOutput, ConvertInputError> {
    match convert_to_hex_string(convert_inputs.value, convert_inputs.value_type) {
        Ok(hex_string) => Ok(ConvertOutput{ value: hex_string }),
        Err(_err) => Err(ConvertInputError::IntStringInvalid(IntStringInvalid))
    }
}

pub fn api_convert_to_int(convert_inputs: ConvertInputs) -> Result<ConvertOutput, ConvertInputError> {
    match convert_to_int_string(convert_inputs.value, convert_inputs.value_type) {
        Ok(int_string) => Ok(ConvertOutput{ value: int_string }),
        Err(_err) => Err(ConvertInputError::HexStringInvalid(HexStringInvalid))
    }
}

// Hex.
pub fn api_sha256(sha_inputs: ShaInputs) -> Result<ShaOutput, ConvertInputError> {
    match sha256(sha_inputs.value) {
        Ok(hex_string) => Ok(ShaOutput{ value: hex_string }),
        Err(_err) => Err(ConvertInputError::HexStringInvalid(HexStringInvalid))
    }
}

// Hex.
pub fn api_sha512(sha_inputs: ShaInputs) -> Result<ShaOutput, ConvertInputError> {
    match sha512(sha_inputs.value) {
        Ok(hex_string) => Ok(ShaOutput{ value: hex_string }),
        Err(_err) => Err(ConvertInputError::HexStringInvalid(HexStringInvalid))
    }
}

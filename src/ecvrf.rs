use serde::Serialize;
use serde::Deserialize;
use rand_core::{OsRng};
use vrf_r255::{PublicKey, SecretKey, Proof};

use crate::utils;

macro_rules! to_string {
    ($e:expr) => {
        format!("{}", ::hex::encode($e.to_bytes().as_ref()))
    };
}

macro_rules! from_string {
    (VRFPublicKey, $e:expr) => {{
        let v: &[u8] = &::hex::decode($e).unwrap();
        VRFPublicKey::try_from(v).unwrap()
    }};
    ($t:ty, $e:expr) => {
        <$t>::try_from(::hex::decode($e).unwrap().as_ref()).unwrap()
    };
}

#[derive(Debug)]
pub struct AlphaInvalid;

#[derive(Debug)]
pub struct SKInvalid;

#[derive(Debug)]
pub struct PKInvalid;

#[derive(Debug)]
pub struct PiInvalid;

#[derive(Debug)]
pub struct VRFVerifyFailed;

pub enum VRFInputError {
    AlphaInvalid(AlphaInvalid),
    SKInvalid(SKInvalid),
}

pub enum VRFVerifyInputError {
    AlphaInvalid(AlphaInvalid),
    PKInvalid(PKInvalid),
    PiInvalid(PiInvalid),
    VRFVerifyFailed(VRFVerifyFailed),
}

#[derive(Deserialize)]
pub struct VRFInputs {
    sk   : String,
    alpha: String
}

#[derive(Serialize)]
pub struct VRFOutput {
    pub Gamma: String,
    pub c    : String,
    pub s    : String,
}

#[derive(Serialize)]
pub struct SKOutput {
    pub sk: String,
}

#[derive(Deserialize)]
pub struct PKInputs {
    sk: String
}

#[derive(Serialize)]
pub struct PKOutput {
    pub pk: String,
}

#[derive(Deserialize)]
pub struct VRFVerifyInputs {
    pk   : String,
    alpha: String,
    Gamma: String,
    c    : String,
    s    : String,
}

#[derive(Serialize)]
pub struct VRFVerifyOutput {
    pub beta: String
}

fn decode_sk(sk_string: String) -> Result<SecretKey, SKInvalid> {
    match hex::decode(sk_string) {
        Ok(sk_vec) => {
            match sk_vec.try_into() {
                Ok(sk_bytes) => {
                    let sk = SecretKey::from_bytes(sk_bytes);
                    if sk.is_some().into() {
                        return Ok(sk.unwrap())
                    } else {
                        return Err(SKInvalid)
                    }
                },
                Err(_err) => Err(SKInvalid)
            }
        },
        Err(_err) => Err(SKInvalid)
    }
}

fn decode_pk(pk_string: String) -> Result<PublicKey, PKInvalid> {
    match hex::decode(pk_string) {
        Ok(pk_vec) => {
            match pk_vec.try_into() {
                Ok(pk_bytes) => {
                    let pk = PublicKey::from_bytes(pk_bytes);
                    if pk.is_some().into() {
                        return Ok(pk.unwrap())
                    } else {
                        return Err(PKInvalid)
                    }
                },
                Err(_err) => Err(PKInvalid)
            }
        },
        Err(_err) => Err(PKInvalid)
    }
}

fn decode_pi(pi_string: String) -> Result<Proof, PiInvalid> {
    match hex::decode(pi_string) {
        Ok(pi_vec) => {
            match pi_vec.try_into() {
                Ok(pi_bytes) => {
                    let pi = Proof::from_bytes(pi_bytes);
                    if pi.is_some().into() {
                        return Ok(pi.unwrap())
                    } else {
                        return Err(PiInvalid)
                    }
                },
                Err(_err) => Err(PiInvalid)
            }
        },
        Err(_err) => Err(PiInvalid)
    }
}

pub fn api_vrf_prove(vrf_inputs: VRFInputs) -> Result<VRFOutput, VRFInputError> {
    match decode_sk(vrf_inputs.sk) {
        Ok(sk) => {
            match hex::decode(vrf_inputs.alpha) {
                Ok(alpha) => {
                    let pi: Proof = sk.prove(&alpha);
                    let pi_string: String = to_string!(pi);

                    Ok(VRFOutput {
                        Gamma: pi_string[..64].to_string(),
                        c    : pi_string[64..96].to_string(),
                        s    : pi_string[96..].to_string(),
                    })
                },
                Err(_err) => Err(VRFInputError::AlphaInvalid(AlphaInvalid))
            }
        },
        Err(_err) => Err(VRFInputError::SKInvalid(SKInvalid))
    }
}

pub fn generate_sk() -> String {
    let sk: SecretKey = SecretKey::generate(OsRng);
    return to_string!(sk);
}

pub fn api_generate_sk() -> SKOutput {
    return SKOutput{ sk: generate_sk() };
}

pub fn get_pk(sk: SecretKey) -> String {
    let pk: PublicKey = PublicKey::from(sk);
    return to_string!(pk);
}

pub fn api_get_pk(pk_inputs: PKInputs) -> Result<PKOutput, VRFInputError> {
    match decode_sk(pk_inputs.sk) { // -> vrf_r255::SecretKey
        Ok(sk) => Ok(PKOutput{ pk: get_pk(sk) }),
        Err(_err) => Err(VRFInputError::SKInvalid(SKInvalid))
    }
}

// Hex.
pub fn api_vrf_verify(vrf_verify_inputs: VRFVerifyInputs) -> Result<VRFVerifyOutput, VRFVerifyInputError> {
    match decode_pk(vrf_verify_inputs.pk) { // -> vrf_r255::PublicKey
        Ok(pk) => {
            match hex::decode(vrf_verify_inputs.alpha) {
                Ok(alpha) => {
                    let pi_string: String = vrf_verify_inputs.Gamma + &vrf_verify_inputs.c + &vrf_verify_inputs.s;

                    match decode_pi(pi_string) { // -> vrf_r255::Proof
                        Ok(pi) => {
                            let beta = pk.verify(&alpha, &pi);
                            if beta.is_some().into() {
                                return Ok(VRFVerifyOutput{ beta: hex::encode(beta.unwrap()) })
                            } else {
                                return Err(VRFVerifyInputError::VRFVerifyFailed(VRFVerifyFailed))
                            }
                        },
                        Err(_err) => Err(VRFVerifyInputError::PiInvalid(PiInvalid))
                    }
                },
                Err(_err) => Err(VRFVerifyInputError::AlphaInvalid(AlphaInvalid))
            }
        },
        Err(_err) => Err(VRFVerifyInputError::PKInvalid(PKInvalid))
    }
}

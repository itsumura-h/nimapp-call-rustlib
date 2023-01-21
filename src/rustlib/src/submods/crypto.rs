use hex::decode as hex_decode;
use hex::encode as hex_encode;
use p256::ecdsa::signature::{Signer, Verifier};
use p256::ecdsa::{Signature, SigningKey, VerifyingKey};
use p256::pkcs8::DecodePublicKey;
use rand_core::OsRng;
use std::ffi::c_char;

use crate::submods::c_ffi::{cstirng_to_string, string_to_cstring};

#[no_mangle]
pub extern "C" fn create_secret_key() -> *mut Vec<u8> {
    let secret_key = SigningKey::random(&mut OsRng);
    let v = secret_key.to_bytes().to_vec();
    Box::into_raw(Box::new(v))
}

// #[no_mangle]
// pub extern "C" fn hex_key_to_vec(ptr: &mut c_char) -> *const Vec<u8> {
//     let key = cstirng_to_string(ptr);
//     let vector = hex_decode(key).unwrap();
//     Box::into_raw(Box::new(vector))
// }

#[no_mangle]
pub extern "C" fn get_secret_key_len(v: &mut Vec<u8>) -> usize {
    v.len()
}

#[no_mangle]
pub extern "C" fn get_secret_key_item(v: &mut Vec<u8>, offset: usize) -> u8 {
    v[offset]
}

#[no_mangle]
pub extern "C" fn create_secret_key_hex() -> *mut c_char {
    let secret_key = SigningKey::random(&mut OsRng);
    let bytes = secret_key.to_bytes();
    let slices = bytes.as_slice();
    let hex_str = hex_encode(&slices);
    string_to_cstring(hex_str)
}

#[no_mangle]
pub extern "C" fn create_verifying_key(_secret_key: &mut c_char) -> *mut c_char {
    let str_secret_key = cstirng_to_string(_secret_key);
    let b_key = &(hex_decode(str_secret_key).unwrap());
    let signing_key = SigningKey::from_bytes(b_key).unwrap();
    let verifying_key = signing_key.verifying_key();
    let encoded_point = verifying_key.to_encoded_point(true);
    let str_signature = encoded_point.to_string();
    string_to_cstring(str_signature)
}

#[no_mangle]
pub extern "C" fn sign_message(_secret_key: &mut c_char, _msg: &mut c_char) -> *mut c_char {
    let str_secret_key = cstirng_to_string(_secret_key);
    let b_key = &(hex_decode(str_secret_key).unwrap());
    let signing_key = SigningKey::from_bytes(b_key).unwrap();

    let msg = cstirng_to_string(_msg);
    let b_msg = msg.as_bytes();

    let verifying_key = signing_key.sign(b_msg);
    let str_signature = verifying_key.to_string().to_lowercase();
    string_to_cstring(str_signature)
}

#[no_mangle]
pub extern "C" fn verify_sign(
    _verifying_key: &mut c_char,
    _msg: &mut c_char,
    _signature: &mut c_char,
) -> bool {
    let str_verifying_key = cstirng_to_string(_verifying_key);
    let b_key = &(hex_decode(str_verifying_key).unwrap());
    let slice_b_key = b_key.as_slice();
    let verifying_key = match VerifyingKey::from_sec1_bytes(slice_b_key) {
        Ok(verifying_key) => verifying_key,
        Err(_e) => return false,
    };

    let msg = cstirng_to_string(_msg);
    let b_msg = msg.as_bytes();

    let str_signature = cstirng_to_string(_signature);
    let vec_signature = hex_decode(str_signature).unwrap();
    let b_signature = vec_signature.as_slice();
    let signature = match Signature::try_from(b_signature) {
        Ok(signature) => signature,
        Err(_e) => return false,
    };

    verifying_key.verify(b_msg, &signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = create_secret_key();
        println!("{:?}", res);
        let length = get_secret_key_len(&res);
        println!("length…{:?}", length);
        println!("0…{:?}", get_secret_key_item(&res, 0));
        println!("1…{:?}", get_secret_key_item(&res, 1));
    }

    #[test]
    fn test() {
        // Signing
        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        println!("{:?}", signing_key.to_bytes());
        let message =
            b"ECDSA proves knowledge of a secret number in the context of a single message";
        let signature = signing_key.sign(message);
        println!("{:?}", signature);

        // Verification
        use p256::ecdsa::{signature::Verifier, VerifyingKey};

        let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
        assert!(verifying_key.verify(message, &signature).is_ok());
    }
}

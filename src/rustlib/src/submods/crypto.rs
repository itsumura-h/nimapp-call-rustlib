use hex::decode as hex_decode;
use hex::encode as hex_encode;
use p256::ecdsa::signature::{Signer, Verifier};
use p256::ecdsa::{Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;
use std::ffi::c_char;

use crate::submods::c_ffi::{cstirng_to_string, string_to_cstring};

#[no_mangle]
pub extern "C" fn create_secret_key() -> *mut Vec<u8> {
    let secret_key = SigningKey::random(&mut OsRng);
    let v = secret_key.to_bytes().to_vec();
    Box::into_raw(Box::new(v))
}

#[no_mangle]
pub extern "C" fn hex_key_to_vec(ptr: &mut c_char) -> *mut Vec<u8> {
    let mut key = cstirng_to_string(ptr);
    key = key.split_off(2);
    let vector = hex_decode(key).unwrap();
    Box::into_raw(Box::new(vector))
}

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
    let hex_str = "0x".to_string() + &hex_encode(&slices);
    string_to_cstring(hex_str)
}

#[no_mangle]
pub extern "C" fn sign_message(_msg: &mut c_char, _secret_key: &mut c_char) -> *mut c_char {
    let msg = cstirng_to_string(_msg);
    let mut secret_key = cstirng_to_string(_secret_key);
    secret_key = secret_key.split_off(2);

    let b_msg = msg.as_bytes();
    let b_key = &(hex_decode(secret_key).unwrap());
    let typed_key = SigningKey::from_bytes(b_key).unwrap();

    let verifying_key = typed_key.sign(b_msg);

    let str_signature = "0x".to_string() + &verifying_key.to_string().to_lowercase();
    string_to_cstring(str_signature)
}

#[no_mangle]
pub extern "C" fn verify_sign(
    _secret_key: &mut c_char,
    _msg: &mut c_char,
    _signature: &mut c_char,
) -> bool {
    let mut secret_key = cstirng_to_string(_secret_key);
    secret_key = secret_key.split_off(2);
    let b_key = &(hex_decode(secret_key).unwrap());
    let signing_key = SigningKey::from_bytes(b_key).unwrap();

    let msg = cstirng_to_string(_msg);
    let b_msg = msg.as_bytes();

    let mut str_signature = cstirng_to_string(_signature);
    str_signature = str_signature.split_off(2);

    let vec_signature = hex_decode(str_signature).unwrap();
    let b_signature = vec_signature.as_slice();
    let signature = Signature::try_from(b_signature).unwrap();

    let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`

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

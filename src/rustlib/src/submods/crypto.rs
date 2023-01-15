use ::safer_ffi::prelude::*;
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use p256::U256;
use rand_core::OsRng;
use std::io::Bytes;
use std::slice;
use std::str::from_utf8; // requires 'getrandom' feature

#[no_mangle]
pub extern "C" fn create_secret_key() -> *mut Vec<u8> {
    let secret_key = SigningKey::random(&mut OsRng);
    let v = secret_key.to_bytes().to_vec();
    println!("{:#?}", v);
    Box::into_raw(Box::new(v))
}

#[no_mangle]
pub extern "C" fn get_length(v: &Vec<u8>) -> usize {
    v.len()
}

#[no_mangle]
pub extern "C" fn get_item_of_secret_key(v: &Vec<u8>, offset: usize) -> u8 {
    v[offset]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = create_secret_key();
        println!("{:#?}", res);
        let length = get_length(res);
        println!("{:#?}", length);
        println!("{:#?}", v[0]);
        println!("{:#?}", v[1]);
    }

    #[test]
    fn test() {
        // Signing
        let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        println!("{:?}", signing_key.to_bytes());
        // println!("{:?}", from_utf8(signing_key));
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

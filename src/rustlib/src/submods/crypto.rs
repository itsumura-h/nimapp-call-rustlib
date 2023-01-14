#[cfg(test)]
mod tests {
    use  std::str::from_utf8;
    use p256::ecdsa::{signature::Signer, Signature, SigningKey};
    use rand_core::OsRng; // requires 'getrandom' feature

    #[test]
    fn test1() {
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

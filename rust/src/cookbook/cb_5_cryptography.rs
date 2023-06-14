// sha256 implemented in cb_4_concurrency

// Sign and verify a message with HMAC digest
//
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{hmac, rand};

#[allow(dead_code)]
pub fn sign_message() -> Result<(), Unspecified> {
    // The sender generates a secure key value and signs the message with it.
    // Note that in a real protocol, a key agreement protocol would be used to
    // derive `key_value`.
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    // random 48-bit key value
    rng.fill(&mut key_value)?;
    // secure key
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";

    // calculate signature
    let signature = hmac::sign(&key, message.as_bytes());
    // The receiver (somehow! e.g. diffie hellmann) knows the key value, and uses it to verify the
    // integrity of the message.
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}

use data_encoding::HEXUPPER;
// use ring::error::Unspecified;
// use ring::rand::SecureRandom;
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

#[allow(dead_code)]
fn salt_hash_password() -> Result<(), Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let password = "Guess Me If You Can!";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );
    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_message() {
        sign_message().unwrap();
    }

    #[test]
    fn test_salt_hash_password() {
        salt_hash_password().unwrap()
    }
}

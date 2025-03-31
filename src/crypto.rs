use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, generic_array::GenericArray}};
use anyhow::{Result, Context};
use rand::Rng;

pub fn encrypt_key(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    
    let mut nonce = [0u8; 12];
    rand::thread_rng().try_fill(&mut nonce)?;
    let nonce = GenericArray::from_slice(&nonce);
    
    Ok(cipher.encrypt(nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?)
}

pub fn decrypt_key(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    
    let nonce = GenericArray::from_slice(&ciphertext[..12]);
    let ciphertext = &ciphertext[12..];
    
    Ok(cipher.decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?)
}
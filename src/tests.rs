#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::SecretKey;
    
    #[test]
    fn test_taproot_address_generation() {
        let wallet = Wallet::new(Network::Regtest);
        let (secret_key, tweaked_pubkey) = wallet.generate_taproot_keypair();
        let address = wallet.get_taproot_address(tweaked_pubkey);
        
        assert_eq!(address.address_type(), Some(AddressType::P2tr));
    }
    
    #[tokio::test]
    async fn test_key_encryption() {
        let key = [1u8; 32];
        let plaintext = b"test secret";
        
        let ciphertext = encrypt_key(&key, plaintext).unwrap();
        let decrypted = decrypt_key(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }
}
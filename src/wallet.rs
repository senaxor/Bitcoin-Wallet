use bitcoin::secp256k1::{Secp256k1, SecretKey, KeyPair};
use bitcoin::key::{TweakedPublicKey, TweakedKeyPair};
use bitcoin::Address;
use bitcoin::Network;

pub struct Wallet {
    secp: Secp256k1<bitcoin::secp256k1::All>,
    network: Network,
}

impl Wallet {
    pub fn new(network: Network) -> Self {
        Wallet {
            secp: Secp256k1::new(),
            network,
        }
    }

    pub fn generate_taproot_keypair(&self) -> (SecretKey, TweakedPublicKey) {
        let keypair = KeyPair::new(&self.secp, &mut rand::thread_rng());
        let tweaked = TweakedKeyPair::dangerous_assume_tweaked(keypair);
        (tweaked.to_inner().secret_key(), tweaked.into())
    }

    pub fn get_taproot_address(&self, tweaked_pubkey: TweakedPublicKey) -> Address {
        Address::p2tr_tweaked(tweaked_pubkey, self.network)
    }
}
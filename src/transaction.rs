use bitcoin::{
    absolute, Address, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness,
    secp256k1::{Secp256k1, SecretKey, KeyPair},
    key::TweakedPublicKey,
    sighash::{SighashCache, TapSighashType},
};

pub struct TransactionBuilder {
    secp: Secp256k1<bitcoin::secp256k1::All>,
    network: bitcoin::Network,
}

impl TransactionBuilder {
    pub fn new(network: bitcoin::Network) -> Self {
        TransactionBuilder {
            secp: Secp256k1::new(),
            network,
        }
    }

    pub fn create_transaction(
        &self,
        inputs: Vec<(OutPoint, ScriptBuf, Amount)>,
        outputs: Vec<(Address, Amount)>,
        change_address: &Address,
        fee: Amount,
    ) -> Transaction {
        let total_in: Amount = inputs.iter().map(|(_, _, amount)| *amount).sum();
        let total_out: Amount = outputs.iter().map(|(_, amount)| *amount).sum();
        let change = total_in.checked_sub(total_out + fee).unwrap_or(Amount::ZERO);

        let mut tx = Transaction {
            version: 2,
            lock_time: absolute::LockTime::ZERO,
            input: inputs
                .into_iter()
                .map(|(outpoint, _, _)| TxIn {
                    previous_output: outpoint,
                    script_sig: ScriptBuf::new(),
                    sequence: Sequence::MAX,
                    witness: Witness::new(),
                })
                .collect(),
            output: outputs
                .into_iter()
                .map(|(address, amount)| TxOut {
                    value: amount.to_sat(),
                    script_pubkey: address.script_pubkey(),
                })
                .collect(),
        };

        if change > Amount::ZERO {
            tx.output.push(TxOut {
                value: change.to_sat(),
                script_pubkey: change_address.script_pubkey(),
            });
        }

        tx
    }

    pub fn sign_taproot_transaction(
        &self,
        mut tx: Transaction,
        input_index: usize,
        secret_key: &SecretKey,
        tweaked_pubkey: &TweakedPublicKey,
    ) -> Transaction {
        let mut sighash_cache = SighashCache::new(&tx);
        let prevouts = vec![TxOut {
            value: 0, // This should be the actual UTXO value
            script_pubkey: ScriptBuf::new(),
        }];
        
        let sighash = sighash_cache
            .taproot_key_spend_signature_hash(
                input_index,
                &bitcoin::sighash::Prevouts::All(&prevouts),
                TapSighashType::Default,
            )
            .expect("Failed to compute sighash");

        let msg = bitcoin::secp256k1::Message::from(sighash);
        let keypair = KeyPair::from_secret_key(&self.secp, secret_key);
        let signature = self.secp.sign_schnorr(&msg, &keypair);

        let mut witness = Witness::new();
        witness.push(signature.as_ref());

        tx.input[input_index].witness = witness;
        tx
    }
}
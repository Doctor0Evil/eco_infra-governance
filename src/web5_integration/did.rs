#[derive(Clone, Debug)]
pub struct DidKeyPair {
    pub did: String,
    // opaque handles to ML-DSA / ML-KEM keys
}

impl DidKeyPair {
    pub fn sign_ml_dsa(&self, msg: &[u8]) -> Vec<u8> {
        // call into ML-DSA library
        unimplemented!()
    }
}

pub fn verify_ml_dsa(did: &str, msg: &[u8], sig: &[u8]) -> bool {
    // resolve DID Document, extract ML-DSA pubkey, verify
    unimplemented!()
}

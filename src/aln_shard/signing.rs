use super::AlnShard;
use crate::nanopolygon::NanopolygonSafetyObject;
use crate::web5_integration::did::DidKeyPair;

// ML-DSA keypair abstraction; impl uses your chosen PQC lib.
pub fn sign_shard(payload: NanopolygonSafetyObject, keypair: &DidKeyPair) -> AlnShard {
    let did = keypair.did.clone();
    let bytes = bincode::serialize(&payload).expect("serialize payload");
    let signature = keypair.sign_ml_dsa(&bytes);
    AlnShard { payload, did, signature }
}

pub fn verify_shard(shard: &AlnShard) -> bool {
    let bytes = bincode::serialize(&shard.payload).ok()?;
    crate::web5_integration::did::verify_ml_dsa(&shard.did, &bytes, &shard.signature)
}

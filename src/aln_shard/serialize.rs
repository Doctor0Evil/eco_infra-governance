use crate::nanopolygon::NanopolygonSafetyObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlnShard {
    pub payload: NanopolygonSafetyObject,
    pub did: String,
    pub signature: Vec<u8>, // ML-DSA signature bytes
}

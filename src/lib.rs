pub mod nanopolygon;
pub mod aln;
pub mod policy;
pub mod audit;
pub mod enclave;

// High-level invariant marker for the entire crate.
pub trait InvariantGovernance: Sized {}

impl InvariantGovernance for crate::nanopolygon::NanopolygonSafetyObject {}

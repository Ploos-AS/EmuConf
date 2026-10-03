use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fidelity {
    Exact,
    Mapped,
    Approximate,
    Unsupported,
    Preserved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub field: String,
    pub fidelity: Fidelity,
    pub message: String,
}

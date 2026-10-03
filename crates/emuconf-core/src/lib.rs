pub mod adapter;
pub mod api;
pub mod detect;
pub mod diagnostic;
pub mod ecim;
pub mod emulator;
pub mod report;

pub use adapter::{export, import, AdapterError};
pub use api::{convert, load, load_as, ApiError, LoadedConfig};
pub use detect::{detect_format, Detection};
pub use diagnostic::{Diagnostic, Fidelity};
pub use ecim::{Ecim, ECIM_VERSION};
pub use emulator::Emulator;
pub use report::{compatibility, semantic_diff, CompatibilityReport, FieldReport};

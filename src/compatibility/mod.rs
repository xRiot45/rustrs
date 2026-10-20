pub mod capabilities;
pub mod matrix;
pub mod resolver;
pub mod types;

pub use capabilities::validate_capabilities;
pub use matrix::evaluate;
pub use resolver::{check_compatibility, compatibility_status};
pub use types::{CompatibilityError, CompatibilityStatus};

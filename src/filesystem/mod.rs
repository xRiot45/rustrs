pub mod conflicts;
pub mod paths;
pub mod report;
pub mod writer;

pub use conflicts::{ConflictStatus, detect_conflict};
pub use paths::{PathError, ensure_within_root, validate_relative_path};
pub use report::preview_plan;
pub use writer::apply_plan;

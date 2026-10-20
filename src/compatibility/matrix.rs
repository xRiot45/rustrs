use crate::compatibility::types::CompatibilityStatus;
use crate::manifest::model::{DataAccess, Database, Framework};

pub fn evaluate(
    framework: Framework,
    database: Database,
    data_access: DataAccess,
) -> CompatibilityStatus {
    let supported = matches!(
        (framework, database, data_access),
        (Framework::Axum, Database::Postgres, DataAccess::Sqlx)
    );

    if supported {
        CompatibilityStatus::Supported
    } else {
        CompatibilityStatus::Unsupported {
            reason: "combination is not implemented in the current support matrix".to_string(),
            alternatives: vec![
                "axum".to_string(),
                "postgres".to_string(),
                "sqlx".to_string(),
            ],
        }
    }
}

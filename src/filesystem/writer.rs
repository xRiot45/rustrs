use crate::filesystem::conflicts::{ConflictStatus, detect_conflict};
use crate::filesystem::paths::ensure_within_root;
use crate::generation::plan::{
    GenerationPlan, OperationOutcome, OperationReport, OperationResult, WritePolicy,
};
use std::io::Write;

pub fn apply_plan(plan: &GenerationPlan, policy: WritePolicy) -> OperationReport {
    let mut results = Vec::with_capacity(plan.operations.len());

    for operation in &plan.operations {
        let relative_path = operation.relative_path.clone();
        let target = match ensure_within_root(&plan.target_root, &relative_path) {
            Ok(path) => path,
            Err(error) => {
                results.push(OperationResult {
                    relative_path,
                    outcome: OperationOutcome::Failed {
                        error: error.to_string(),
                    },
                });
                continue;
            }
        };

        match detect_conflict(&plan.target_root, &operation.relative_path) {
            ConflictStatus::Absent => {}
            ConflictStatus::ExistsFile
            | ConflictStatus::ExistsDirectory
            | ConflictStatus::Invalid => {
                if policy != WritePolicy::Overwrite && !operation.overwrite {
                    results.push(OperationResult {
                        relative_path,
                        outcome: OperationOutcome::Skipped {
                            reason: "target already exists".to_string(),
                        },
                    });
                    continue;
                }
            }
        }

        match write_atomic(&target, &operation.contents) {
            Ok(()) => {
                results.push(OperationResult {
                    relative_path,
                    outcome: OperationOutcome::Created,
                });
            }
            Err(error) => {
                results.push(OperationResult {
                    relative_path,
                    outcome: OperationOutcome::Failed {
                        error: error.to_string(),
                    },
                });
            }
        }
    }

    OperationReport {
        target_root: plan.target_root.clone(),
        results,
    }
}

fn write_atomic(path: &std::path::Path, contents: &[u8]) -> Result<(), std::io::Error> {
    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "target has no parent")
    })?;
    std::fs::create_dir_all(parent)?;

    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(contents)?;
    temp.flush()?;
    temp.persist(path).map_err(|error| error.error)?;
    Ok(())
}

use crate::filesystem::conflicts::{ConflictStatus, detect_conflict};
use crate::generation::plan::{GenerationPlan, OperationOutcome, OperationReport, OperationResult};

pub fn preview_plan(plan: &GenerationPlan) -> OperationReport {
    let mut results = Vec::with_capacity(plan.operations.len());

    for operation in &plan.operations {
        let outcome = match detect_conflict(&plan.target_root, &operation.relative_path) {
            ConflictStatus::Absent => OperationOutcome::Planned,
            ConflictStatus::ExistsFile
            | ConflictStatus::ExistsDirectory
            | ConflictStatus::Invalid => {
                if operation.overwrite {
                    OperationOutcome::Planned
                } else {
                    OperationOutcome::Skipped {
                        reason: "target already exists".to_string(),
                    }
                }
            }
        };

        results.push(OperationResult {
            relative_path: operation.relative_path.clone(),
            outcome,
        });
    }

    OperationReport {
        target_root: plan.target_root.clone(),
        results,
    }
}

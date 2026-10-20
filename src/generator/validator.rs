use crate::error::CliError;
use crate::generator::context::GeneratorContext;

pub fn plan(_ctx: &GeneratorContext) -> Result<crate::generation::plan::GenerationPlan, CliError> {
    Err(CliError::InvalidInput(
        "validator generator requires a validation crate/infrastructure that is not available in the current template".to_string(),
    ))
}

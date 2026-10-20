use crate::error::CliError;
use crate::generator::context::GeneratorContext;
use crate::generator::handler;

pub fn plan(ctx: &GeneratorContext) -> Result<crate::generation::plan::GenerationPlan, CliError> {
    handler::plan_controller(ctx)
}

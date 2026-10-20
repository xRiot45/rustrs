use crate::error::CliError;
use crate::generation::plan::GenerationPlan;
use crate::generator::context::GeneratorContext;
use crate::generator::kind::GeneratorKind;

pub fn plan_for(kind: GeneratorKind, ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    match kind {
        GeneratorKind::Module => crate::generator::module::plan(ctx),
        GeneratorKind::Entity => crate::generator::entity::plan(ctx),
        GeneratorKind::Dto => crate::generator::dto::plan(ctx),
        GeneratorKind::Service => crate::generator::service::plan(ctx),
        GeneratorKind::Repository => crate::generator::repository::plan(ctx),
        GeneratorKind::Controller => crate::generator::controller::plan(ctx),
        GeneratorKind::Handler => crate::generator::handler::plan(ctx, false),
        GeneratorKind::Middleware => crate::generator::middleware::plan(ctx),
        GeneratorKind::Guard => crate::generator::guard::plan(ctx),
        GeneratorKind::Validator => crate::generator::validator::plan(ctx),
        GeneratorKind::Test => crate::generator::test::plan(ctx),
        GeneratorKind::Migration => crate::generator::migration::plan(ctx),
        GeneratorKind::Job => crate::generator::job::plan(ctx),
        GeneratorKind::Event | GeneratorKind::EventHandler => crate::generator::event::plan(ctx),
        GeneratorKind::Resource => crate::generator::resource::plan(ctx),
        GeneratorKind::Docker
        | GeneratorKind::Compose
        | GeneratorKind::CiGithub
        | GeneratorKind::CiGitlab
        | GeneratorKind::Release => Err(CliError::InvalidInput(
            "asset generator is handled by the application layer".to_string(),
        )),
    }
}

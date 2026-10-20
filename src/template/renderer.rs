use crate::template::context::TemplateContext;
use crate::template::metadata::TemplateError;

pub fn render(source: &str, context: &TemplateContext) -> Result<String, TemplateError> {
    let tera_context =
        tera::Context::from_serialize(context).map_err(|error| TemplateError::Context {
            message: error.to_string(),
        })?;

    tera::Tera::one_off(source, &tera_context, false).map_err(|error| TemplateError::Render {
        message: error.to_string(),
    })
}

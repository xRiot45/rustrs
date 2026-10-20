use crate::template::metadata::TemplateError;
use serde::Serialize;

pub fn render<T: Serialize>(source: &str, context: &T) -> Result<String, TemplateError> {
    let tera_context =
        tera::Context::from_serialize(context).map_err(|error| TemplateError::Context {
            message: error.to_string(),
        })?;

    tera::Tera::one_off(source, &tera_context, false).map_err(|error| TemplateError::Render {
        message: error.to_string(),
    })
}

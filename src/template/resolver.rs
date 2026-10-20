use crate::compatibility::check_compatibility;
use crate::manifest::model::ProjectManifest;
use crate::template::metadata::{BASIC_TEMPLATE, TemplateError, TemplateMetadata};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedTemplate {
    pub metadata: &'static TemplateMetadata,
}

pub fn resolve_template(
    template: crate::manifest::model::TemplateId,
    manifest: &ProjectManifest,
) -> Result<ResolvedTemplate, TemplateError> {
    check_compatibility(manifest)?;

    match template {
        crate::manifest::model::TemplateId::Minimal => {
            BASIC_TEMPLATE.validate()?;
            Ok(ResolvedTemplate {
                metadata: &BASIC_TEMPLATE,
            })
        }
        other => Err(TemplateError::Unavailable { template: other }),
    }
}

use crate::compatibility::check_compatibility;
use crate::manifest::model::ProjectManifest;
use crate::template::metadata::MINIMAL_TEMPLATE;
use crate::template::metadata::{
    ENTERPRISE_TEMPLATE, SIMPLE_TEMPLATE, STANDARD_TEMPLATE, TemplateError, TemplateMetadata,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedTemplate {
    pub metadata: &'static TemplateMetadata,
}

pub fn resolve_template(
    template: crate::manifest::model::TemplateId,
    manifest: &ProjectManifest,
) -> Result<ResolvedTemplate, TemplateError> {
    check_compatibility(manifest)?;

    let metadata = match template {
        crate::manifest::model::TemplateId::Minimal => &MINIMAL_TEMPLATE,
        crate::manifest::model::TemplateId::Simple => &SIMPLE_TEMPLATE,
        crate::manifest::model::TemplateId::Standard => &STANDARD_TEMPLATE,
        crate::manifest::model::TemplateId::Enterprise => &ENTERPRISE_TEMPLATE,
    };
    metadata.validate()?;
    Ok(ResolvedTemplate { metadata })
}

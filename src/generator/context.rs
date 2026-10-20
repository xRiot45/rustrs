use crate::domain::NormalizedName;
use crate::manifest::model::{ProjectManifest, TemplateId};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct GenContext {
    pub project_name: String,
    pub module: String,
    pub singular_type: String,
    pub plural_type: String,
    pub file_stem: String,
    pub route_segment: String,
}

#[derive(Debug, Clone)]
pub struct GeneratorContext {
    pub project_root: PathBuf,
    pub manifest: ProjectManifest,
    pub names: NormalizedName,
    pub skip_tests: bool,
    pub skip_docs: bool,
}

impl GeneratorContext {
    pub fn template(&self) -> TemplateId {
        self.manifest.template
    }

    pub fn is_minimal(&self) -> bool {
        self.manifest.template == TemplateId::Minimal
    }

    pub fn is_simple(&self) -> bool {
        self.manifest.template == TemplateId::Simple
    }

    pub fn is_standard(&self) -> bool {
        self.manifest.template == TemplateId::Standard
    }

    pub fn is_enterprise(&self) -> bool {
        self.manifest.template == TemplateId::Enterprise
    }

    pub fn module_dir(&self) -> String {
        match self.manifest.template {
            TemplateId::Minimal => "src".to_string(),
            TemplateId::Simple => format!("src/modules/{}", self.names.module),
            TemplateId::Standard | TemplateId::Enterprise => {
                format!("src/modules/{}", self.names.module)
            }
        }
    }

    pub fn requires_database(&self) -> bool {
        self.manifest.database != crate::manifest::model::Database::None
    }

    pub fn gen_context(&self) -> GenContext {
        GenContext {
            project_name: self.manifest.project_name.clone(),
            module: self.names.module.clone(),
            singular_type: self.names.singular_type.clone(),
            plural_type: self.names.plural_type.clone(),
            file_stem: self.names.file_stem.clone(),
            route_segment: self.names.route_segment.clone(),
        }
    }
}

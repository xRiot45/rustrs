use crate::filesystem::paths;
use crate::generation::plan::GenerationPlan;
use crate::generation::plan::PlannedOperation;
use crate::manifest::model::ProjectManifest;
use crate::template::asset::select_assets;
use crate::template::context::TemplateContext;
use crate::template::metadata::TemplateError;
use crate::template::renderer::render;
use crate::template::resolver::resolve_template;
use std::path::Path;

pub fn build_plan(
    manifest: &ProjectManifest,
    template: crate::manifest::model::TemplateId,
    target_root: &Path,
) -> Result<GenerationPlan, TemplateError> {
    if target_root.exists() && !target_root.is_dir() {
        return Err(TemplateError::InvalidPath {
            path: target_root.display().to_string(),
        });
    }

    let resolved = resolve_template(template, manifest)?;
    let assets = select_assets(resolved.metadata, manifest)?;
    let context = TemplateContext::from_manifest(manifest);

    let mut operations = Vec::new();
    for asset in assets {
        let relative_path =
            paths::validate_relative_path(Path::new(asset.relative_path)).map_err(|_| {
                TemplateError::InvalidPath {
                    path: asset.relative_path.to_string(),
                }
            })?;

        let contents = match asset.source {
            crate::template::metadata::AssetSource::Static(source) => source.as_bytes().to_vec(),
            crate::template::metadata::AssetSource::Template(source) => {
                render(source, &context)?.into_bytes()
            }
        };

        operations.push(PlannedOperation {
            relative_path,
            contents,
            description: format!("generate {}", asset.relative_path),
        });
    }

    let manifest_contents =
        toml::to_string_pretty(manifest).map_err(|error| TemplateError::Manifest {
            message: error.to_string(),
        })?;
    let manifest_path = Path::new(".rustrs").join("config.toml");
    let manifest_relative =
        paths::validate_relative_path(&manifest_path).map_err(|_| TemplateError::InvalidPath {
            path: manifest_path.display().to_string(),
        })?;
    operations.push(PlannedOperation {
        relative_path: manifest_relative,
        contents: manifest_contents.into_bytes(),
        description: "generate .rustrs/config.toml".to_string(),
    });

    Ok(GenerationPlan {
        target_root: target_root.to_path_buf(),
        operations,
    })
}

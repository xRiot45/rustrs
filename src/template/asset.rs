use crate::manifest::model::ProjectManifest;
use crate::template::metadata::{AssetSource, TemplateAsset, TemplateError, TemplateMetadata};

pub fn select_assets(
    metadata: &TemplateMetadata,
    manifest: &ProjectManifest,
) -> Result<Vec<TemplateAsset>, TemplateError> {
    metadata.validate()?;

    let mut selected = Vec::new();
    for asset in metadata.assets {
        let include = match asset.condition {
            None => true,
            Some(capability) => manifest.features.is_enabled(capability),
        };
        if include {
            selected.push(*asset);
        }
    }

    for asset in &selected {
        if asset.required {
            match asset.source {
                AssetSource::Static(source) | AssetSource::Template(source) => {
                    if source.is_empty() {
                        return Err(TemplateError::MissingAsset {
                            template: metadata.id,
                            path: asset.relative_path.to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(selected)
}

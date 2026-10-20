use crate::manifest::error::ManifestError;
use crate::manifest::model::ProjectManifest;

pub fn parse_manifest(input: &str) -> Result<ProjectManifest, ManifestError> {
    toml::from_str(input).map_err(|error| ManifestError::Parse {
        message: error.to_string(),
    })
}

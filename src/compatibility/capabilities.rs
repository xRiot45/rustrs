use crate::manifest::error::CapabilityError;
use crate::manifest::model::Features;

pub fn validate_capabilities(features: &Features) -> Result<(), CapabilityError> {
    if features.rbac && !features.auth {
        return Err(CapabilityError::Dependency {
            capability: "rbac",
            requires: "auth",
        });
    }
    Ok(())
}

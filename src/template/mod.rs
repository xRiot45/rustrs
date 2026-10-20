pub mod asset;
pub mod context;
pub mod metadata;
pub mod renderer;
pub mod resolver;

pub use asset::select_assets;
pub use context::TemplateContext;
pub use metadata::TemplateAsset;
pub use metadata::{
    BASIC_TEMPLATE, ENTERPRISE_TEMPLATE, MINIMAL_TEMPLATE, SIMPLE_TEMPLATE, STANDARD_TEMPLATE,
    TemplateError, TemplateMetadata,
};
pub use renderer::render;
pub use resolver::{ResolvedTemplate, resolve_template};

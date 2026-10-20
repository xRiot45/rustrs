use rustrs::compatibility::check_compatibility;
use rustrs::filesystem::paths::{ensure_within_root, validate_relative_path};
use rustrs::generation::planner::build_plan;
use rustrs::manifest::parse_manifest;
use rustrs::template::context::TemplateContext;
use rustrs::template::metadata::{BASIC_TEMPLATE, TemplateError};
use rustrs::template::renderer::render;
use rustrs::template::resolver::resolve_template;
use rustrs::template::{TemplateAsset, select_assets};
use std::path::Path;
use tempfile::TempDir;

const VALID_MANIFEST: &str = r#"
schema_version = 1
project_name = "demo"
template = "minimal"
framework = "axum"
architecture = "clean_modular"
database = "postgres"
data_access = "sqlx"
"#;

#[test]
fn metadata_is_valid() {
    BASIC_TEMPLATE.validate().unwrap();
}

#[test]
fn resolver_accepts_available_minimal_template() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    manifest.validate().unwrap();
    check_compatibility(&manifest).unwrap();

    let resolved = resolve_template(manifest.template, &manifest).unwrap();
    assert_eq!(resolved.metadata.id, manifest.template);
}

#[test]
fn resolver_rejects_unavailable_template() {
    let input = VALID_MANIFEST.replace("template = \"minimal\"", "template = \"enterprise\"");
    let manifest = parse_manifest(&input).unwrap();
    manifest.validate().unwrap();

    let err = resolve_template(manifest.template, &manifest).unwrap_err();
    assert!(matches!(err, TemplateError::Unavailable { .. }));
}

#[test]
fn resolver_rejects_unsupported_combination() {
    let input = VALID_MANIFEST.replace("framework = \"axum\"", "framework = \"rocket\"");
    let manifest = parse_manifest(&input).unwrap();
    manifest.validate().unwrap();

    let err = resolve_template(manifest.template, &manifest).unwrap_err();
    assert!(matches!(err, TemplateError::Compatibility(_)));
}

#[test]
fn context_is_built_from_manifest() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let context = TemplateContext::from_manifest(&manifest);

    assert_eq!(context.project_name, "demo");
    assert_eq!(context.template, "minimal");
    assert_eq!(context.framework, "axum");
    assert_eq!(context.database, "postgres");
    assert_eq!(context.data_access, "sqlx");
    assert_eq!(context.architecture, "clean_modular");
}

#[test]
fn renderer_produces_expected_output() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let context = TemplateContext::from_manifest(&manifest);

    let rendered = render("# {{ project_name }}\n", &context).unwrap();
    assert_eq!(rendered, "# demo\n");
}

#[test]
fn renderer_reports_invalid_template() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let context = TemplateContext::from_manifest(&manifest);

    let err = render("{{ project_name", &context).unwrap_err();
    assert!(matches!(err, TemplateError::Render { .. }));
}

#[test]
fn renderer_reports_missing_variable() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let context = TemplateContext::from_manifest(&manifest);

    let err = render("{{ missing }}", &context).unwrap_err();
    assert!(matches!(err, TemplateError::Render { .. }));
}

#[test]
fn rendering_is_deterministic() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let context = TemplateContext::from_manifest(&manifest);
    let source = "# {{ project_name }}\n{{ framework }}\n";

    let first = render(source, &context).unwrap();
    let second = render(source, &context).unwrap();
    assert_eq!(first, second);
}

#[test]
fn asset_selection_includes_required_assets() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let assets = select_assets(&BASIC_TEMPLATE, &manifest).unwrap();

    assert!(
        assets
            .iter()
            .any(|asset| asset.relative_path == "README.md")
    );
    assert!(
        assets
            .iter()
            .any(|asset| asset.relative_path == "config.toml")
    );
}

#[test]
fn asset_selection_respects_capability_condition() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let assets = select_assets(&BASIC_TEMPLATE, &manifest).unwrap();
    assert!(
        !assets
            .iter()
            .any(|asset| asset.relative_path == "Dockerfile")
    );

    // features default false in manifest; enable docker through direct struct not parsed from this fixture
    let mut manifest = parse_manifest(VALID_MANIFEST).unwrap();
    manifest.features.docker = true;
    let assets = select_assets(&BASIC_TEMPLATE, &manifest).unwrap();
    assert!(
        assets
            .iter()
            .any(|asset| asset.relative_path == "Dockerfile")
    );
}

#[test]
fn planning_does_not_mutate_filesystem() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let dir = TempDir::new().unwrap();
    let before: Vec<_> = std::fs::read_dir(dir.path()).unwrap().collect();

    let plan = build_plan(&manifest, manifest.template, dir.path()).unwrap();
    assert!(!plan.operations.is_empty());

    let after: Vec<_> = std::fs::read_dir(dir.path()).unwrap().collect();
    assert_eq!(before.len(), after.len());
}

#[test]
fn generated_plan_contains_rendered_content() {
    let manifest = parse_manifest(VALID_MANIFEST).unwrap();
    let dir = TempDir::new().unwrap();

    let plan = build_plan(&manifest, manifest.template, dir.path()).unwrap();
    let readme = plan
        .operations
        .iter()
        .find(|operation| operation.relative_path == Path::new("README.md"))
        .unwrap();
    assert!(String::from_utf8_lossy(&readme.contents).contains("demo"));
}

#[test]
fn path_validation_rejects_traversal_and_absolute_paths() {
    let err = validate_relative_path(Path::new("../secret.txt")).unwrap_err();
    assert!(matches!(
        err,
        rustrs::filesystem::PathError::ParentTraversal { .. }
    ));

    let err = validate_relative_path(Path::new("/etc/passwd")).unwrap_err();
    assert!(matches!(
        err,
        rustrs::filesystem::PathError::Absolute { .. }
    ));
}

#[test]
fn ensure_within_root_rejects_escape() {
    let dir = TempDir::new().unwrap();
    let err = ensure_within_root(dir.path(), Path::new("../escape.txt")).unwrap_err();
    assert!(matches!(
        err,
        rustrs::filesystem::PathError::ParentTraversal { .. }
    ));
}

#[test]
fn template_asset_type_is_available() {
    let asset = TemplateAsset {
        relative_path: "x.txt",
        source: rustrs::template::metadata::AssetSource::Static("x"),
        required: true,
        condition: None,
    };
    assert_eq!(asset.relative_path, "x.txt");
}

use heck::{ToPascalCase, ToSnakeCase};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedName {
    pub input: String,
    pub module: String,
    pub singular_type: String,
    pub plural_type: String,
    pub file_stem: String,
    pub route_segment: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NamingError {
    #[error("name must not be empty")]
    Empty,
    #[error("name must not contain whitespace")]
    Whitespace,
    #[error("name must not contain path separators or traversal segments")]
    Traversal,
    #[error("name contains invalid or control characters")]
    InvalidCharacter,
    #[error("name uses a reserved Rust keyword: {0}")]
    Reserved(String),
    #[error("normalized name is empty")]
    EmptyNormalized,
    #[error("name contains uppercase or invalid characters for an identifier override")]
    InvalidOverride,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NameInput {
    pub name: String,
    pub singular: Option<String>,
    pub plural: Option<String>,
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen",
];

fn validate_raw(value: &str) -> Result<(), NamingError> {
    if value.trim().is_empty() {
        return Err(NamingError::Empty);
    }
    if value.chars().any(char::is_whitespace) {
        return Err(NamingError::Whitespace);
    }
    if value.contains('/') || value.contains('\\') {
        return Err(NamingError::Traversal);
    }
    if value == "." || value == ".." {
        return Err(NamingError::Traversal);
    }
    if value.chars().any(|ch| ch.is_control()) {
        return Err(NamingError::InvalidCharacter);
    }
    if RUST_KEYWORDS.contains(&value) {
        return Err(NamingError::Reserved(value.to_string()));
    }
    Ok(())
}

fn validate_override(value: &str) -> Result<(), NamingError> {
    validate_raw(value)?;
    // Overrides should already be close to identifiers; reject obvious path-like input.
    if value.contains('-') && value.chars().any(char::is_uppercase) {
        // allow mixed; no strict rejection
    }
    Ok(())
}

fn module_name(value: &str) -> Result<String, NamingError> {
    validate_raw(value)?;
    let module = value.to_snake_case();
    if module.is_empty() {
        return Err(NamingError::EmptyNormalized);
    }
    if module
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
    {
        return Err(NamingError::InvalidCharacter);
    }
    Ok(module)
}

fn default_singular_module(module: &str) -> String {
    if module.ends_with('s') && module.len() > 1 {
        module.trim_end_matches('s').to_string()
    } else {
        module.to_string()
    }
}

fn default_plural_module(module: &str) -> String {
    if module.ends_with('s') {
        module.to_string()
    } else {
        format!("{module}s")
    }
}

pub fn normalize_name(input: &NameInput) -> Result<NormalizedName, NamingError> {
    validate_raw(&input.name)?;
    let module = module_name(&input.name)?;

    let singular_raw = input.singular.as_deref().unwrap_or("");
    let plural_raw = input.plural.as_deref().unwrap_or("");

    let singular_module = if singular_raw.is_empty() {
        default_singular_module(&module)
    } else {
        validate_override(singular_raw)?;
        module_name(singular_raw)?
    };

    let plural_module = if plural_raw.is_empty() {
        default_plural_module(&module)
    } else {
        validate_override(plural_raw)?;
        module_name(plural_raw)?
    };

    let singular_type = singular_module.to_pascal_case();
    let plural_type = plural_module.to_pascal_case();

    if singular_type.is_empty() || plural_type.is_empty() {
        return Err(NamingError::EmptyNormalized);
    }

    Ok(NormalizedName {
        input: input.name.clone(),
        module: module.clone(),
        singular_type,
        plural_type,
        file_stem: singular_module,
        route_segment: plural_module,
    })
}

pub fn names_collide(a: &str, b: &str) -> bool {
    if a == b {
        return false;
    }
    match (module_name(a), module_name(b)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_basic_name() {
        let result = normalize_name(&NameInput {
            name: "products".to_string(),
            singular: None,
            plural: None,
        })
        .unwrap();
        assert_eq!(result.module, "products");
        assert_eq!(result.singular_type, "Product");
        assert_eq!(result.plural_type, "Products");
        assert_eq!(result.file_stem, "product");
        assert_eq!(result.route_segment, "products");
    }

    #[test]
    fn accepts_explicit_overrides() {
        let result = normalize_name(&NameInput {
            name: "product-item".to_string(),
            singular: Some("product_item".to_string()),
            plural: Some("product_items".to_string()),
        })
        .unwrap();
        assert_eq!(result.module, "product_item");
        assert_eq!(result.singular_type, "ProductItem");
        assert_eq!(result.plural_type, "ProductItems");
    }

    #[test]
    fn rejects_empty() {
        assert!(matches!(
            normalize_name(&NameInput::default()),
            Err(NamingError::Empty)
        ));
    }

    #[test]
    fn rejects_traversal() {
        assert!(matches!(
            normalize_name(&NameInput {
                name: "../products".to_string(),
                ..Default::default()
            }),
            Err(NamingError::Traversal)
        ));
    }

    #[test]
    fn rejects_keyword() {
        assert!(matches!(
            normalize_name(&NameInput {
                name: "mod".to_string(),
                ..Default::default()
            }),
            Err(NamingError::Reserved(_))
        ));
    }

    #[test]
    fn detects_collision() {
        assert!(names_collide("product-item", "product_item"));
    }
}

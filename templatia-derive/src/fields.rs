use crate::error::generate_unsupported_compile_error;
use crate::utils::get_type_name;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use syn::GenericArgument;

/// Distinguishes the specific collection type for code generation.
/// This is a compile-time only type and must never enter `quote!` blocks.
#[derive(Debug, Clone, Copy)]
pub(crate) enum CollectionKind {
    Vec,
    HashSet,
    BTreeSet,
}

impl Display for CollectionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CollectionKind::Vec => write!(f, "Vec"),
            CollectionKind::HashSet => write!(f, "HashSet"),
            CollectionKind::BTreeSet => write!(f, "BTreeSet"),
        }
    }
}

/// Represents only the field types that templatia supports.
/// This is a compile-time only type used to decide what `TokenStream` to generate.
/// It must never enter `quote!` blocks directly.
pub(crate) enum SupportedFieldKind<'a> {
    Primitive(&'a syn::Type),
    Option(&'a syn::Type),
    Collection {
        inner: &'a syn::Type,
        kind: CollectionKind,
    },
}

impl Display for SupportedFieldKind<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedFieldKind::Primitive(ty) => write!(f, "{}", get_type_name(ty)),
            SupportedFieldKind::Option(ty) => write!(f, "Option<{}>", get_type_name(ty)),
            SupportedFieldKind::Collection { inner, kind } => {
                write!(f, "{}<{}>", kind, get_type_name(inner))
            }
        }
    }
}

/// Internal enum used only within `analyze_fields` to classify all possible field types,
/// including unsupported ones. This is converted to `SupportedFieldKind` before leaving
/// the analysis phase.
enum FieldKind<'a> {
    Primitive(&'a syn::Type),
    Option(&'a syn::Type),
    Result(&'a syn::Type, &'a syn::Type),
    Vec(&'a syn::Type),
    HashSet(&'a syn::Type),
    BTreeSet(&'a syn::Type),
    HashMap(&'a syn::Type, &'a syn::Type),
    BTreeMap(&'a syn::Type, &'a syn::Type),
    Tuple,
    Unknown,
}

impl Display for FieldKind<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldKind::Primitive(ty) => write!(f, "{}", get_type_name(ty)),
            FieldKind::Option(ty) => write!(f, "Option<{}>", get_type_name(ty)),
            FieldKind::Result(ok_ty, err_ty) => write!(
                f,
                "Result<{}, {}>",
                get_type_name(ok_ty),
                get_type_name(err_ty)
            ),
            FieldKind::Vec(ty) => write!(f, "Vec<{}>", get_type_name(ty)),
            FieldKind::HashSet(ty) => write!(f, "HashSet<{}>", get_type_name(ty)),
            FieldKind::BTreeSet(ty) => write!(f, "BTreeSet<{}>", get_type_name(ty)),
            FieldKind::HashMap(k_ty, v_ty) => write!(
                f,
                "HashMap<{}, {}>",
                get_type_name(k_ty),
                get_type_name(v_ty)
            ),
            FieldKind::BTreeMap(k_ty, v_ty) => write!(
                f,
                "BTreeMap<{}, {}>",
                get_type_name(k_ty),
                get_type_name(v_ty)
            ),
            FieldKind::Tuple => write!(f, "(<tuple>)"),
            FieldKind::Unknown => write!(f, "<unknown>"),
        }
    }
}

/// Converts a `FieldKind` to a `SupportedFieldKind`, returning an error tuple
/// `(field_display, type_display)` for unsupported types.
fn to_supported(kind: FieldKind<'_>) -> Result<SupportedFieldKind<'_>, String> {
    match kind {
        FieldKind::Primitive(ty) => Ok(SupportedFieldKind::Primitive(ty)),
        FieldKind::Option(ty) => Ok(SupportedFieldKind::Option(ty)),
        FieldKind::Vec(ty) => Ok(SupportedFieldKind::Collection {
            inner: ty,
            kind: CollectionKind::Vec,
        }),
        FieldKind::HashSet(ty) => Ok(SupportedFieldKind::Collection {
            inner: ty,
            kind: CollectionKind::HashSet,
        }),
        FieldKind::BTreeSet(ty) => Ok(SupportedFieldKind::Collection {
            inner: ty,
            kind: CollectionKind::BTreeSet,
        }),
        unsupported => Err(unsupported.to_string()),
    }
}

pub(crate) struct Fields<'a> {
    fields: &'a [syn::Field],
    idents_type: HashMap<&'a syn::Ident, SupportedFieldKind<'a>>,
    /// Compile errors for unsupported field types, collected during construction.
    unsupported_errors: Vec<proc_macro2::TokenStream>,
}

impl<'a> Fields<'a> {
    pub(crate) fn new(fields: &'a [syn::Field]) -> Self {
        let (idents_type, unsupported_errors) = analyze_fields(fields);

        Self {
            fields,
            idents_type,
            unsupported_errors,
        }
    }

    /// Returns compile errors for any unsupported field types found during construction.
    /// If non-empty, these should be emitted and macro expansion should halt.
    pub(crate) fn unsupported_type_errors(&self) -> &[proc_macro2::TokenStream] {
        &self.unsupported_errors
    }

    pub(crate) fn get_type_kind_by_name(&'_ self, name: &str) -> Option<&SupportedFieldKind<'_>> {
        let name = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
        self.idents_type.get(&name)
    }

    pub(crate) fn used_fields_in_template(
        &self,
        placeholders: &HashSet<String>,
    ) -> Vec<&syn::Field> {
        self.fields
            .iter()
            .filter(|field| {
                if let Some(ident) = field.ident.as_ref() {
                    placeholders.contains(&ident.to_string())
                } else {
                    false
                }
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn get_field_kind(&'_ self, ident: &syn::Ident) -> Option<&SupportedFieldKind<'_>> {
        self.idents_type.get(ident)
    }

    pub(crate) fn idents(&self) -> HashSet<&syn::Ident> {
        self.fields
            .iter()
            .filter_map(|f| f.ident.as_ref())
            .collect()
    }

    pub(crate) fn field_names(&self) -> HashSet<String> {
        self.idents()
            .iter()
            .map(|ident| ident.to_string())
            .collect()
    }

    pub(crate) fn option_fields(&self) -> HashMap<&syn::Ident, &syn::Type> {
        self.idents_type
            .iter()
            .filter(|(_, kind)| matches!(kind, SupportedFieldKind::Option(_)))
            .map(|(&ident, kind)| {
                let ty = match kind {
                    SupportedFieldKind::Option(ty) => *ty,
                    _ => unreachable!(),
                };

                (ident, ty)
            })
            .collect()
    }

    fn missing_placeholders(&self, placeholders_names: &HashSet<String>) -> Vec<&syn::Ident> {
        self.idents()
            .iter()
            .filter(|ident| !placeholders_names.contains(&ident.to_string()))
            .copied()
            .collect()
    }

    pub(crate) fn missing_placeholders_sep_opt(
        &self,
        placeholder_names: &HashSet<String>,
    ) -> (Vec<&syn::Ident>, Vec<&syn::Ident>) {
        let mut missing_placeholders_sep_opt = Vec::new();
        let mut missing_placeholders_sep_non_opt = Vec::new();

        let option_fields = self.option_fields();
        let missing_placeholders = self.missing_placeholders(placeholder_names);

        for missing_placeholder in missing_placeholders {
            if option_fields.contains_key(missing_placeholder) {
                missing_placeholders_sep_opt.push(missing_placeholder);
            } else {
                missing_placeholders_sep_non_opt.push(missing_placeholder);
            }
        }

        (
            missing_placeholders_sep_opt,
            missing_placeholders_sep_non_opt,
        )
    }
}

fn analyze_fields(
    fields: &'_ [syn::Field],
) -> (
    HashMap<&'_ syn::Ident, SupportedFieldKind<'_>>,
    Vec<proc_macro2::TokenStream>,
) {
    let mut result = HashMap::new();
    let mut errors = Vec::new();

    for field in fields {
        // If the field is not named, skip it. Currently, only named fields are supported.
        if field.ident.is_none() {
            continue;
        }

        let field_kind = classify_field_type(field);
        let ident = field.ident.as_ref().unwrap();

        match to_supported(field_kind) {
            Ok(supported) => {
                result.insert(ident, supported);
            }
            Err(type_display) => {
                errors.push(generate_unsupported_compile_error(ident, &type_display));
            }
        }
    }

    (result, errors)
}

/// Classifies a single field's type into a `FieldKind`. This handles all type variants
/// including unsupported ones.
fn classify_field_type<'a>(field: &'a syn::Field) -> FieldKind<'a> {
    match &field.ty {
        syn::Type::Path(type_path) => {
            if let Some(last_segment) = type_path.path.segments.last() {
                match &last_segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        let ident = &last_segment.ident.to_string();
                        match ident.as_str() {
                            "Option" => {
                                if args.args.len() == 1
                                    && let Some(GenericArgument::Type(ty)) = args.args.first()
                                {
                                    return FieldKind::Option(ty);
                                }
                            }
                            "Vec" => {
                                if args.args.len() == 1
                                    && let Some(GenericArgument::Type(ty)) = args.args.first()
                                {
                                    return FieldKind::Vec(ty);
                                }
                            }
                            "HashSet" => {
                                if args.args.len() == 1
                                    && let Some(GenericArgument::Type(ty)) = args.args.first()
                                {
                                    return FieldKind::HashSet(ty);
                                }
                            }
                            "BTreeSet" => {
                                if args.args.len() == 1
                                    && let Some(GenericArgument::Type(ty)) = args.args.first()
                                {
                                    return FieldKind::BTreeSet(ty);
                                }
                            }
                            "HashMap" => {
                                if args.args.len() == 2
                                    && let (
                                        Some(GenericArgument::Type(key_ty)),
                                        Some(GenericArgument::Type(value_ty)),
                                    ) = (args.args.first(), args.args.last())
                                {
                                    return FieldKind::HashMap(key_ty, value_ty);
                                }
                            }
                            "BTreeMap" => {
                                if args.args.len() == 2
                                    && let (
                                        Some(GenericArgument::Type(key_ty)),
                                        Some(GenericArgument::Type(value_ty)),
                                    ) = (args.args.first(), args.args.last())
                                {
                                    return FieldKind::BTreeMap(key_ty, value_ty);
                                }
                            }
                            "Result" => {
                                if args.args.len() == 2
                                    && let (
                                        Some(GenericArgument::Type(ok_ty)),
                                        Some(GenericArgument::Type(err_ty)),
                                    ) = (args.args.first(), args.args.last())
                                {
                                    return FieldKind::Result(ok_ty, err_ty);
                                }
                            }
                            _ => {}
                        }
                        FieldKind::Unknown
                    }
                    syn::PathArguments::None => FieldKind::Primitive(&field.ty),
                    syn::PathArguments::Parenthesized(_) => FieldKind::Unknown,
                }
            } else {
                FieldKind::Unknown
            }
        }
        syn::Type::Tuple(_) => FieldKind::Tuple,
        _ => FieldKind::Unknown,
    }
}

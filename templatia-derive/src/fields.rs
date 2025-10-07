use std::collections::{HashMap, HashSet};
use syn::GenericArgument;

// TODO: Add support for Result<T, E>, Collection<T>, KVCollection<K, V>
#[allow(dead_code)]
pub(crate) enum FieldKind<'a> {
    Primitive(&'a syn::Type),
    Option(&'a syn::Type),
    Result(&'a syn::Type, &'a syn::Type),
    Collection(&'a syn::Type),
    KVCollection(&'a syn::Type, &'a syn::Type),
    Tuple,
    Unknown,
}

pub(crate) struct Fields<'a> {
    fields: &'a [syn::Field],
    idents_type: HashMap<&'a syn::Ident, FieldKind<'a>>,
}

impl<'a> Fields<'a> {
    pub(crate) fn new(fields: &'a [syn::Field]) -> Self {
        let idents_type = analyze_fields(fields);

        Self { fields, idents_type }
    }

    pub(crate) fn get_type_kind_by_name(&'_ self, name: &str) -> Option<&'_ FieldKind<'_>> {
        let name = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
        self.idents_type.get(&name)
    }

    pub(crate) fn used_fields_in_template(&self, placeholders: &HashSet<String>) -> Vec<&syn::Field> {
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

    pub(crate) fn get_field_kind(&'_ self, ident: &syn::Ident) -> Option<&'_ FieldKind<'_>> {
        self.idents_type.get(ident)
    }

    pub(crate) fn idents(&self) -> HashSet<&syn::Ident> {
        self.fields.iter().filter_map(|f| f.ident.as_ref()).collect()
    }

    pub(crate) fn field_names(&self) -> HashSet<String> {
        self.idents().iter().map(|ident| ident.to_string()).collect()
    }

    pub(crate) fn option_fields(&self) -> HashMap<&syn::Ident, &syn::Type> {
        self.idents_type
            .iter()
            .filter(|(_, kind)| matches!(kind, FieldKind::Option(_)))
            .map(|(&ident, kind)| {
                let ty = match kind {
                    FieldKind::Option(ty) => *ty,
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
            .map(|ident| *ident)
            .collect()
    }

    pub(crate) fn missing_placeholders_sep_opt(&self, placeholder_names: &HashSet<String>) -> (Vec<&syn::Ident>, Vec<&syn::Ident>) {
        let mut missing_placeholders_sep_opt = Vec::new();
        let mut missing_placeholders_sep_non_opt = Vec::new();

        let option_fields = self.option_fields();

        for missing_placeholder in self.missing_placeholders(placeholder_names) {
            if option_fields.contains_key(missing_placeholder) {
                missing_placeholders_sep_opt.push(missing_placeholder);
            } else {
                missing_placeholders_sep_non_opt.push(missing_placeholder);
            }
        }

        (missing_placeholders_sep_opt, missing_placeholders_sep_non_opt)
    }
}

fn analyze_fields(fields: &'_ [syn::Field]) -> HashMap<&'_ syn::Ident, FieldKind<'_>> {
    let mut result = HashMap::new();

    for field in fields {
        // If the field is not named, skip it. Currently only named fields are supported.
        if field.ident.is_none() {
            continue;
        }

        match &field.ty {
            syn::Type::Path(type_path) => {
                if let Some(last_segment) = type_path.path.segments.last() {
                    match &last_segment.arguments {
                        syn::PathArguments::AngleBracketed(args) => {
                            let ident = &last_segment.ident.to_string();
                            match ident.as_str() {
                                "Option" => {
                                    // Option<T> has only one argument which is T.
                                    if args.args.len() == 1 {
                                        if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                            result.insert(field.ident.as_ref().unwrap(), FieldKind::Option(ty));
                                            continue;
                                        }
                                    }
                                    result.insert(field.ident.as_ref().unwrap(), FieldKind::Unknown);
                                },
                                "Vec" | "HashSet" | "BTreeSet" => {
                                    // Vec<T>, HashSet<T>, BTreeSet<T> has only one argument which is T.
                                    if args.args.len() == 1 {
                                        if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                            result.insert(field.ident.as_ref().unwrap(), FieldKind::Collection(ty));
                                            continue;
                                        }
                                    }
                                    result.insert(field.ident.as_ref().unwrap(), FieldKind::Unknown);
                                },
                                "HashMap" | "BTreeMap" => {
                                    if args.args.len() == 2 {
                                        if let (Some(GenericArgument::Type(key_ty)), Some(GenericArgument::Type(value_ty))) = (args.args.first(), args.args.last()) {
                                            result.insert(field.ident.as_ref().unwrap(), FieldKind::KVCollection(key_ty, value_ty));
                                            continue;
                                        }
                                    }
                                    result.insert(field.ident.as_ref().unwrap(), FieldKind::Unknown);
                                },
                                "Result" => {
                                    if args.args.len() == 2 {
                                        if let (Some(GenericArgument::Type(ok_ty)), Some(GenericArgument::Type(err_ty))) = (args.args.first(), args.args.last()) {
                                            result.insert(field.ident.as_ref().unwrap(), FieldKind::Result(ok_ty, err_ty));
                                            continue;
                                        }
                                        result.insert(field.ident.as_ref().unwrap(), FieldKind::Unknown);
                                    }
                                },
                                _ => {
                                    result.insert(field.ident.as_ref().unwrap(), FieldKind::Unknown);
                                }
                            }
                        },
                        syn::PathArguments::None => {
                            result.insert(field.ident.as_ref().unwrap(), FieldKind::Primitive(&field.ty));
                        },
                        syn::PathArguments::Parenthesized(_) => {
                            result.insert(field.ident.as_ref().unwrap(), FieldKind::Unknown);
                        }
                    }
                }
            },
            syn::Type::Tuple(_) => {
                result.insert(field.ident.as_ref().unwrap(), FieldKind::Tuple);
            },
            _ => {
                result.insert(field.ident.as_ref().unwrap(), FieldKind::Unknown);
            }
        }
    }

    result
}
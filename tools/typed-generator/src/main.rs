use std::{
    collections::{HashMap, HashSet},
    env, fs,
};
use syn::visit_mut::VisitMut;

struct RedactDebug;
impl VisitMut for RedactDebug {
    fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
        if item
            .trait_
            .as_ref()
            .is_some_and(|(_, path, _)| path.segments.last().is_some_and(|s| s.ident == "Debug"))
        {
            for member in &mut item.items {
                if let syn::ImplItem::Fn(method) = member
                    && method.sig.ident == "fmt"
                {
                    method.block =
                        syn::parse_quote!({ f.write_str("ConversionError([REDACTED])") });
                }
            }
        }
        syn::visit_mut::visit_item_impl_mut(self, item);
    }
    fn visit_attribute_mut(&mut self, attr: &mut syn::Attribute) {
        if attr.path().is_ident("derive") {
            let paths = attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                )
                .expect("derive paths");
            let safe: syn::punctuated::Punctuated<syn::Path, syn::Token![,]> = paths
                .into_iter()
                .filter(|p| p.segments.last().is_none_or(|s| s.ident != "Debug"))
                .collect();
            *attr = syn::parse_quote!(#[derive(#safe)]);
        }
        syn::visit_mut::visit_attribute_mut(self, attr);
    }
    fn visit_field_mut(&mut self, field: &mut syn::Field) {
        // Emit line docs: Prettyplease block docs introduce indentation that
        // rustdoc treats as executable code. The schema stays verbatim.
        let mut attributes = Vec::new();
        for attr in std::mem::take(&mut field.attrs) {
            if attr.path().is_ident("doc")
                && let syn::Meta::NameValue(meta) = &attr.meta
                && let syn::Expr::Lit(expr) = &meta.value
                && let syn::Lit::Str(text) = &expr.lit
            {
                for line in text.value().lines() {
                    let prose = syn::LitStr::new(line.trim_start(), text.span());
                    attributes.push(syn::parse_quote!(#[doc = #prose]));
                }
            } else {
                attributes.push(attr);
            }
        }
        field.attrs = attributes;
        syn::visit_mut::visit_field_mut(self, field);
    }
}

fn inner_option(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(path) = ty {
        let segment = path.path.segments.last()?;
        if segment.ident == "Option"
            && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
            && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
        {
            return Some(inner.clone());
        }
    }
    None
}

fn model_schema(model: &syn::ItemStruct) -> serde_json::Value {
    // Typify emits the complete schema for named and anonymous types in its
    // documented JSON block. Parse that structured schema, never infer a schema
    // from the generated Rust name or silently skip inline objects.
    let docs = model
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let syn::Meta::NameValue(meta) = &attr.meta
                && let syn::Expr::Lit(expr) = &meta.value
                && let syn::Lit::Str(text) = &expr.lit
            {
                Some(text.value())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut lines = docs
        .lines()
        .skip_while(|line| line.trim() != "<details><summary>JSON schema</summary>");
    lines.next().expect("Typify model schema documentation");
    let mut json = lines.skip_while(|line| line.trim() != "```json");
    json.next().expect("Typify model schema JSON fence");
    let schema = json
        .take_while(|line| line.trim() != "```")
        .collect::<Vec<_>>()
        .join("\n");
    serde_json::from_str(&schema).expect("Typify model schema JSON")
}

fn fix_models(ast: &mut syn::File) {
    let mut debug = Vec::new();
    let mut optional_fields: HashMap<String, Vec<String>> = HashMap::new();
    for item in &mut ast.items {
        match item {
            syn::Item::Struct(model) => {
                let schema = model_schema(model);
                let name = &model.ident;
                debug.push(syn::parse_quote! { impl std::fmt::Debug for #name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(concat!(stringify!(#name), "([REDACTED])")) } } });
                for field in &mut model.fields {
                    let Some(ident) = &field.ident else { continue };
                    let wire = ident
                        .to_string()
                        .trim_start_matches("r#")
                        .trim_end_matches('_')
                        .to_string();
                    let Some(property) = schema.get("properties").and_then(|p| p.get(&wire)) else {
                        continue;
                    };
                    let nullable = property
                        .get("anyOf")
                        .and_then(|v| v.as_array())
                        .is_some_and(|a| a.iter().any(|v| v["type"] == "null"));
                    let required = schema
                        .get("required")
                        .and_then(|v| v.as_array())
                        .is_some_and(|r| r.iter().any(|v| v == &wire));
                    if !required {
                        optional_fields
                            .entry(name.to_string())
                            .or_default()
                            .push(ident.to_string());
                    }
                    // Defaults are annotations, not permission to omit required fields.
                    // Optional fields keep presence, including explicitly empty arrays/maps.
                    for attr in &mut field.attrs {
                        if attr.path().is_ident("serde") {
                            let meta = attr.parse_args_with(syn::punctuated::Punctuated::<syn::Meta,syn::Token![,]>::parse_terminated).unwrap();
                            let kept: syn::punctuated::Punctuated<syn::Meta, syn::Token![,]> = meta
                                .into_iter()
                                .filter(|m| {
                                    !m.path().is_ident("default")
                                        && !m.path().is_ident("skip_serializing_if")
                                })
                                .collect();
                            *attr = syn::parse_quote!(#[serde(#kept)]);
                        }
                    }
                    if nullable {
                        let inner =
                            inner_option(&field.ty).expect("Typify nullable field must be Option");
                        if required {
                            field.attrs.push(syn::parse_quote!(#[serde(deserialize_with = "crate::typed::required_nullable")]));
                        } else {
                            field.ty = syn::parse_quote!(Option<Option<#inner>>);
                            field.attrs.push(syn::parse_quote!(#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::typed::optional_nullable")]));
                        }
                    } else if !required {
                        if inner_option(&field.ty).is_none() {
                            let inner = &field.ty;
                            field.ty = syn::parse_quote!(Option<#inner>);
                        }
                        field.attrs.push(syn::parse_quote!(#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::typed::optional_non_null")]));
                    }
                }
            }
            syn::Item::Enum(model) => {
                let name = &model.ident;
                debug.push(syn::parse_quote! { impl std::fmt::Debug for #name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(concat!(stringify!(#name), "([REDACTED])")) } } });
            }
            _ => {}
        }
    }
    struct OptionalDefaults<'a> {
        fields: &'a HashMap<String, Vec<String>>,
        current: String,
    }
    impl VisitMut for OptionalDefaults<'_> {
        fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
            if let syn::Type::Path(path) = item.self_ty.as_ref() {
                self.current = path.path.segments.last().unwrap().ident.to_string();
            }
            syn::visit_mut::visit_item_impl_mut(self, item);
            self.current.clear();
        }
        fn visit_expr_struct_mut(&mut self, expr: &mut syn::ExprStruct) {
            if let Some(fields) = self.fields.get(&self.current) {
                for field in &mut expr.fields {
                    if let syn::Member::Named(name) = &field.member
                        && fields.contains(&name.to_string())
                    {
                        field.expr = syn::parse_quote!(None);
                    }
                }
            }
            syn::visit_mut::visit_expr_struct_mut(self, expr);
        }
    }
    OptionalDefaults {
        fields: &optional_fields,
        current: String::new(),
    }
    .visit_file_mut(ast);
    ast.items
        .retain(|item| !matches!(item, syn::Item::Mod(module) if module.ident == "defaults"));
    ast.items.extend(debug);
}

fn add_builders(ast: &mut syn::File) {
    let names: HashSet<String> = ast
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(model) => Some(model.ident.to_string()),
            syn::Item::Enum(model) => Some(model.ident.to_string()),
            _ => None,
        })
        .collect();
    for item in &mut ast.items {
        let syn::Item::Struct(model) = item else {
            continue;
        };
        if !matches!(model.fields, syn::Fields::Named(_)) {
            continue;
        }
        assert!(
            !names.contains(&format!("{}Builder", model.ident)),
            "builder type name collision"
        );
        let schema = model_schema(model);
        let field_names: HashSet<String> = model
            .fields
            .iter()
            .filter_map(|field| field.ident.as_ref())
            .map(|name| name.to_string().trim_start_matches("r#").to_owned())
            .collect();
        model
            .attrs
            .push(syn::parse_quote!(#[derive(::typed_builder::TypedBuilder)]));
        model.attrs.push(syn::parse_quote!(#[builder(doc)]));
        for field in &mut model.fields {
            let ident = field.ident.as_ref().expect("named field");
            let name = ident.to_string();
            let wire = name.trim_start_matches("r#").trim_end_matches('_');
            let flattened = field.attrs.iter().any(|attr| {
                attr.path().is_ident("serde") && attr.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                ).expect("serde attributes").iter().any(|meta| meta.path().is_ident("flatten"))
            });
            if flattened {
                field.attrs.push(syn::parse_quote!(#[builder(default)]));
                continue;
            }
            let required = schema
                .get("required")
                .and_then(|value| value.as_array())
                .is_some_and(|properties| properties.iter().any(|property| property == wire));
            let optional = schema
                .get("properties")
                .and_then(|value| value.get(wire))
                .is_some()
                && !required;
            let setter_type = if optional {
                inner_option(&field.ty).expect("presence-aware optional field")
            } else {
                field.ty.clone()
            };
            let string = matches!(&setter_type, syn::Type::Path(path)
                if path.path.segments.last().is_some_and(|segment| segment.ident == "String"));
            if optional {
                let fallback_name = format!("opt_{}", name.trim_start_matches("r#"));
                assert!(
                    !field_names.contains(&fallback_name),
                    "optional builder setter name collision"
                );
                let fallback = syn::Ident::new(&fallback_name, ident.span());
                field.attrs.push(if string {
                    syn::parse_quote!(#[builder(default, setter(strip_option(fallback = #fallback), into))])
                } else {
                    syn::parse_quote!(#[builder(default, setter(strip_option(fallback = #fallback)))])
                });
            } else if string {
                field
                    .attrs
                    .push(syn::parse_quote!(#[builder(setter(into))]));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let raw: serde_json::Value = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let mut generation_schema = raw.clone();
    fn defaults(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(object) => {
                // Typify intentionally drops unconstrained extension fields.
                // The equivalent empty schema asks it to preserve these as a
                // flattened map while all declared properties stay typed.
                if object.get("type").is_some_and(|v| v == "object")
                    && object
                        .get("properties")
                        .and_then(|v| v.as_object())
                        .is_some_and(|p| !p.is_empty())
                    && object.get("additionalProperties").is_none_or(|v| v == true)
                {
                    object.insert("additionalProperties".into(), serde_json::json!({}));
                }
                if object
                    .get("enum")
                    .and_then(|v| v.as_array())
                    .is_some_and(|items| object.get("default").is_some_and(|d| !items.contains(d)))
                {
                    object.remove("default");
                }
                for child in object.values_mut() {
                    defaults(child);
                }
            }
            serde_json::Value::Array(array) => {
                for child in array {
                    defaults(child);
                }
            }
            _ => {}
        }
    }
    defaults(&mut generation_schema);
    let schema: schemars::schema::RootSchema = serde_json::from_value(generation_schema)?;
    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_struct_builder(false);
    let mut space = typify::TypeSpace::new(&settings);
    space.add_ref_types(schema.definitions)?;
    let mut ast = syn::parse2::<syn::File>(space.to_stream())?;
    RedactDebug.visit_file_mut(&mut ast);
    fix_models(&mut ast);
    add_builders(&mut ast);
    ast.attrs
        .push(syn::parse_quote!(#![allow(clippy::clone_on_copy, clippy::derivable_impls)]));
    let models: Vec<_> = ast
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(model) => {
                Some(serde_json::json!({"name":model.ident.to_string(),"kind":"struct"}))
            }
            syn::Item::Enum(model) => {
                Some(serde_json::json!({"name":model.ident.to_string(),"kind":"enum"}))
            }
            _ => None,
        })
        .collect();
    fs::write(
        "tools/typed-model-manifest.json",
        serde_json::to_string_pretty(
            &serde_json::json!({"generator":"Typify 0.5.0", "modelCount":models.len(),"typespaceCount":space.iter_types().count(),"models":models}),
        )? + "\n",
    )?;
    fs::write(&args[2], prettyplease::unparse(&ast))?;
    println!(
        "Generated {} Rust types with Typify 0.5.0; Debug derives removed",
        space.iter_types().count()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(properties: serde_json::Value) -> syn::File {
        let schema: schemars::schema::RootSchema = serde_json::from_value(serde_json::json!({
            "definitions": { "Example": { "type": "object", "properties": properties,
                "required": ["required_nullable", "required_defaulted"], "additionalProperties": {} } }
        })).unwrap();
        let mut settings = typify::TypeSpaceSettings::default();
        settings.with_struct_builder(false);
        let mut space = typify::TypeSpace::new(&settings);
        space.add_ref_types(schema.definitions).unwrap();
        let mut ast = syn::parse2::<syn::File>(space.to_stream()).unwrap();
        RedactDebug.visit_file_mut(&mut ast);
        fix_models(&mut ast);
        add_builders(&mut ast);
        ast
    }

    fn properties() -> serde_json::Value {
        serde_json::json!({
            "required_nullable": {"anyOf": [{"type": "string"}, {"type": "null"}]},
            "required_defaulted": {"type": "integer", "default": 7},
            "optional_nullable": {"anyOf": [{"type": "string"}, {"type": "null"}]},
            "optional_text": {"type": "string", "default": "annotation"},
            "optional_list": {"type": "array", "items": {"type": "string"}}
        })
    }

    #[test]
    fn builders_follow_final_presence_not_rust_option_or_source_defaults() {
        let ast = generate(properties());
        let model = ast
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Struct(model) if model.ident == "Example" => Some(model),
                _ => None,
            })
            .unwrap();
        for field in &model.fields {
            let name = field.ident.as_ref().unwrap().to_string();
            let defaults = field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("builder"))
                .flat_map(|attr| {
                    attr.parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .unwrap()
                })
                .any(|meta| meta.path().is_ident("default"));
            assert_eq!(defaults, !name.starts_with("required_"), "{name}");
            if name == "optional_nullable" {
                assert!(inner_option(&inner_option(&field.ty).unwrap()).is_some());
            }
        }
    }

    #[test]
    #[should_panic(expected = "optional builder setter name collision")]
    fn fallback_collisions_fail_generation() {
        let mut schema = properties();
        schema["opt_optional_text"] = serde_json::json!({"type": "string"});
        generate(schema);
    }
}

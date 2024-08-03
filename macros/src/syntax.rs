use quote::ToTokens;

use crate::*;

pub fn expand(input: &mut syn::DeriveInput) -> syn::Result<TokenStream2> {
    let root_attr = parse_root_attributes(input)?;

    let teleparse = crate_ident();

    let (extra_derive, output) = match &mut input.data {
        syn::Data::Struct(data) => (None, expand_struct(&input.vis, input.ident.clone(), data, &root_attr)?),
        syn::Data::Enum(data) => (
            Some(quote! {#[derive(#teleparse::ToSpan)]}),
            expand_enum(&input.vis, input.ident.clone(), data, &root_attr)?,
        ),
        _ => syn_error!(
            input,
            "derive_syntax can only be used with structs or enums"
        ),
    };

    let root_test = if !root_attr.root || root_attr.no_test {
        None
    } else {
        Some(root::expand_test(&input.ident))
    };

    let output = quote! {
        #extra_derive
        #input
        #output
        #root_test
    };

    Ok(output)
}

fn expand_struct(
    vis: &syn::Visibility,
    ident: syn::Ident,
    input: &mut syn::DataStruct,
    root_attr: &RootAttr,
) -> syn::Result<TokenStream2> {
    match &mut input.fields {
        syn::Fields::Unnamed(fields) => expand_struct_unnamed(vis, ident, fields, root_attr),
        syn::Fields::Named(fields) => expand_struct_named(vis, ident, fields, root_attr),
        syn::Fields::Unit => {
            syn_error!(ident, "derive_syntax does not support unit structs");
        }
    }
}

fn expand_struct_unnamed(
    vis: &syn::Visibility,
    ident: syn::Ident,
    input: &mut syn::FieldsUnnamed,
    root_attr: &RootAttr,
) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();
    let mut field_attrs = Vec::with_capacity(input.unnamed.len());
    for field in &mut input.unnamed {
        field_attrs.push(strip_take_attrs(&mut field.attrs));
    }

    let pt_ty = input.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let first_ty = match pt_ty.first() {
        Some(x) => x,
        None => syn_error!(&ident, "derive_syntax needs at least 1 field in the struct"),
    };

    let mut apply_semantic_impl = TokenStream2::new();
    for ((i, field), attrs) in
        std::iter::zip(input.unnamed.iter().enumerate(), field_attrs.into_iter())
    {
        let field_attr = parse_field_attributes(field, attrs)?;
        let idx = syn::Index::from(i);
        if let Some(semantic) = field_attr.semantic {
            apply_semantic_impl.extend(quote! {
                parser.apply_semantic(
                    &result.#idx,
                    #teleparse::token_set!(<Self::Prod as #teleparse::syntax::Production>::L{
                        #( #semantic )|*
                    })
                );
            });
        }
    }

    let last = syn::Index::from(pt_ty.len() - 1);
    let root_derive = if root_attr.root {
        Some(root::expand(&ident))
    } else {
        None
    };
    let (name, prod_struct) = derived_prod_name(&ident);
    let output = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #vis struct #prod_struct;
        #[automatically_derived]
        impl #teleparse::syntax::Production for #prod_struct {
            type L = <<#first_ty as #teleparse::parser::Produce>::Prod as #teleparse::syntax::Production>::L;
            #[inline]
            fn debug() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#name)
            }
            fn register(meta: &mut #teleparse::syntax::MetadataBuilder<Self::L>) {
                #teleparse::register_sequence!(meta, #( <#pt_ty as #teleparse::parser::Produce>::Prod ),*);
            }
        }
        #[automatically_derived]
        impl #teleparse::ToSpan for #ident {
            fn lo(&self) -> #teleparse::Pos { self.0.lo() }
            fn hi(&self) -> #teleparse::Pos { self.#last.hi() }
        }
        #[automatically_derived]
        impl #teleparse::parser::Produce for #ident {
            type Prod = #prod_struct;
            fn produce(
                parser: &mut #teleparse::parser::Parser<'_, <Self::Prod as #teleparse::syntax::Production>::L>,
                meta: &#teleparse::syntax::Metadata<<Self::Prod as #teleparse::syntax::Production>::L>,
            ) -> #teleparse::syntax::Result<Self, <Self::Prod as #teleparse::syntax::Production>::L> {
                use #teleparse::parser::Produce;
                let mut errors = ::std::vec::Vec::new();
                let result = Self(
            #(
                #teleparse::handle_result!(errors, <#pt_ty>::produce(parser, meta))
            ),*
                );
                #apply_semantic_impl
                (result, errors).into()
            }
        }
        #root_derive
    };

    Ok(anon_const_block(output))
}

fn expand_struct_named(
    vis: &syn::Visibility,
    ident: syn::Ident,
    input: &mut syn::FieldsNamed,
    root_attr: &RootAttr,
) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();
    let mut field_attrs = Vec::with_capacity(input.named.len());
    for field in &mut input.named {
        field_attrs.push(strip_take_attrs(&mut field.attrs));
    }

    let pt_ty = input.named.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let pt_ident = input.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let first_ty = match pt_ty.first() {
        Some(x) => x,
        None => syn_error!(&ident, "derive_syntax needs at least 1 field in the struct"),
    };

    let mut apply_semantic_impl = TokenStream2::new();
    for (field, attrs) in std::iter::zip(input.named.iter(), field_attrs.into_iter()) {
        let field_ident = field.ident.as_ref().unwrap();
        let field_attr = parse_field_attributes(field, attrs)?;
        if let Some(semantic) = field_attr.semantic {
            apply_semantic_impl.extend(quote! {
                parser.apply_semantic(
                    &result.#field_ident,
                    #teleparse::token_set!(<Self::Prod as #teleparse::syntax::Production>::L{
                        #( #semantic )|*
                    })
                );
            });
        }
    }

    let first_ident = pt_ident.first().unwrap();
    let last_ident = pt_ident.last().unwrap();
    let root_derive = if root_attr.root {
        Some(root::expand(&ident))
    } else {
        None
    };

    let (name, prod_struct) = derived_prod_name(&ident);

    let output = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #vis struct #prod_struct;
        #[automatically_derived]
        impl #teleparse::syntax::Production for #prod_struct {
            type L = <<#first_ty as #teleparse::parser::Produce>::Prod as #teleparse::syntax::Production>::L;
            #[inline]
            fn debug() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#name)
            }
            fn register(meta: &mut #teleparse::syntax::MetadataBuilder<Self::L>) {
                #teleparse::register_sequence!(meta, #( <#pt_ty as #teleparse::parser::Produce>::Prod ),*);
            }
        }

        #[automatically_derived]
        impl #teleparse::ToSpan for #ident {
            fn lo(&self) -> #teleparse::Pos { self.#first_ident.lo() }
            fn hi(&self) -> #teleparse::Pos { self.#last_ident.hi() }
        }
        #[automatically_derived]
        impl #teleparse::parser::Produce for #ident {
            type Prod = #prod_struct;
            fn produce(
                parser: &mut #teleparse::parser::Parser<'_, <Self::Prod as #teleparse::syntax::Production>::L>,
                meta: &#teleparse::syntax::Metadata<<Self::Prod as #teleparse::syntax::Production>::L>,
            ) -> #teleparse::syntax::Result<Self, <Self::Prod as #teleparse::syntax::Production>::L> {
                use #teleparse::parser::Produce;
                let mut errors = ::std::vec::Vec::new();
                let result = Self {
            #(
                #pt_ident: #teleparse::handle_result!(errors, <#pt_ty>::produce(parser, meta))
            ),*
                };
                #apply_semantic_impl
                (result, errors).into()
            }
        }
        #root_derive
    };
    Ok(anon_const_block(output))
}

fn expand_enum(
    vis: &syn::Visibility,
    ident: syn::Ident,
    input: &mut syn::DataEnum,
    root_attr: &RootAttr,
) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();

    let mut variant_attrs = Vec::with_capacity(input.variants.len());
    for variant in &mut input.variants {
        variant_attrs.push(strip_take_attrs(&mut variant.attrs));
    }

    let mut pt_ident = Vec::with_capacity(input.variants.len());
    let mut pt_ty = Vec::with_capacity(input.variants.len());
    for variant in &input.variants {
        if variant.discriminant.is_some() {
            syn_error!(
                variant,
                "derive_syntax does not support enums with discriminants"
            );
        }
        pt_ident.push(&variant.ident);
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                let x = match ensure_one(fields.unnamed.iter()) {
                    EnsureOne::One(x) => x,
                    _ => {
                        syn_error!(
                            fields,
                            "derive_syntax only supports variant with exactly one unnamed field"
                        );
                    }
                };
                pt_ty.push(&x.ty);
            }
            _ => {
                syn_error!(
                    variant,
                    "derive_syntax only supports variant with exactly one unnamed field"
                );
            }
        }
    }
    let first_ty = match pt_ty.first() {
        Some(x) => x,
        None => syn_error!(&ident, "derive_syntax needs at least 1 variant in the enum"),
    };

    let mut apply_semantic_impl = Vec::with_capacity(input.variants.len());
    for (variant, attrs) in std::iter::zip(input.variants.iter(), variant_attrs.into_iter()) {
        let variant_attr = parse_field_attributes(variant, attrs)?;
        if let Some(semantic) = variant_attr.semantic {
            apply_semantic_impl.push(quote! {
                parser.apply_semantic(
                    &inner,
                    #teleparse::token_set!(<Self::Prod as #teleparse::syntax::Production>::L{
                        #( #semantic )|*
                    })
                );
            });
        } else {
            apply_semantic_impl.push(quote! {});
        }
    }

    let i = 0..pt_ty.len();

    let root_derive = if root_attr.root {
        Some(root::expand(&ident))
    } else {
        None
    };

    let (name, prod_struct) = derived_prod_name(&ident);
    let output = quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #vis struct #prod_struct;
        #[automatically_derived]
        impl #teleparse::syntax::Production for #prod_struct {
            type L = <<#first_ty as #teleparse::parser::Produce>::Prod as #teleparse::syntax::Production>::L;
            #[inline]
            fn debug() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#name)
            }
            fn register(meta: &mut #teleparse::syntax::MetadataBuilder<Self::L>) {
                #teleparse::register_union!(meta, #( <#pt_ty as #teleparse::parser::Produce>::Prod ),*);
            }
        }
        #[automatically_derived]
        impl #teleparse::parser::Produce for #ident {
            type Prod = #prod_struct;
            fn produce(
                parser: &mut #teleparse::parser::Parser<'_, <Self::Prod as #teleparse::syntax::Production>::L>,
                meta: &#teleparse::syntax::Metadata<<Self::Prod as #teleparse::syntax::Production>::L>,
            ) -> #teleparse::syntax::Result<Self, <Self::Prod as #teleparse::syntax::Production>::L> {
                use #teleparse::syntax::Production;
                let t = Self::prod_id();
                let token_src = parser.peek_token_src();
                match meta.jump.look_up(&t, token_src) {
                #(
                    Some(#i) => {
                        <#pt_ty>::produce(parser, meta).map(|inner| {
                            #apply_semantic_impl
                            Self::#pt_ident(inner)
                        })
                    }
                )*
                    _ => {
                        let first = meta.first.get(&t);
                        let err = parser.expecting(first.clone());
                        let err_vec = ::std::vec![err];
                        #teleparse::syntax::Result::Panic(err_vec)
                    },
                }
            }
        }
        #root_derive
    };

    Ok(anon_const_block(output))
}

fn derived_prod_name(ident: &syn::Ident) -> (String, syn::Ident) {
    let name = ident.to_string();
    let len = name.len();
    (name, format_ident!("DerivedProd_{}_{}", len, ident))
}

struct RootAttr {
    root: bool,
    no_test: bool,
}

fn parse_root_attributes(input: &mut syn::DeriveInput) -> syn::Result<RootAttr> {
    let root_metas = parse_strip_root_meta_optional(input)?;
    let mut root = false;
    let mut no_test = false;
    if let Some(root_metas) = root_metas {
        for meta in root_metas {
            match meta {
                syn::Meta::Path(path) => {
                    if path.is_ident("root") {
                        root = true;
                    } else if path.is_ident("no_test") {
                        no_test = true;
                    } else {
                        syn_error!(path, "Unknown attribute");
                    }
                }
                _ => {
                    syn_error!(meta, "Unknown attribute");
                }
            }
        }
    }
    if no_test && !root {
        syn_error!(input, "no_test can only be used with root");
    }
    Ok(RootAttr { root, no_test })
}

struct FieldAttr {
    semantic: Option<Vec<syn::Ident>>,
    // hook: Option<syn::Ident>,
}

fn parse_field_attributes<T: ToTokens>(
    field: &T,
    attrs: Vec<syn::Attribute>,
) -> syn::Result<FieldAttr> {
    let mut semantic = None;
    let mut hook = None;
    let attr = match ensure_one(attrs) {
        EnsureOne::None => return Ok(FieldAttr { semantic }),
        EnsureOne::More => syn_error!(
            field,
            "Multiple {} attributes found! You might want to merge them.",
            CRATE
        ),
        EnsureOne::One(attr) => attr,
    };
    for meta in parse_crate_attr_meta(&attr)? {
        match meta {
            syn::Meta::List(meta) => {
                if meta.path.is_ident("semantic") {
                    if semantic.is_some() {
                        syn_error!(
                            meta,
                            "Duplicated `semantic` attribute. You might want to merge them."
                        );
                    }
                    semantic = Some(
                        meta.parse_args_with(
                            Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated,
                        )?
                        .into_iter()
                        .collect::<Vec<_>>(),
                    );
                    continue;
                }
                if meta.path.is_ident("hook") {
                    if hook.is_some() {
                        syn_error!(meta, "Duplicated `hook` attribute. There can only be one hook per field. You can wrap the hooks in one function.");
                    }
                    hook = Some(meta.parse_args::<syn::Ident>()?);
                    continue;
                }
            }
            _ => syn_error!(meta, "Invalid attribute format. Expected <attr>(<args>)"),
        }
    }

    Ok(FieldAttr { semantic })
}

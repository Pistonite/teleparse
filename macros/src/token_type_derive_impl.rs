use crate::*;

/// Derive the `TokenType` trait for an enum and generate variant representation
pub fn expand(input: TokenStream, derive_ident: &syn::Ident) -> TokenStream {
    let mut derive_input = {
        let input = input.clone();
        parse_macro_input!(input as syn::DeriveInput)
    };
    let result = expand_internal(&mut derive_input, derive_ident);
    from_result_keep_input(quote!{#derive_input}, result)
}

fn expand_internal(input: &mut syn::DeriveInput, derive_ident: &syn::Ident) -> syn::Result<TokenStream2> {
    // parse the root attributes
    let root_metas = parse_strip_root_meta(input,derive_ident)?;
    let mut ignore_regexes = Vec::new();
    let mut context_ty = None;
    for meta in root_metas {
        match meta {
            syn::Meta::List(meta) => {
                if meta.path.is_ident("ignore") {
                    ignore_regexes.push(meta.parse_args::<syn::LitStr>()?);
                    continue;
                }
                if meta.path.is_ident("context") {
                    if context_ty.is_some() {
                        syn_error!(meta, "Multiple `context` attributes found for TokenType! Keep only 1");
                    }
                    context_ty = Some(meta.parse_args::<syn::Type>()?);
                    continue;
                }
                syn_error!(meta, "Unknown attribute for TokenType");
            },
            _ => syn_error!(meta, "Unknown attribute for TokenType")
        }
    }

    let context_ty = context_ty.unwrap_or(unit_type());

    // has to be enum
    let enum_data = match &mut input.data {
        syn::Data::Enum(data) => data,
        _ => syn_error!(input, "TokenType can only be derived for enums")
    };
    if !input.generics.params.is_empty() {
        syn_error!(input, "TokenType derive cannot be used with generics")
    }
    let (repr, repr_str) = match enum_data.variants.len() {
        0 => syn_error!(input, "TokenType derive cannot be used with enums with no variants"),
        1..=8 => (quote!(u8), "u8"),
        9..=16 => (quote!(u16), "u16"),
        17..=32 => (quote!(u32), "u32"),
        33..=64 => (quote!(u64), "u64"),
        65..=128 => (quote!(u128), "u128"),
        _ => syn_error!(input, "TokenType derive can have at most 128 variants")
    };

    // strip attributes early for better error experience
    let variant_attrs = enum_data.variants.iter_mut().map(|v| {
        strip_take_attrs(&mut v.attrs)
    }).collect::<Vec<_>>();

    let llnparse = crate_ident();
    let enum_ident = &input.ident; 
    let enum_vis = &input.vis;
    // parse enum body
    let mut x = 1u128;
    let mut enum_body = TokenStream2::new();
    let mut should_extract_match_clauses = TokenStream2::new();
    let mut extra_derives = TokenStream2::new();
    for (variant, attrs) in enum_data.variants.iter_mut().zip(variant_attrs) {
        if !matches!(variant.fields, syn::Fields::Unit) {
            syn_error!(variant, "TokenType derive must be used with enums with only unit variants, as integer representation will be generated");
        }
        if attrs.is_empty() {
            syn_error!(variant, "Missing `llnparse` attribute for variant to derive TokenType");
        }

        // make enum body
        let ident = &variant.ident;
        let num = syn::LitInt::new(&format!("0x{x:x}{repr_str}"), Span::call_site());
        enum_body.extend(quote! {
            #ident = #num,
        });
        x <<= 1;

        // check for attributes
        let mut terminal = None;
        let mut regex = None;
        for attr in attrs {
            let metas = parse_crate_attr_meta(&attr)?;
            for meta in metas {
                let meta = match meta {
                    syn::Meta::List(meta) => meta,
                    _ => syn_error!(meta, "Unknown attribute for deriving TokenType enum variant")
                };
                if meta.path.is_ident("regex") {
                    if regex.is_some() {
                        syn_error!(meta, "Multiple `regex` attributes found for TokenType enum variant! You can put all regexes into the same attribute and separate them with comma");
                    }
                    regex = Some(
                        meta
                            .parse_args_with(
                                Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated
                            )?);
                    continue;
                }
                if meta.path.is_ident("terminal") {
                    if terminal.is_some() {
                        syn_error!(meta, "Multiple `terminal` attributes found for TokenType enum variant! You might want to merge them.");
                    }
                    terminal = Some(meta.
                    parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)?);
                    continue;
                }
            }
        }

        // derive the terminals
    //     let mut infer_literals = Some(Vec::new());
    //     if let Some(terminal) = terminal {
    //         for meta in terminal {
    //             let ident = match meta.path().get_ident() {
    //                 Some(ident) => ident,
    //                 None => syn_error!(meta, "Identifier for terminal struct expected."),
    //             };
    //             let original_doc = get_doc(&variant.attrs);
    //             match meta {
    //                 syn::Meta::Path(meta) => {
    //                     infer_literals = None;
    //                     let doc = format!(
    //                         "SyntaxTree terminal derived from [`{}`] with `terminal({})`", enum_ident
    //                         , ident
    //                     ) ;
    //                     extra_derives.extend(quote! {
    //                         #[doc = #doc]
    //                         #[automatically_derived]
    //                         #enum_vis struct #ident(#llnparse::Token<#enum_ident>);
    //                         #[automatically_derived]
    //                         const _: () = {
    //                             use #llnparse::SyntaxTree;
    //                             impl SyntaxTree for #ident {
    //                                 type T = #enum_ident;
    //                                 type Ctx  = #context_ty;
    //                                 type AST = #llnparse::Token<#enum_ident>;
    //
    //                                 #[inline]
    //                                 fn span_of(ast: &Self::AST) -> Span { ast.span }
    // fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
    //     parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    // ) -> SyntaxResult<Self::AST>;
    //                             }
    //                         }
    //                     });
    //                 }
    //             }
    //            
    //         }
    //     }

        // for attr in &variant.attrs {
        //     for meta in metas {
        //         match meta {
        //             syn::Meta::Path(path) => {
        //                 if path.is_ident("extract") {
        //                     extract = true;
        //                 }
        //             }
        //             meta => {
        //                 syn_error!(meta, "Unknown attribute for deriving TokenType");
        //             }
        //         }
        //     }
        // }
        //
        // if extract {
        //     should_extract_match_clauses.extend(quote! {
        //         Self::#ident => true,
        //     });
        // }
        
    }
    // let enum_first_ident = &enum_data.variants.first().unwrap().ident;
    // let enum_last_ident = &enum_data.variants.last().unwrap().ident;
    // let out = quote! {
    //     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    //     #[repr(#repr)]
    //     #enum_vis enum #enum_name {
    //         #enum_body
    //     }
    //     #[automatically_derived]
    //     impl #llnparse::TokenType for #enum_name {
    //         type Repr = #repr;
    //         #[inline]
    //         fn should_extract(&self) -> bool {
    //             match self {
    //                 #should_extract_match_clauses
    //                 _ => false
    //             }
    //         }
    //         #[inline]
    //         fn to_repr(&self) -> Self::Repr {
    //             *self as Self::Repr
    //         }
    //         #[inline]
    //         fn first() -> Self {
    //             Self::#enum_first_ident
    //         }
    //         fn next(&self) -> Option<Self> {
    //             match self {
    //                 Self::#enum_last_ident => None,
    //                 _ => {
    //                     let repr = self.to_repr();
    //                     let next = repr << 1;
    //                     Some(unsafe { std::mem::transmute(next) })
    //                 }
    //             }
    //         }
    //     }
    // };

    // Ok(out)
    todo!()
}

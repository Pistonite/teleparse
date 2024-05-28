use crate::*;

/// Derive the `Lexer` trait for a struct that has a LexerState field
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
    let root_metas = parse_strip_root_meta(input, derive_ident)?;
    let mut token_ty = None;
    // // TODO - context
    // // TODO - start_map
    for meta in &root_metas {
        let meta = match meta {
            syn::Meta::List(meta) => meta,
            _ => continue
        };
        if !meta.path.is_ident("token") {
            continue;
        }
        if token_ty.is_some() {
            syn_error!(meta, "multiple `token` attributes found for SyntaxTree! Keep only 1");
        }
        token_ty = Some(meta.parse_args::<syn::Path>()?);
    }

    let token_ty = match token_ty {
        Some(token_ty) => token_ty,
        None => syn_error!(derive_ident, "SyntaxTree must have a `token` attribute"),
    };

    let root_meta = RootMeta {
        token_ty
    };

    match &mut input.data {
        syn::Data::Struct(data) => {
            expand_struct(data, root_meta)
        }
        syn::Data::Enum(data) => {
            expand_enum(data, root_meta)
        }
        _ => syn_error!(input, "SyntaxTree can only be derived for structs or enums")
    }
}

struct RootMeta {
    token_ty: syn::Path,
}

fn expand_struct(input: &mut syn::DataStruct, root_meta: RootMeta) -> syn::Result<TokenStream2> {
    todo!()
}

fn expand_enum(input: &mut syn::DataEnum, root_meta: RootMeta) -> syn::Result<TokenStream2> {
    todo!()
}

fn parse_meta<'a>(field: &'a syn::Field, metas: Vec<syn::Meta>) -> syn::Result<Vec<ParseMeta>> {
    // let attrs = strip_take_attrs(&mut field.attrs);
    // if attrs.is_empty() {
    //     // should not be empty as at least one should be inferred
    //     panic!("todo");
    // }
    let mut out = Vec::with_capacity(metas.len());
    for (i, meta) in metas.into_iter().enumerate() {
        let path = meta.path();
        if path.is_ident("blanket") {
            if i != 0 {
                syn_error!(field, "`blanket` must be first (innermost) in the parser chain");
            }
            if let syn::Meta::List(meta) = meta {
                let ty = meta.parse_args::<syn::Type>()?;
                out.push(ParseMeta::Blanket(ty));
                continue;
            }
            syn_error!(meta, "Usage: `blanket(Type)`");
        } else if path.is_ident("token") {
            if i != 0 {
                syn_error!(field, "`token` must be first (innermost) in the parser chain");
            }
            if let syn::Meta::List(meta) = meta {
                if let Ok(meta) = meta.parse_args::<syn::MetaNameValue>() {
                        let ident = match meta.path.get_ident() {
                            Some(ident) => ident.clone(),
                            None => syn_error!(meta, "Usage: `token(Ident = \"match\")`"),
                        };
                        let lit = match meta.value {
                            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => lit_str,
                            _ => syn_error!(meta, "Expected the match expression to be a literal string. Usage: `token(Ident = \"match\")`"),
                        };
                        out.push(ParseMeta::Token(ident.clone(), Some(lit)));
                    continue;
                }
                let ident = meta.parse_args::<syn::Ident>()?;
                out.push(ParseMeta::Token(ident, None));
                continue;
            }
            syn_error!(meta, "Usage: `token(Ident)` or `token(Ident = \"match\")`");
        }
    }
    Ok(out)
    // // assume the field is a SyntaxTree, and use blanket
    // return Ok(vec![ParseAttr::Blanket(&field.ty)]);
}

enum ParseMeta {
    /// blanket(Type)
    Blanket(syn::Type),
    /// token(Ident) | token(Ident = "match")
    Token(syn::Ident, Option<syn::LitStr>),
}

struct StructDeriveState<'a> {
    token_ty: &'a syn::Path,
    parse_body: TokenStream2,
}

impl<'a> StructDeriveState<'a> {
    /// Derive for one field
    fn derive_field(&mut self, field: &syn::Field, attrs: &[ParseMeta]) -> syn::Result<()> {
        // expression used to construct the parser
        let mut parser = TokenStream2::new();
        let llnparse = crate_ident();
        for (i, attr) in attrs.iter().enumerate() {
            match attr {
                ParseMeta::Blanket(ty) => {
                    parser = quote! {
                        #llnparse::imp::blanket::BlanketParser::<#ty>::new()
                    };
                }
                ParseMeta::Token(ident, match_lit) => {
                    let token_ty = &self.token_ty;
                    match match_lit {
                        Some(lit) => {
                            parser = quote! {
                                #llnparse::imp::token::TokenParser::with_match_lit(#token_ty::#ident, #lit)
                            };
                        }
                        None => {
                            parser = quote! {
                                #llnparse::imp::token::TokenParser::new(#token_ty::#ident)
                            };
                        }
                    }
                }
            }
        }
        match &field.ident {
            Some(ident) => {
                // named field
                self.parse_body.extend(quote! {
                    #ident: #parser.try_parse(parser).unwrap_or_extend_errors(&mut errors)?,
                });

            }
            None => {
                self.parse_body.extend(quote! {
                    #parser.try_parse(parser).unwrap_or_extend_errors(&mut errors)?,
                });

            }
        }

        Ok(())
    }
}


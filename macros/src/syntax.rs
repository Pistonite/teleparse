use quote::quote_spanned;

use crate::*;

pub fn expand(input: &mut syn::DeriveInput, lex_ty: syn::Type) -> syn::Result<TokenStream2> {
    parse_root_attributes(input)?;

    let output = match &mut input.data {
        syn::Data::Struct(data) => {
            expand_struct(input.ident.clone(), data, lex_ty)?
        }
        syn::Data::Enum(data) => {
            expand_enum(input.ident.clone(), data, lex_ty)?
        }
        _ => syn_error!(input, "derive_syntax can only be used with structs or enums")
    };

    let output = quote! {
        #input
        #output
    };

    Ok(output)
}

fn expand_struct(ident: syn::Ident, input: &mut syn::DataStruct, lex_ty: syn::Type) -> syn::Result<TokenStream2> {
    match &mut input.fields {
        syn::Fields::Unnamed(fields) => {
            expand_struct_unnamed(ident, fields, lex_ty)
        }
        syn::Fields::Named(fields) => {
            expand_struct_named(ident, fields, lex_ty)
        }
        syn::Fields::Unit => {
            syn_error!(ident, "derive_syntax does not support unit structs");
        }
    }
}

fn expand_struct_unnamed(ident: syn::Ident, input: &mut syn::FieldsUnnamed, lex_ty: syn::Type) -> syn::Result<TokenStream2> {
    let pt_ty = input.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let teleparse = crate_ident();
    let last = pt_ty.len()-1;
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#lex_ty)]
        struct DerivedAST(#( #teleparse::parser::AstOf< #pt_ty > ),*);
        #[automatically_derived]
        impl #teleparse::ToSpan for DerivedAST {
            fn to_span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;
            fn from_ast<'s>(ast: Self::AST, parser: &mut #teleparse::parser::Parser<'s, Self::AST>) -> Self {
                Self( #( <#pt_ty>::from_ast(ast, parser)),*)
            }
        }
    };

    Ok(anon_const_block(output))
}

fn expand_struct_named(ident: syn::Ident, input: &mut syn::FieldsNamed, lex_ty: syn::Type) -> syn::Result<TokenStream2> {
    let pt_ty = input.named.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let pt_ident = input.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let teleparse = crate_ident();
    let last = pt_ty.len()-1;
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#lex_ty)]
        struct DerivedAST(#( #teleparse::parser::AstOf< #pt_ty > ),*);
        #[automatically_derived]
        impl #teleparse::ToSpan for DerivedAST {
            fn to_span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;
            fn from_ast<'s>(ast: Self::AST, parser: &mut #teleparse::parser::Parser<'s, Self::AST>) -> Self {
                let ( #(#pt_ident),* ) = ( #( <#pt_ty>::from_ast(ast, parser)),*);
                Self { #(#pt_ident),* }
            }
        }
    };
    Ok(anon_const_block(output))
}

fn expand_enum(ident: syn::Ident, input: &mut syn::DataEnum, lex_ty: syn::Type) -> syn::Result<TokenStream2> {
    let mut pt_ident = Vec::with_capacity(input.variants.len());
    let mut pt_ty: Vec<syn::Type> = Vec::with_capacity(input.variants.len());
    for variant in &mut input.variants {
        if variant.discriminant.is_some() {
            syn_error!(variant, "derive_syntax does not support enums with discriminants");
        }
        pt_ident.push(variant.ident.clone());
        match &mut variant.fields {
            syn::Fields::Named(fields) => {
                syn_error!(fields, "derive_syntax does not support named fields in enums. Please extract them into a struct");
            }
            syn::Fields::Unnamed(fields) => {
                let mut iter = fields.unnamed.iter();
                let first = match iter.next() {
                    Some(x) => x,
                    None => {
                        syn_error!(fields, "enum variant in derive_syntax must either be unit or have a single unnamed field");
                    }
                };
                if iter.next().is_some() {
                    syn_error!(fields, "enum variant in derive_syntax must either be unit or have a single unnamed field");
                }
                pt_ty.push(first.ty.clone());
            }
            unit => {
                let id = &variant.ident;
                let t = quote_spanned! { id.span() => #id(#id) };
                let v = syn::parse2::<syn::Variant>(t).expect("internal error in derive_syntax: fail to parse enum variant");
                *unit = v.fields;
                let ty = ident_to_type(id);
                pt_ty.push(ty);
            }
        }
    }

    let teleparse = crate_ident();

    
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#lex_ty)]
        #[derive(#teleparse::ToSpan)]
        #[doc(hidden)]
        enum DerivedAST {
            #( #pt_ident(#teleparse::parser::AstOf< #pt_ty >), )*
        }

        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;

            fn from_ast<'s>(ast: Self::AST, parser: &mut #teleparse::parser::Parser<'s, Self::AST>) -> Self {
                match ast {
                    #(
                        DerivedAST::#pt_ident(ast) => Self::#pt_ident(<#pt_ty>::from_ast(ast, parser)),
                    )*
                }
            }
        }
    };

    Ok(anon_const_block(output))
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

fn parse_root_attributes(input: &mut syn::DeriveInput) -> syn::Result<()> {
    let root_metas = parse_strip_root_meta_optional(input)?;
    if root_metas.is_some() {
        syn_error!(input, "derive_syntax does not support any root attributes");
    }
    Ok(())
}

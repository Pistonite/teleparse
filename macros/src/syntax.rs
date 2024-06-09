use quote::quote_spanned;

use crate::*;

pub fn expand(input: &mut syn::DeriveInput) -> syn::Result<TokenStream2> {
    let root_attr = parse_root_attributes(input)?;

    let teleparse = crate_ident();

    let (extra_derive, output) = match &mut input.data {
        syn::Data::Struct(data) => {
            (None, expand_struct(input.ident.clone(), data, &root_attr)?)
        }
        syn::Data::Enum(data) => {
            (Some(
                quote! {#[derive(#teleparse::ToSpan)]}
            ), expand_enum(input.ident.clone(), data, &root_attr)?)
        }
        _ => syn_error!(input, "derive_syntax can only be used with structs or enums")
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

fn expand_struct(ident: syn::Ident, input: &mut syn::DataStruct, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
    match &mut input.fields {
        syn::Fields::Unnamed(fields) => {
            expand_struct_unnamed(ident, fields, root_attr)
        }
        syn::Fields::Named(fields) => {
            expand_struct_named(ident, fields, root_attr)
        }
        syn::Fields::Unit => {
            syn_error!(ident, "derive_syntax does not support unit structs");
        }
    }
}

fn expand_struct_unnamed(ident: syn::Ident, input: &mut syn::FieldsUnnamed, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
    if input.unnamed.is_empty() {
        syn_error!(input, "derive_syntax does not support struct with no fields");
    }
    let pt_ty = input.unnamed.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let teleparse = crate_ident();
    let last = syn::Index::from(pt_ty.len()-1);
    let indices = (0..pt_ty.len()).map(syn::Index::from);
    let root_derive = if root_attr.root {
        Some(root::expand(quote! { DerivedAST }, &ident))
    } else {
        None
    };
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#ident)]
        struct DerivedAST(#( #teleparse::parser::AstOf< #pt_ty > ),*);
        #[automatically_derived]
        impl #teleparse::ToSpan for DerivedAST {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::ToSpan for #ident {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;
            fn from_ast<'s>(
                ast: Self::AST, 
                parser: &mut #teleparse::parser::Parser<'s, <Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L>
            ) -> Self {
                Self( #( <#pt_ty>::from_ast(ast.#indices, parser)),*)
            }
        }
        #root_derive
    };

    Ok(anon_const_block(output))
}

fn expand_struct_named(ident: syn::Ident, input: &mut syn::FieldsNamed, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
    if input.named.is_empty() {
        syn_error!(input, "derive_syntax does not support struct with no fields");
    }
    let pt_ty = input.named.iter().map(|f| &f.ty).collect::<Vec<_>>();
    let pt_ident = input.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let first_ident = pt_ident.first().unwrap();
    let last_ident = pt_ident.last().unwrap();
    let teleparse = crate_ident();
    let last = syn::Index::from(pt_ty.len()-1);
    let indices = (0..pt_ty.len()).map(syn::Index::from);
    let root_derive = if root_attr.root {
        Some(root::expand(quote! { DerivedAST }, &ident))
    } else {
        None
    };
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#ident)]
        struct DerivedAST(#( #teleparse::parser::AstOf< #pt_ty > ),*);
        #[automatically_derived]
        impl #teleparse::ToSpan for DerivedAST {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.0.span().lo, self.#last.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::ToSpan for #ident {
            fn span(&self) -> #teleparse::Span {
                #teleparse::Span::new(self.#first_ident.span().lo, self.#last_ident.span().hi)
            }
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;
            fn from_ast<'s>(
                ast: Self::AST, 
                parser: &mut #teleparse::parser::Parser<'s, <Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L>
            ) -> Self {
                let ( #(#pt_ident),* ) = ( #( <#pt_ty>::from_ast(ast.#indices, parser)),*);
                Self { #(#pt_ident),* }
            }
        }
        #root_derive
    };
    Ok(anon_const_block(output))
}

fn expand_enum(ident: syn::Ident, input: &mut syn::DataEnum, root_attr: &RootAttr) -> syn::Result<TokenStream2> {
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

    let root_derive = if root_attr.root {
        Some(root::expand(quote! { DeriveAST }, &ident))
    } else {
        None
    };
    
    let output = quote! {
        #[#teleparse::__priv::derive_ast(#ident)]
        #[derive(#teleparse::ToSpan)]
        #[doc(hidden)]
        enum DerivedAST {
            #( #pt_ident(#teleparse::parser::AstOf< #pt_ty >), )*
        }
        #[automatically_derived]
        impl #teleparse::parser::ParseTree for #ident {
            type AST = DerivedAST;

            fn from_ast<'s>(
                ast: Self::AST, 
                parser: &mut #teleparse::parser::Parser<'s, <Self::AST as #teleparse::syntax::AbstractSyntaxTree>::L>
            ) -> Self {
                match ast {
                    #(
                        DerivedAST::#pt_ident(ast) => Self::#pt_ident(<#pt_ty>::from_ast(ast, parser)),
                    )*
                }
            }
        }
        #root_derive
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

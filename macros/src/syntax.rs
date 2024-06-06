use quote::quote_spanned;

use crate::*;

pub fn expand(input: &mut syn::DeriveInput, lexicon_ident: syn::Ident) -> syn::Result<TokenStream2> {
    parse_root_attributes(input)?;

    let output = match &mut input.data {
        syn::Data::Struct(data) => {
            expand_struct(data, lexicon_ident)?
        }
        syn::Data::Enum(data) => {
            expand_enum(input.ident.clone(), data, lexicon_ident)?
        }
        _ => syn_error!(input, "derive_syntax can only be used with structs or enums")
    };

    let output = quote! {
        #input
        #output
    };

    Ok(output)
}

fn expand_struct(input: &mut syn::DataStruct, lexicon_ident: syn::Ident) -> syn::Result<TokenStream2> {
    todo!()
}

fn expand_enum(ident: syn::Ident, input: &mut syn::DataEnum, lexicon_ident: syn::Ident) -> syn::Result<TokenStream2> {
    let mut pt_idents = Vec::with_capacity(input.variants.len());
    let mut pt_types: Vec<syn::Type> = Vec::with_capacity(input.variants.len());
    for variant in &mut input.variants {
        if variant.discriminant.is_some() {
            syn_error!(variant, "derive_syntax does not support enums with discriminants");
        }
        pt_idents.push(variant.ident.clone());
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
                pt_types.push(first.ty.clone());
            }
            unit => {
                let id = &variant.ident;
                let t = quote_spanned! { id.span() => #id(#id) };
                let v = syn::parse2::<syn::Variant>(t).expect("internal error in derive_syntax: fail to parse enum variant");
                *unit = v.fields;
                let ty = ident_to_type(id);
                pt_types.push(ty);
            }
        }
    }

    let teleparse = crate_ident();
    let ident_str = ident.to_string();

    let mut ast_body = TokenStream2::new();
    let mut type_id_list = TokenStream2::new();
    let mut build_first = TokenStream2::new();
    let mut check_left_recursive = TokenStream2::new();
    let mut check_first_conflict = TokenStream2::new();
    let mut check_first_conflict_recur = TokenStream2::new();
    let mut build_follow = TokenStream2::new();
    let mut check_first_follow_conflict = TokenStream2::new();
    let mut build_jump = TokenStream2::new();
    let mut build_jump_recur = TokenStream2::new();
    let mut parse = TokenStream2::new();
    let mut from_ast = TokenStream2::new();
    let pt_count = pt_idents.len();
    for (i, (id, ty)) in std::iter::zip(pt_idents.iter(), pt_types.iter()).enumerate() {
        let ast_ty = ast_of(ty);
        ast_body.extend(quote! {
            #id(#ast_ty),
        });
        type_id_list.extend(quote! {
            <#ast_ty>::type_id(),
        });
        build_first.extend(quote! {
            <#ast_ty>::build_first(builder);
        });
        build_follow.extend(quote! {
            <#ast_ty>::build_follow(builder);
        });
        let is_last = i == pt_count - 1;
        if is_last {
            check_left_recursive.extend(quote! {
                let r = <#ast_ty>::check_left_recursive(stack, seen);
            });
        } else {
            check_left_recursive.extend(quote! {
                if let Err(e) = <#ast_ty>::check_left_recursive(stack, seen) {
                    stack.pop();
                    seen.remove(&t);
                    return Err(e);
                }
            });
        }
        check_first_conflict.extend(quote! {
            let first_set = first.get(<#ast_ty>::type_id());
            if check_set.intersects(&first_set) {
                let self_name = Self::debug().into_owned();
                let produce_name = <#ast_ty>::debug().into_owned();
                let intersection = check_set
                    .intersection_repr(&first_set)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ");

                return Err(GrammarError::FirstFirstConflict(
                    self_name,
                    produce_name,
                    intersection));
            }
        });
        if !is_last {
            check_first_conflict.extend(quote! {
                check_set.union(&first_set);
            });
        }
        check_first_conflict_recur.extend(quote! {
            <#ast_ty>::check_first_conflict(seen, first)?;
        });
        check_first_follow_conflict.extend(quote! {
            <#ast_ty>::check_first_follow_conflict(seen, first, follow)?;
        });
        build_jump.extend(quote! {
            let id = <#ast_ty>::type_id();
            let first_set = first.get(&id);
            jump.register(t, first_set, #i);
        });
        build_jump_recur.extend(quote! {
            <#ast_ty>::build_jump(seen, jump);
        });
        parse.extend(quote! {
            Some(#i) => <#ast_ty>::parse(parser, meta),
        });
        from_ast.extend(quote! {
            DerivedAST::#id(ast) => Self::#id(<#ty>::from_ast(ast, parser)),
        });
    }

    
    let output = quote! {
        #[automatically_derived]
        const _: () = {
            use #teleparse::syntax::{
                First, FirstBuilder, FirstSet, 
                Follow, Jump,
                AbstractSyntaxTree, Metadata
            };
            use #teleparse::parser::{Parser, ParseTree, AstOf};
            use #teleparse::{ToSpan, GrammarError};
            use ::std::borrow::Cow;
            use ::std::collections::BTreeSet;
            use ::std::vec::Vec;
            #[derive(ToSpan)]
            #[doc(hidden)]
            pub enum DerivedAST {
                #ast_body
            }
            impl AbstractSyntaxTree for DerivedAST {
                type L = #lexicon_ident;

                #[inline]
                fn debug() -> Cow<'static, str> {
                    Cow::Borrowed(#ident_str)
                }

                fn build_first(builder: &mut FirstBuilder<Self::L>) {
                    let t = Self::type_id();
                    if builder.visit(t) {
                        #build_first
                        builder.build_enum(t, &[#type_id_list]);
                    }
                }

                fn check_left_recursive(stack: &mut Vec<String>, seen: &mut BTreeSet<TypeId>) -> Result<(), GrammarError> {
                    let t = Self::type_id();
                    if !seen.insert(t) {
                        return Err(GrammarError::left_recursion(&stack, &Self::debug()));
                    }
                    stack.push(Self::debug().into_owned());
                    #check_left_recursive
                    stack.pop();
                    seen.remove(&t);
                    r
                }

                fn check_first_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError> {
                    let t = Self::type_id();
                    if !seen.insert(t) {
                        return Ok(());
                    }
                    let mut check_set = FirstSet::new();
                    #check_first_conflict
                    #check_first_conflict_recur
                    Ok(())
                }

                fn build_follow(builder: &mut FollowBuilder<Self::L>) {
                    let t = Self::type_id();
                    if builder.visit(t) {
                        #build_follow
                        builder.build_enum(t, &[#type_id_list]);
                    }
                }

                fn check_first_follow_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, follow: &Follow<Self::L>) -> Result<(), GrammarError> {
                    let t = Self::type_id();
                    if !seen.insert(t) {
                        return Ok(());
                    }
                    Self::check_self_first_follow_conflict(first, follow)?;
                    #check_first_follow_conflict
                    Ok(())
                }

                fn build_jump(seen: &mut BTreeSet<TypeId>, jump: &mut Jump<Self::L>) {
                    let t = Self::type_id();
                    if !seen.insert(t) {
                        return;
                    }
                    #build_jump
                    #build_jump_recur
                }

                fn parse_ast<'s>(
                    parser: &mut Parser<'s, Self::L>, 
                    meta: &Metadata<Self::L>,
                ) -> #teleparse::syntax::Result<Self, Self::L> {
                    let t = Self::type_id();
                    let token_src = parser.peek_token_src();
                    match meta.jump.look_up(&t, token_src) {
                        #parse
                        _ => {
                            let first = meta.first.get(&t);
                            let err = parser.expecting(first.clone());
                            let err_vec = ::std::vec![err];
                            #teleparse::syntax::Result::Panic(err_vec)
                        },
                    }
                }

            }

            impl ParseTree for #ident {
                type AST = DerivedAST;

                fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::AST>) -> Self {
                    match ast {
                        #from_ast
                    }
                }
            }
        };
    };

    Ok(output)
}

fn ast_of(ty: &syn::Type) -> TokenStream2 {
    // note there needs to be spaces around ty
    quote_spanned! { ty.span() => AstOf < #ty > }
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

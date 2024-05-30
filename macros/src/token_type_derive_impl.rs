use std::collections::BTreeSet;

use crate::*;

/// Derive the `TokenType` trait for an enum and generate variant representation
pub fn expand(input: TokenStream, derive_ident: &syn::Ident) -> TokenStream {
    let mut derive_input = {
        let input = input.clone();
        parse_macro_input!(input as syn::DeriveInput)
    };
    let result = expand_internal(&mut derive_input, derive_ident);
    from_result_keep_input(quote! {#derive_input}, result)
}

fn expand_internal(
    input: &mut syn::DeriveInput,
    derive_ident: &syn::Ident,
) -> syn::Result<TokenStream2> {
    // parse the root attributes
    let RootAttr { ignore, context_ty } = parse_root_attributes(input, derive_ident)?;
    let mut rule_exprs = Vec::new();
    let teleparse = crate_ident();
    for ignore in ignore {
        checked_regex_rule(&ignore)?;
        rule_exprs.push(quote! {
            #teleparse::lexer::LexerRule::ignore(#ignore),
        });
    }
    let (variant_attrs, enum_data) = {
        // strip attributes early for better error experience
        let mut variant_attrs = Vec::new();
        // has to be enum
        match &mut input.data {
            syn::Data::Enum(data) => {
            for variant in &mut data.variants {
                variant_attrs.push(strip_take_attrs(&mut variant.attrs));
            }
            },
            _ => syn_error!(input, "TokenType can only be derived for enums"),
        };

        let enum_data = match &input.data {
            syn::Data::Enum(data) => data,
            _ => unreachable!(),
        };

        (variant_attrs, enum_data)
    };
    if !input.generics.params.is_empty() {
        syn_error!(input, "TokenType derive cannot be used with generics")
    }
    let (repr_ty, repr_one) = match enum_data.variants.len() {
        0 => syn_error!(
            input,
            "TokenType derive cannot be used with enums with no variants"
        ),
        1..=8 => (quote!(u8), "1u8"),
        9..=16 => (quote!(u16), "1u16"),
        17..=32 => (quote!(u32), "1u32"),
        33..=64 => (quote!(u64), "1u64"),
        65..=128 => (quote!(u128), "1u128"),
        _ => syn_error!(input, "TokenType derive can have at most 128 variants"),
    };
    let repr_one = syn::LitInt::new(repr_one, Span::call_site());

    let enum_ident = &input.ident;
    let enum_vis = &input.vis;
    // parse enum body
    let mut enum_body = TokenStream2::new();
    let mut should_extract_match_clauses = TokenStream2::new();
    let mut extra_derives = TokenStream2::new();
    for (i, (variant, attrs)) in enum_data.variants.iter().zip(variant_attrs).enumerate() {
        if !matches!(variant.fields, syn::Fields::Unit) {
            syn_error!(variant, "TokenType derive must be used with enums with only unit variants, as integer representation will be generated");
        }
        if attrs.is_empty() {
            syn_error!(
                variant,
                "Missing `{}` attribute for variant to derive TokenType",
                CRATE
            );
        }

        let variant_ident = &variant.ident;
        enum_body.extend(quote! {
            #variant_ident = #i,
        });

        // check for attributes
        let mut terminal = None;
        let mut regex = None;
        for attr in attrs {
            let metas = parse_crate_attr_meta(&attr)?;
            for meta in metas {
                let meta = match meta {
                    syn::Meta::List(meta) => meta,
                    _ => syn_error!(
                        meta,
                        "Unknown attribute for deriving TokenType enum variant"
                    ),
                };
                if meta.path.is_ident("regex") {
                    if regex.is_some() {
                        syn_error!(meta, "Multiple `regex` attributes found for TokenType enum variant! You can put all regexes into the same attribute and separate them with comma");
                    }
                    regex = Some(meta.parse_args_with(
                        Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated,
                    )?);
                    continue;
                }
                if meta.path.is_ident("terminal") {
                    if terminal.is_some() {
                        syn_error!(meta, "Multiple `terminal` attributes found for TokenType enum variant! You might want to merge them.");
                    }
                    terminal = Some(meta.parse_args_with(
                        Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )?);
                    continue;
                }
            }
        }

        // if no terminal is provided, this token is extract type (like comments)
        if terminal.as_ref().map(|x| x.is_empty()).unwrap_or(true) {
            should_extract_match_clauses.extend(quote! {
                Self::#variant_ident => true,
            });
        }

        // derive the terminals
        let mut infer_literals = Some(Vec::new());
        let mut infer_literal_set = BTreeSet::new();
        if let Some(terminal) = terminal {
            for meta in terminal {
                let ident = match meta.path().get_ident() {
                    Some(ident) => ident,
                    None => syn_error!(meta, "Identifier for terminal struct expected."),
                };
                match &meta {
                    // terminal(Ident)
                    syn::Meta::Path(_) => {
                        // since this will match any content of that token type
                        // we cannot infer literal patterns for the lexer
                        if infer_literals.is_none() {
                            syn_error!(
                                &meta,
                                "There can only be one terminals without a matching literal. Having multiple is likely a mistake since they are interchangable in the AST."
                            );
                        }
                        infer_literals = None;
                        let doc = format!(
                            " SyntaxTree terminal derived from [`{}`] with `terminal({})`",
                            enum_ident, ident
                        );
                        extra_derives.extend(derive_terminal(
                            &doc,
                            enum_vis,
                            &context_ty,
                            enum_ident,
                            variant_ident,
                            ident,
                            None,
                        ));
                    }
                    // terminal(Ident = "literal")
                    syn::Meta::NameValue(meta) => {
                        let value = match &meta.value {
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit_str),
                                ..
                            }) => lit_str,
                            _ => syn_error!(meta, "Expected string literal"),
                        };
                        let match_lit = value.value();
                        if match_lit.is_empty() {
                            syn_error!(value, "Cannot use empty string for matching");
                        }
                        if let Some(infer_literals) = &mut infer_literals {
                            if infer_literal_set.contains(&match_lit) {
                                syn_error!(
                                    value,
                                    "Duplicate literal pattern `{}` for token type `{}`.",
                                    match_lit,
                                    variant_ident
                                );
                            }
                            infer_literals.push(match_lit.clone());
                        }
                        infer_literal_set.insert(match_lit);
                        let doc = format!(
                            " SyntaxTree terminal derived from [`{}`] with `terminal({} = {})`",
                            enum_ident,
                            ident,
                            quote! {#value}
                        );
                        extra_derives.extend(derive_terminal(
                            &doc,
                            enum_vis,
                            &context_ty,
                            enum_ident,
                            variant_ident,
                            ident,
                            Some(&value),
                        ));
                    }
                    _ => syn_error!(
                        &meta,
                        "Invalid form for terminal. Maybe you meant `terminal({} = ...)`?",
                        ident
                    ),
                }
            }
        }

        // if liternal is not inferred, regex must be provided
        if infer_literals
            .as_ref()
            .map(|x| x.is_empty())
            .unwrap_or(true)
            && regex.is_none()
        {
            syn_error!(variant, "Missing lexer rule for this variant. If all `terminal` are specified with a literal pattern, the rule can be inferred. Otherwise you must provide a `regex`");
        }
        // add the rules
        if let Some(regexes) = regex {
            if let Some(infer_literals) = infer_literals {
                if !infer_literals.is_empty() {
                    syn_error!(
                        variant,
                        "Defining `regex` here is redundant because all terminals have a literal match pattern, so the rule can already be inferred."
                    );
                }
            }
            for regex in regexes {
                let regex_obj = checked_regex_rule(&regex)?;
                for match_lit in &infer_literal_set {
                    // if we are able to match, we must be able to match the entire string
                    if let Some(mat)= regex_obj.find(&match_lit) {
                        if mat.end() != match_lit.len() {
                            syn_error!(
                                regex,
                                "The literal pattern `{}` is not fully matched by the regex. This is like a mistake because a token produced using this regex will never be able to match the literal",
                                match_lit,
                            );
                        }
                    }
                }
                rule_exprs.push(quote! {
                    #teleparse::lexer::LexerRule::token(#enum_ident::#variant_ident, #regex),
                });
            }
        } else if let Some(infer_literals) = infer_literals {
            let mut slice = TokenStream2::new();
            for lit in infer_literals {
                slice.extend(quote! {
                    #lit,
                });
            }
            rule_exprs.push(quote! {
                #teleparse::lexer::LexerRule::token_literal(#enum_ident::#variant_ident, &[#slice]),
            });
        }
    }
    // variants size checked when determining repr
    let enum_len = enum_data.variants.len();
    let enum_first_variant = &enum_data.variants.first().unwrap().ident;
    let enum_last_variant = &enum_data.variants.last().unwrap().ident;
    let rule_count = rule_exprs.len();
    let rule_exprs = rule_exprs
        .into_iter()
        .fold(TokenStream2::new(), |mut acc, expr| {
            acc.extend(expr);
            acc
        });
    let out = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        #enum_vis enum #enum_ident {
            #enum_body
        }
        #extra_derives
        #[automatically_derived]
        const _: () = {
            use #teleparse::Lexer as _;
            impl #teleparse::TokenType for #enum_ident {
                type Bit = #repr_ty;
                type Lexer<'s> = DerivedLexer<'s>;
                type Follow = [ #teleparse::table::LitSet; #enum_len];
                type Ctx = #context_ty;

                #[inline]
                fn id(&self) -> usize {
                    *self as usize
                }

                #[inline]
                fn to_bit(&self) -> Self::Bit {
                    (1 << self.id()) as Self::Bit
                }

                #[inline]
                fn first() -> Self {
                    Self::#enum_first_variant
                }

                fn next(&self) -> Option<Self> {
                    match self {
                        Self::#enum_last_variant => None,
                        _ => {
                            let next = self.id() + 1;
                            Some(unsafe { std::mem::transmute(next) })
                        }
                    }
                }

                #[inline]
                fn should_extract(&self) -> bool {
                    match self {
                        #should_extract_match_clauses
                        _ => false
                    }
                }

                #[inline]
                fn lexer<'s>(source: &'s str) -> Self::Lexer<'s> {
                    DerivedLexer::new(source)
                }
            }
            #[doc(hidden)]
            type Rules = [ #teleparse::lexer::LexerRule<#enum_ident>; #rule_count];
            #[doc(hidden)]
            fn derived_lexer_rules() -> &'static Rules {
                static RULES: std::sync::OnceLock<Rules> = std::sync::OnceLock::new();
                RULES.get_or_init(|| { [ #rule_exprs ] })
            }
            #[doc(hidden)]
            #enum_vis struct DerivedLexer<'s>(#teleparse::lexer::LexerState<'s>, &'static Rules);
            impl<'s> #teleparse::Lexer<'s> for DerivedLexer<'s> {
                type T = #enum_ident;
                #[inline]
                fn next(&mut self) -> (Option<#teleparse::Span>, Option<#teleparse::Token<Self::T>>) {
                    self.0.next(self.1)
                }
            }
            #[doc(hidden)]
            impl<'s> DerivedLexer<'s> {
                #[inline]
                fn new(source: &'s str) -> Self {
                    Self(#teleparse::lexer::LexerState::new(source), derived_lexer_rules())
                }
            }
        };
    };

    Ok(out)
}

struct RootAttr {
    ignore: Vec<syn::LitStr>,
    context_ty: syn::Type,
}

fn parse_root_attributes(
    input: &mut syn::DeriveInput,
    derive_ident: &syn::Ident,
) -> syn::Result<RootAttr> {
    let root_metas = parse_strip_root_meta_optional(input, derive_ident)?;
    let mut ignore = None;
    let mut context_ty = None;
    if let Some(root_metas) = root_metas {
        for meta in root_metas {
            match meta {
                syn::Meta::List(meta) => {
                    if meta.path.is_ident("ignore") {
                        if ignore.is_some() {
                            syn_error!(meta, "Multiple `ignore` attributes found for TokenType! You can put all regexes into the same `ignore` and separate them with comma");
                        }
                        ignore = Some(meta.parse_args_with(
                            Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated,
                        )?);
                        continue;
                    }
                    if meta.path.is_ident("context") {
                        if context_ty.is_some() {
                            syn_error!(
                                meta,
                                "Multiple `context` attributes found for TokenType! Keep only 1"
                            );
                        }
                        context_ty = Some(meta.parse_args::<syn::Type>()?);
                        continue;
                    }
                    syn_error!(meta, "Unknown attribute for TokenType");
                }
                _ => syn_error!(meta, "Unknown attribute for TokenType"),
            }
        }
    }

    let ignore = match ignore {
        Some(ignore) => ignore.into_iter().collect::<Vec<_>>(),
        None => Vec::new(),
    };
    let context_ty = context_ty.unwrap_or(unit_type());
    Ok(RootAttr { ignore, context_ty })
}

/// Derive terminal struct and SyntaxTree implementation
fn derive_terminal(
    doc: &str,
    vis: &syn::Visibility,
    context_ty: &syn::Type,
    enum_ident: &syn::Ident,
    variant_ident: &syn::Ident,
    terminal_ident: &syn::Ident,
    match_lit: Option<&syn::LitStr>,
) -> TokenStream2 {
    let teleparse = crate_ident();
    let parse_impl = match match_lit {
        Some(match_lit) => quote! {
            parser.parse_token_match(#enum_ident::#variant_ident, follows, #match_lit)
        },
        None => quote! {
            parser.parse_token(#enum_ident::#variant_ident, follows)
        },
    };
    let s_table_insert_impl = match match_lit {
        Some(match_lit) => quote! {
            let lit = lits.get_or_add(#match_lit);
            set.insert_token_type_match(#enum_ident::#variant_ident, lit);
        },
        None => quote! {
            set.insert_token_type(#enum_ident::#variant_ident);
        },
    };
    quote! {
        #[doc = #doc]
        #[automatically_derived]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, #teleparse::ToSpan)]
        #vis struct #terminal_ident(pub #teleparse::Token<#enum_ident>);
        #[automatically_derived]
        const _: () = {
            use #teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
            use #teleparse::parser::ParserState;
            use #teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
            use core::ops::Deref;
            impl SyntaxTree for #terminal_ident {
                type T = #enum_ident;
                type AST = Token<#enum_ident>;

                fn build_start_table(s_table: &mut SyntaxTreeTable<Self::T>, lits: &mut LitTable) -> bool{
                    let t = ::core::any::TypeId::of::<Self>();
                    s_table.init(t, |_| {
                        let mut set = TermSet::default();
                        #s_table_insert_impl
                        (set, true)
                    })
                }

                fn build_follow_table<'s>(
                    s_table: &'s SyntaxTreeTable<Self::T>, 
                    f_table: &mut SyntaxTreeTable<Self::T>,
                    follows: &TermSet<Self::T>,
                ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
                    let t = ::core::any::TypeId::of::<Self>();
                    f_table.get_mut(t).union(follows);
                    (s_table.get(t), true)
                }

                #[inline]
                fn try_parse_ast<'s>(parser: &mut Parser<'s, Self::T>, f_table: &SyntaxTreeTable<Self::T>, _should_recover: bool) -> SyntaxResult<Self::T, Self::AST> {
                    let t = ::core::any::TypeId::of::<Self>();
                    let f= f_table.get(t);
                    let follows = f.deref();
                    let result = #parse_impl;
                    match result {
                        Ok(ast) => Ok(ast),
                        Err(e) => e.into(),
                    }
                }

                #[inline]
                fn into_parse_tree<'s>(ast: Self::AST, _parser: &mut Parser<'s, Self::T>) -> Self {
                    Self(ast)
                }
            }
        };
    }
}

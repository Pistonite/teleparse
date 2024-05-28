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
    let RootAttr { ignore, context_ty } = parse_root_attributes(input, derive_ident)?;
    let mut rule_exprs = Vec::new();
    let teleparse = crate_ident();
    for ignore in ignore {
        checked_regex_rule(&ignore)?;
        rule_exprs.push(quote! {
            #teleparse::lexer::LexerRule::ignore(#ignore),
        });
    }

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
            syn_error!(variant, "Missing `{}` attribute for variant to derive TokenType", CRATE);
        }

        // make enum body
        let variant_ident = &variant.ident;
        let num = syn::LitInt::new(&format!("0x{x:x}{repr_str}"), Span::call_site());
        enum_body.extend(quote! {
            #variant_ident = #num,
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

        // if no terminal is provided, this token is extract type (like comments)
        if terminal.as_ref().map(|x| x.is_empty()).unwrap_or(true) {
            should_extract_match_clauses.extend(quote! {
                Self::#variant_ident => true,
            });
        }

        // derive the terminals
        let mut infer_literals = Some(Vec::new());
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
                        infer_literals = None;
                        let doc = format!(
                            " SyntaxTree terminal derived from [`{}`] with `terminal({})`", enum_ident
                            , ident
                        ) ;
                        extra_derives.extend(derive_terminal(
                            &doc,
                            enum_vis,
                            &context_ty,enum_ident, variant_ident, ident, None
                        ));
                    }
                    // terminal(Ident = "literal")
                    syn::Meta::NameValue(meta) => {
                        let value = match &meta.value {
                            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => lit_str,
                            _ => syn_error!(meta, "Expected string literal"),
                        };
                        let match_lit = value.value();
                        if match_lit.is_empty() {
                            syn_error!(value, "Cannot use empty string for matching");
                        }
                        if let Some(infer_literals) = &mut infer_literals {
                            infer_literals.push(match_lit);
                        }
                        let doc = format!(
                            " SyntaxTree terminal derived from [`{}`] with `terminal({} = {})`", enum_ident
                            , ident, quote!{#value}
                        ) ;
                        extra_derives.extend(derive_terminal(
                            &doc,
                            enum_vis,
                            &context_ty,enum_ident, variant_ident, ident, Some(&value)
                        ));

                    }
                    _ => syn_error!(&meta, "Invalid form for terminal. Maybe you meant `terminal({} = ...)`?", ident)
                }
               
            }
        }

        // if liternal is not inferred, regex must be provided
        if infer_literals.as_ref().map(|x| x.is_empty()).unwrap_or(true) && regex.is_none() {
            syn_error!(variant, "Missing lexer rule for this variant. If all `terminal` are specified with a literal pattern, the rule can be inferred. Otherwise you must provide a `regex`");
        }
        // add the rules
        if let Some(regexes) = regex {
            for regex in regexes {
                checked_regex_rule(&regex)?;
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
    let enum_first_variant = &enum_data.variants.first().unwrap().ident;
    let enum_last_variant = &enum_data.variants.last().unwrap().ident;
    let rule_count = rule_exprs.len();
    let rule_exprs = rule_exprs.into_iter().fold(TokenStream2::new(), |mut acc, expr| {
        acc.extend(expr);
        acc
    });
    let out = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(#repr)]
        #enum_vis enum #enum_ident {
            #enum_body
        }
        #extra_derives
        #[automatically_derived]
        const _: () = {
            use #teleparse::Lexer as _;
            impl #teleparse::TokenType for #enum_ident {
                type Repr = #repr;
                type Lexer<'s> = DerivedLexer<'s>;
                #[inline]
                fn should_extract(&self) -> bool {
                    match self {
                        #should_extract_match_clauses
                        _ => false
                    }
                }
                #[inline]
                fn to_repr(&self) -> Self::Repr {
                    *self as Self::Repr
                }
                #[inline]
                fn first() -> Self {
                    Self::#enum_first_variant
                }
                fn next(&self) -> Option<Self> {
                    match self {
                        Self::#enum_last_variant => None,
                        _ => {
                            let repr = self.to_repr();
                            let next = repr << 1;
                            Some(unsafe { std::mem::transmute(next) })
                        }
                    }
                }
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
                fn new(source: &'s str) -> Self {
                    Self(#teleparse::lexer::LexerState::new(source), derived_lexer_rules())
                }
                #[inline]
                fn next(&mut self) -> (Option<#teleparse::Span>, Option<#teleparse::Token<Self::T>>) {
                    self.0.next(self.1)
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

fn parse_root_attributes(input: &mut syn::DeriveInput, derive_ident: &syn::Ident) -> syn::Result<RootAttr> {
    let root_metas = parse_strip_root_meta_optional(input,derive_ident)?;
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
                        ignore = Some(
                            meta
                                .parse_args_with(
                                    Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated
                                )?);
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
    }

    let ignore = match ignore {
        Some(ignore) => ignore.into_iter().collect::<Vec<_>>(),
        None => Vec::new()
    };
    let context_ty = context_ty.unwrap_or(unit_type());
    Ok(RootAttr {
        ignore,
        context_ty
    })
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
            #teleparse::imp::token::parse_token_match(#enum_ident::#variant_ident, #match_lit, parser)
        },
        None => quote! {
            #teleparse::imp::token::parse_token(#enum_ident::#variant_ident, parser)
        },
    };
    quote! {
        #[doc = #doc]
        #[automatically_derived]
        #vis struct #terminal_ident(pub #teleparse::Token<#enum_ident>);
        #[automatically_derived]
        const _: () = {
            use #teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
            impl SyntaxTree for #terminal_ident {
                type T = #enum_ident;
                type Ctx  = #context_ty;
                type AST = Token<#enum_ident>;

                #[inline]
                fn span_of(ast: &Self::AST) -> Span { ast.span }
                #[inline]
                fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
                    parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
                ) -> SyntaxResult<Self::AST> {
                    #parse_impl
                }
                #[inline]
                fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
                    ast: Self::AST,
                    _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
                ) -> Self {
                    Self(ast)
                }
            }
        };
    }
}

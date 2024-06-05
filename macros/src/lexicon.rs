use std::collections::BTreeSet;

use crate::*;

pub fn expand(input: &mut syn::DeriveInput) -> syn::Result<TokenStream2> {
    if !input.generics.params.is_empty() {
        syn_error!(input, "derive_lexicon cannot be used with generics")
    }

    let teleparse = crate_ident();

    // parse the root attributes
    let RootAttr { ignore } = parse_root_attributes(input)?;

    // check ignore regex
    let mut rule_exprs = Vec::new();
    for ignore in ignore {
        checked_regex_rule(&ignore)?;
        rule_exprs.push(quote! {
            #teleparse::lex::Rule::ignore(#ignore),
        });
    }

    // Separate the attributes on the variants and the enum data
    let (variant_attrs, enum_data) = {
        // strip attributes early for better error experience
        let mut variant_attrs = Vec::new();
        // has to be enum
        match &mut input.data {
            syn::Data::Enum(data) => {
                for variant in &mut data.variants {
                    variant_attrs.push(strip_take_attrs(&mut variant.attrs));
                }
            }
            _ => syn_error!(input, "derive_lexicon can only be used with enums"),
        };

        let enum_data = match &input.data {
            syn::Data::Enum(data) => data,
            _ => unreachable!(),
        };

        (variant_attrs, enum_data)
    };

    check_enum_precondition(enum_data, &variant_attrs)?;

    let repr_ty = match enum_data.variants.len() {
        0 => syn_error!(
            input,
            "derive_lexicon needs at least one variant in the enum"
        ),
        1..=8 => quote!(u8),
        9..=16 => quote!(u16),
        17..=32 => quote!(u32),
        33..=64 => quote!(u64),
        65..=128 => quote!(u128),
        _ => syn_error!(input, "derive_lexicon can have at most 128 variants"),
    };

    let enum_ident = &input.ident;
    let enum_vis = &input.vis;

    // parse enum body
    let mut enum_body = TokenStream2::new();
    let mut should_extract_match_clauses = TokenStream2::new();
    let mut extra_derives = TokenStream2::new();
    for (i, (variant, attrs)) in enum_data.variants.iter().zip(variant_attrs).enumerate() {
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
                    _ => syn_error!(meta, "Unknown {} attribute", CRATE),
                };
                if meta.path.is_ident("regex") {
                    if regex.is_some() {
                        syn_error!(meta, "Multiple `regex` attributes found for enum variant! You can put all regexes into the same attribute and separate them with comma");
                    }
                    regex = Some(meta.parse_args_with(
                        Punctuated::<syn::LitStr, syn::Token![,]>::parse_terminated,
                    )?);
                    continue;
                }
                if meta.path.is_ident("terminal") {
                    if terminal.is_some() {
                        syn_error!(meta, "Multiple `terminal` attributes found for enum variant! You might want to merge them.");
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
                            " Terminal symbol derived from [`{}`] with `terminal({})`",
                            enum_ident, ident
                        );
                        extra_derives.extend(derive_terminal(
                            &doc,
                            enum_vis,
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
                                    "Duplicate literal pattern `{}` for variant `{}`.",
                                    match_lit,
                                    variant_ident
                                );
                            }
                            infer_literals.push(match_lit.clone());
                        }
                        infer_literal_set.insert(match_lit);
                        let doc = format!(
                            " Terminal symbol derived from [`{}`] with `terminal({} = {})`",
                            enum_ident,
                            ident,
                            quote! {#value}
                        );
                        extra_derives.extend(derive_terminal(
                            &doc,
                            enum_vis,
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
                    // For example, if the regex matches `key` and the literal is `keyboard`.
                    // If we were to match `keyboard`, `key` should be matched instead
                    if let Some(mat) = regex_obj.find(&match_lit) {
                        if mat.end() != match_lit.len() {
                            syn_error!(
                                regex,
                                "This regex matches a proper prefix of `{}`. This is likely a mistake, because the terminal will never be matched (the prefix will instead)",
                                match_lit,
                            );
                        }
                    } else {
                        syn_error!(
                            regex,
                            "This regex does not match the literal `{}`. This is likely a mistake, because the terminal will never be matched",
                            match_lit,
                        );
                    }
                }
                rule_exprs.push(quote! {
                    #teleparse::lex::Rule::token(#enum_ident::#variant_ident, #regex),
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
                #teleparse::lex::Rule::token_literal(#enum_ident::#variant_ident, &[#slice]),
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
            impl #teleparse::Lexicon for #enum_ident {
                type Bit = #repr_ty;
                type Lexer<'s> = #teleparse::lex::LexerImpl<'s, Self>;
                type Map<T: Default + Clone> = [T; #enum_len];

                #[inline]
                fn id(&self) -> usize {
                    *self as usize
                }

                #[inline]
                fn from_id(id: usize) -> Self {
                    unsafe { std::mem::transmute(id) }
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
                            Some(Self::from_id(next))
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

                fn lexer<'s>(source: &'s str) -> Result<Self::Lexer<'s>, #teleparse::GrammarError> {
                    static RULES: ::std::sync::OnceLock<
            [ #teleparse::lex::Rule<#enum_ident>; #rule_count]
                    > = ::std::sync::OnceLock::new();
                    let rules = RULES.get_or_init(|| { [ #rule_exprs ] });
                    Ok(#teleparse::lex::LexerImpl::new(source, rules)?)
                }
            }
        };
    };

    Ok(out)
}

struct RootAttr {
    ignore: Vec<syn::LitStr>,
}

fn parse_root_attributes(input: &mut syn::DeriveInput) -> syn::Result<RootAttr> {
    let root_metas = parse_strip_root_meta_optional(input)?;
    let mut ignore = None;
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
    Ok(RootAttr { ignore })
}

fn check_enum_precondition(enum_data: &syn::DataEnum, variant_attrs: &[Vec<syn::Attribute>]) -> syn::Result<()> {
    for (variant, attrs) in std::iter::zip(enum_data.variants.iter(), variant_attrs.iter()) {
        if !matches!(variant.fields, syn::Fields::Unit) {
            syn_error!(variant, "derive_lexicon must be used with enums with only unit variants. The integer values will be generated");
        }
        if attrs.is_empty() {
            syn_error!(
                variant,
                "derive_lexicon needs an `{}` attribute to derive the lexer",
                CRATE
            );
        }
    }

    Ok(())
}

/// Derive terminal struct and SyntaxTree implementation
fn derive_terminal(
    doc: &str,
    vis: &syn::Visibility,
    enum_ident: &syn::Ident,
    variant_ident: &syn::Ident,
    terminal_ident: &syn::Ident,
    match_lit: Option<&syn::LitStr>,
) -> TokenStream2 {
    let teleparse = crate_ident();
    let terminal_ident_str = terminal_ident.to_string();
    let match_option_impl = match match_lit {
        Some(match_lit) => quote! {Some(#match_lit) },
        None => quote! {None },
    };
    quote! {
        #[doc = #doc]
        #[automatically_derived]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, #teleparse::ToSpan)]
        #vis struct #terminal_ident(pub #teleparse::lex::Token<#enum_ident>);
        #[automatically_derived]
        const _: () = {
            impl ::core::convert::From<#teleparse::lex::Token<#enum_ident>> for #terminal_ident {
                #[inline]
                fn from(token: #teleparse::lex::Token<#enum_ident>) -> Self {
                    Self(token)
                }
            }
            impl #teleparse::syntax::Terminal for #terminal_ident {
                type L = #enum_ident;

                #[inline]
                fn ident() -> &'static str {
                    #terminal_ident_str
                }

                #[inline]
                fn token_type() -> Self::L {
                    #enum_ident::#variant_ident
                }

                #[inline]
                fn match_literal() -> ::core::option::Option<&'static str> {
                    #match_option_impl
                }

            }
        };
    }
}

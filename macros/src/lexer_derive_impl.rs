use regex::Regex;

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
    let root_metas = parse_strip_root_meta(input,derive_ident)?;

    // has to be struct
    let struct_data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => syn_error!(input, "Lexer can only be derived for structs with named fields")
    };

    let fields = match &struct_data.fields {
        syn::Fields::Named(fields) => fields,
        _ => syn_error!(input, "Lexer can only be derived for structs with named fields")
    };

    let lexer_state_ident = match (&fields.named.len(), &fields.named.first()) {
        (1, Some(field)) => match &field.ident{
            Some(ident) => ident,
            None => syn_error!(field, "field must have an identifier"),
        },
        _ => syn_error!(input, "To derive Lexer, the struct must have exactly 1 field of type `{CRATE}::LexerState`"),
    };

    let mut token_ty = None;
    for meta in &root_metas {
        let meta = match meta {
            syn::Meta::List(meta) => meta,
            _ => continue
        };
        if !meta.path.is_ident("token") {
            continue;
        }
        if token_ty.is_some() {
            syn_error!(meta, "multiple `token` attributes found for Lexer! Keep only 1");
        }
        token_ty = Some(meta.parse_args::<syn::Path>()?);
    }

    let llnparse = crate_ident();
    let mut rule_exprs = TokenStream2::new();
    let mut rule_count: usize = 0;
    for meta in root_metas {
        let meta = match meta {
            syn::Meta::NameValue(meta) => meta,
            _ => continue
        };
        let name = match meta.path.get_ident() {
            Some(ident) => ident,
            None => syn_error!(meta, "expected a variant of the enum or `ignore`"),
        };
        let value = match meta.value {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => {
                let content = lit_str.value();
                if !content.starts_with("^") {
                    syn_error!(lit_str, "expected a regular expression starting with `^`, because it always needs to match the beginning of the remaining input");
                }
                match Regex::new(&content) {
                    Err(e) => {
                        syn_error!(lit_str, e.to_string());
                    }
                    Ok(regex) => {
                        if regex.find("").is_some() {
                            syn_error!(lit_str, "the rule must not match the empty string");
                        }
                    }
                }
                lit_str
            },
            _ => {
                syn_error!(meta, "expected a string literal containg a regular expression")
            }
        };
        // unwrap is safe because we checked the regex above
        if name == "ignore" {
            rule_exprs.extend(quote! {
                #llnparse::LexerRule::ignore(#value).unwrap(),
            });
        } else {
            rule_exprs.extend(quote! {
                #llnparse::LexerRule::token(#token_ty::#name, #value).unwrap(),
            });
        }
        rule_count += 1;
    }

    let input_ident = &input.ident;


    let out = quote! {
        #input
        #[automatically_derived]
        const _: () = {
            #[doc(hidden)]
            fn _the_rules() -> &'static [ #llnparse::LexerRule<#token_ty>; #rule_count] {
                static RULES: std::sync::OnceLock<[ #llnparse::LexerRule<#token_ty>; #rule_count]> =
                std::sync::OnceLock::new();
                RULES.get_or_init(|| {
                    [
                        #rule_exprs
                    ]
                })
            }
            impl<'s> #llnparse::Lexer<'s> for #input_ident<'s> {
                type T = #token_ty;

                fn new(source: &'s str) -> Self {
                    Self {
                        #lexer_state_ident: #llnparse::LexerState::new(source)
                    }
                }
                fn next(&mut self) -> (Option<#llnparse::Span>, Option<#llnparse::Token<Self::T>>) {
                    self.#lexer_state_ident.next(_the_rules())
                }
            }
        };
    };

    Ok(out)
}

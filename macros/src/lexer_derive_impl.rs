use regex::Regex;

use crate::*;

/// Derive the `Lexer` trait for a struct that has a LexerState field
pub fn expand(input: TokenStream) -> TokenStream {
    let derive_input = {
        let input = input.clone();
        parse_macro_input!(input as syn::DeriveInput)
    };
    from_result_keep_input(input, expand_internal(derive_input))
}

fn expand_internal(mut input: syn::DeriveInput) -> syn::Result<TokenStream2> {
    // parse the root attributes
    let mut root_metas = None;
    for attr in &input.attrs {
        let metas = match parse_attr_meta(attr)? {
            Some(metas) => metas,
            None => continue,
        };
        // only one root attribute is allowed
        if root_metas.is_some() {
            syn_error!(attr, "multiple root {CRATE} attributes found for Lexer! Keep only 1");
        }
        root_metas = Some(metas);
        break;
    }
    strip_attrs(&mut input.attrs);

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

    let root_metas = match root_metas {
        Some(metas) => metas,
        None => syn_error!(input, "Deriving Lexer requires a {CRATE} attribute to define the rules."),
    };

    let mut token_type_ident = None;
    for meta in &root_metas {
        let meta = match meta {
            syn::Meta::List(meta) => meta,
            _ => continue
        };
        if !meta.path.is_ident("token") {
            continue;
        }
        if token_type_ident.is_some() {
            syn_error!(meta, "multiple `token` attributes found for Lexer! Keep only 1");
        }
        token_type_ident = Some(meta.parse_args::<syn::Ident>()?);
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
                #llnparse::LexerRule::token(#token_type_ident::#name, #value).unwrap(),
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
            fn _the_rules() -> &'static [ #llnparse::LexerRule<#token_type_ident>; #rule_count] {
                static RULES: std::sync::OnceLock<[ #llnparse::LexerRule<#token_type_ident>; #rule_count]> =
                std::sync::OnceLock::new();
                RULES.get_or_init(|| {
                    [
                        #rule_exprs
                    ]
                })
            }
            impl<'s> #llnparse::Lexer<'s> for #input_ident<'s> {
                type T = #token_type_ident;

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

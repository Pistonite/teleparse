use proc_macro::TokenStream;
use std::cell::OnceCell;

use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

type TokenStream2 = proc_macro2::TokenStream;

/// Attribute name, to avoid conflicts with other crates
const CRATE: &str = "llnparse";

thread_local! {
    static CRATE_IDENT: OnceCell<Ident> = OnceCell::new();
}

fn crate_ident() -> Ident {
    Ident::new(CRATE, Span::call_site())
    // CRATE_IDENT.with(|ident| {
    //     ident.get_or_init(|| {
    //         match proc_macro_crate::crate_name(CRATE) {
    //             Ok(proc_macro_crate::FoundCrate::Itself) => {
    //                 // match std::env::var("CARGO_CRATE_NAME") {
    //                 //     Ok(name) => {
    //                 //         if name == CRATE {
    //                 //             Ident::new("crate", Span::call_site())
    //                 //         } else {
    //                 //             Ident::new(CRATE, Span::call_site())
    //                 //         }
    //                 //     },
    //                 //     Err(_) => {
    //                 //         // probably just crate
    //                 //         Ident::new("crate", Span::call_site())
    //                 //     }
    //                 // }
    //             }
    //             Ok(proc_macro_crate::FoundCrate::Name(name)) => {
    //                 Ident::new(&name, Span::call_site())
    //             }
    //             Err(_) => {
    //                 panic!("cannot find `{0}` crate. Make sure you are adding {0} to Cargo.toml and not using the macros stand-alone", CRATE);
    //             }
    //         }
    //     }).clone()
    // })
}

/// Derive trait from llnparse along with other required traits
#[proc_macro_attribute]
pub fn llnparse_derive(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as Ident);
    let c = crate_ident();
    let derived = match ident.to_string().as_str() {
        "TokenType" => quote! {
            #[derive(#c::TokenType, Debug, Clone, Copy, PartialEq, Eq, Hash)]
        },
        _ => panic!("unknown llnparse derive input `{}`", ident),
    };
    let input = TokenStream2::from(input);
    let out = quote! {
        #derived
        #input
    };

    out.into()
}

/// Derive the `TokenType` trait for an enum
#[proc_macro_derive(TokenType, attributes(llnparse))]
pub fn token_type_derive_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    // has to be enum
    let enum_data = match &derive_input.data {
        Data::Enum(data) => data,
        _ => panic!("TokenType can only be derived for enums"),
    };
    let mut should_extract_match_clauses = TokenStream2::new();
    for variant in &enum_data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            panic!("TokenType can only be derived for enums with unit variants");
        }

        // check for attributes
        let attr_idents = variant.attrs.iter().filter_map(|attr| {
            if attr.path().is_ident(CRATE) {
                Some(attr.parse_args::<Ident>().expect(
                    &format!("expected identifier in {} attribute, found `{}`", CRATE, quote!(attr))
                ))
            } else {
                None
            }
        });

        let mut extract = false;

        for id in attr_idents {
            if id == "extract" {
                extract = true;
            } else {
                panic!("unknown {} attribute `{}`", CRATE, id);
            }
        }

        let ident = &variant.ident;

        if extract {
            should_extract_match_clauses.extend(quote! {
                Self::#ident => true,
            });
        }
        
    }
    let enum_name = &derive_input.ident; 
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();
    let c = crate_ident();
    let out = quote! {
        #[automatically_derived]
        impl #impl_generics #c::TokenType for #enum_name #ty_generics #where_clause {
            fn should_extract(&self) -> bool {
                match self {
                    #should_extract_match_clauses
                    _ => false
                }
            }
        }
    };

    out.into()
}


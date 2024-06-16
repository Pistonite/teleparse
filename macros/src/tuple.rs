use crate::*;

pub fn expand(tuple: &syn::ExprTuple) -> syn::Result<TokenStream2> {
    let mut types = Vec::with_capacity(tuple.elems.len());
    for expr in &tuple.elems {
        let ty = syn::parse2::<syn::Type>(quote!{#expr})?;
        types.push(ty);
    }

    let first_ty = types.first().unwrap();
    let middle_ty = &types[1..];

    let teleparse = crate_ident();

    let output = quote! {
        #[automatically_derived]
        impl<
            #first_ty: #teleparse::syntax::Production,
            #( #middle_ty: #teleparse::syntax::Production<L=<#first_ty as #teleparse::syntax::Production>::L> ),*
        > #teleparse::syntax::Production for #tuple {
            type L = <#first_ty as #teleparse::syntax::Production>::L;
            fn debug() -> ::std::borrow::Cow<'static, str> {
                let mut s = ::std::string::String::from("(");
                s.push_str(&<#first_ty>::debug()); 
            #( 
                s.push_str(", ");
                s.push_str(&<#middle_ty>::debug()); 
            )*
                s.push_str(")");

                ::std::borrow::Cow::Owned(s)
            }
            fn register(meta: &mut #teleparse::syntax::MetadataBuilder<Self::L>) {
                let t = Self::id();
                if meta.visit(t, ||Self::debug().into_owned()) {
                    meta.add_sequence(t, &[#( <#types>::id() ),*]);
                #(
                    #types::register(meta);
                )*
                }
            }
        }
        #[automatically_derived]
        impl<
            #first_ty: #teleparse::parser::Produce,
            #( #middle_ty: #teleparse::parser::Produce ),*
        > #teleparse::parser::Produce for #tuple 
        where 
        #(
            <#middle_ty as #teleparse::parser::Produce>::Prod 
            : #teleparse::syntax::Production<
                L=<<#first_ty>::Prod as #teleparse::syntax::Production>::L
            >
        ),* {
            type Prod = (#( <#types>::Prod ),*);
            fn produce<'s>(
                parser: &mut #teleparse::parser::Parser<'s, <Self::Prod as #teleparse::syntax::Production>::L>, 
                meta: &#teleparse::syntax::Metadata<<Self::Prod as #teleparse::syntax::Production>::L>,
            ) -> #teleparse::syntax::Result<Self, <Self::Prod as #teleparse::syntax::Production>::L> {
                use #teleparse::syntax::Production;
                // let token = parser.peek_token_src();
                // let t = Self::prod_id();
                // let first = meta.first.get(&t);
                // if !first.contains(token) {
                //     return #teleparse::syntax::Result::Panic(::std::vec![
                //         parser.expecting(first.clone())
                //     ]);
                // }
                let mut errors = ::std::vec::Vec::new();
                let result = (
            #(
                match <#types as #teleparse::parser::Produce>::produce(parser, meta) {
                    #teleparse::syntax::Result::Success(x) => x,
                    #teleparse::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    #teleparse::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return #teleparse::syntax::Result::Panic(errors);
                    }
                }
            ),*
                );
                if errors.is_empty() {
                    #teleparse::syntax::Result::Success(result)
                } else {
                    #teleparse::syntax::Result::Recovered(result, errors)
                }
            }
        }
    };

    Ok(anon_const_block(output))
}

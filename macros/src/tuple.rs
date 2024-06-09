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
    let implementation = sequence::expand_ast_sequence(&tuple, &types)?;
    let parse_ast_check = sequence::expand_parse_ast_check();
    let parse_ast_core = types.iter().map(sequence::expand_parse_ast_step);
    let parse_ast_end = sequence::expand_parse_ast_end();

    let output = quote! {
        #[automatically_derived]
        impl<
            #first_ty: #teleparse::AbstractSyntaxTree,
            #( #middle_ty: #teleparse::AbstractSyntaxTree<L=<#first_ty as #teleparse::AbstractSyntaxTree>::L> ),*
        > #teleparse::AbstractSyntaxTree for #tuple {
            #implementation
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
            fn parse_ast<'s>(
                parser: &mut #teleparse::parser::Parser<'s, Self::L>, 
                meta: &#teleparse::syntax::Metadata<Self::L>,
            ) -> #teleparse::syntax::Result<Self, Self::L> {
                #parse_ast_check
                let result = ( #( #parse_ast_core ),* );
                #parse_ast_end
            }
        }
    };

    Ok(anon_const_block(output))
}

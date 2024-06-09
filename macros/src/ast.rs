use crate::*;

pub fn expand(input: &syn::DeriveInput, name: syn::Ident) -> syn::Result<TokenStream2> {

    let output = match &input.data {
        syn::Data::Struct(data) => {
            expand_struct(input.ident.clone(), data, name)?
        }
        syn::Data::Enum(data) => {
            expand_enum(input.ident.clone(), data, name)?
        }
        _ => syn_error!(input, "derive_ast can only be used with structs or enums")
    };

    let output = quote! {
        #input
        #output
    };

    Ok(output)
}

fn expand_struct(ident: syn::Ident, input: &syn::DataStruct, name: syn::Ident) -> syn::Result<TokenStream2> {
    let fields = match &input.fields {
        syn::Fields::Unnamed(fields) => fields,
        _ => syn_error!(&ident, "Only unnamed fields are supported for AST nodes")
    };

    let mut iter = fields.unnamed.iter();
    let first = match iter.next() {
        Some(f) => f,
        None => syn_error!(ident, "No fields found for AST nodes")
    };

    let first_ty = &first.ty;
    let teleparse = crate_ident();
    let name_str = name.to_string();

    if iter.next().is_none() {
        let output = quote! {
            #[automatically_derived]
            impl #teleparse::AbstractSyntaxTree for #ident {
                #teleparse::ast_passthrough!(#first_ty);
                fn parse_ast<'s>(
                    parser: &mut #teleparse::parser::Parser<'s, Self::L>, 
                    meta: &#teleparse::syntax::Metadata<Self::L>,
                ) -> #teleparse::syntax::Result<Self, Self::L> {
                    <#first_ty>::parse_ast(parser, meta).map(Self)
                }
            }
        };
        return Ok(output);
    }

    let types = fields.unnamed.iter().map(|f| f.ty.clone()).collect::<Vec<_>>();
    let implementation = sequence::expand_ast_sequence(&ident, &types)?;

    let parse_ast_check = sequence::expand_parse_ast_check();
    let parse_ast_core = types.iter().map(sequence::expand_parse_ast_step);
    let parse_ast_end = sequence::expand_parse_ast_end();

    let output = quote! {
        #[automatically_derived]
        impl #teleparse::AbstractSyntaxTree for #ident {
            #implementation
            fn debug() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#name_str)
            }
            fn parse_ast<'s>(
                parser: &mut #teleparse::parser::Parser<'s, Self::L>, 
                meta: &#teleparse::syntax::Metadata<Self::L>,
            ) -> #teleparse::syntax::Result<Self, Self::L> {
                #parse_ast_check
                let result = Self( #( #parse_ast_core ),* );
                #parse_ast_end
            }
        }
    };

    Ok(output)
}

fn expand_enum(ident: syn::Ident, input: &syn::DataEnum, name: syn::Ident) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();

    if input.variants.is_empty() {
        syn_error!(ident, "No variants found for AST nodes")
    }

    let variant_ident = input.variants.iter().map(|v| &v.ident);
    let mut variant_ty = Vec::new();
    for variant in &input.variants {
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                let mut iter = fields.unnamed.iter();
                let first = match iter.next() {
                    Some(ty) => ty,
                    None => syn_error!(variant, "No fields found for AST nodes")
                };
                if iter.next().is_some() {
                    syn_error!(variant, "Only 1 field is supported for AST nodes")
                }
                variant_ty.push(&first.ty);
            }
            _ => syn_error!(variant, "Only unnamed fields are supported for AST nodes")
        }
    }

    let i = (0..variant_ty.len()).collect::<Vec<_>>();
    let middle_ty = &variant_ty[0..variant_ty.len()-1];
    let last_ty = variant_ty.last().unwrap();

    let name_str = name.to_string();

    let output = quote! {
        #[automatically_derived]
        impl #teleparse::AbstractSyntaxTree for #ident {
            type L = <#last_ty as #teleparse::AbstractSyntaxTree>::L;
            fn debug() -> ::std::borrow::Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(#name_str)
            }
            fn build_first(builder: &mut #teleparse::syntax::FirstBuilder<Self::L>) {
                let t = Self::type_id();
                if builder.visit(t, &Self::debug()) {
                    #( <#variant_ty>::build_first(builder);   )*
                    builder.build_enum(t, &[ #( <#variant_ty>::type_id() ),* ]);
                }
            }
            fn check_left_recursive(
                seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
                stack: &mut ::std::vec::Vec<::std::string::String>,
                set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
                first: &#teleparse::syntax::First<Self::L>,
            ) -> ::core::result::Result<(), #teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Ok(());
                }
                if !set.insert(t) {
                    return Err(#teleparse::GrammarError::left_recursion(&stack, &Self::debug()));
                }
                stack.push(Self::debug().into_owned());
            #(
                if let Err(e) = <#middle_ty>::check_left_recursive(seen, stack, set, first) {
                    stack.pop();
                    set.remove(&t);
                    return Err(e);
                }
            )*
                let r = <#last_ty>::check_left_recursive(seen, stack, set, first);
                stack.pop();
                set.remove(&t);
                r
            }
            fn check_first_conflict(
                seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
                first: &#teleparse::syntax::First<Self::L>,
            ) -> ::core::result::Result<(), #teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Ok(());
                }
                #[allow(unused_mut)]
                let mut check_set = #teleparse::syntax::FirstSet::new();
            #(
                let first_set = first.get(&<#middle_ty>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <#middle_ty>::debug().into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");

                    return Err(#teleparse::GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection));
                }
                check_set.union(&first_set);
            )*
                let first_set = first.get(&<#last_ty>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <#last_ty>::debug().into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");

                    return Err(#teleparse::GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection));
                }

                #( <#variant_ty>::check_first_conflict(seen, first)?;)*
                Ok(())
            }
            fn build_follow(builder: &mut #teleparse::syntax::FollowBuilder<Self::L>) {
                let t = Self::type_id();
                if builder.visit(t) {
                    #( <#variant_ty>::build_follow(builder);   )*
                    builder.build_enum(t, &[ #( <#variant_ty>::type_id() ),* ]);
                }
            }
            fn check_first_follow_conflict(
                seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
                first: &#teleparse::syntax::First<Self::L>, 
                follow: &#teleparse::syntax::Follow<Self::L>
            ) -> ::core::result::Result<(), #teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Ok(());
                }
                Self::check_self_first_follow_conflict(first, follow)?;
                #( <#variant_ty>::check_first_follow_conflict(seen, first, follow)?;)*
                Ok(())
            }
            fn build_jump(
                seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
                first: &#teleparse::syntax::First<Self::L>,
                jump: &mut #teleparse::syntax::Jump<Self::L>
            ) {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return;
                }
            #(
                let first_set = first.get(&<#variant_ty>::type_id());
                jump.register(t, first_set, #i);
            )*
                #( <#variant_ty>::build_jump(seen, first, jump); )*
            }
            fn parse_ast<'s>(
                parser: &mut #teleparse::parser::Parser<'s, Self::L>, 
                meta: &#teleparse::syntax::Metadata<Self::L>,
            ) -> #teleparse::syntax::Result<Self, Self::L> {
                let t = Self::type_id();
                let token_src = parser.peek_token_src();
                match meta.jump.look_up(&t, token_src) {
                #(
                    Some(#i) => <#variant_ty>::parse_ast(parser, meta).map(Self::#variant_ident),
                )*
                    _ => {
                        let first = meta.first.get(&t);
                        let err = parser.expecting(first.clone());
                        let err_vec = ::std::vec![err];
                        #teleparse::syntax::Result::Panic(err_vec)
                    },
                }
            }

        }
    };

    Ok(output)
}

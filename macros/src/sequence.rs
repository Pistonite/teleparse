use crate::*;

pub fn expand_ast_sequence<T: quote::ToTokens>(input: &T, types: &[syn::Type]) -> syn::Result<TokenStream2> {
    if types.len() < 2 {
        syn_error!(input, "Expected at least two types in sequence")
    }

    let first_ty = &types[0];
    let middle_types = &types[1..types.len()-1];
    let last_ty = types.last().unwrap();

    let teleparse = crate_ident();

    let build_first_impl = expand_build_first_fn(&types);
    let check_left_recursive_impl = expand_check_left_recursive_fn(&first_ty, &middle_types, &last_ty);
    let check_first_conflict_impl = expand_check_first_conflict_fn(&first_ty, &middle_types, &last_ty);
    let build_follow_impl = expand_build_follow_fn(&types);
    let check_first_follow_conflict_impl = expand_check_first_follow_conflict_fn(&types);
    let build_jump_impl = expand_build_jump_fn(&types);

    Ok(quote! {
        type L = <#first_ty as #teleparse::AbstractSyntaxTree>::L;
        #build_first_impl
        #check_left_recursive_impl
        #check_first_conflict_impl
        #build_follow_impl
        #check_first_follow_conflict_impl
        #build_jump_impl
    })
}

// fn expand_tuple(input: syn::ExprTuple) -> syn::Result<TokenStream2> {
//     let mut elem_iter = input.elems.iter();
//     let first_ty = match elem_iter.next() {
//         None => syn_error!(input, "Expected at least two types in tuple"),
//         Some(elem) => {
//             syn::parse2::<syn::Type>(quote! { #elem })?
//         }
//     };
//     let mut all_types = vec![first_ty.clone()];
//     let mut middle_types = Vec::new();
//     for elem in elem_iter {
//         let ty = syn::parse2::<syn::Type>(quote! { #elem })?;
//         middle_types.push(ty.clone());
//         all_types.push(ty);
//     }
//     let last_ty = match middle_types.pop() {
//         None => syn_error!(input, "Expected at least two types in tuple"),
//         Some(ty) => ty
//     };
//
//     let teleparse = crate_ident();
//
//     let build_first_impl = expand_build_first_fn(quote! { Self }, &all_types);
//     let check_left_recursive_impl = expand_check_left_recursive_fn(quote! { Self }, &first_ty, &middle_types, &last_ty);
//     let check_first_conflict_impl = expand_check_first_conflict_fn(&first_ty, &middle_types, &last_ty);
//     let build_follow_impl = expand_build_follow_fn(&all_types);
//     let check_first_follow_conflict_impl = expand_check_first_follow_conflict_fn(&all_types);
//     let build_jump_impl = expand_build_jump_fn(&all_types);
//
//     let parse_ast_check = expand_parse_ast_check();
//     let parse_ast_core = all_types.iter().map(expand_parse_ast_step);
//     let parse_ast_end = expand_parse_ast_end();
//
//     let output = quote! {
//         use #teleparse::AbstractSyntaxTree;
//         #[automatically_derived]
//         impl<
//             #first_ty: AbstractSyntaxTree, 
//             #( #middle_types: AbstractSyntaxTree<L=<#first_ty>::L>, )*
//             #last_ty: AbstractSyntaxTree<L=<#first_ty>::>
//         > AbstractSyntaxTree for TODO {
//             type L = <#first_ty>::L;
//             #build_first_impl
//             #check_left_recursive_impl
//             #check_first_conflict_impl
//             #build_follow_impl
//             #check_first_follow_conflict_impl
//             #build_jump_impl
//
//             fn parse_ast<'s>(
//                 parser: &mut #teleparse::parser::Parser<'s, Self::L>,
//                 meta: &#teleparse::syntax::Metadata<Self::L>,
//             ) -> #teleparse::syntax::Result<Self, Self::L> {
//                 #parse_ast_check
//                 let result = ( #( #parse_ast_core ),* );
//                 #parse_ast_end
//             }
//         }
//     };
//
//     Ok(anon_const_block(output))
// }

pub fn expand_build_first_fn(types: &[syn::Type]) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {

        fn build_first(builder: &mut #teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            #( <#types>::build_first(builder); )*
            builder.build_sequence(t, &[ #( <#types>::type_id() ),* ]);
        }

    }
}

pub fn expand_check_left_recursive_fn(
    ty_1: &syn::Type,
    ty_mid: &[syn::Type],
    ty_last: &syn::Type,
) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {
        fn check_left_recursive(
            stack: &mut ::std::vec::Vec<::std::string::String>, 
            set: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
            first: &#teleparse::syntax::First<Self::L>
        ) -> ::core::result::Result<(), #teleparse::GrammarError> {
            let t = Self::type_id();
            if !set.insert(t) {
                return Err(#teleparse::GrammarError::left_recursion(&stack, &Self::debug()));
            }
            stack.push(Self::debug().into_owned());

            if let Err(e) = <#ty_1>::check_left_recursive(stack, set, first) {
                stack.pop();
                set.remove(&t);
                return Err(e);
            }

            let mut temp_stack = ::std::vec::Vec::new();
            let mut temp_set = ::std::collections::BTreeSet::new();

            let (cur_stack, cur_set, need_pop) = if first.get(&<#ty_1>::type_id()).contains_epsilon() {
                (stack, set, true)
            } else {
                stack.pop();
                set.remove(&t);
                (&mut temp_stack, &mut temp_set, false)
            };
        #(
            if let Err(e) = <#ty_mid>::check_left_recursive(cur_stack, cur_set, first) {
                if need_pop {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                return Err(e);
            }
            let (cur_stack, cur_set, need_pop) = if first.get(&<#ty_mid>::type_id()).contains_epsilon() {
                (cur_stack, cur_set, need_pop)
            } else {
                if need_pop {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                temp_stack.clear();
                temp_set.clear();
                (&mut temp_stack, &mut temp_set, false)
            };

        )*
            let check = <#ty_last>::check_left_recursive(cur_stack, cur_set, first);
            if need_pop {
                cur_stack.pop();
                cur_set.remove(&t);
            }
            check

        }
    }
}

pub fn expand_check_first_conflict_fn(
    ty_1: &syn::Type,
    ty_mid: &[syn::Type],
    ty_last: &syn::Type,
) -> TokenStream2 { 
    let teleparse = crate_ident();
    // this is technically first/follow conflict, but checking it here
    // allows better error message
    // X -> Y1 Y2 .. Yi .. Yj .. Yn has conflict if:
    // - FIRST(Yi..Yj-1) has epsilon
    // - UNION(FIRST(Yi..Yj-1)) intersects { FIRST(Yj) - epsilon }
    quote! {

        fn check_first_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
            first: &#teleparse::syntax::First<Self::L>
        ) -> ::core::result::Result<(), #teleparse::GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            let cur = <#ty_1>::type_id();
            let cur_first = first.get(&cur);
            let cur_name = <#ty_1>::debug();
            #[allow(unused_mut)]
            let mut cur_check = if cur_first.contains_epsilon() {
                cur_first.clone()
            } else {
                #teleparse::syntax::FirstSet::new()
            };

        #(
            let next = <#ty_mid>::type_id();
            let next_first = first.get(&next);
            let next_name = <#ty_mid>::debug();
            if cur_check.intersects_minus_epsilon(next_first) {
                let cur_name = cur_name.into_owned();
                let next_name = next_name.into_owned();
                let self_name = Self::debug().into_owned();
                let terminals = cur_check
                    .intersection_repr_minus_epsilon(next_first)
                    .into_iter()
                    .collect::<::std::vec::Vec<_>>()
                    .join(", ");
                return Err(#teleparse::GrammarError::FirstFollowSeqConflict(self_name, cur_name, next_name, terminals));
            }
            let (cur, cur_name) = if next_first.contains_epsilon() {
                cur_check.union_minus_epsilon(next_first);
                (cur, cur_name)
            } else {
                cur_check.clear();
                (next, next_name)
            };
        )*
            let next = <#ty_last>::type_id();
            let next_first = first.get(&next);
            if cur_check.intersects_minus_epsilon(next_first) {
                let cur_name = cur_name.into_owned();
                let next_name = <#ty_last>::debug().into_owned();
                let self_name = Self::debug().into_owned();
                let terminals = cur_check
                    .intersection_repr_minus_epsilon(next_first)
                    .into_iter()
                    .collect::<::std::vec::Vec<_>>()
                    .join(", ");
                return Err(#teleparse::GrammarError::FirstFollowSeqConflict(self_name, cur_name, next_name, terminals));
            }

            <#ty_1>::check_first_conflict(seen, first)?;
            #( <#ty_mid>::check_first_conflict(seen, first)?; )*
            <#ty_last>::check_first_conflict(seen, first)

        }

    }
}

pub fn expand_build_follow_fn(types: &[syn::Type]) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {

        fn build_follow(builder: &mut #teleparse::syntax::FollowBuilder<Self::L>) {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            #( <#types>::build_follow(builder); )*
            builder.build_sequence(t, &[ #( <#types>::type_id() ),* ]);
        }

    }
}

pub fn expand_check_first_follow_conflict_fn(types: &[syn::Type]) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {
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
            #(
                <#types>::check_first_follow_conflict(seen, first, follow)?;
            )*

            Ok(())
        }
    }
}

pub fn expand_build_jump_fn(types: &[syn::Type]) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {
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
                <#types>::build_jump(seen, first, jump);
            )*
        }
    }
}

pub fn expand_parse_ast_check() -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {
        let token = parser.peek_token_src();
        let t = Self::type_id();
        let first = meta.first.get(&t);
        if !first.contains(token) {
            return #teleparse::syntax::Result::Panic(::std::vec![
                parser.expecting(first.clone())
            ]);
        }
        let mut errors = ::std::vec::Vec::new();
    }
}

pub fn expand_parse_ast_end() -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {
        if errors.is_empty() {
            #teleparse::syntax::Result::Success(result)
        } else {
            #teleparse::syntax::Result::Recovered(result, errors)
        }
    }
}

pub fn expand_parse_ast_step(
    ty: &syn::Type
) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {

        match <#ty>::parse_ast(parser, meta) {
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
        
    }
}

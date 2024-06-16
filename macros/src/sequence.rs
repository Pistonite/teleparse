use crate::*;

pub fn expand_production_sequence<T: quote::ToTokens>(input: &T, types: &[syn::Type]) -> syn::Result<TokenStream2> {
    if types.len() < 2 {
        syn_error!(input, "Expected at least two types in sequence")
    }

    let first_ty = &types[0];
    let middle_ty = &types[1..types.len()-1];
    let last_ty = types.last().unwrap();

    let teleparse = crate_ident();

    // let check_left_recursive_impl = expand_check_left_recursive_fn(&first_ty, &middle_types, &last_ty);
    // let check_first_conflict_impl = expand_check_first_conflict_fn(&first_ty, &middle_types, &last_ty);
    let build_follow_impl = expand_build_follow_fn(&types);
    let check_first_follow_conflict_impl = expand_check_first_follow_conflict_fn(&types);
    let build_jump_impl = expand_build_jump_fn(&types);

    Ok(quote! {
        type L = <#first_ty as #teleparse::syntax::Production>::L;
        fn build_first(builder: &mut #teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::id();
            if !builder.visit(t, &Self::debug()) {
                return;
            }
            #( <#types>::build_first(builder); )*
            builder.build_sequence(t, &[ #( <#types>::id() ),* ]);
        }
        fn check_left_recursive(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
            stack: &mut ::std::vec::Vec<::std::string::String>, 
            set: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
            first: &#teleparse::syntax::First<Self::L>
        ) -> ::core::result::Result<(), #teleparse::GrammarError> {
            let t = Self::id();
            if !seen.insert(t) {
                return Ok(());
            }
            if !set.insert(t) {
                return Err(#teleparse::GrammarError::left_recursion(&stack, &Self::debug()));
            }
            stack.push(Self::debug().into_owned());

            if let Err(e) = <#first_ty>::check_left_recursive(seen, stack, set, first) {
                stack.pop();
                set.remove(&t);
                return Err(e);
            }

            let mut temp_stack = ::std::vec::Vec::new();
            let mut temp_set = ::std::collections::BTreeSet::new();

            let (cur_stack, cur_set, need_pop) = if first.get(&<#first_ty>::id()).contains_epsilon() {
                (stack, set, true)
            } else {
                stack.pop();
                set.remove(&t);
                (&mut temp_stack, &mut temp_set, false)
            };
        #(
            if let Err(e) = <#middle_ty>::check_left_recursive(seen, cur_stack, cur_set, first) {
                if need_pop {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                return Err(e);
            }
            let (cur_stack, cur_set, need_pop) = if first.get(&<#middle_ty>::id()).contains_epsilon() {
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
            let check = <#last_ty>::check_left_recursive(seen, cur_stack, cur_set, first);
            if need_pop {
                cur_stack.pop();
                cur_set.remove(&t);
            }
            check

        }
        // this is technically first/follow conflict, but checking it here
        // allows better error message
        // X -> Y1 Y2 .. Yi .. Yj .. Yn has conflict if:
        // - FIRST(Yi..Yj-1) has epsilon
        // - UNION(FIRST(Yi..Yj-1)) intersects { FIRST(Yj) - epsilon }
        fn check_first_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
            first: &#teleparse::syntax::First<Self::L>
        ) -> ::core::result::Result<(), #teleparse::GrammarError> {
            let t = Self::id();
            if !seen.insert(t) {
                return Ok(());
            }
            let cur = <#first_ty>::id();
            let cur_first = first.get(&cur);
            let cur_name = <#first_ty>::debug();
            #[allow(unused_mut)]
            let mut cur_check = if cur_first.contains_epsilon() {
                cur_first.clone()
            } else {
                #teleparse::syntax::FirstSet::new()
            };

        #(
            let next = <#middle_ty>::id();
            let next_first = first.get(&next);
            let next_name = <#middle_ty>::debug();
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
            let next = <#last_ty>::id();
            let next_first = first.get(&next);
            if cur_check.intersects_minus_epsilon(next_first) {
                let cur_name = cur_name.into_owned();
                let next_name = <#last_ty>::debug().into_owned();
                let self_name = Self::debug().into_owned();
                let terminals = cur_check
                    .intersection_repr_minus_epsilon(next_first)
                    .into_iter()
                    .collect::<::std::vec::Vec<_>>()
                    .join(", ");
                return Err(#teleparse::GrammarError::FirstFollowSeqConflict(self_name, cur_name, next_name, terminals));
            }

            <#first_ty>::check_first_conflict(seen, first)?;
            #( <#middle_ty>::check_first_conflict(seen, first)?; )*
            <#last_ty>::check_first_conflict(seen, first)

        }
        #build_follow_impl
        #check_first_follow_conflict_impl
        #build_jump_impl
    })
}


pub fn expand_check_first_conflict_fn(
    ty_1: &syn::Type,
    ty_mid: &[syn::Type],
    ty_last: &syn::Type,
) -> TokenStream2 { 
    let teleparse = crate_ident();
    quote! {


    }
}

pub fn expand_build_follow_fn(types: &[syn::Type]) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {

        fn build_follow(builder: &mut #teleparse::syntax::FollowBuilder<Self::L>) {
            use #teleparse::syntax::Production;
            let t = Self::id();
            if !builder.visit(t) {
                return;
            }
            #( <#types>::build_follow(builder); )*
            builder.build_sequence(t, &[ #( <#types>::id() ),* ]);
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
            use #teleparse::syntax::Production;
            let t = Self::id();
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
            use #teleparse::syntax::Production;
            let t = Self::id();
            if !seen.insert(t) {
                return;
            }
            #(
                <#types>::build_jump(seen, first, jump);
            )*
        }
    }
}

pub fn expand_produce_start() -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {
        use #teleparse::parser::Parse;
        let token = parser.peek_token_src();
        let t = Self::id();
        let first = meta.first.get(&t);
        if !first.contains(token) {
            return #teleparse::syntax::Result::Panic(::std::vec![
                parser.expecting(first.clone())
            ]);
        }
        let mut errors = ::std::vec::Vec::new();
    }
}

pub fn expand_produce_end() -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {
        if errors.is_empty() {
            #teleparse::syntax::Result::Success(result)
        } else {
            #teleparse::syntax::Result::Recovered(result, errors)
        }
    }
}

pub fn expand_produce_step(
    pt_ty: &syn::Type
) -> TokenStream2 {
    let teleparse = crate_ident();
    quote! {

        match <#pt_ty as #teleparse::parser::Produce>::produce(parser, meta) {
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

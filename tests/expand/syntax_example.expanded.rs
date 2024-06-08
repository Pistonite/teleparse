use teleparse::prelude::*;
pub enum UnionTest {
    Foo(Foo),
    Bar(Bar),
    Biz(Biz),
}
const _: () = {
    #[doc(hidden)]
    enum DerivedAST {
        Foo(teleparse::parser::AstOf<Foo>),
        Bar(teleparse::parser::AstOf<Bar>),
        Biz(teleparse::parser::AstOf<Biz>),
    }
    #[automatically_derived]
    impl teleparse::ToSpan for DerivedAST {
        fn span(&self) -> teleparse::Span {
            match self {
                Self::Foo(x, ..) => x.span(),
                Self::Bar(x, ..) => x.span(),
                Self::Biz(x, ..) => x.span(),
            }
        }
    }
    const _: () = {
        #[automatically_derived]
        impl teleparse::AbstractSyntaxTree for DerivedAST {
            type L = TokenType;
            fn debug() -> ::alloc::borrow::Cow<'static, str> {
                ::alloc::borrow::Cow::Borrowed("DerivedAST")
            }
            fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
                let t = Self::type_id();
                if builder.visit(t) {
                    <teleparse::parser::AstOf<Foo>>::build_first(builder);
                    <teleparse::parser::AstOf<Bar>>::build_first(builder);
                    <teleparse::parser::AstOf<Biz>>::build_first(builder);
                    builder
                        .build_enum(
                            t,
                            &[
                                <teleparse::parser::AstOf<Foo>>::type_id(),
                                <teleparse::parser::AstOf<Bar>>::type_id(),
                                <teleparse::parser::AstOf<Biz>>::type_id(),
                            ],
                        );
                }
            }
            fn check_left_recursive(
                stack: &mut ::alloc::vec::Vec<::alloc::string::String>,
                seen: &mut ::alloc::collections::BTreeSet<::core::any::TypeId>,
                first: &teleparse::syntax::First<Self::L>,
            ) -> ::core::result::Result<(), teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Err(
                        teleparse::GrammarError::left_recursion(&stack, &Self::debug()),
                    );
                }
                stack.push(Self::debug().into_owned());
                if let Err(e) = <teleparse::parser::AstOf<
                    Foo,
                >>::check_left_recursive(stack, seen) {
                    stack.pop();
                    seen.remove(&t);
                    return Err(e);
                }
                if let Err(e) = <teleparse::parser::AstOf<
                    Bar,
                >>::check_left_recursive(stack, seen) {
                    stack.pop();
                    seen.remove(&t);
                    return Err(e);
                }
                let r = <teleparse::parser::AstOf<
                    Biz,
                >>::check_left_recursive(stack, seen);
                stack.pop();
                seen.remove(&t);
                r
            }
            fn check_first_conflict(
                seen: &mut ::alloc::collections::BTreeSet<::core::any::TypeId>,
                first: &teleparse::syntax::First<Self::L>,
            ) -> ::core::result::Result<(), teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Ok(());
                }
                let check_set = teleparse::syntax::FirstSet::new();
                let first_set = first.get(<teleparse::parser::AstOf<Foo>>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <teleparse::parser::AstOf<Foo>>::debug()
                        .into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(
                        teleparse::GrammarError::FirstFirstConflict(
                            self_name,
                            produce_name,
                            intersection,
                        ),
                    );
                }
                let check_set = check_set.union(&first_set);
                let first_set = first.get(<teleparse::parser::AstOf<Bar>>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <teleparse::parser::AstOf<Bar>>::debug()
                        .into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(
                        teleparse::GrammarError::FirstFirstConflict(
                            self_name,
                            produce_name,
                            intersection,
                        ),
                    );
                }
                let check_set = check_set.union(&first_set);
                let first_set = first.get(<teleparse::parser::AstOf<Biz>>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <teleparse::parser::AstOf<Biz>>::debug()
                        .into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(
                        teleparse::GrammarError::FirstFirstConflict(
                            self_name,
                            produce_name,
                            intersection,
                        ),
                    );
                }
                <teleparse::parser::AstOf<Foo>>::check_first_conflict(seen, first)?;
                <teleparse::parser::AstOf<Bar>>::check_first_conflict(seen, first)?;
                <teleparse::parser::AstOf<Biz>>::check_first_conflict(seen, first)?;
                Ok(())
            }
            fn build_follow(builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {
                let t = Self::type_id();
                if builder.visit(t) {
                    <teleparse::parser::AstOf<Foo>>::build_follow(builder);
                    <teleparse::parser::AstOf<Bar>>::build_follow(builder);
                    <teleparse::parser::AstOf<Biz>>::build_follow(builder);
                    builder
                        .build_enum(
                            t,
                            &[
                                <teleparse::parser::AstOf<Foo>>::type_id(),
                                <teleparse::parser::AstOf<Bar>>::type_id(),
                                <teleparse::parser::AstOf<Biz>>::type_id(),
                            ],
                        );
                }
            }
            fn check_first_follow_conflict(
                seen: &mut ::alloc::collections::BTreeSet<::core::any::TypeId>,
                first: &teleparse::syntax::First<Self::L>,
                follow: &teleparse::syntax::Follow<Self::L>,
            ) -> ::core::result::Result<(), teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Ok(());
                }
                Self::check_self_first_follow_conflict(first, follow)?;
                <teleparse::parser::AstOf<
                    Foo,
                >>::check_first_follow_conflict(seen, first)?;
                <teleparse::parser::AstOf<
                    Bar,
                >>::check_first_follow_conflict(seen, first)?;
                <teleparse::parser::AstOf<
                    Biz,
                >>::check_first_follow_conflict(seen, first)?;
                Ok(())
            }
            fn build_jump(
                seen: &mut ::alloc::collecitons::BTreeSet<::core::any::TypeId>,
                jump: &mut teleparse::syntax::Jump<Self::L>,
            ) {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return;
                }
                let first_set = first.get(&<teleparse::parser::AstOf<Foo>>::type_id());
                jump.register(t, first_set, 0usize);
                let first_set = first.get(&<teleparse::parser::AstOf<Bar>>::type_id());
                jump.register(t, first_set, 1usize);
                let first_set = first.get(&<teleparse::parser::AstOf<Biz>>::type_id());
                jump.register(t, first_set, 2usize);
                <teleparse::parser::AstOf<Foo>>::build_jump(seen, jump);
                <teleparse::parser::AstOf<Bar>>::build_jump(seen, jump);
                <teleparse::parser::AstOf<Biz>>::build_jump(seen, jump);
            }
            fn parse_ast<'s>(
                parser: &mut teleparse::parser::Parser<'s, Self::L>,
                meta: &teleparse::syntax::Metadata<Self::L>,
            ) -> teleparse::syntax::Result<Self, Self::L> {
                let t = Self::type_id();
                let token_src = parser.peek_token_src();
                match meta.jump.look_up(&t, token_src) {
                    Some(0usize) => {
                        <teleparse::parser::AstOf<Foo>>::parse_ast(parser, meta)
                            .map(Self::Foo)
                    }
                    Some(1usize) => {
                        <teleparse::parser::AstOf<Bar>>::parse_ast(parser, meta)
                            .map(Self::Bar)
                    }
                    Some(2usize) => {
                        <teleparse::parser::AstOf<Biz>>::parse_ast(parser, meta)
                            .map(Self::Biz)
                    }
                    _ => {
                        let first = meta.first.get(&t);
                        let err = parser.expecting(first.clone());
                        let err_vec = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([err]),
                        );
                        teleparse::syntax::Result::Panic(err_vec)
                    }
                }
            }
        }
    };
    #[automatically_derived]
    impl teleparse::parser::ParseTree for UnionTest {
        type AST = DerivedAST;
        fn from_ast<'s>(
            ast: Self::AST,
            parser: &mut teleparse::parser::Parser<'s, Self::AST>,
        ) -> Self {
            match ast {
                DerivedAST::Foo(ast) => Self::Foo(<Foo>::from_ast(ast, parser)),
                DerivedAST::Bar(ast) => Self::Bar(<Bar>::from_ast(ast, parser)),
                DerivedAST::Biz(ast) => Self::Biz(<Biz>::from_ast(ast, parser)),
            }
        }
    }
};
pub enum UnionTestOverride {
    Foo(Foo),
    Bar(Quaak),
    Biz(Box<Biz>),
}
const _: () = {
    #[doc(hidden)]
    enum DerivedAST {
        Foo(teleparse::parser::AstOf<Foo>),
        Bar(teleparse::parser::AstOf<Quaak>),
        Biz(teleparse::parser::AstOf<Box<Biz>>),
    }
    #[automatically_derived]
    impl teleparse::ToSpan for DerivedAST {
        fn span(&self) -> teleparse::Span {
            match self {
                Self::Foo(x, ..) => x.span(),
                Self::Bar(x, ..) => x.span(),
                Self::Biz(x, ..) => x.span(),
            }
        }
    }
    const _: () = {
        #[automatically_derived]
        impl teleparse::AbstractSyntaxTree for DerivedAST {
            type L = TokenType;
            fn debug() -> ::alloc::borrow::Cow<'static, str> {
                ::alloc::borrow::Cow::Borrowed("DerivedAST")
            }
            fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
                let t = Self::type_id();
                if builder.visit(t) {
                    <teleparse::parser::AstOf<Foo>>::build_first(builder);
                    <teleparse::parser::AstOf<Quaak>>::build_first(builder);
                    <teleparse::parser::AstOf<Box<Biz>>>::build_first(builder);
                    builder
                        .build_enum(
                            t,
                            &[
                                <teleparse::parser::AstOf<Foo>>::type_id(),
                                <teleparse::parser::AstOf<Quaak>>::type_id(),
                                <teleparse::parser::AstOf<Box<Biz>>>::type_id(),
                            ],
                        );
                }
            }
            fn check_left_recursive(
                stack: &mut ::alloc::vec::Vec<::alloc::string::String>,
                seen: &mut ::alloc::collections::BTreeSet<::core::any::TypeId>,
                first: &teleparse::syntax::First<Self::L>,
            ) -> ::core::result::Result<(), teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Err(
                        teleparse::GrammarError::left_recursion(&stack, &Self::debug()),
                    );
                }
                stack.push(Self::debug().into_owned());
                if let Err(e) = <teleparse::parser::AstOf<
                    Foo,
                >>::check_left_recursive(stack, seen) {
                    stack.pop();
                    seen.remove(&t);
                    return Err(e);
                }
                if let Err(e) = <teleparse::parser::AstOf<
                    Quaak,
                >>::check_left_recursive(stack, seen) {
                    stack.pop();
                    seen.remove(&t);
                    return Err(e);
                }
                let r = <teleparse::parser::AstOf<
                    Box<Biz>,
                >>::check_left_recursive(stack, seen);
                stack.pop();
                seen.remove(&t);
                r
            }
            fn check_first_conflict(
                seen: &mut ::alloc::collections::BTreeSet<::core::any::TypeId>,
                first: &teleparse::syntax::First<Self::L>,
            ) -> ::core::result::Result<(), teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Ok(());
                }
                let check_set = teleparse::syntax::FirstSet::new();
                let first_set = first.get(<teleparse::parser::AstOf<Foo>>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <teleparse::parser::AstOf<Foo>>::debug()
                        .into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(
                        teleparse::GrammarError::FirstFirstConflict(
                            self_name,
                            produce_name,
                            intersection,
                        ),
                    );
                }
                let check_set = check_set.union(&first_set);
                let first_set = first.get(<teleparse::parser::AstOf<Quaak>>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <teleparse::parser::AstOf<Quaak>>::debug()
                        .into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(
                        teleparse::GrammarError::FirstFirstConflict(
                            self_name,
                            produce_name,
                            intersection,
                        ),
                    );
                }
                let check_set = check_set.union(&first_set);
                let first_set = first
                    .get(<teleparse::parser::AstOf<Box<Biz>>>::type_id());
                if check_set.intersects(&first_set) {
                    let self_name = Self::debug().into_owned();
                    let produce_name = <teleparse::parser::AstOf<Box<Biz>>>::debug()
                        .into_owned();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Err(
                        teleparse::GrammarError::FirstFirstConflict(
                            self_name,
                            produce_name,
                            intersection,
                        ),
                    );
                }
                <teleparse::parser::AstOf<Foo>>::check_first_conflict(seen, first)?;
                <teleparse::parser::AstOf<Quaak>>::check_first_conflict(seen, first)?;
                <teleparse::parser::AstOf<Box<Biz>>>::check_first_conflict(seen, first)?;
                Ok(())
            }
            fn build_follow(builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {
                let t = Self::type_id();
                if builder.visit(t) {
                    <teleparse::parser::AstOf<Foo>>::build_follow(builder);
                    <teleparse::parser::AstOf<Quaak>>::build_follow(builder);
                    <teleparse::parser::AstOf<Box<Biz>>>::build_follow(builder);
                    builder
                        .build_enum(
                            t,
                            &[
                                <teleparse::parser::AstOf<Foo>>::type_id(),
                                <teleparse::parser::AstOf<Quaak>>::type_id(),
                                <teleparse::parser::AstOf<Box<Biz>>>::type_id(),
                            ],
                        );
                }
            }
            fn check_first_follow_conflict(
                seen: &mut ::alloc::collections::BTreeSet<::core::any::TypeId>,
                first: &teleparse::syntax::First<Self::L>,
                follow: &teleparse::syntax::Follow<Self::L>,
            ) -> ::core::result::Result<(), teleparse::GrammarError> {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return Ok(());
                }
                Self::check_self_first_follow_conflict(first, follow)?;
                <teleparse::parser::AstOf<
                    Foo,
                >>::check_first_follow_conflict(seen, first)?;
                <teleparse::parser::AstOf<
                    Quaak,
                >>::check_first_follow_conflict(seen, first)?;
                <teleparse::parser::AstOf<
                    Box<Biz>,
                >>::check_first_follow_conflict(seen, first)?;
                Ok(())
            }
            fn build_jump(
                seen: &mut ::alloc::collecitons::BTreeSet<::core::any::TypeId>,
                jump: &mut teleparse::syntax::Jump<Self::L>,
            ) {
                let t = Self::type_id();
                if !seen.insert(t) {
                    return;
                }
                let first_set = first.get(&<teleparse::parser::AstOf<Foo>>::type_id());
                jump.register(t, first_set, 0usize);
                let first_set = first.get(&<teleparse::parser::AstOf<Quaak>>::type_id());
                jump.register(t, first_set, 1usize);
                let first_set = first
                    .get(&<teleparse::parser::AstOf<Box<Biz>>>::type_id());
                jump.register(t, first_set, 2usize);
                <teleparse::parser::AstOf<Foo>>::build_jump(seen, jump);
                <teleparse::parser::AstOf<Quaak>>::build_jump(seen, jump);
                <teleparse::parser::AstOf<Box<Biz>>>::build_jump(seen, jump);
            }
            fn parse_ast<'s>(
                parser: &mut teleparse::parser::Parser<'s, Self::L>,
                meta: &teleparse::syntax::Metadata<Self::L>,
            ) -> teleparse::syntax::Result<Self, Self::L> {
                let t = Self::type_id();
                let token_src = parser.peek_token_src();
                match meta.jump.look_up(&t, token_src) {
                    Some(0usize) => {
                        <teleparse::parser::AstOf<Foo>>::parse_ast(parser, meta)
                            .map(Self::Foo)
                    }
                    Some(1usize) => {
                        <teleparse::parser::AstOf<Quaak>>::parse_ast(parser, meta)
                            .map(Self::Bar)
                    }
                    Some(2usize) => {
                        <teleparse::parser::AstOf<Box<Biz>>>::parse_ast(parser, meta)
                            .map(Self::Biz)
                    }
                    _ => {
                        let first = meta.first.get(&t);
                        let err = parser.expecting(first.clone());
                        let err_vec = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([err]),
                        );
                        teleparse::syntax::Result::Panic(err_vec)
                    }
                }
            }
        }
    };
    #[automatically_derived]
    impl teleparse::parser::ParseTree for UnionTestOverride {
        type AST = DerivedAST;
        fn from_ast<'s>(
            ast: Self::AST,
            parser: &mut teleparse::parser::Parser<'s, Self::AST>,
        ) -> Self {
            match ast {
                DerivedAST::Foo(ast) => Self::Foo(<Foo>::from_ast(ast, parser)),
                DerivedAST::Bar(ast) => Self::Bar(<Quaak>::from_ast(ast, parser)),
                DerivedAST::Biz(ast) => Self::Biz(<Box<Biz>>::from_ast(ast, parser)),
            }
        }
    }
};

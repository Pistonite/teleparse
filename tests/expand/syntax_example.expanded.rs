use teleparse::prelude::*;
pub enum UnionTest {
    Foo(Foo),
    Bar(Bar),
    Biz(Biz),
}
#[automatically_derived]
const _: () = {
    use teleparse::syntax::{
        First, FirstBuilder, FirstSet, Follow, Jump, AbstractSyntaxTree, Metadata,
    };
    use teleparse::parser::{Parser, ParseTree, AstOf};
    use teleparse::{ToSpan, GrammarError};
    use ::std::borrow::Cow;
    use ::std::collections::BTreeSet;
    use ::std::vec::Vec;
    #[doc(hidden)]
    pub enum DerivedAST {
        Foo(AstOf<Foo>),
        Bar(AstOf<Bar>),
        Biz(AstOf<Biz>),
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
    impl AbstractSyntaxTree for DerivedAST {
        type L = TokenType;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("UnionTest")
        }
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t) {
                <AstOf<Foo>>::build_first(builder);
                <AstOf<Bar>>::build_first(builder);
                <AstOf<Biz>>::build_first(builder);
                builder
                    .build_enum(
                        t,
                        &[
                            <AstOf<Foo>>::type_id(),
                            <AstOf<Bar>>::type_id(),
                            <AstOf<Biz>>::type_id(),
                        ],
                    );
            }
        }
        fn check_left_recursive(
            stack: &mut Vec<String>,
            seen: &mut BTreeSet<TypeId>,
        ) -> Result<(), GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Err(GrammarError::left_recursion(&stack, &Self::debug()));
            }
            stack.push(Self::debug().into_owned());
            if let Err(e) = <AstOf<Foo>>::check_left_recursive(stack, seen) {
                stack.pop();
                seen.remove(&t);
                return Err(e);
            }
            if let Err(e) = <AstOf<Bar>>::check_left_recursive(stack, seen) {
                stack.pop();
                seen.remove(&t);
                return Err(e);
            }
            let r = <AstOf<Biz>>::check_left_recursive(stack, seen);
            stack.pop();
            seen.remove(&t);
            r
        }
        fn check_first_conflict(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            let mut check_set = FirstSet::new();
            let first_set = first.get(<AstOf<Foo>>::type_id());
            if check_set.intersects(&first_set) {
                let self_name = Self::debug().into_owned();
                let produce_name = <AstOf<Foo>>::debug().into_owned();
                let intersection = check_set
                    .intersection_repr(&first_set)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(
                    GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection,
                    ),
                );
            }
            check_set.union(&first_set);
            let first_set = first.get(<AstOf<Bar>>::type_id());
            if check_set.intersects(&first_set) {
                let self_name = Self::debug().into_owned();
                let produce_name = <AstOf<Bar>>::debug().into_owned();
                let intersection = check_set
                    .intersection_repr(&first_set)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(
                    GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection,
                    ),
                );
            }
            check_set.union(&first_set);
            let first_set = first.get(<AstOf<Biz>>::type_id());
            if check_set.intersects(&first_set) {
                let self_name = Self::debug().into_owned();
                let produce_name = <AstOf<Biz>>::debug().into_owned();
                let intersection = check_set
                    .intersection_repr(&first_set)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(
                    GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection,
                    ),
                );
            }
            <AstOf<Foo>>::check_first_conflict(seen, first)?;
            <AstOf<Bar>>::check_first_conflict(seen, first)?;
            <AstOf<Biz>>::check_first_conflict(seen, first)?;
            Ok(())
        }
        fn build_follow(builder: &mut FollowBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t) {
                <AstOf<Foo>>::build_follow(builder);
                <AstOf<Bar>>::build_follow(builder);
                <AstOf<Biz>>::build_follow(builder);
                builder
                    .build_enum(
                        t,
                        &[
                            <AstOf<Foo>>::type_id(),
                            <AstOf<Bar>>::type_id(),
                            <AstOf<Biz>>::type_id(),
                        ],
                    );
            }
        }
        fn check_first_follow_conflict(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::L>,
            follow: &Follow<Self::L>,
        ) -> Result<(), GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            Self::check_self_first_follow_conflict(first, follow)?;
            <AstOf<Foo>>::check_first_follow_conflict(seen, first, follow)?;
            <AstOf<Bar>>::check_first_follow_conflict(seen, first, follow)?;
            <AstOf<Biz>>::check_first_follow_conflict(seen, first, follow)?;
            Ok(())
        }
        fn build_jump(seen: &mut BTreeSet<TypeId>, jump: &mut Jump<Self::L>) {
            let t = Self::type_id();
            if !seen.insert(t) {
                return;
            }
            let id = <AstOf<Foo>>::type_id();
            let first_set = first.get(&id);
            jump.register(t, first_set, 0usize);
            let id = <AstOf<Bar>>::type_id();
            let first_set = first.get(&id);
            jump.register(t, first_set, 1usize);
            let id = <AstOf<Biz>>::type_id();
            let first_set = first.get(&id);
            jump.register(t, first_set, 2usize);
            <AstOf<Foo>>::build_jump(seen, jump);
            <AstOf<Bar>>::build_jump(seen, jump);
            <AstOf<Biz>>::build_jump(seen, jump);
        }
        fn parse<'s>(
            parser: &mut Parser<'s, Self::L>,
            meta: &Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let t = Self::type_id();
            let token_src = parser.peek_token_src();
            match meta.jump.look_up(&t, token_src) {
                Some(0usize) => <AstOf<Foo>>::parse(parser, meta),
                Some(1usize) => <AstOf<Bar>>::parse(parser, meta),
                Some(2usize) => <AstOf<Biz>>::parse(parser, meta),
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
    impl ParseTree for UnionTest {
        type AST = DerivedAST;
        fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::AST>) -> Self {
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
#[automatically_derived]
const _: () = {
    use teleparse::syntax::{
        First, FirstBuilder, FirstSet, Follow, Jump, AbstractSyntaxTree, Metadata,
    };
    use teleparse::parser::{Parser, ParseTree, AstOf};
    use teleparse::{ToSpan, GrammarError};
    use ::std::borrow::Cow;
    use ::std::collections::BTreeSet;
    use ::std::vec::Vec;
    #[doc(hidden)]
    pub enum DerivedAST {
        Foo(AstOf<Foo>),
        Bar(AstOf<Quaak>),
        Biz(AstOf<Box<Biz>>),
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
    impl AbstractSyntaxTree for DerivedAST {
        type L = TokenType;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("UnionTestOverride")
        }
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t) {
                <AstOf<Foo>>::build_first(builder);
                <AstOf<Quaak>>::build_first(builder);
                <AstOf<Box<Biz>>>::build_first(builder);
                builder
                    .build_enum(
                        t,
                        &[
                            <AstOf<Foo>>::type_id(),
                            <AstOf<Quaak>>::type_id(),
                            <AstOf<Box<Biz>>>::type_id(),
                        ],
                    );
            }
        }
        fn check_left_recursive(
            stack: &mut Vec<String>,
            seen: &mut BTreeSet<TypeId>,
        ) -> Result<(), GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Err(GrammarError::left_recursion(&stack, &Self::debug()));
            }
            stack.push(Self::debug().into_owned());
            if let Err(e) = <AstOf<Foo>>::check_left_recursive(stack, seen) {
                stack.pop();
                seen.remove(&t);
                return Err(e);
            }
            if let Err(e) = <AstOf<Quaak>>::check_left_recursive(stack, seen) {
                stack.pop();
                seen.remove(&t);
                return Err(e);
            }
            let r = <AstOf<Box<Biz>>>::check_left_recursive(stack, seen);
            stack.pop();
            seen.remove(&t);
            r
        }
        fn check_first_conflict(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            let mut check_set = FirstSet::new();
            let first_set = first.get(<AstOf<Foo>>::type_id());
            if check_set.intersects(&first_set) {
                let self_name = Self::debug().into_owned();
                let produce_name = <AstOf<Foo>>::debug().into_owned();
                let intersection = check_set
                    .intersection_repr(&first_set)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(
                    GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection,
                    ),
                );
            }
            check_set.union(&first_set);
            let first_set = first.get(<AstOf<Quaak>>::type_id());
            if check_set.intersects(&first_set) {
                let self_name = Self::debug().into_owned();
                let produce_name = <AstOf<Quaak>>::debug().into_owned();
                let intersection = check_set
                    .intersection_repr(&first_set)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(
                    GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection,
                    ),
                );
            }
            check_set.union(&first_set);
            let first_set = first.get(<AstOf<Box<Biz>>>::type_id());
            if check_set.intersects(&first_set) {
                let self_name = Self::debug().into_owned();
                let produce_name = <AstOf<Box<Biz>>>::debug().into_owned();
                let intersection = check_set
                    .intersection_repr(&first_set)
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(
                    GrammarError::FirstFirstConflict(
                        self_name,
                        produce_name,
                        intersection,
                    ),
                );
            }
            <AstOf<Foo>>::check_first_conflict(seen, first)?;
            <AstOf<Quaak>>::check_first_conflict(seen, first)?;
            <AstOf<Box<Biz>>>::check_first_conflict(seen, first)?;
            Ok(())
        }
        fn build_follow(builder: &mut FollowBuilder<Self::L>) {
            let t = Self::type_id();
            if builder.visit(t) {
                <AstOf<Foo>>::build_follow(builder);
                <AstOf<Quaak>>::build_follow(builder);
                <AstOf<Box<Biz>>>::build_follow(builder);
                builder
                    .build_enum(
                        t,
                        &[
                            <AstOf<Foo>>::type_id(),
                            <AstOf<Quaak>>::type_id(),
                            <AstOf<Box<Biz>>>::type_id(),
                        ],
                    );
            }
        }
        fn check_first_follow_conflict(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::L>,
            follow: &Follow<Self::L>,
        ) -> Result<(), GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            Self::check_self_first_follow_conflict(first, follow)?;
            <AstOf<Foo>>::check_first_follow_conflict(seen, first, follow)?;
            <AstOf<Quaak>>::check_first_follow_conflict(seen, first, follow)?;
            <AstOf<Box<Biz>>>::check_first_follow_conflict(seen, first, follow)?;
            Ok(())
        }
        fn build_jump(seen: &mut BTreeSet<TypeId>, jump: &mut Jump<Self::L>) {
            let t = Self::type_id();
            if !seen.insert(t) {
                return;
            }
            let id = <AstOf<Foo>>::type_id();
            let first_set = first.get(&id);
            jump.register(t, first_set, 0usize);
            let id = <AstOf<Quaak>>::type_id();
            let first_set = first.get(&id);
            jump.register(t, first_set, 1usize);
            let id = <AstOf<Box<Biz>>>::type_id();
            let first_set = first.get(&id);
            jump.register(t, first_set, 2usize);
            <AstOf<Foo>>::build_jump(seen, jump);
            <AstOf<Quaak>>::build_jump(seen, jump);
            <AstOf<Box<Biz>>>::build_jump(seen, jump);
        }
        fn parse<'s>(
            parser: &mut Parser<'s, Self::L>,
            meta: &Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let t = Self::type_id();
            let token_src = parser.peek_token_src();
            match meta.jump.look_up(&t, token_src) {
                Some(0usize) => <AstOf<Foo>>::parse(parser, meta),
                Some(1usize) => <AstOf<Quaak>>::parse(parser, meta),
                Some(2usize) => <AstOf<Box<Biz>>>::parse(parser, meta),
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
    impl ParseTree for UnionTestOverride {
        type AST = DerivedAST;
        fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::AST>) -> Self {
            match ast {
                DerivedAST::Foo(ast) => Self::Foo(<Foo>::from_ast(ast, parser)),
                DerivedAST::Bar(ast) => Self::Bar(<Quaak>::from_ast(ast, parser)),
                DerivedAST::Biz(ast) => Self::Biz(<Box<Biz>>::from_ast(ast, parser)),
            }
        }
    }
};

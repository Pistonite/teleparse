//! optional syntax tree nodes ([`Option`], [`Exists`])
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::option::Option as StdOption;
use std::marker::PhantomData;

use crate::{prelude::*, TokenType};
use crate::parser::ParserState;
use crate::table::{LitTable, SyntaxTreeTable, TermSet};
use crate::{Parser, SyntaxTree, SyntaxResult};

use super::Node;

/// Node that stores an optional subtree.
/// This is the core of epsilon-derivations
#[teleparse_derive(Node)]
pub struct Option<ST: SyntaxTree>(Node<StdOption<ST>>);

impl<ST: SyntaxTree + 'static> SyntaxTree for Option<ST> {
    type T = ST::T;
    type AST = Result<ST::AST, Span>;

    #[inline]
    fn can_be_empty() -> bool {
        true
    }

    #[inline]
    fn check_left_recursive(stack: &mut Vec<TypeId>, set: &mut BTreeSet<TypeId>) -> bool {
        ST::check_left_recursive(stack, set)
    }

    fn build_first_table( s_table: &mut SyntaxTreeTable<Self::T>, lits: &mut LitTable)  {
        s_table.init(Self::type_id(), |s_table| {
            ST::build_first_table(s_table, lits);
            let mut first = s_table.get(ST::type_id()).into_owned();
            first.insert_empty();
            first
        })
    }

    #[inline]
    fn has_first_collision(first: &SyntaxTreeTable<Self::T>) -> bool {
        // Self -> Inner | e
        // Collides if Inner contains e
        first.get(ST::type_id()).contains_empty()
    }

    fn build_follow_table(
        first: &SyntaxTreeTable<Self::T>, 
        follow: &mut SyntaxTreeTable<Self::T>,
    ) -> bool {
        let t = Self::type_id();
        let inner = ST::type_id();
        // Self -> Inner | e
        // Everything in FOLLOW(Self) is in FOLLOW(Inner), so:
        // - FOLLOW(Inner) = FOLLOW(Inner) U FOLLOW(Self)
        let changed = follow.union(inner, t);
        // For the production Inner -> ...
        // something would ever change if FOLLOW(Inner) changed
        if changed {
            ST::build_follow_table(first, follow);
        }
        changed
    }

    #[inline]
    fn try_parse_ast<'s>(
        parser: &mut Parser<'s, Self::T>,
        f_table: &SyntaxTreeTable<Self::T>,
        _should_recover: bool,
    ) -> SyntaxResult<Self::T, Self::AST> {
        try_parse_ast_impl::<ST>(parser, f_table)
    }

    #[inline]
    fn into_parse_tree<'s>(
        ast: Self::AST,
        parser: &mut Parser<'s, Self::T>
    ) -> Self {
        match ast {
            Ok(ast) => Node::new(ast.span(), Some(ST::into_parse_tree(ast, parser))).into(),
            Err(span) => Node::new(span, None).into(),
        }
    }
}

/// Node that stores if an optional subtree is parsed
#[teleparse_derive(Node)]
pub struct Exists<ST: SyntaxTree>(Node<bool>, PhantomData<ST>);

impl<ST: SyntaxTree + 'static> SyntaxTree for Exists<ST> {
    type T = ST::T;
    type AST = Result<ST::AST, Span>;

    #[inline]
    fn can_be_empty() -> bool {
        true
    }

    #[inline]
    fn check_left_recursive(stack: &mut Vec<TypeId>, set: &mut BTreeSet<TypeId>) -> bool {
        ST::check_left_recursive(stack, set)
    }

    #[inline]
    fn build_first_table( s_table: &mut SyntaxTreeTable<Self::T>, lits: &mut LitTable){
        ST::build_first_table(s_table, lits);
        build_first_table_impl::<ST::T>(Self::type_id(), ST::type_id(), s_table, lits)
    }

    #[inline]
    fn has_first_collision(s_table: &SyntaxTreeTable<Self::T>) -> bool {
        has_first_collision_impl::<ST::T>(s_table, ST::type_id())
    }

    #[inline]
    fn build_follow_table(
        first: &SyntaxTreeTable<Self::T>, 
        follow: &mut SyntaxTreeTable<Self::T>,
    ) -> bool {
        build_follow_table_impl::<ST>(Self::type_id(), s_table, f_table, follows)
    }

    #[inline]
    fn try_parse_ast<'s>(
        parser: &mut Parser<'s, Self::T>,
        f_table: &SyntaxTreeTable<Self::T>,
        _should_recover: bool,
    ) -> SyntaxResult<Self::T, Self::AST> {
        try_parse_ast_impl::<ST>(parser, f_table)
    }

    #[inline]
    fn into_parse_tree<'s>(
        ast: Self::AST,
        parser: &mut Parser<'s, Self::T>
    ) -> Self {
        match ast {
            Ok(ast) => {
                let pt = Node::new(ast.span(), true).into();
                ST::into_parse_tree(ast, parser);
                pt
            },
            Err(span) => Node::new(span, false).into(),
        }
    }
}

fn build_first_table_impl<T: TokenType>(t: TypeId, inner_t: TypeId, s_table: &mut SyntaxTreeTable<T>, lits: &mut LitTable) {
    s_table.init(t, |s_table| {
        let mut first = s_table.get(inner_t).into_owned();
        first.insert_empty();
        first
    })
}

#[inline]
fn has_first_collision_impl<T: TokenType>(s_table: &SyntaxTreeTable<T>, inner_t: TypeId) -> bool {
    // Self -> Inner | e
    s_table.get(inner_t).contains_empty()
}

#[inline]
fn build_follow_table_impl(
    t: TypeId,
    inner: TypeId,
    // first: &SyntaxTreeTable<ST::T>, 
    follow: &mut SyntaxTreeTable<ST::T>,
) -> bool {
    // Self -> Inner | e
    // Everything in FOLLOW(Self) is in FOLLOW(Inner)
    // FOLLOW(Inner) = FOLLOW(Inner) U FOLLOW(Self)
    follow.union(inner, t)
}
    fn try_parse_ast_impl<'s, ST: SyntaxTree>(
        parser: &mut Parser<'s, ST::T>,
        f_table: &SyntaxTreeTable<ST::T>,
    ) -> SyntaxResult<ST::T, Result<ST::AST, Span>> {
        if let Err(e) = parser.push_state() {
            return e.into();
        }
        // parse the subtree with recovery off
        // since if any error happen we will return epsilon
        let result = match ST::try_parse_ast(parser, f_table, false) {
            Ok(ast) => {
                // subtree success
                Ok(ast)
            },
            Err(_) => {
                // partial success (should not happen)
                // or failure
                
                // restore the position so we use the span before trying to parse
                // this
                parser.restore_state();
                let mut span = parser.current_span();
                // make the span empty
                span.hi = span.lo;
                Err(span)
            }
        };

        parser.pop_state();
        Ok(result)
    }


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn use_optional_as_option() {
    //     let o = Option(Node::new(Span::new(0, 0), Some(42)));
    //     assert_eq!(o.as_ref().copied(), Some(42));
    //     let opt: &Option<i32> = &o;
    //     assert_eq!(opt.as_ref().copied(), Some(42));
    // }
    // #[test]
    // fn use_exists() {
    //     let e = Exists::<String>::new(Span::new(0, 0), true);
    //     assert!(e.exists());
    // }
}

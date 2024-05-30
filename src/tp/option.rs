//! optional syntax tree nodes ([`Option`], [`Exists`])
use std::any::TypeId;
use std::borrow::Cow;
use std::option::Option as StdOption;
use std::marker::PhantomData;

use crate::prelude::*;
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
    fn build_start_table( s_table: &mut SyntaxTreeTable<Self::T>, lits: &mut LitTable) {
        build_start_table_impl::<ST>(TypeId::of::<Self>(), s_table, lits);
    }

    #[inline]
    fn build_follow_table<'s>(
        s_table: &'s SyntaxTreeTable<Self::T>, 
        f_table: &mut SyntaxTreeTable<Self::T>,
        follows: &TermSet<Self::T>
    ) -> Cow<'s, TermSet<Self::T>> {
        build_follow_table_impl::<ST>(TypeId::of::<Self>(), s_table, f_table, follows)
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
    fn build_start_table( s_table: &mut SyntaxTreeTable<Self::T>, lits: &mut LitTable) {
        build_start_table_impl::<ST>(TypeId::of::<Self>(), s_table, lits);
    }

    #[inline]
    fn build_follow_table<'s>(
        s_table: &'s SyntaxTreeTable<Self::T>, 
        f_table: &mut SyntaxTreeTable<Self::T>,
        follows: &TermSet<Self::T>
    ) -> Cow<'s, TermSet<Self::T>> {
        build_follow_table_impl::<ST>(TypeId::of::<Self>(), s_table, f_table, follows)
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

fn build_start_table_impl<ST: SyntaxTree + 'static>(t: TypeId, s_table: &mut SyntaxTreeTable<ST::T>, lits: &mut LitTable) {
    s_table.init(t, |s_table| {
        let mut set = TermSet::default();
        // option => ε | ST
        ST::build_start_table(s_table, lits);
        set.insert_eof();
        set.union(&s_table.get(TypeId::of::<ST>()));
        set
    });
}

fn build_follow_table_impl<'s, ST: SyntaxTree + 'static>(
    t: TypeId,
    s_table: &'s SyntaxTreeTable<ST::T>, 
    f_table: &mut SyntaxTreeTable<ST::T>,
    follows: &TermSet<ST::T>
) -> Cow<'s, TermSet<ST::T>> {
    f_table.get_mut(t).union(follows);
    // the follow set for something before this
    // is (ST | e) <follow>
    // however the final set only has e if follow has e
    let mut set = follows.clone();
    set.union(&s_table.get(t));
    if !follows.contains_eof() {
        set.remove_eof();
    }
    Cow::Owned(set)
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

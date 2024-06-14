use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use itertools::Itertools;

use crate::syntax::{self, ErrorKind, First, FirstBuilder, FirstRel, Follow, FollowBuilder, FollowRel, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, ParseTree, Parser, Span, ToSpan};

use super::{Node, OptionAST};
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct LoopAST<T: AbstractSyntaxTree>(pub Vec<T>);

impl<T: AbstractSyntaxTree> ToSpan for LoopAST<T> {
    fn span(&self) -> Span {
        let lo = self.0.first().map(|t| t.span().lo).unwrap_or_default();
        let hi = self.0.last().map(|t| t.span().hi).unwrap_or_default();
        Span::new(lo, hi)
    }
}
impl<T: AbstractSyntaxTree> AbstractSyntaxTree for LoopAST<T> {
    type L=T::L;
    fn debug() -> Cow<'static, str> {
        Cow::Owned(format!("Loop<{}>", T::debug()))
    }
    fn build_first(builder: &mut FirstBuilder<Self::L>) {
        // Loop<T> => T Loop<T>
        // Loop<T> => e
        let t = Self::type_id();
        if builder.visit(t, &Self::debug()) {
            // recursive build
            T::build_first(builder);
            let inner = T::type_id();
            builder.build_sequence(t, &[inner, t]);
            builder.add(FirstRel::insert_epsilon(t));
        }
    }
    fn check_left_recursive(
        seen: &mut BTreeSet<TypeId>,
        stack: &mut Vec<String>,
        set: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
    ) -> Result<(), GrammarError> {
        let t = Self::type_id();
        if !seen.insert(t) {
            return Ok(());
        }
        if !set.insert(t) {
            return Err(GrammarError::left_recursion(&stack, &Self::debug()));
        }
        stack.push(Self::debug().into_owned());
        let result = T::check_left_recursive(seen, stack, set, first);
        stack.pop();
        set.remove(&t);
        result
    }
    fn check_first_conflict(
        seen: &mut BTreeSet<TypeId>, 
        first: &First<Self::L>
    ) -> Result<(), GrammarError> {
        let t = Self::type_id();
        if !seen.insert(t) {
            return Ok(());
        }
        let inner = T::type_id();
        let first_set = first.get(&inner);
        if first_set.contains_epsilon() {
            return Err(GrammarError::FirstFirstConflict(
                Self::debug().into_owned(),
                T::debug().into_owned(),
                "<epsilon>".to_string(),
            ))
        }

        T::check_first_conflict(seen, first)
    }
    fn build_follow( builder: &mut FollowBuilder<Self::L>) {
        let t = Self::type_id();
        if builder.visit(t) {
            // recursive build
            T::build_follow(builder);
            let inner = T::type_id();
            builder.build_sequence(t, &[inner, t]);
        }
    }
    fn check_first_follow_conflict(
        seen: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
        follow: &Follow<Self::L>,
    ) -> Result<(), GrammarError> {
        if !seen.insert(Self::type_id()) {
            return Ok(());
        }
        Self::check_self_first_follow_conflict(first, follow)?;
        T::check_first_follow_conflict(seen, first, follow)
    }
    fn build_jump(
        seen: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
        jump: &mut Jump<Self::L>
    ) {
        if seen.insert(Self::type_id()) {
            T::build_jump(seen, first, jump);
        }
    }

    fn parse_ast<'s>(
        parser: &mut Parser<'s, Self::L>,
        meta: &Metadata<Self::L>,
    ) -> SynResult<Self, Self::L> {
        let mut errors: Vec<syntax::Error<Self::L>> = Vec::new();
        let mut output = Vec::new();
        let first = meta.first.get(&Self::type_id());

        loop {
            let mut token = parser.peek_token_src();
            if token.is_none() {
                break;
            }
            if !first.contains(token) {
                let skip_lo = parser.current_span().lo;
                // we need to keep track of hi instead of using the lo
                // of a valid token, because there could be skipped characters between.
                let mut skip_hi = parser.current_span().hi;
                parser.consume_token();
                token = parser.peek_token_src();
                while token.is_some() && !first.contains(token) {
                    skip_hi = parser.current_span().hi;
                    parser.consume_token();
                    token = parser.peek_token_src();
                }
                errors.push(syntax::Error::new(
                    skip_lo..skip_hi,
                    ErrorKind::UnexpectedTokens,
                ));
                if token.is_none() {
                    break;
                }
            }
            let lo_before = parser.current_span().lo;
            match T::parse_ast(parser, meta) {
                SynResult::Success(t) => {
                    output.push(t);
                }
                SynResult::Recovered(t, e) => {
                    output.push(t);
                    errors.extend(e);
                }
                SynResult::Panic(e) => {
                    errors.extend(e);
                }
            }
            if lo_before == parser.current_span().lo {
                errors.push(syntax::Error::new(
                    parser.current_span(),
                    ErrorKind::UnexpectedNoAdvanceInLoop,
                ));
                break;
            }
        }
        if errors.is_empty() {
            SynResult::Success(Self(output))
        } else {
            SynResult::Recovered(Self(output), errors)
        }
    }
}

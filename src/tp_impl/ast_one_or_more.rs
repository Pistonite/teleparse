use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::syntax::{First, FirstBuilder, Follow, FollowBuilder, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, Parser, Span, ToSpan};

use super::OptionAST;

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct OneOrMore<T: AbstractSyntaxTree>(pub Vec<T>);

impl<T: AbstractSyntaxTree> ToSpan for OneOrMore<T> {
    fn span(&self) -> Span {
        let lo = self.0.first().map(|t| t.span().lo).unwrap_or_default();
        let hi = self.0.last().map(|t| t.span().hi).unwrap_or_default();
        Span::new(lo, hi)
    }
}

impl<T: AbstractSyntaxTree> AbstractSyntaxTree for OneOrMore<T> {
    type L=T::L;
    fn debug() -> Cow<'static, str> {
        Cow::Owned(format!("OneOrMore<{}>", T::debug()))
    }
    fn build_first(builder: &mut FirstBuilder<Self::L>) {
        // OneOrMore<T> => T Option<OneOrMore<T>>
        let t = Self::type_id();
        if builder.visit(t, &Self::debug()) {
            // recursive build
            T::build_first(builder);
            // OptionAST::<T>::build_first(builder);
            OptionAST::<OneOrMore<T>>::build_first(builder);
            let inner = T::type_id();
            builder.build_sequence(t, &[inner, OptionAST::<OneOrMore<T>>::type_id()]);
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
        let first_set = first.get(&t);
        if first_set.contains_epsilon() {
            return Err(GrammarError::FirstFollowSeqConflict(
                Self::debug().into_owned(),
                T::debug().into_owned(),
                OptionAST::<OneOrMore<T>>::debug().into_owned(),
                "<epsilon>".to_string(),
            ))
        }

        T::check_first_conflict(seen, first)?;

        Ok(())
    }
    fn build_follow( builder: &mut FollowBuilder<Self::L>) {
        let t = Self::type_id();
        if builder.visit(t) {
            // recursive build
            T::build_follow(builder);
            // OptionAST::<T>::build_follow(builder);
            OptionAST::<OneOrMore<T>>::build_follow(builder);
            let inner = T::type_id();
            builder.build_sequence(t, &[inner, OptionAST::<OneOrMore<T>>::type_id()]);
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
        T::check_first_follow_conflict(seen, first, follow)?;
        OptionAST::<OneOrMore<T>>::check_first_follow_conflict(seen, first, follow)
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
        let (mut output, mut errors) = match T::parse_ast(parser, meta) {
            SynResult::Success(t) => {
                (vec![t], Vec::new())
            }
            SynResult::Recovered(t, e) => {
                (vec![t], e)
            }
            SynResult::Panic(e) => {
                return SynResult::Panic(e);
            }
        };
    let t_type = T::type_id();
    let first = meta.first.get(&t_type);
    loop {
        let token = parser.peek_token_src();
        if token.is_none() {
            break;
        }
        if !first.contains(token) {
            break;
        }
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
                    break;
                }
        }
    }
    if errors.is_empty() {
        SynResult::Success(Self(output))
    } else {
        SynResult::Recovered(Self(output), errors)
    }
    }
}

use itertools::Itertools;
use teleparse_macros::ToSpan;

use crate::{Lexicon, Span};

use super::FirstSet;

/// Error encountered during syntax analysis
#[derive(Debug, Clone, ToSpan, PartialEq)]
pub struct Error<L: Lexicon> {
    pub span: Span,
    pub data: ErrorKind<L>,
}

impl<L: Lexicon> Error<L> {
    pub fn new<S: Into<Span>>(span: S, data: ErrorKind<L>) -> Self {
        Self { span: span.into(), data }
    }

    pub fn message(&self, input: &str) -> String {
        match &self.data {
            ErrorKind::Custom(msg) => msg.clone(),
            ErrorKind::UnexpectedCharacters => {
                format!("Unexpected: {}", self.span.get(input))
            },
            ErrorKind::UnexpectedTokens => {
                format!("Unexpected token(s): {}", self.span.get(input))
            },
            ErrorKind::Expecting(set) => {
                let set = set.as_terminal_set().to_repr().into_iter().join(", ");
                format!("Expecting one of {}", set)
            },
            ErrorKind::UnexpectedEof => "Unexpected end of file".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind<L: Lexicon> {
    Custom(String),
    UnexpectedCharacters,
    UnexpectedTokens,
    Expecting(FirstSet<L>),
    UnexpectedEof,
}

/// Result of parsing an AST node
pub enum Result<T, L: Lexicon> {
    /// The AST node was successfully parsed, with the corresponding tokens consumed
    Success(T),
    /// The parser panicked while parsing the AST node, but it was able to skip some tokens and
    /// recover.
    Recovered(T, Vec<Error<L>>),
    /// The parser panicked while parsing the AST node, and was unable to recover.
    /// The parser might have advanced its position in the input.
    Panic(Vec<Error<L>>),
}

impl<T, L: Lexicon> Result<T, L> {
    #[inline]
    pub fn map<T2, F: FnOnce(T) -> T2>(self, f: F) -> Result<T2, L> {
        match self {
            Self::Success(obj) => Result::Success(f(obj)),
            Self::Recovered(obj, errors) => Result::Recovered(f(obj), errors),
            Self::Panic(errors) => Result::Panic(errors),
        }
    }
    
}

// impl<T: TokenType, O> SyntaxResultExt<T, O> for SyntaxResult<T, O> {
//     fn map_ext<O2, F: FnOnce(O) -> O2>(self, f: F) -> SyntaxResult<T, O2> {
//         match self {
//             Ok(tree) => Ok(f(tree)),
//             Err(errors) => {
//                 Err(errors.map(f))
//             }
//         }
//     }
//    
//     // fn as_value(&self) -> Option<&O> {
//     //     match &self {
//     //         Ok(tree) => Some(tree),
//     //         Err(errors) => errors.obj.as_ref(),
//     //     }
//     // }
//     //
//     //  fn into_value(self) -> Option<O> {
//     //     match self {
//     //         Ok(tree) => Some(tree),
//     //         Err(errors) => errors.obj,
//     //     }
//     // }
//     //
//     // fn unwrap_or_extend_errors<ST2: SyntaxTree<T=ST::T>>(self, errors: &mut Vec<SyntaxError<ST::T>>) -> Result<ST, SyntaxErrors<ST2>> {
//     //     match self {
//     //         Ok(x) => Ok(x),
//     //         Err(e) => e.unwrap_or_extend(errors),
//     //     }
//     // }
// }
//
// /// Wrapper for errors encountered during parsing. 
// /// This is used to determine which errors should be reported
// /// when all paths are throwing errors
// #[derive(Debug)]
// pub struct SyntaxErrors<T: TokenType, O> {
//     // pub obj: Option<O>,
//     pub errors: Vec<SyntaxError<T>>,
//     highest: Pos,
// }
//
// impl<T: TokenType, O> Clone for SyntaxErrors<T, O> where O: Clone {
//     fn clone(&self) -> Self {
//         Self {
//             obj: self.obj.clone(),
//             errors: self.errors.clone(),
//             highest: self.highest,
//         }
//     }
// }
//
// impl<T: TokenType, O> SyntaxErrors<T, O> {
//     /// Initialize the error state with parsing result and errors
//     ///
//     /// If `obj` is `Some`, it indicates partial success (i.e. there were errors
//     /// but it was recovered
//     pub fn new(obj: Option<O>, errors: Vec<SyntaxError<T>>) -> Self {
//         let highest = Self::compute_highest(&errors);
//         Self {
//             obj,
//             errors,
//             highest
//         }
//     }
//
//     /// Replace the current result if the new one is better
//     ///
//     /// A result is better if:
//     /// - It is `Some` and the current result is `None`
//     /// - It has a higher error position, and is not `None` when current is `Some`
//     pub fn replace_if_better(&mut self, obj: Option<O>, errors: Vec<SyntaxError<T>>) -> bool {
//         if obj.is_none() && self.obj.is_some() {
//             return false;
//         }
//         if obj.is_some() && self.obj.is_none() {
//             let highest = Self::compute_highest(&errors);
//             self.do_replace(obj, errors, highest);
//             return true;
//         }
//         let highest = Self::compute_highest(&errors);
//         if highest > self.highest {
//             self.do_replace(obj, errors, highest);
//             return true;
//         }
//
//         false
//     }
//        
//         /// Map the internal object to another type
//         #[inline]
//         pub fn map<O2, F: FnOnce(O) -> O2>(self, f: F) -> SyntaxErrors<T, O2> {
//         SyntaxErrors {
//             obj: self.obj.map(f),
//             errors: self.errors,
//             highest: self.highest,
//         }
//         }
//
//     // pub fn unwrap_or_into<ST2: SyntaxTree<T=ST::T>>(self) -> Result<(ST, Vec<SyntaxError<ST::T>>), SyntaxErrors<ST2>> {
//     //     match self.obj {
//     //         Some(obj) => Ok((obj, self.errors)),
//     //         None => Err(SyntaxErrors::new(None, self.errors)),
//     //     }
//     // }
//     //
//     // pub fn unwrap_or_extend<ST2: SyntaxTree<T=ST::T>>(self, errors: &mut Vec<SyntaxError<ST::T>>) -> Result<ST, SyntaxErrors<ST2>> {
//     //     let (value, new_errors) = self.unwrap_or_into()?;
//     //     errors.extend(new_errors);
//     //     Ok(value)
//     // }
//
//     fn do_replace(&mut self, obj: Option<O>, errors: Vec<SyntaxError<T>>, new_highest: Pos) {
//         self.obj = obj;
//         self.errors = errors;
//         self.highest = new_highest;
//     }
//
//     fn compute_highest(errors: &[SyntaxError<T>]) -> Pos {
//         errors.iter().map(|e| e.span.lo).min().unwrap_or(0)
//     }
//    
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::Token;
//
//     use super::*;
//
//     // #[crate::teleparse_derive(TokenType)]
//     // pub enum TT {
//     //     #[teleparse(regex(r"^//.*"))]
//     //     Comment,
//     //     #[teleparse(terminal(Keyword = r"fn"))]
//     //     Keyword,
//     // }
//     //
//     // fn mock_syntax_error(lo: Pos) -> SyntaxError<TT> {
//     //     SyntaxError::new(Span { lo, hi: lo + 1 }, SyntaxErrorKind::UnexpectedToken)
//     // }
//     //
//     // fn mock_some_st() -> Option<Keyword> {
//     //     Some(Keyword(Token::new((0, 1), TT::Keyword)))
//     // }
//     //
//     // #[test]
//     // fn replace_if_none_to_some() {
//     //     let errors = SyntaxErrors::<TT, Keyword>::new(None, vec![
//     //         mock_syntax_error(2),
//     //     ]);
//     //     assert!(errors.clone().replace_if_better(mock_some_st(), vec![]));
//     //     for i in 1..4 {
//     //         assert!(errors.clone().replace_if_better(mock_some_st(), vec![
//     //             mock_syntax_error(i),
//     //         ]));
//     //     }
//     // }
//     //
//     // #[test]
//     // fn no_replace_if_some_to_none() {
//     //     let errors = SyntaxErrors::new(mock_some_st(), vec![
//     //         mock_syntax_error(2),
//     //     ]);
//     //     assert!(!errors.clone().replace_if_better(None, vec![]));
//     //     for i in 1..4 {
//     //         assert!(!errors.clone().replace_if_better(None, vec![
//     //             mock_syntax_error(i),
//     //         ]));
//     //     }
//     // }
//     //
//     // #[test]
//     // fn replace_if_reach_farther() {
//     //     for x in [mock_some_st(), None] {
//     //         let errors = SyntaxErrors::new(x, vec![
//     //             mock_syntax_error(2),
//     //         ]);
//     //         assert!(errors.clone().replace_if_better(x, vec![
//     //             mock_syntax_error(3),
//     //         ]));
//     //     }
//     // }
//     //
//     // #[test]
//     // fn no_replace_if_reach_less() {
//     //     for x in [mock_some_st(), None] {
//     //         let errors = SyntaxErrors::new(x, vec![
//     //             mock_syntax_error(2),
//     //             mock_syntax_error(2),
//     //         ]);
//     //         assert!(!errors.clone().replace_if_better(x, vec![
//     //             mock_syntax_error(1),
//     //         ]));
//     //         assert!(!errors.clone().replace_if_better(x, vec![
//     //             mock_syntax_error(1),
//     //             mock_syntax_error(3),
//     //         ]));
//     //         assert!(!errors.clone().replace_if_better(x, vec![
//     //             mock_syntax_error(3),
//     //             mock_syntax_error(1),
//     //         ]));
//     //     }
//     // }
// }

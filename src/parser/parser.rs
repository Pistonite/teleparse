
use std::marker::PhantomData;

use crate::lex::{Lexer, Set, Token, TokenSrc};
use crate::syntax::{self, Error, ErrorKind, FirstSet, FollowSet, Metadata, Result as SynResult};
use crate::{GrammarError, Lexicon, Production, Span, ToSpan};

use super::{Info, Root, Produce};

pub struct Parser<'s, L: Lexicon> {
    /// The core state of the parser
    info: Info<'s, L>,
    /// The lexer
    lexer: L::Lexer<'s>,
    /// The next token
    peeked: Option<Token<L>>,
}

impl<'s, L: Lexicon> Parser<'s, L> {
    /// Create a new Parser for the lexicon `L`
    pub fn new(source: &'s str) -> Result<Self, GrammarError> {
        Ok(Self {
            info: Info::new(source),
            lexer: L::lexer(source)?,
            peeked: None,
        })
    }

    #[inline]
    pub fn info(&self) -> &Info<'s, L> {
        &self.info
    }

    #[inline]
    pub fn info_mut(&mut self) -> &mut Info<'s, L> {
        &mut self.info
    }

    ///////////////////////////////////////////////////////////
    // High-level API
    ///////////////////////////////////////////////////////////
    
    // /Attempt to parse the syntax tree root once.
    // ///
    // /Note that if you are parsing the same root multiple times, 
    // /it's more efficient to use [`Parser::iter`]
    #[inline]
    pub fn parse<T: Root>(&mut self) -> Result<Option<T>, GrammarError> 
    where T::Prod : Production<L=L>
    {
        let meta = match T::metadata() {
            Ok(meta) => meta,
            Err(err) => return Err(err.clone()),
        };
        Ok(self.parse_with_meta::<T>(&meta))
    }

    // /// Create an iterator that can be used to parse all syntax tree roots in the source
    // pub fn iter<'p, R: ParseRoot>(&'p mut self) -> Result<ParserIter<'s, 'p, L, R>, GrammarError>
    // where R::AST : AbstractSyntaxRoot<L=L>
    // {
    //     ParserIter::<'s, 'p, L, R>::new(self)
    // }
    //
    // /// Parses all roots in the source until the end, returning the results in a Vec.
    // /// This is equivalent to calling `Parser::iter` and collecting the results.
    // #[inline]
    // pub fn parse_all<R: ParseRoot>(&mut self) -> Result<Vec<R> , GrammarError>
    // where R::AST : AbstractSyntaxRoot<L=L> {
    //     Ok(self.iter()?.collect())
    // }

    fn parse_with_meta<T: Root>(&mut self, meta: &Metadata<L>) -> Option<T> 
    where T::Prod : Production<L=L>
    {
        match T::produce(self, meta) {
            SynResult::Success(tree) => {
                Some(tree)
            }
            SynResult::Recovered(tree, errors) => {
                self.info.errors.extend(errors);
                Some(tree)
            }
            SynResult::Panic(errors) => {
                self.info.errors.extend(errors);
                None
            }
        }
    }

    // /// Attempt to parse one syntax tree into the state, may skip invalid tokens and characters
    // /// to form a valid syntax tree
    // ///
    // /// Return None if a valid syntax tree could not be formed
    // /// when the end of the source is reached
    // fn parse_ast_with_meta<AST: AbstractSyntaxTree<L=L>>(&mut self, meta: &Metadata<L>) -> Option<AST> {
    //     // assume the input will have 0 or more invalid tokens at front and in the end
    //     let first = meta.first.get(&AST::type_id());
    //     let mut peek_token = self.peek_token_src();
    //
    //     while !first.contains(peek_token) && peek_token.is_some() {
    //         if let Some(t) = self.consume_token() {
    //             self.info.invalid_tokens.push(t);
    //         }
    //         peek_token = self.peek_token_src();
    //     }
    //
    //     let lo = self.current_span().lo;
    //
    //     if lo != 0 {
    //         self.info.errors.push(Error::new(
    //             Span::new(0, lo), 
    //             ErrorKind::UnexpectedTokens));
    //     }
    //
    //         let ast = match AST::parse_ast(self, meta) {
    //             SynResult::Success(tree) => {
    //                 Some(tree)
    //             }
    //             SynResult::Recovered(tree, errors) => {
    //                 self.info.errors.extend(errors);
    //                 Some(tree)
    //             }
    //             SynResult::Panic(errors) => {
    //                 self.info.errors.extend(errors);
    //             None
    //             }
    //         };
    //
    //     if let Some(t) = self.peek_token() {
    //         self.info.errors.push(Error::new(
    //             Span::new(t.span.hi, self.info.source.len()),
    //             ErrorKind::UnexpectedTokens));
    //     }
    //
    //     ast
    // }

    ///////////////////////////////////////////////////////////
    // Stream API
    ///////////////////////////////////////////////////////////
    
    /// Parse a token of a specific type
    ///
    /// ## Consumption
    /// The parser only consumes the next token if it matches the expected type
    ///
    /// ## Recovery
    /// None
    ///
    /// ## Panic
    /// If the token is not of the expected type, the parser panics, and will not consume the
    /// token.
    pub fn parse_token( &mut self, ty: L) -> SynResult<Token<L>, L> {
        let token = match self.peek_token() {
            Some(token) => token,
            None => return SynResult::Panic(vec![self.unexpected_eof()]),
        };
        if token.ty != ty {
            let expected = FirstSet::one(ty, None);
            let error = syntax::Error::new(token.span, ErrorKind::Expecting(expected));
            return SynResult::Panic(vec![error]);
        }
        self.consume_token();
        SynResult::Success(token)
    }

    /// Parse a token of a specific type, matching a specific literal
    ///
    /// ## Consumption
    /// The parser only consumes the next token if it matches the expected type and literal.
    ///
    /// ## Recovery
    /// If the next token does not match, but is in the follow set passed in,
    /// the parser will insert the expected token with a 0-length span and continue.
    /// This also applies to the EOF token.
    ///
    /// ## Panic
    /// The parser panics if the next token does not match and is not in the follow set.
    /// The next token is not consumed.
    /// 
    pub fn parse_token_lit( &mut self, ty: L, match_lit: &'static str
        , follow: &FollowSet<L>
    ) -> SynResult<Token<L>, L> {
        let token = match self.peek_token() {
            Some(token) => token,
            None =>  {
                if follow.contains_eof() {
                    let expecting = FirstSet::one(ty, Some(match_lit));
                    let token = Token::new(self.current_span(), ty);
                    return SynResult::Recovered(token, vec![self.expecting(expecting)]);
                }
                return SynResult::Panic(vec![self.unexpected_eof()]);
            }
        };

        let token_src = self.info.to_src(&token);

        if token.ty == ty && token_src.src == match_lit {
            self.consume_token();
            return SynResult::Success(token);
        }

        let expecting = FirstSet::one(ty, Some(match_lit));
        if follow.contains(Some(token_src)) {
            // do not consume the next token
            let token = Token::new(self.current_span_empty(), ty);
            return SynResult::Recovered(token, vec![self.expecting(expecting)]);
        }

        SynResult::Panic(vec![self.expecting(expecting)])
    }

    pub fn peek_token_src(&mut self) -> Option<TokenSrc<'s, L>> {
        self.peek_token().map(|t| self.info.to_src(&t))
    }

    /// Create a syntax error for an unexpected end of file
    pub fn unexpected_eof(&mut self) -> Error<L>{
        Error::new(self.current_span(), ErrorKind::UnexpectedEof)
    }

    pub fn expecting(&mut self, expected: FirstSet<L>) -> Error<L> {
        Error::new(self.current_span(), ErrorKind::Expecting(expected))
    }

    /// Consume the next token
    pub fn consume_token(&mut self) -> Option<Token<L>> {
        if self.peeked.is_none() {
            self.try_read_next_token();
        }
        match self.peeked.take() {
            Some(token) => {
                self.info.tokens.push_unchecked(token);
                return Some(token);
            }
            None => {
                return None;
            }
        }
    }

    /// Return the next token in the input but does not consume it
    pub fn peek_token(&mut self) -> Option<Token<L>> {
        if self.peeked.is_none() {
            self.try_read_next_token();
        }
        return self.peeked;

    }

    /// If the next token is available, read it and store it in `peeked`
    fn try_read_next_token(&mut self) {
        debug_assert!(self.peeked.is_none());
        // read until a token that is not extract
        loop {
            let (invalid, token) = self.lexer.next();
            let info = &mut self.info;
            if let Some(span) = invalid {
                info.invalid_source.push(span);
                info.errors.push(Error::new(span, ErrorKind::UnexpectedCharacters));
            }
            if let Some(token) = token {
                if !token.ty.should_extract() {
                    self.peeked = Some(token);
                    return;
                } 
                info.extracted_tokens.push(token);
                continue;
            }
            return;
        }
    }

    pub fn remaining(&mut self) -> &'s str {
        let lo = self.current_span().lo;
        &self.info.source[lo..]
    }

    /// Get the span of the lookahead token, or the EOF span if there is no more tokens
    pub fn current_span(&mut self) -> Span {
        self.peek_token().map(|t| t.span).unwrap_or_else(|| self.info.eof())
    }

    /// Get start of the lookahead token as a 0-length span
    pub fn current_span_empty(&mut self) -> Span {
        let span = self.current_span();
        Span::new(span.lo, span.lo)
    }

    pub fn apply_semantic<S: ToSpan>(&mut self, span: &S, semantic: Set<L>) {
        self.info.tokens.inside_mut(span.span()).apply_semantic(semantic);
    }




}


//
// pub struct ParserIter<'s, 'p, L: Lexicon, R>
//     {
//     parser: &'p mut Parser<'s, L>,
//     metadata: &'static Metadata<L>,
//         _marker: PhantomData<R>,
// }
//
// impl<'s, 'p, L: Lexicon, R: ParseRoot>  ParserIter<'s, 'p, L, R>
//     where R::AST : AbstractSyntaxRoot<L=L>
// {
//     pub fn new(parser: &'p mut Parser<'s, L>) -> Result<Self, GrammarError> {
//         let metadata = match R::AST::metadata() {
//             Ok(meta) => meta,
//             Err(err) => return Err(err.clone()),
//         };
//         Ok(Self {
//             parser,
//             metadata,
//             _marker: PhantomData,
//         })
//     }
// }
//
// impl<'s, 'p, L: Lexicon, R: ParseTree> Iterator for ParserIter<'s, 'p, L, R>
//     where R::AST : AbstractSyntaxTree<L=L>
// {
//     type Item = R;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.parser.next_root(&self.metadata)
//     }
// }
//
//
// pub struct ParseRootIter<'s, L: Lexicon, R>
//     {
//     parser: Parser<'s, L>,
//     metadata: &'static Metadata<L>,
//         _marker: PhantomData<R>,
// }
//
// impl<'s, L: Lexicon, R: ParseRoot>  ParseRootIter<'s, L, R>
//     where R::AST : AbstractSyntaxRoot<L=L>
// {
//     pub fn new(parser: Parser<'s, L>) -> Result<Self, GrammarError> {
//         let metadata = match R::AST::metadata() {
//             Ok(meta) => meta,
//             Err(err) => return Err(err.clone()),
//         };
//         Ok(Self {
//             parser,
//             metadata,
//             _marker: PhantomData,
//         })
//     }
// }
//
// impl<'s, L: Lexicon, R: ParseTree> Iterator for ParseRootIter<'s, L, R>
//     where R::AST : AbstractSyntaxTree<L=L>
// {
//     type Item = R;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.parser.next_root(&self.metadata)
//     }
// }
//


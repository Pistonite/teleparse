
use std::marker::PhantomData;

use crate::lex::{Lexer, Set, Token, TokenSrc};
use crate::syntax::{self, Error, ErrorKind, FirstSet, FollowSet, Metadata, Result as SynResult};
use crate::{AbstractSyntaxRoot, AbstractSyntaxTree, GrammarError, Lexicon, Span, ToSpan};

use super::{Info, ParseRoot, ParseTree};

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
    
    /// Attempt to parse the syntax tree root once.
    ///
    /// Note that if you are parsing the same root multiple times, 
    /// it's more efficient to use [`Parser::iter`]
    #[inline]
    pub fn parse_one<R: ParseRoot>(&mut self) -> Result<Option<R>, GrammarError>
    where R::AST : AbstractSyntaxRoot<L=L>
    {
        Ok(self.iter()?.next())
    }

    /// Create an iterator that can be used to parse all syntax tree roots in the source
    pub fn iter<'p, R: ParseRoot>(&'p mut self) -> Result<ParserIter<'s, 'p, L, R>, GrammarError>
    where R::AST : AbstractSyntaxRoot<L=L>
    {
        ParserIter::<'s, 'p, L, R>::new(self)
    }
    
    /// Parses all roots in the source until the end, returning the results in a Vec.
    /// This is equivalent to calling `Parser::iter` and collecting the results.
    #[inline]
    pub fn parse_all<R: ParseRoot>(&mut self) -> Result<Vec<R> , GrammarError>
    where R::AST : AbstractSyntaxRoot<L=L> {
        Ok(self.iter()?.collect())
    }

    fn next_root<R: ParseTree>(&mut self, meta: &Metadata<L>) -> Option<R> 
    where R::AST : AbstractSyntaxTree<L=L>
    {
        let ast = self.next_ast(meta)?;
        Some(R::from_ast(ast, self))
    }

    /// Attempt to parse one syntax tree into the state, may skip invalid tokens and characters
    /// to form a valid syntax tree
    ///
    /// Return None if a valid syntax tree could not be formed
    /// when the end of the source is reached
    fn next_ast<AST: AbstractSyntaxTree<L=L>>(&mut self, meta: &Metadata<L>) -> Option<AST> {
        // if there are no more tokens then we don't need to even try
        if self.peek_token().is_none() {
            return None;
        }
        loop {
            let start = self.current_span();
            match AST::parse_ast(self, meta) {
                SynResult::Success(tree) => {
                    return Some(tree);
                }
                SynResult::Recovered(tree, errors) => {
                    self.info.errors.extend(errors);
                    return Some(tree);
                }
                SynResult::Panic(errors) => {
                    self.info.errors.extend(errors);
                }
            }
            // panic recover
            // if the parser did not advance, then we need to manually
            // advance so we don't get stuck in the same position
            if start.lo == self.current_span().lo {
                self.consume_token();
            }

            // since the parser only looks ahead 1 token,
            // it will not generate errors for further tokens
            // which means it's valid to generate errors for skipped tokens here

            let mut skipped_token = false;
            let mut span = self.current_span();

            let first = meta.first.get(&AST::type_id());
            loop {
                match self.peek_token() {
                    None => {
                        // no more tokens
                        if skipped_token {
                            span.hi = self.info.source.len();
                            self.info.errors.push(
                                Error::new(
                                    span, 
                                    ErrorKind::UnexpectedTokens));
                        }
                        return None;
                    }
                    Some(token) => {
                        let token_src = self.info.to_src(&token);
                        if first.contains(Some(token_src)) {
                            if skipped_token {
                                span.hi = token.span.hi;
                                self.info.errors.push(
                                    Error::new(
                                        span, 
                                        ErrorKind::UnexpectedTokens));
                            }
                            break;
                        }
                        skipped_token = true;
                        self.consume_token();
                    }
                };
            }
            // try again
        }
    }

    ///////////////////////////////////////////////////////////
    // Stream API
    ///////////////////////////////////////////////////////////
    
    /// Parse a token of a specific type
    ///
    /// ## Recovery
    /// None. If the token is not of the expected type, the token is consumed and the parser panics
    pub fn parse_token( &mut self, ty: L) -> SynResult<Token<L>, L> {
        let token = match self.consume_token() {
            Some(token) => token,
            None => return SynResult::Panic(vec![self.unexpected_eof()]),
        };
        if token.ty != ty {
            let expected = FirstSet::one(ty, None);
            let error = syntax::Error::new(token.span, ErrorKind::Expecting(expected));
            return SynResult::Panic(vec![error]);
        }
        SynResult::Success(token)
    }

    /// Parse a token of a specific type, matching a specific literal
    ///
    /// ## Recovery
    /// The parser peeks the next token. If it matches, the token is then consumed and returned.
    /// Otherwise:
    /// - If the next token is EOF and EOF is in FOLLOW, the missing token will be inserted.
    /// - If the next token is in FOLLOW, the missing token will be inserted
    /// - Otherwise, the parser panics without consuming the token
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
    fn consume_token(&mut self) -> Option<Token<L>> {
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



pub struct ParserIter<'s, 'p, L: Lexicon, R>
    {
    parser: &'p mut Parser<'s, L>,
    metadata: &'static Metadata<L>,
        _marker: PhantomData<R>,
}

impl<'s, 'p, L: Lexicon, R: ParseRoot>  ParserIter<'s, 'p, L, R>
    where R::AST : AbstractSyntaxRoot<L=L>
{
    pub fn new(parser: &'p mut Parser<'s, L>) -> Result<Self, GrammarError> {
        let metadata = match R::AST::metadata() {
            Ok(meta) => meta,
            Err(err) => return Err(err.clone()),
        };
        Ok(Self {
            parser,
            metadata,
            _marker: PhantomData,
        })
    }
}

impl<'s, 'p, L: Lexicon, R: ParseTree> Iterator for ParserIter<'s, 'p, L, R>
    where R::AST : AbstractSyntaxTree<L=L>
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next_root(&self.metadata)
    }
}


pub struct ParseRootIter<'s, L: Lexicon, R>
    {
    parser: Parser<'s, L>,
    metadata: &'static Metadata<L>,
        _marker: PhantomData<R>,
}

impl<'s, L: Lexicon, R: ParseRoot>  ParseRootIter<'s, L, R>
    where R::AST : AbstractSyntaxRoot<L=L>
{
    pub fn new(parser: Parser<'s, L>) -> Result<Self, GrammarError> {
        let metadata = match R::AST::metadata() {
            Ok(meta) => meta,
            Err(err) => return Err(err.clone()),
        };
        Ok(Self {
            parser,
            metadata,
            _marker: PhantomData,
        })
    }
}

impl<'s, L: Lexicon, R: ParseTree> Iterator for ParseRootIter<'s, L, R>
    where R::AST : AbstractSyntaxTree<L=L>
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next_root(&self.metadata)
    }
}



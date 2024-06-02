use std::marker::PhantomData;

use crate::{token::TokenSrc, Lexer, Root, Span, SyntaxError, SyntaxErrorKind, SyntaxResult, SyntaxResultExt, SyntaxTree, Token, TokenStorage, TokenType};
// use crate::root::RootMetadata;

pub struct Parser<'s, T: TokenType> {
    /// The context
    pub context: T::Ctx,
    /// The parts of the source code that were unable to be parsed
    pub invalid_source: Vec<Span>,
    /// The tokens that were unable to be parsed with the syntax
    pub invalid_tokens: Vec<Token<T>>,
    /// The valid tokens parsed
    pub tokens: TokenStorage<T>,
    /// The encountered tokens that were marked with extract
    pub extracted_tokens: TokenStorage<T>,
    /// Errors encountered during parsing
    pub errors: Vec<SyntaxError<T>>,
    /// The lexer
    lexer: T::Lexer<'s>,
    /// The source code to parse
    source: &'s str,

    // position stack

    /// Position stack for backtracking. Elements are indices into tokens
    pos_stack: Vec<usize>,
    /// Current index in the tokens, this might be equal to tokens.len()
    /// meaning a new token should be read from the lexer
    idx: usize,
    /// The max state size for the pos stack
    max_stack_size: usize,
}

impl<'s, T: TokenType> Parser<'s, T> {
    /// Create a new ParseState
    pub fn new_with_context(source: &'s str, context: T::Ctx) -> Self {
        Self {
            source,
            lexer: T::lexer(source),
            context,
            extracted_tokens: TokenStorage::new(),
            invalid_source: Vec::new(),
            invalid_tokens: Vec::new(),
            tokens: TokenStorage::new(),
            pos_stack: Vec::new(),
            idx: 0,
            errors: Vec::new(),
            max_stack_size: 2048,
        }
    }

    pub fn set_max_stack_size(&mut self, size: usize) {
        self.max_stack_size = size;
    }

    // /// Attempt to parse the syntax tree root once.
    // ///
    // /// Note that if you are parsing multiple roots, it's more efficient to use [`Parser::iter`]
    // pub fn once<ST: Root<T=T>>(&mut self) -> Option<ST> {
    //     self.iter().next()
    // }

    // /// Attempt to parse one syntax tree into the state, may skip invalid tokens and characters
    // /// to form a valid syntax tree
    // ///
    // /// Return None if a valid syntax tree could not be formed
    // /// when the end of the source is reached
    // fn next_with_follow_table<ST: Root<T=T>>(&mut self, f_table: &SyntaxTreeTable<T>) -> Option<ST> {
    //     let mut error_already_added = false;
    //     loop {
    //         match self.try_parse_internal(f_table) {
    //             Ok(tree) => {
    //                 return Some(tree);
    //             }
    //             Err(errors) => {
    //                 self.errors.extend(errors.errors);
    //                 if errors.obj.is_some() {
    //                     return errors.obj;
    //                 }
    //             }
    //         }
    //         // cannot parse a tree, skip a token and try again
    //         let token = match self.consume_token() {
    //             None => {
    //                 // no more tokens
    //                 return None;
    //             }
    //             Some(token) => token,
    //         };
    //         // only add error if this is the first token being skipped
    //         // for this attempt
    //         if !error_already_added {
    //             self.errors.push(
    //                 SyntaxError::new(
    //                     token.span, 
    //                     SyntaxErrorKind::UnexpectedToken));
    //             error_already_added = true;
    //         }
    //     }
    // }

    // /// Create an iterator that can be used to parse all syntax tree roots in the source
    // pub fn iter<ST: Root<T=T>>(&mut self) -> ParserIter<'s, '_, T, ST> {
    //     ParserIter { state: self, metadata: ST::root_metadata() }
    // }
    //
    // /// Parses all syntax trees in the source until the end
    // pub fn parse_all <ST:Root<T=T>> (&mut self) -> Vec<ST> {
    //     self.iter().collect()
    // }

    // /// Internally parse a syntax tree node from the state and apply semantic info
    // fn try_parse_internal <ST: SyntaxTree<T=T>> (&mut self, f_table: &SyntaxTreeTable<T>) -> SyntaxResult<T, ST> {
    //     ST::try_parse_ast(self, f_table, true).map_ext(|ast| {
    //         ST::into_parse_tree(ast, self)
    //     })
    // }

}

// pub struct ParserIter<'s, 'p, T: TokenType, ST: Root<T=T>> {
//     state: &'p mut Parser<'s, T>,
//     metadata: &'static RootMetadata<ST>,
// }
//
// impl<'s, 'p, T: TokenType, ST: Root<T=T>> Iterator for ParserIter<'s, 'p, T, ST> {
//     type Item = ST;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.state.next_with_follow_table(&self.metadata.follow_table)
//     }
// }


/// Internal state management for the parser
///
/// ## Note
/// This trait is meant to be only used internally by the [`SyntaxTree`] nodes.
pub trait ParserState<'s> {
    type T: TokenType;

    // /// Push the current position to the stack so it can be restored later
    // ///
    // /// Returns false if the stack is full
    // fn push_state(&mut self) -> Result<(), SyntaxError<Self::T>>;

    /// Pop the stack but do not restore the position
    fn pop_state(&mut self);

    /// Restore the position to stack top without popping
    fn restore_state(&mut self);

    /// Get the source code of a token
    fn get_src(&self, token: &Token<Self::T>) -> &'s str;

    /// Get the source code of a span
    fn get_src_span(&self, span: Span) -> &'s str;

    /// Get the span of the current token
    fn current_span(&mut self) -> Span;

    /// Get and consume the current token, advancing the token stream position
    fn consume_token(&mut self) -> Option<Token<Self::T>>;

    /// Get the next token but does not consume it
    fn peek_token(&mut self) -> Option<Token<Self::T>>;

    /// Peek the token as TokenSrc
    fn peek_token_src(&mut self) -> Option<TokenSrc<'s, Self::T>>;

    // /// Create a syntax error for an unexpected end of file
    // fn unexpected_eof(&mut self) -> SyntaxError<Self::T>{
    //     SyntaxError::new(self.current_span(), SyntaxErrorKind::UnexpectedEof)
    // }

    // fn expecting(&mut self, expected: FirstSet<Self::T>) -> SyntaxError<Self::T> {
    //     SyntaxError::new(self.current_span(), SyntaxErrorKind::Expecting(expected))
    // }

    // /// Parse a token of a specific type
    // #[inline(never)]
    // fn parse_token( &mut self, ty: Self::T) -> Result<Token<Self::T>, SyntaxError<Self::T>> {
    //     let token = self.parse_token_type(ty)?;
    //     // let next = self.peek_token_src();
    //     Ok(token)
    //     // if follows.contains(next) {
    //     // } else {
    //     //     Err(self.expecting(follows.clone()))
    //     // }
    // }

    // #[inline(never)]
    // fn parse_token_match( &mut self, ty: Self::T, match_lit: &'static str) -> Result<Token<Self::T>, SyntaxError<Self::T>> {
    //     let token = self.parse_token_type(ty)?;
    //     if self.get_src(&token) != match_lit {
    //         let mut expected = FirstSet::default();
    //         expected.insert_token_type_match(ty, match_lit.into());
    //         return Err(self.expecting(expected));
    //     }
    //     // let next = self.peek_token_src();
    //     // if follows.contains(next) {
    //         Ok(token)
    //     // } else {
    //     //     Err(self.expecting(follows.clone()))
    //     // }
    // }

    // #[inline]
    // fn parse_token_type( &mut self, ty: Self::T) -> Result<Token<Self::T>, SyntaxError<Self::T>> {
    //     let token = match self.consume_token() {
    //         Some(token) => token,
    //         None => Err(self.unexpected_eof())?,
    //     };
    //     if token.ty != ty {
    //         Err(token.unexpected())
    //     } else {
    //         Ok(token)
    //     }
    // }
}

impl<'s, T: TokenType> ParserState<'s> for Parser<'s, T> {
    type T = T;

    // fn push_state(&mut self) -> Result<(), SyntaxError<T>> {
    //     if self.pos_stack.len() >= self.max_stack_size {
    //         return Err(SyntaxError::new(
    //             self.current_span(),
    //             SyntaxErrorKind::StackOverflow,
    //         ))
    //     }
    //     self.pos_stack.push(self.idx);
    //     Ok(())
    // }

    #[inline]
    fn pop_state(&mut self) {
        self.pos_stack.pop();
    }

    #[inline]
    fn restore_state(&mut self) {
        if let Some(pos) = self.pos_stack.last() {
            self.idx = *pos;
        }
    }

    #[inline]
    fn get_src(&self, token: &Token<T>) -> &'s str {
        token.get_src(self.source)
    }

    #[inline]
    fn get_src_span(&self, span: Span) -> &'s str {
        span.get_src(self.source)
    }

    #[inline]
    fn current_span(&mut self) -> Span {
        self.peek_token().map(|t| t.span).unwrap_or_else(|| Span::new(self.source.len(), self.source.len()))
    }

    fn consume_token(&mut self) -> Option<Token<T>> {
        let token = self.peek_token();
        if token.is_some() {
            self.idx += 1;
        }
        token
    }

    fn peek_token(&mut self) -> Option<Token<T>> {
        if let Some(token) = self.tokens.at(self.idx) {
            return Some(**token);
        }
        // read until a token that is not extract
        loop {
            let (invalid, token) = self.lexer.next();
            if let Some(span) = invalid {
                self.invalid_source.push(span);
                self.errors.push(SyntaxError::new(span, SyntaxErrorKind::UnexpectedCharacters));
            }
            if let Some(token) = token {
                if !token.ty.should_extract() {
                    self.tokens.add_last(token);
                    return Some(token);
                } 
                self.extracted_tokens.add_last(token);
                continue;
            }
            return None;
        }
    }

    fn peek_token_src(&mut self) -> Option<TokenSrc<'s, Self::T>> {
        self.peek_token().map(|t| t.to_src(self.source))
    }
}

use std::ops::Deref;
use crate::{lexer::Lexer, Span, Token, TokenStorage, TokenType};


pub trait SyntaxTree<'s>: Sized {
    type T: TokenType;
    type Ctx;

    /// Create an iterator to continue parsing this syntax tree from a state until the end of the
    /// source is reached
    fn parse_iter<'p, L: Lexer<'s, T= Self::T>>(state: &'p mut ParseState<'s, Self::T, L, Self>) -> SyntaxTreeIter<'s, 'p, Self::T, L, Self> {
        SyntaxTreeIter { state }
    }

    /// Attempt to parse one syntax tree into the state, may skip invalid tokens and characters
    /// to form a valid syntax tree
    ///
    /// Return None if a valid syntax tree could not be formed
    /// when the end of the source is reached
    fn parse_one<L: Lexer<'s, T = Self::T>>(
        state: &mut ParseState<'s, Self::T, L, Self>) -> Option<Self> {
        let mut error_already_added = false;
        loop {
            if let Some(tree) = Self::try_parse(state) {
                return Some(tree);
            }
            // cannot parse a tree, skip a token and try again
            let token = match state.consume_token() {
                None => {
                    // no more tokens
                    return None;
                }
                Some(token) => token,
            };
            // only add error if this is the first token being skipped
            // for this attempt
            if !error_already_added {
                state.errors.push(
                    SyntaxError::new(
                        token.span, 
                        SyntaxErrorKind::UnexpectedToken));
                error_already_added = true;
            }
        }
    }

    /// Parse a syntax tree node from the state and apply semantic info
    fn try_parse<L: Lexer<'s, T = Self::T>>(state: &mut ParseState<'s, Self::T, L, Self>) -> Option<Self> {
        let out = Self::try_make_tree(state);
        if let Some(tree) = out.as_ref() {
            tree.apply_semantic(state);
        }
        out
    }

    /// Attempt to parse one syntax tree node from the state
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn try_make_tree<L: Lexer<'s, T=Self::T>>(state: &mut ParseState<'s, Self::T, L, Self>) -> Option<Self>;

    /// Apply the semantic info to the token storage in the state
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn apply_semantic<L: Lexer<'s, T=Self::T>>(&self, state: &mut ParseState<'s, Self::T, L, Self>);
}

pub struct SyntaxTreeIter<'s, 'p, T, L, ST> 
    where
        T: TokenType,
        L: Lexer<'s, T= T>,
        ST: SyntaxTree<'s, T= T>,
    {
    state: &'p mut ParseState<'s, T, L, ST>,
}

impl<'s, 'p, T, L,ST> Iterator for SyntaxTreeIter<'s, 'p, T, L, ST> 
    where
        T: TokenType,
        L: Lexer<'s, T= T>,
        ST: SyntaxTree<'s, T= T>,
    {
    type Item = ST;

    fn next(&mut self) -> Option<Self::Item> {
        ST::parse_one(&mut self.state)
    }
}

/// Output of parse_one
pub struct ParseState<'s, T, L, ST>
    where
        T: TokenType,
        L: Lexer<'s, T= T>,
        ST: SyntaxTree<'s, T= T>,
    {
    /// The source code to parse
    source: &'s str,
    /// The lexer
    pub lexer: L,
    /// The context
    pub context: ST::Ctx,
    /// The encountered tokens that were marked with extract
    pub extracted_tokens: Vec<Token<T>>,
    /// The parts of the source code that were unable to be parsed
    pub invalid_source: Vec<Span>,
    /// The tokens that were unable to be parsed with the syntax
    pub invalid_tokens: Vec<Token<T>>,
    /// The valid tokens parsed
    pub tokens: TokenStorage<T>,
    /// Position stack for backtracking. Elements are indices into tokens
    pub pos_stack: Vec<usize>,
    /// Current index in the tokens, this might be equal to tokens.len()
    /// meaning a new token should be read from the lexer
    pub idx: usize,
    /// The max state size for the pos stack
    max_stack_size: usize,
    /// Errors encountered during parsing
    pub errors: Vec<SyntaxError>,
}

impl<'s, T, L, ST> ParseState<'s, T, L, ST> 
    where
        T: TokenType,
        L: Lexer<'s, T = T>,
        ST: SyntaxTree<'s, T = T>,
    {
    /// Create a new ParseState
    pub fn new_with_context(source: &'s str, context: ST::Ctx) -> Self {
        Self {
            source,
            lexer: L::new(source),
            context,
            extracted_tokens: Vec::new(),
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

    /// Get and consume the current token, advancing the token stream position
    pub fn consume_token(&mut self) -> Option<Token<T>> {
        let token = self.get_or_read_token();
        if token.is_some() {
            self.idx += 1;
        }
        token
    }

    /// Push the current position to the stack so it can be restored later
    ///
    /// Returns false if the stack is full
    #[must_use]
    pub fn push(&mut self) -> bool {
        if self.pos_stack.len() >= self.max_stack_size {
            return false;
        }
        self.pos_stack.push(self.idx);
        true
    }

    /// Pop the stack but do not restore the position
    #[inline]
    pub fn pop(&mut self) {
        self.pos_stack.pop();
    }

    /// Pop the stack and restore the position
    pub fn restore(&mut self) {
        if let Some(pos) = self.pos_stack.pop() {
            self.idx = pos;
        }
    }

    /// Get the token at self.idx, or None if the end of the source is reached
    fn get_or_read_token(&mut self) -> Option<Token<T>> {
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
                self.extracted_tokens.push(token);
                continue;
            }
            return None;
        }
    }
}

pub struct SyntaxError {
    pub span: Span,
    pub data: SyntaxErrorKind,
}

impl SyntaxError {
    pub fn new(span: Span, data: SyntaxErrorKind) -> Self {
        Self { span, data }
    }
}

pub enum SyntaxErrorKind {
    UnexpectedCharacters,
    UnexpectedToken,
}

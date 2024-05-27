use crate::{Lexer, Parser, ParserState, SyntaxResult, SyntaxResultExt, SyntaxTreeParser};

/// Parser for `#[llnparse(optional)]`, or inferred from [`Option`]
pub struct OptionParser<'s, In> where
    In: SyntaxTreeParser<'s>,
    {
    pub inner: In,
    _phantom: std::marker::PhantomData<&'s ()>,
}

impl<'s, In: SyntaxTreeParser<'s>> OptionParser<'s, In> {
    pub fn new(inner: In) -> Self {
        Self {
            inner,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'s, In: SyntaxTreeParser<'s>> SyntaxTreeParser<'s> for OptionParser<'s, In> {
    type T = In::T;
    type Ctx = In::Ctx;
    type Target = Option<In::Target>;

    fn try_parse<L: Lexer<'s, T=Self::T>>(
        &self, parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> SyntaxResult<Self::Target> {
        parser.push_state().map_ext(|_| {
            let result = self.inner.try_parse(parser).into_value();
            if result.is_none() {
                parser.restore_state();
            }
            parser.pop_state();
            result
        })
    }
}

/// Parser for `#[llnparse(presence)]`, or inferred from `bool`
pub struct PresenceParser<'s, In> where
    In: SyntaxTreeParser<'s>,
    {
    inner: OptionParser<'s, In>,
    _phantom: std::marker::PhantomData<&'s ()>,
}

impl<'s, In: SyntaxTreeParser<'s>> PresenceParser<'s, In> {
    pub fn new(inner: In) -> Self {
        Self {
            inner: OptionParser::new(inner),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'s, In: SyntaxTreeParser<'s>> SyntaxTreeParser<'s> for PresenceParser<'s, In> {
    type T = In::T;
    type Ctx = In::Ctx;
    type Target = bool;

    fn try_parse<L: Lexer<'s, T=Self::T>>(
        &self, parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> SyntaxResult<Self::Target> {
        self.inner.try_parse(parser).map_ext(|x| x.is_some())
    }
}

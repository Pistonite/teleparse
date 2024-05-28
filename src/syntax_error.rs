use crate::{Pos, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SyntaxError {
    pub span: Span,
    pub data: SyntaxErrorKind,
}

impl SyntaxError {
    pub fn new(span: Span, data: SyntaxErrorKind) -> Self {
        Self { span, data }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyntaxErrorKind {
    UnexpectedCharacters,
    UnexpectedToken,
    UnexpectedEof,
    StackOverflow,
}

impl<T> Into<SyntaxResult<T>> for SyntaxError {
    fn into(self) -> SyntaxResult<T> {
        Err(SyntaxErrors::new(None, vec![self]))
    }
}

/// Alias of result returned by a parsing setup
pub type SyntaxResult<T> = Result<T, SyntaxErrors<T>>;

/// Internal trait for mapping the result of a parsing setup
pub trait SyntaxResultExt<T> {
    fn map_ext<T2, F: FnOnce(T) -> T2>(self, f: F) -> SyntaxResult<T2>;

    fn as_value(&self) -> Option<&T>;
    fn into_value(self) -> Option<T>;

    fn has_value(&self) -> bool {
        self.as_value().is_some()
    }
    fn unwrap_or_extend_errors<T2>(self, errors: &mut Vec<SyntaxError>) -> Result<T, SyntaxErrors<T2>>;
}

impl<T> SyntaxResultExt<T> for SyntaxResult<T> {
    fn map_ext<T2, F: FnOnce(T) -> T2>(self, f: F) -> SyntaxResult<T2> {
        match self {
            Ok(tree) => Ok(f(tree)),
            Err(errors) => {
                Err(errors.map(f))
            }
        }
    }

    fn as_value(&self) -> Option<&T> {
        match &self {
            Ok(tree) => Some(tree),
            Err(errors) => errors.obj.as_ref(),
        }
    }

     fn into_value(self) -> Option<T> {
        match self {
            Ok(tree) => Some(tree),
            Err(errors) => errors.obj,
        }
    }

    fn unwrap_or_extend_errors<T2>(self, errors: &mut Vec<SyntaxError>) -> Result<T, SyntaxErrors<T2>> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => e.unwrap_extend_or_into(errors),
        }
    }
}

/// Wrapper for errors encountered during parsing. 
/// This is used to determine which errors should be reported
/// when all paths are throwing errors
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SyntaxErrors<T> {
    pub obj: Option<T>,
    pub errors: Vec<SyntaxError>,
    highest: Pos,
}

impl<T> Clone for SyntaxErrors<T> where T: Clone {
    fn clone(&self) -> Self {
        Self {
            obj: self.obj.clone(),
            errors: self.errors.clone(),
            highest: self.highest,
        }
    }
}

impl<T> SyntaxErrors<T> {
    /// Initialize the error state with parsing result and errors
    ///
    /// If `obj` is `Some`, it indicates partial success (i.e. there were errors
    /// but it was recovered
    pub fn new(obj: Option<T>, errors: Vec<SyntaxError>) -> Self {
        let highest = Self::compute_highest(&errors);
        Self {
            obj,
            errors,
            highest
        }
    }

    /// Replace the current result if the new one is better
    ///
    /// A result is better if:
    /// - It is `Some` and the current result is `None`
    /// - It has a higher error position, and is not `None` when current is `Some`
    pub fn replace_if_better(&mut self, obj: Option<T>, errors: Vec<SyntaxError>) -> bool {
        if obj.is_none() && self.obj.is_some() {
            return false;
        }
        if obj.is_some() && self.obj.is_none() {
            let highest = Self::compute_highest(&errors);
            self.do_replace(obj, errors, highest);
            return true;
        }
        let highest = Self::compute_highest(&errors);
        if highest > self.highest {
            self.do_replace(obj, errors, highest);
            return true;
        }

        false
    }

        /// Map the internal object to another type
        pub fn map<T2, F: FnOnce(T) -> T2>(self, f: F) -> SyntaxErrors<T2> {
        SyntaxErrors {
            obj: self.obj.map(f),
            errors: self.errors,
            highest: self.highest,
        }
        }

    pub fn unwrap_or_into<T2>(self) -> Result<(T, Vec<SyntaxError>), SyntaxErrors<T2>> {
        match self.obj {
            Some(obj) => Ok((obj, self.errors)),
            None => Err(SyntaxErrors::new(None, self.errors)),
        }
    }

    pub fn unwrap_extend_or_into<T2>(self, errors: &mut Vec<SyntaxError>) -> Result<T, SyntaxErrors<T2>> {
        let (value, new_errors) = self.unwrap_or_into()?;
        errors.extend(new_errors);
        Ok(value)
    }

    fn do_replace(&mut self, obj: Option<T>, errors: Vec<SyntaxError>, new_highest: Pos) {
        self.obj = obj;
        self.errors = errors;
        self.highest = new_highest;
    }

    fn compute_highest(errors: &[SyntaxError]) -> Pos {
        errors.iter().map(|e| e.span.lo).min().unwrap_or(0)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_syntax_error(lo: Pos) -> SyntaxError {
        SyntaxError::new(Span { lo, hi: lo + 1 }, SyntaxErrorKind::UnexpectedToken)
    }

    #[test]
    fn replace_if_none_to_some() {
        let errors = SyntaxErrors::new(None, vec![
            mock_syntax_error(2),
        ]);
        assert!(errors.clone().replace_if_better(Some(1), vec![]));
        for i in 1..4 {
            assert!(errors.clone().replace_if_better(Some(1), vec![
                mock_syntax_error(i),
            ]));
        }
    }

    #[test]
    fn no_replace_if_some_to_none() {
        let errors = SyntaxErrors::new(Some(1), vec![
            mock_syntax_error(2),
        ]);
        assert!(!errors.clone().replace_if_better(None, vec![]));
        for i in 1..4 {
            assert!(!errors.clone().replace_if_better(None, vec![
                mock_syntax_error(i),
            ]));
        }
    }

    #[test]
    fn replace_if_reach_farther() {
        for x in [Some(1), None] {
            let errors = SyntaxErrors::new(x, vec![
                mock_syntax_error(2),
            ]);
            assert!(errors.clone().replace_if_better(x, vec![
                mock_syntax_error(3),
            ]));
        }
    }

    #[test]
    fn no_replace_if_reach_less() {
        for x in [Some(1), None] {
            let errors = SyntaxErrors::new(x, vec![
                mock_syntax_error(2),
                mock_syntax_error(2),
            ]);
            assert!(!errors.clone().replace_if_better(x, vec![
                mock_syntax_error(1),
            ]));
            assert!(!errors.clone().replace_if_better(x, vec![
                mock_syntax_error(1),
                mock_syntax_error(3),
            ]));
            assert!(!errors.clone().replace_if_better(x, vec![
                mock_syntax_error(3),
                mock_syntax_error(1),
            ]));
        }
    }
}

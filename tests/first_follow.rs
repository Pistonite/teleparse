use teleparse::{prelude::*, GrammarError, Parser};
use teleparse::syntax::{Error, ErrorKind, FirstSet, FollowSet};

// This tests the "textbook" grammar:
// E -> T E'
// E' -> + T E' | ε
// T -> F T'
// T' -> * F T' | ε
// F -> ( E ) | id
//
// Notice that E' and T' are recursive to produces a delimited list,
// but this library already has utilities for them
// See:
// - tp::Sep<X, p> (X p X p ... X, no trail) 
// - tp::Punc<X, p> (X p X p ... X p? optional trail), 
// - tp::Vec<X> (X ... X) = X*
// - tp::Nev<X> (X ... X) = X+  a.k.a NonEmptyVec
//   - There's also tp::VecDeque and tp::NevDeque which are backed by VecDeque

#[derive_lexicon]
#[teleparse(
    ignore(r#"^\s+"#), // ignore whitespaces, separate multiple with comma
)]
pub enum TokenType {
    #[teleparse(regex(r#"^\w+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(
        OpAdd = "+",
        OpMul = "*",
    ))]
    Op,
    /// Parentheses
    #[teleparse(terminal(
        ParenOpen = "(",
        ParenClose = ")"
    ))]
    Paren,
}

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq)]
struct E{ 
    first: T,
    rest: Eprime,
}

impl E {
    fn to_string(&self, input: &str) -> String {
        format!("{}{}", self.first.to_string(input), self.rest.to_string(input))
    }
}

// Eplus has to be a separate struct because it contains Eprime.
// Eprime(tp::Option<(OpAdd, T, Box<Eprime>)>) 
// will cause a loop in Eprime -> tp::Option -> Eprime when trying
// to determine if traits are satisfied
#[derive_syntax]
#[derive(Debug, PartialEq)]
struct Eprime(tp::Option<Eplus>);
impl Eprime {
    fn to_string(&self, input: &str) -> String {
        match &*self.0 {
            Some(eplus) => format!(" {}", eplus.to_string(input)),
            None => "".to_string()
        }
    }
}

#[derive_syntax]
#[derive(Debug, PartialEq)]
struct Eplus {
    op: OpAdd,
    t: T,
    rest: Box<Eprime>,
}
impl Eplus {
    fn to_string(&self, input: &str) -> String {
        format!("{} {}{}", self.op.0.src(input), self.t.to_string(input), self.rest.to_string(input))
    }
}

#[derive_syntax]
#[derive(Debug, PartialEq)]
struct T {
    first: F,
    rest: Tprime,
}
impl T {
    fn to_string(&self, input: &str) -> String {
        format!("{}{}", self.first.to_string(input), self.rest.to_string(input))
    }
}

#[derive_syntax]
#[derive(Debug, PartialEq)]
struct Tprime(tp::Option<Tstar>);
impl Tprime {
    fn to_string(&self, input: &str) -> String {
        match &*self.0 {
            Some(tstar) => format!(" {}", tstar.to_string(input)),
            None => "".to_string()
        }
    }
}

#[derive_syntax]
#[derive(Debug, PartialEq)]
struct Tstar {
    op: OpMul,
    f: F,
    rest: Box<Tprime>,
}
impl Tstar {
    fn to_string(&self, input: &str) -> String {
        format!("{} {}{}", self.op.0.src(input), self.f.to_string(input), self.rest.to_string(input))
    }
}

#[derive_syntax]
#[derive(Debug, PartialEq)]
enum F {
    Paren((ParenOpen, Box<E>, ParenClose)),
    Ident
}

impl F {
    fn to_string(&self, input: &str) -> String {
        match self {
            F::Paren((open, e, close)) => {
                format!("{}{}{}",
                open.0.src(input), e.to_string(input), close.0.src(input))
            },
            F::Ident(id) => id.0.src(input).to_string()
        }
    
    }
}

#[test]
fn first_table() {
    // reference: https://www.geeksforgeeks.org/first-set-in-syntax-analysis/?ref=lbp

    let mut first_e_t_f = FirstSet::new();
    first_e_t_f.insert(TokenType::Ident, None);
    first_e_t_f.insert(TokenType::Paren, Some("("));

    let mut first_e_prime = FirstSet::new();
    first_e_prime.insert(TokenType::Op, Some("+"));
    first_e_prime.insert_epsilon();

    let mut first_t_prime = FirstSet::new();
    first_t_prime.insert(TokenType::Op, Some("*"));
    first_t_prime.insert_epsilon();

    let meta = E::metadata().as_ref().unwrap();
    assert_eq!(meta.first.get_pt::<E>() , &first_e_t_f);
    assert_eq!(meta.first.get_pt::<T>() , &first_e_t_f);
    assert_eq!(meta.first.get_pt::<F>() , &first_e_t_f);

    assert_eq!(meta.first.get_pt::<Eprime>() , &first_e_prime);
    assert_eq!(meta.first.get_pt::<Tprime>() , &first_t_prime);
}

#[test]
fn follow_table() {
    // reference: https://www.geeksforgeeks.org/follow-set-in-syntax-analysis/?ref=lbp

    let mut follow_e_eprime = FollowSet::new();
    follow_e_eprime.insert(TokenType::Paren, Some(")"));
    follow_e_eprime.insert_eof();

    let mut follow_t_tprime = FollowSet::new();
    follow_t_tprime.insert(TokenType::Op, Some("+"));
    follow_t_tprime.insert(TokenType::Paren, Some(")"));
    follow_t_tprime.insert_eof();

    let mut follow_f = follow_t_tprime.clone();
    follow_f.insert(TokenType::Op, Some("*"));

    let meta = E::metadata().as_ref().unwrap();
    assert_eq!(meta.follow.get_pt::<E>() , &follow_e_eprime);
    assert_eq!(meta.follow.get_pt::<Eprime>() , &follow_e_eprime);
    assert_eq!(meta.follow.get_pt::<T>() , &follow_t_tprime);
    assert_eq!(meta.follow.get_pt::<Tprime>() , &follow_t_tprime);
    assert_eq!(meta.follow.get_pt::<F>() , &follow_f);

}

#[test]
fn parse() {
    let source = "a+b*c";
    let t = E::parse(source).unwrap().unwrap();

    // 0   1 2 3 4 5
    // E--------------
    // T---E'---------
    // F T'+ T-------E'
    // a     F T'----
    //       b * F T'
    //           c

    let c = Ident(Token::new((4, 5), TokenType::Ident));
    let b = Ident(Token::new((2, 3), TokenType::Ident));
    let a = Ident(Token::new((0, 1), TokenType::Ident));
    let plus = OpAdd(Token::new((1, 2), TokenType::Op));
    let mul = OpMul(Token::new((3, 4), TokenType::Op));

    let plus_b_mul_c = Eplus {
        op: plus,
        t: T {
            first: F::Ident(b),
            rest: Tprime(Node::new(3..5, Some(
                Tstar {
                    op: mul,
                    f: F::Ident(c),
                    rest: Box::new(Tprime(Node::new(5..5, None).into())),
                }
            )).into()),
        },
        rest: Box::new(Eprime(Node::new(5..5, None).into())),
    };

    let a_plus_b_mul_c = E {
        first: T {
            first: F::Ident(a),
            rest: Tprime(Node::new(1..1, None).into())
        },
        rest: Eprime(Node::new(1..5, Some(plus_b_mul_c)).into()),
    };

    assert_eq!(t, a_plus_b_mul_c);
    assert_eq!(t.to_string(source), "a + b * c");
}

#[test]
fn parse_paren() {
    let source = "(a+b)*(c+d)";
    let t = E::parse(source).unwrap().unwrap();

    assert_eq!(t.to_string(source), "(a + b) * (c + d)");
}

#[test]
fn parse_with_error() -> Result<(), GrammarError> {
    let source = "a+(b*)+c";
    let mut parser = Parser::new(source)?;
    let t = parser.iter::<E>()?.next().unwrap();

    let mut expecting = FirstSet::new();
    expecting.insert(TokenType::Ident, None);
    expecting.insert(TokenType::Paren, Some("("));
    assert_eq!(parser.info().errors, vec![
        Error::new(5..6, ErrorKind::Expecting(expecting))
    ]);

    assert_eq!(t.to_string(source), "a + (b) + c");

    Ok(())
}

use std::{collections::VecDeque, mem};

use crate::{Error, Pattern, PatternObject, Quantifier, Result, SetPatternObject, Token};

pub(crate) struct Parser {
    tokens: VecDeque<Token>,
    curr_tok: Token,
    next_tok: Token,
    capture_id: u8,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Pattern> {
        let mut parser = Self {
            tokens: tokens.into(),
            // initialize with dummy `Eof` tokens
            curr_tok: Token::Eof,
            next_tok: Token::Eof,
            capture_id: 1,
        };
        // advance the parser twice so that curr_tok and next_tok have correct values
        parser.next();
        parser.next();
        parser.parse_pattern(Token::Eof)
    }

    fn next(&mut self) {
        // swap next_tok and curr_tok in memory so that what was curr_tok is now next_tok
        mem::swap(&mut self.next_tok, &mut self.curr_tok);
        // overwrite next_tok (which is now what curr_tok was) with the next token
        self.next_tok = self.tokens.pop_front().unwrap_or(Token::Eof);
    }

    fn check_quantifier(&mut self, child: PatternObject) -> PatternObject {
        match Quantifier::try_from(self.curr_tok) {
            Ok(quantifier) => {
                self.next();
                PatternObject::Quantifier(quantifier, Box::new(child))
            }
            Err(()) => child,
        }
    }

    /////////////////////////

    fn parse_pattern(&mut self, end: Token) -> Result<Pattern> {
        let mut objects = vec![];

        while self.curr_tok != end {
            match self.curr_tok {
                Token::Char(_) => objects.push(self.parse_string()),
                Token::LBrack => {
                    let set = self.parse_set()?;
                    objects.push(self.check_quantifier(PatternObject::Set(set.0, set.1)));
                }
                Token::LParen => objects.push(self.parse_capture()?),

                Token::Start => {
                    self.next();
                    objects.push(PatternObject::Start);
                }
                Token::End => {
                    self.next();
                    objects.push(PatternObject::End);
                }
                Token::Balanced(open, close) => {
                    self.next();
                    objects.push(PatternObject::Balanced(open, close));
                }
                Token::Frontier => {
                    self.next();
                    let set = self.parse_set()?;
                    objects.push(PatternObject::Frontier(set.0, set.1));
                }
                Token::CaptureRef(id) if id <= self.capture_id => {
                    self.next();
                    objects.push(PatternObject::CaptureRef(id));
                }
                Token::CaptureRef(id) => return Err(Error::InvalidCaptureRef(id)),

                Token::Any => {
                    self.next();
                    objects.push(self.check_quantifier(PatternObject::Any));
                }
                Token::Escaped(char) => {
                    self.next();
                    objects.push(self.check_quantifier(PatternObject::Escaped(char)));
                }
                Token::Class(class) => {
                    self.next();
                    objects.push(self.check_quantifier(PatternObject::Class(class)));
                }

                tok if tok == end => {}
                tok => return Err(Error::UnexpectedToken(tok)),
            }
        }

        Ok(objects)
    }

    fn parse_string(&mut self) -> PatternObject {
        let mut string = String::new();

        while let Token::Char(char) = self.curr_tok {
            string.push(char);
            self.next();

            if Quantifier::try_from(self.next_tok).is_ok() {
                break;
            }
        }

        self.check_quantifier(PatternObject::String(string))
    }

    fn parse_set(&mut self) -> Result<(bool, Vec<SetPatternObject>)> {
        self.next();
        let mut children = vec![];
        let inverted = self.curr_tok == Token::Inverse;
        if inverted {
            self.next();
        }

        while self.curr_tok != Token::RBrack {
            match self.curr_tok {
                Token::Class(class) => {
                    self.next();
                    children.push(SetPatternObject::Class(class));
                }
                Token::Char(start) if self.next_tok == Token::Char('-') => {
                    self.next();
                    self.next();
                    let end = match self.curr_tok {
                        Token::Char(end) => end,
                        tok => return Err(Error::OpenEndedRange(tok)),
                    };
                    self.next();

                    children.push(SetPatternObject::Range(start, end));
                }
                Token::Char(char) => {
                    self.next();
                    children.push(SetPatternObject::Char(char));
                }
                Token::Escaped(char) => {
                    self.next();
                    children.push(SetPatternObject::Escaped(char));
                }
                tok => return Err(Error::UnexpectedToken(tok)),
            }
        }
        self.next();

        Ok((inverted, children))
    }

    fn parse_capture(&mut self) -> Result<PatternObject> {
        let id = self.capture_id;
        self.capture_id += 1;

        self.next();
        let children = self.parse_pattern(Token::RParen)?;
        self.next();

        Ok(PatternObject::Capture(id, children))
    }
}

#[cfg(test)]
mod tests {
    use crate::Class;

    use super::*;

    #[test]
    fn everything() {
        let input = vec![
            Token::Start,
            Token::Char('^'),
            Token::Char('c'),
            Token::Char('h'),
            Token::Char('a'),
            Token::Char('r'),
            Token::Char('s'),
            Token::Char('q'),
            Token::OneOrMore,
            Token::Char('w'),
            Token::ZeroOrMoreLazy,
            Token::Char('e'),
            Token::ZeroOrMore,
            Token::Char('r'),
            Token::ZeroOrOne,
            Token::Any,
            Token::Escaped('.'),
            Token::LParen,
            Token::Class(Class::Letters),
            Token::Class(Class::Controls),
            Token::Class(Class::Digits),
            Token::Class(Class::Printable),
            Token::Class(Class::Lowercase),
            Token::Class(Class::Punctuations),
            Token::Class(Class::Spaces),
            Token::Class(Class::Uppercase),
            Token::Class(Class::Alphanumerics),
            Token::Class(Class::Hexadecimals),
            Token::Class(Class::ZeroByte),
            Token::Class(Class::NotLetters),
            Token::RParen,
            Token::LBrack,
            Token::Char('a'),
            Token::Char('s'),
            Token::Char('d'),
            Token::RBrack,
            Token::Frontier,
            Token::LBrack,
            Token::Inverse,
            Token::Char('n'),
            Token::Char('o'),
            Token::Char('t'),
            Token::RBrack,
            Token::CaptureRef(1),
            Token::Balanced('{', '}'),
            Token::LBrack,
            Token::Escaped(']'),
            Token::Char('a'),
            Token::Char('-'),
            Token::Char('z'),
            Token::RBrack,
            Token::Char('$'),
            Token::End,
        ];

        let output = Ok(vec![
            PatternObject::Start,
            PatternObject::String("^chars".to_owned()),
            PatternObject::Quantifier(
                Quantifier::OneOrMore,
                PatternObject::String("q".to_owned()).into(),
            ),
            PatternObject::Quantifier(
                Quantifier::ZeroOrMoreLazy,
                PatternObject::String("w".to_owned()).into(),
            ),
            PatternObject::Quantifier(
                Quantifier::ZeroOrMore,
                PatternObject::String("e".to_owned()).into(),
            ),
            PatternObject::Quantifier(
                Quantifier::ZeroOrOne,
                PatternObject::String("r".to_owned()).into(),
            ),
            PatternObject::Any,
            PatternObject::Escaped('.'),
            PatternObject::Capture(
                1,
                vec![
                    PatternObject::Class(Class::Letters),
                    PatternObject::Class(Class::Controls),
                    PatternObject::Class(Class::Digits),
                    PatternObject::Class(Class::Printable),
                    PatternObject::Class(Class::Lowercase),
                    PatternObject::Class(Class::Punctuations),
                    PatternObject::Class(Class::Spaces),
                    PatternObject::Class(Class::Uppercase),
                    PatternObject::Class(Class::Alphanumerics),
                    PatternObject::Class(Class::Hexadecimals),
                    PatternObject::Class(Class::ZeroByte),
                    PatternObject::Class(Class::NotLetters),
                ],
            ),
            PatternObject::Set(
                false,
                vec![
                    SetPatternObject::Char('a'),
                    SetPatternObject::Char('s'),
                    SetPatternObject::Char('d'),
                ],
            ),
            PatternObject::Frontier(
                true,
                vec![
                    SetPatternObject::Char('n'),
                    SetPatternObject::Char('o'),
                    SetPatternObject::Char('t'),
                ],
            ),
            PatternObject::CaptureRef(1),
            PatternObject::Balanced('{', '}'),
            PatternObject::Set(
                false,
                vec![
                    SetPatternObject::Escaped(']'),
                    SetPatternObject::Range('a', 'z'),
                ],
            ),
            PatternObject::String("$".to_owned()),
            PatternObject::End,
        ]);

        assert_eq!(Parser::parse(input), output);
    }
}

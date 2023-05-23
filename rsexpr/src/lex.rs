use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token<'src> {
    /// (
    LParen,
    /// )
    RParen,
    /// [
    LBrack,
    /// ]
    RBrack,

    String(Cow<'src, [u8]>),
    Atom(&'src [u8]),
    #[cfg(feature = "comments")]
    Comment(&'src [u8]),
}

const WHITESPACE: &[u8] = &[b' ', b'\t', b'\r', b'\n'];
const LINE_BREAKS: &[u8] = &[b'\r', b'\n'];
const NOT_IN_ATOM: &[u8] = &[b'(', b')', b'[', b']', b'"'];

pub(crate) fn lex(input: &[u8]) -> Vec<Token<'_>> {
    let mut index = 0;
    let mut tokens = vec![];

    while index < input.len() {
        match input[index] {
            byte if WHITESPACE.contains(&byte) => index += 1,
            b';' => {
                let _start_index = index;
                while index < input.len() && !LINE_BREAKS.contains(&input[index]) {
                    index += 1;
                }
                #[cfg(feature = "comments")]
                tokens.push(Token::Comment(&input[_start_index..index]));
            }
            b'(' => {
                tokens.push(Token::LParen);
                index += 1;
            }
            b')' => {
                tokens.push(Token::RParen);
                index += 1;
            }
            b'[' => {
                tokens.push(Token::LBrack);
                index += 1;
            }
            b']' => {
                tokens.push(Token::RBrack);
                index += 1;
            }
            b'"' => tokens.push(lex_string(input, &mut index)),
            _ => {
                let start_index = index;
                while index < input.len()
                    && !WHITESPACE.contains(&input[index])
                    && !NOT_IN_ATOM.contains(&input[index])
                {
                    index += 1;
                }
                tokens.push(Token::Atom(&input[start_index..index]));
            }
        }
    }

    tokens
}

fn lex_string<'src>(input: &'src [u8], index: &mut usize) -> Token<'src> {
    *index += 1; // skip opening quote
    let start_index = *index;
    let mut end_index = input.len();
    let mut requires_allocation = false;
    let mut allocated_string = vec![];

    while *index < input.len() {
        match input[*index] {
            b'"' => {
                end_index = *index;
                *index += 1;
                break;
            }
            b'\\' if *index == input.len() - 1 => {
                end_index = *index;
                *index += 1;
                break;
            }
            b'\\' => {
                if !requires_allocation {
                    allocated_string = input[start_index..*index].to_vec();
                    requires_allocation = true;
                }
                allocated_string.push(input[*index + 1]);
                *index += 2;
            }
            _ => {
                if requires_allocation {
                    allocated_string.push(input[*index]);
                }
                *index += 1;
            }
        }
    }

    match requires_allocation {
        true => Token::String(Cow::Owned(allocated_string)),
        false => Token::String(Cow::Borrowed(&input[start_index..end_index])),
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    fn string_test(input: &str, expected: Cow<'static, [u8]>) {
        let mut index = 0;
        assert_eq!(
            lex_string(input.as_bytes(), &mut index),
            Token::String(expected)
        );
    }

    #[test]
    fn strings() {
        // TODO: verify owned/borrowed
        string_test(r#""Hello, World!""#, Cow::Borrowed(b"Hello, World!"));
        string_test(r#""Hello, World!"#, Cow::Borrowed(b"Hello, World!"));
        string_test(r#""\"\\""#, Cow::Borrowed(b"\"\\"));
        string_test(r#""\"\"#, Cow::Borrowed(b"\""));
        string_test(r#""a\"#, Cow::Borrowed(b"a"));
    }

    #[test]
    fn parens() {
        assert_eq!(
            lex(b"()[]"),
            vec![Token::LParen, Token::RParen, Token::LBrack, Token::RBrack]
        );
    }

    #[test]
    fn atoms() {
        assert_eq!(
            lex(b"abc def"),
            vec![Token::Atom(b"abc"), Token::Atom(b"def")],
        );
    }

    #[test]
    #[cfg(feature = "comments")]
    fn comments() {
        assert_eq!(
            lex(b"; comment\natom"),
            vec![Token::Comment(b"; comment"), Token::Atom(b"atom")]
        );
    }

    #[test]
    #[cfg(not(feature = "comments"))]
    fn comments() {
        assert_eq!(lex(b"; comment\natom"), vec![Token::Atom(b"atom")]);
    }
}

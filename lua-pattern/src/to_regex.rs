use std::borrow::Cow;

use crate::{Class, PatternObject, Quantifier, SetPatternObject};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum ToRegexError {
    #[error("the given pattern includes a balanced pattern (eg. `%b{{}}`) which cannot be represented by regex")]
    BalancedUsed,

    #[error("the given pattern includes a frontier pattern (eg. `%f[a-z]`) which cannot be represented by regex")]
    FrontierUsed,

    #[error("the given pattern includes a capture backreference, which may not be supported by some regex engines")]
    CaptureRefUsed,
}

pub fn try_to_regex(
    pattern: &[PatternObject],
    allow_capture_refs: bool,
) -> Result<String, ToRegexError> {
    from_pattern(pattern, allow_capture_refs)
}

fn from_pattern(
    pattern: &[PatternObject],
    allow_capture_refs: bool,
) -> Result<String, ToRegexError> {
    pattern
        .iter()
        .map(|obj| from_pattern_object(obj, allow_capture_refs))
        .collect::<Result<_, _>>()
}

fn from_pattern_object(
    object: &PatternObject,
    allow_capture_refs: bool,
) -> Result<Cow<'static, str>, ToRegexError> {
    match object {
        PatternObject::Balanced(_, _) => Err(ToRegexError::BalancedUsed),
        PatternObject::Frontier(_) => Err(ToRegexError::FrontierUsed),
        PatternObject::CaptureRef(_) if !allow_capture_refs => Err(ToRegexError::CaptureRefUsed),

        PatternObject::Any => Ok(".".into()),
        PatternObject::Start => Ok("^".into()),
        PatternObject::End => Ok("$".into()),

        PatternObject::String(string) => {
            Ok(string.chars().map(from_char).collect::<String>().into())
        }
        PatternObject::Escaped(char) => Ok(from_char(*char).into()),

        PatternObject::Quantifier(quantifier, child) => Ok(format!(
            "{}{}",
            from_pattern_object(child, allow_capture_refs)?,
            from_quantifier(quantifier)
        )
        .into()),
        PatternObject::Class(class) => Ok(from_class(class).into()),
        PatternObject::CaptureRef(id) => Ok(format!("\\{id}").into()),
        PatternObject::Capture(_, pattern) => {
            Ok(format!("({})", from_pattern(pattern, allow_capture_refs)?).into())
        }
        PatternObject::Set(set) => Ok(from_set(set, false).into()),
        PatternObject::InverseSet(set) => Ok(from_set(set, true).into()),
    }
}

fn from_quantifier(quantifier: &Quantifier) -> &'static str {
    match quantifier {
        Quantifier::ZeroOrMore => "*",
        Quantifier::OneOrMore => "+",
        Quantifier::ZeroOrMoreLazy => "*?",
        Quantifier::ZeroOrOne => "?",
    }
}

fn from_class(class: &Class) -> &'static str {
    match class {
        Class::Letters => r"[a-zA-Z]",
        Class::Controls => r"[\0-\31]",
        Class::Digits => r"[0-9]",
        Class::Printable => r"[\33-\126]",
        Class::Lowercase => r"[a-z]",
        Class::Punctuations => r##"[!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~]"##,
        Class::Spaces => r"[ \t\n\v\f\r]",
        Class::Uppercase => r"[A-Z]",
        Class::Alphanumerics => r"[a-zA-Z0-9]",
        Class::Hexadecimals => r"[0-9a-fA-F]",
        Class::ZeroByte => r"\0",

        Class::NotLetters => r"[^a-zA-Z]",
        Class::NotControls => r"[^\0-\31]",
        Class::NotDigits => r"[^0-9]",
        Class::NotPrintable => r"[^\33-\126]",
        Class::NotLowercase => r"[^a-z]",
        Class::NotPunctuations => r##"[^!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~]"##,
        Class::NotSpaces => r"[^ \t\n\v\f\r]",
        Class::NotUppercase => r"[^A-Z]",
        Class::NotAlphanumerics => r"[^a-zA-Z0-9]",
        Class::NotHexadecimals => r"[^0-9a-fA-F]",
        Class::NotZeroByte => r"[^\0]",
    }
}

fn from_set(set: &[SetPatternObject], inverse: bool) -> String {
    format!(
        "[{}{}]",
        if inverse { "^" } else { "" },
        set.iter().map(from_set_pattern_object).collect::<String>()
    )
}

fn from_set_pattern_object(object: &SetPatternObject) -> Cow<'static, str> {
    match object {
        SetPatternObject::Char(char) | SetPatternObject::Escaped(char) => from_char(*char).into(),
        SetPatternObject::Range(start, end) => {
            format!("{}-{}", from_char(*start), from_char(*end)).into()
        }
        SetPatternObject::Class(class) => from_class(class).into(),
    }
}

fn from_char(char: char) -> String {
    const SPECIAL_CHARS: &str = "\\.()[]{}|*+?^$/";

    match SPECIAL_CHARS.contains(char) {
        true => format!("\\{char}"),
        false => char.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everything() {
        let input = vec![
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
            PatternObject::Set(vec![
                SetPatternObject::Char('a'),
                SetPatternObject::Char('s'),
                SetPatternObject::Char('d'),
            ]),
            PatternObject::InverseSet(vec![
                SetPatternObject::Char('n'),
                SetPatternObject::Char('o'),
                SetPatternObject::Char('t'),
            ]),
            PatternObject::CaptureRef(1),
            PatternObject::Set(vec![
                SetPatternObject::Escaped(']'),
                SetPatternObject::Range('a', 'z'),
            ]),
            PatternObject::String("$".to_owned()),
            PatternObject::End,
        ];

        assert_eq!(try_to_regex(&input, true), Ok(r##"^\^charsq+w*?e*r?.\.([a-zA-Z][\0-\31][0-9][\33-\126][a-z][!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~][ \t\n\v\f\r][A-Z][a-zA-Z0-9][0-9a-fA-F]\0[^a-zA-Z])[asd][^not]\1[\]a-z]\$$"##.to_owned()));
    }
}

use std::borrow::Cow;

use crate::{Class, PatternObject, Quantifier, SetPatternObject};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
/// The error type for errors that can occur during conversion to regular expressions.
/// See [`try_to_regex`] for more information.
pub enum ToRegexError {
    /// The input pattern includes a balanced pattern (eg. `%b{}`) which cannot be represented by
    /// regular expressions.
    #[error("the input pattern includes a balanced pattern (eg. `%b{{}}`) which cannot be represented by regex")]
    BalancedUsed,

    /// The input pattern includes a capture backreference (eg. `%1`), but `allow_capture_refs` was
    /// set to `false`.
    #[error("the input pattern includes a capture backreference, which may not be supported by some regex engines")]
    CaptureRefUsed,

    /// The input pattern includes a frontier pattern (eg. `%f[a-z]`), but `allow_lookaround` was
    /// set to `false`.
    #[error("the input pattern includes a frontier pattern (eg. `%f[a-z]`) which cannot be represented by regex")]
    FrontierUsed,
}

/// Try to convert a parsed Lua pattern into a regular expression string.
///
/// The `allow_capture_refs` parameter specifies whether to allow backreferences to capture groups.
/// Set this to `false` when using the output with the
/// [`regex` crate](https://crates.io/crates/regex), or to `true` when using the
/// [`fancy-regex` crate](https://crates.io/crates/fancy-regex).
///
/// # Returns
/// The function returns a [`String`] if the conversion was successful, and a [`ToRegexError`]
/// otherwise.
///
/// # Errors
/// Converting a Lua pattern to a RegEx can fail in up to three ways.
///
/// 1. Lua patterns support balanced bracket matching using the `%b` operator. This is not
///    supported by RegEx. Thus, an error will be returned if the input pattern makes use of this
///    feature.
/// 2. Lua patterns support references to previous capture groups. Some RegEx engines also support
///    this feature, but not all. For this reason, uses of such backreferences will result in an
///    error, if `allow_capture_refs` is set to `false`.
/// 3. Lua patterns support so-called frontier patterns. Their behaviour can be emulated using
///    lookaround, but only some RegEx engines support that. Therefore, if the input includes a
///    frontier pattern and `allow_lookaround` is set to `false`, an error will be returned.
///
/// Also see [`ToRegexError`] for further information.
pub fn try_to_regex(
    pattern: &[PatternObject],
    allow_capture_refs: bool,
    allow_lookaround: bool,
) -> Result<String, ToRegexError> {
    from_pattern(pattern, allow_capture_refs, allow_lookaround)
}

fn from_pattern(
    pattern: &[PatternObject],
    allow_capture_refs: bool,
    allow_lookaround: bool,
) -> Result<String, ToRegexError> {
    pattern
        .iter()
        .map(|obj| from_pattern_object(obj, allow_capture_refs, allow_lookaround))
        .collect::<Result<_, _>>()
}

fn from_pattern_object(
    object: &PatternObject,
    allow_capture_refs: bool,
    allow_lookaround: bool,
) -> Result<Cow<'static, str>, ToRegexError> {
    match object {
        PatternObject::Balanced(_, _) => Err(ToRegexError::BalancedUsed),
        PatternObject::Frontier(_, _) if !allow_lookaround => Err(ToRegexError::FrontierUsed),
        PatternObject::CaptureRef(_) if !allow_capture_refs => Err(ToRegexError::CaptureRefUsed),

        PatternObject::Any => Ok("[\\s\\S]".into()),
        PatternObject::Start => Ok("^".into()),
        PatternObject::End => Ok("$".into()),

        PatternObject::String(string) => {
            Ok(string.chars().map(from_char).collect::<String>().into())
        }
        PatternObject::Escaped(char) => Ok(from_char(*char).into()),

        PatternObject::Quantifier(quantifier, child) => Ok(format!(
            "{}{}",
            from_pattern_object(child, allow_capture_refs, allow_lookaround)?,
            from_quantifier(quantifier)
        )
        .into()),
        PatternObject::Class(class) => Ok(from_class(class).into()),
        PatternObject::CaptureRef(id) => Ok(format!("\\{id}").into()),
        PatternObject::Capture(_, pattern) => Ok(format!(
            "({})",
            from_pattern(pattern, allow_capture_refs, allow_lookaround)?
        )
        .into()),
        PatternObject::Set(inverted, set) => Ok(from_set(set, *inverted).into()),
        PatternObject::Frontier(inverted, set) => Ok(format!(
            "(?<{}{})(?{}{})",
            if *inverted { "=" } else { "!" },
            from_set(set, false),
            if *inverted { "!" } else { "=" },
            from_set(set, false),
        )
        .into()),
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
            PatternObject::Set(
                false,
                vec![
                    SetPatternObject::Char('a'),
                    SetPatternObject::Char('s'),
                    SetPatternObject::Char('d'),
                ],
            ),
            PatternObject::Set(
                true,
                vec![
                    SetPatternObject::Char('n'),
                    SetPatternObject::Char('o'),
                    SetPatternObject::Char('t'),
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
            PatternObject::Set(
                false,
                vec![
                    SetPatternObject::Escaped(']'),
                    SetPatternObject::Range('a', 'z'),
                ],
            ),
            PatternObject::String("$".to_owned()),
            PatternObject::End,
        ];

        assert_eq!(try_to_regex(&input, true, true), Ok(r##"^\^charsq+w*?e*r?[\s\S]\.([a-zA-Z][\0-\31][0-9][\33-\126][a-z][!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~][ \t\n\v\f\r][A-Z][a-zA-Z0-9][0-9a-fA-F]\0[^a-zA-Z])[asd][^not](?<=[not])(?![not])\1[\]a-z]\$$"##.to_owned()));
    }
}

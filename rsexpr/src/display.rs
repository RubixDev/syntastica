use std::fmt::{self, Display, Formatter};

use crate::{OwnedSexpr, OwnedSexprs, Sexpr, Sexprs};

impl Display for OwnedSexpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Sexpr::from(self).fmt(f)
    }
}

impl Display for OwnedSexprs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Sexprs::from(self).fmt(f)
    }
}

impl Display for Sexprs<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let last = self.len().saturating_sub(1);
        let mut iter = self.iter().enumerate().peekable();
        while let Some((index, sexpr)) = iter.next() {
            sexpr.fmt(f)?;

            if f.alternate() && index != last {
                match sexpr {
                    #[cfg(feature = "comments")]
                    Sexpr::Comment(_) => writeln!(f)?,
                    _ if matches!(iter.peek(), Some((_, Sexpr::Atom(_)))) => write!(f, " ")?,
                    _ => write!(f, "\n\n")?,
                }
            }
        }

        if f.alternate() {
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Sexpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pretty = f.alternate();
        let mut indent_level = f.width().unwrap_or(0);
        let indent_width = f.precision().unwrap_or(2);
        let indent = |level| " ".repeat(indent_width * level);

        if !pretty {
            match self {
                Sexpr::List(children) | Sexpr::Group(children) => {
                    let (open_paren, close_paren) = match self {
                        Sexpr::List(_) => ('(', ')'),
                        _ => ('[', ']'),
                    };

                    let children = children
                        .iter()
                        .map(|child| format!("{child}"))
                        .collect::<String>();

                    write!(
                        f,
                        "{open_paren}{}{close_paren}",
                        // trim possible whitespace at end
                        children.trim_end(),
                    )?;
                }
                Sexpr::String(string) => write!(
                    f,
                    "\"{}\" ",
                    String::from_utf8_lossy(string)
                        .replace('\\', r"\\")
                        .replace('"', "\\\"")
                )?,
                Sexpr::Atom(atom) => write!(f, "{} ", String::from_utf8_lossy(atom))?,
                #[cfg(feature = "comments")]
                Sexpr::Comment(_) => {}
            }
        } else {
            match self {
                Sexpr::List(children) | Sexpr::Group(children) => {
                    let (open_paren, close_paren) = match self {
                        Sexpr::List(_) => ('(', ')'),
                        _ => ('[', ']'),
                    };

                    // keep lists in one line if they start with a predicate, unless they contain a
                    // comment or have more than 7 children
                    if let (Some(Self::Atom([b'#', ..])), Sexpr::List(_)) = (children.first(), self)
                    {
                        #[cfg(feature = "comments")]
                        let has_comment_child = children
                            .iter()
                            .any(|child| matches!(child, Sexpr::Comment(_)));
                        #[cfg(not(feature = "comments"))]
                        let has_comment_child = false;

                        if !has_comment_child && children.len() <= 7 {
                            // call doesn't cause infinite recursion,
                            // because `f.alternate()` is different
                            #[allow(clippy::recursive_format_impl)]
                            return write!(f, "{self}");
                        }
                    }

                    write!(f, "{open_paren}")?;
                    match children.len() {
                        0 => {}
                        1 => match &children[0] {
                            child @ (Sexpr::List(list) | Sexpr::Group(list)) if list.is_empty() => {
                                write!(f, "{child:#}")?
                            }
                            child @ (Sexpr::List(_) | Sexpr::Group(_)) => {
                                indent_level += 1;
                                write!(f, "\n{}", indent(indent_level))?;
                                write!(f, "{child:#indent_level$.indent_width$}")?;
                                indent_level -= 1;
                                write!(f, "\n{}", indent(indent_level))?;
                            }
                            child @ (Sexpr::String(_) | Sexpr::Atom(_)) => write!(f, "{child:#}")?,
                            #[cfg(feature = "comments")]
                            child @ Sexpr::Comment(_) => {
                                indent_level += 1;
                                write!(f, "\n{}", indent(indent_level))?;
                                write!(f, "{child:#indent_level$.indent_width$}")?;
                                indent_level -= 1;
                                write!(f, "\n{}", indent(indent_level))?;
                            }
                        },
                        _ => {
                            indent_level += 1;
                            let newline = format!("\n{}", indent(indent_level));
                            let mut iter = children.windows(2).peekable();

                            // if the first child is a string or atom, keep it on the same line
                            let mut second_child = None;
                            if let (
                                Some([Sexpr::String(_) | Sexpr::Atom(_), second]),
                                Sexpr::List(_),
                            ) = (iter.peek(), self)
                            {
                                second_child = Some(second);
                                let child = &iter.next().unwrap()[0];
                                write!(f, "{child:#}")?;
                            }

                            write!(f, "{newline}")?;

                            // write the first child
                            if let (Some([child, _]), _) | (_, Some(child)) =
                                (iter.peek(), second_child)
                            {
                                write!(f, "{child:#indent_level$.indent_width$}")?;
                            }

                            // write all other children
                            for item in iter {
                                let prev_child = &item[0];
                                let child = &item[1];
                                match (prev_child, child) {
                                    // if the current child is a quantifier, add it in the same
                                    // line without a leading whitespace
                                    (_, Sexpr::Atom([char @ (b'?' | b'*' | b'+')])) => {
                                        write!(f, "{}", *char as char)?;
                                    }
                                    // if the previous child was an atom ending with `:` or the
                                    // current child is and atom starting with `@`, stay on the
                                    // same line
                                    (
                                        Sexpr::Atom([.., b':']),
                                        child @ (Sexpr::List(_) | Self::Group(_)),
                                    )
                                    | (_, child @ Sexpr::Atom([b'@', ..])) => {
                                        write!(f, " {child:#indent_level$.indent_width$}")?;
                                    }
                                    // else go to the next line
                                    (_, child) => {
                                        write!(f, "{newline}{child:#indent_level$.indent_width$}")?;
                                    }
                                }
                            }

                            indent_level -= 1;
                            write!(f, "\n{}", indent(indent_level))?;
                        }
                    }
                    write!(f, "{close_paren}")?;
                }
                Sexpr::String(string) => write!(
                    f,
                    "\"{}\"",
                    String::from_utf8_lossy(string)
                        .replace('\\', r"\\")
                        .replace('"', "\\\"")
                )?,
                Sexpr::Atom(atom) => write!(f, "{}", String::from_utf8_lossy(atom))?,
                #[cfg(feature = "comments")]
                Sexpr::Comment(comment) => write!(f, "{}", String::from_utf8_lossy(comment))?,
            }
        }

        Ok(())
    }
}

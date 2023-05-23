use std::fmt::{self, Display, Formatter};

use crate::{OwnedSexpr, Sexpr};

const INDENT: &str = "  ";

impl Display for Sexpr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pretty = f.alternate();
        let mut indent_level = f.width().unwrap_or(0);

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
            }
        } else {
            match self {
                Sexpr::List(children) | Sexpr::Group(children) => {
                    let (open_paren, close_paren) = match self {
                        Sexpr::List(_) => ('(', ')'),
                        _ => ('[', ']'),
                    };

                    // keep lists in one line if they start with a predicate
                    if let (Some(Self::Atom([b'#', ..])), Sexpr::List(_)) = (children.first(), self)
                    {
                        // call doesn't cause infinite recursion,
                        // because `f.alternate()` is different
                        #[allow(clippy::recursive_format_impl)]
                        return write!(f, "{self}");
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
                                write!(f, "\n{}", INDENT.repeat(indent_level))?;
                                write!(f, "{child:#indent_level$}")?;
                                indent_level -= 1;
                                write!(f, "\n{}", INDENT.repeat(indent_level))?;
                            }
                            child @ (Sexpr::String(_) | Sexpr::Atom(_)) => write!(f, "{child:#}")?,
                        },
                        _ => {
                            indent_level += 1;
                            let newline = format!("\n{}", INDENT.repeat(indent_level));
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
                                write!(f, "{child:#indent_level$}")?;
                            }

                            // write all other children
                            for item in iter {
                                let prev_child = &item[0];
                                let child = &item[1];
                                match (prev_child, child) {
                                    (
                                        Sexpr::Atom([.., b':']),
                                        child @ (Sexpr::List(_) | Self::Group(_)),
                                    )
                                    | (_, child @ Sexpr::Atom([b'@', ..])) => {
                                        write!(f, " {child:#indent_level$}")?;
                                    }
                                    (_, child) => {
                                        write!(f, "{newline}{child:#indent_level$}")?;
                                    }
                                }
                            }

                            indent_level -= 1;
                            write!(f, "\n{}", INDENT.repeat(indent_level))?;
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
            }
        }

        Ok(())
    }
}

impl Display for OwnedSexpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Sexpr::from(self).fmt(f)
    }
}

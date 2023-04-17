#[cfg(not(feature = "some"))]
compile_error!("current feature set includes no parsers");

macro_rules! langs {
    ($($feat:literal, $name:ident, $src:path);* $(;)?) => {
        $(
            #[cfg(feature = $feat)]
            pub fn $name() -> tree_sitter::Language {
                $src()
            }
        )*
    };
}

#[cfg(feature = "all")]
macro_rules! git_langs {
    ($($name:ident, $src:ident);* $(;)?) => {
        extern "C" {
            $(fn $src() -> tree_sitter::Language;)*
        }
        $(
            pub fn $name() -> tree_sitter::Language {
                unsafe { $src() }
            }
        )*
    };
}

langs! {
    "some", rust, tree_sitter_rust::language;
    "some", python, tree_sitter_python::language;
    "most", asm, tree_sitter_asm::language;
}

#[cfg(feature = "all")]
git_langs! {
    regex, tree_sitter_regex;
}

var sourcesIndex = JSON.parse('{\
"lua_pattern":["",[],["error.rs","lexer.rs","lib.rs","parser.rs","to_regex.rs"]],\
"rsexpr":["",[],["display.rs","error.rs","lex.rs","lib.rs","parser.rs"]],\
"syntastica":["",[],["lib.rs","processor.rs","renderer.rs"]],\
"syntastica_core":["",[],["error.rs","lib.rs","provider.rs","style.rs","theme.rs"]],\
"syntastica_highlight":["",[],["lib.rs"]],\
"syntastica_macros":["",[],["lib.rs","schema.rs"]],\
"syntastica_parsers":["",[],["lib.rs"]],\
"syntastica_parsers_git":["",[],["lib.rs"]],\
"syntastica_parsers_gitdep":["",[],["lib.rs"]],\
"syntastica_queries":["",[],["lib.rs"]],\
"syntastica_themes":["",[],["gruvbox.rs","lib.rs","one.rs"]],\
"tree_sitter_wasm_build_tool":["",[],["lib.rs"]],\
"xtask":["",[["codegen",[],["parser_lists.rs","parsers_dep.rs","parsers_gitdep.rs","queries.rs","theme_gruvbox.rs","theme_one.rs"]]],["codegen.rs","main.rs","set_version.rs"]]\
}');
createSourceSidebar();

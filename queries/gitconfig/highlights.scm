;; Forked from https://raw.githubusercontent.com/the-mikedavis/tree-sitter-git-config/0fbc9f99d5a28865f9de8427fb0672d66f9d83a5/queries/highlights.scm
(section_name) @tag

(
  (section_name) @function.builtin
  (#eq? @function.builtin "include")
)

(
  (section_header
    (section_name) @function.builtin
    (subsection_name)
  )
  (#eq? @function.builtin "includeIf")
)

(variable
  (name) @property
)

[
  (true)
  (false)
] @constant.builtin

(integer) @number

[
  (string)
  (subsection_name)
] @string

(
  (string) @string.special.path
  (#match? @string.special.path "^(~|./|/)")
)

[
  "["
  "]"
  "\""
] @punctuation.bracket

"=" @punctuation.delimiter

(comment) @comment

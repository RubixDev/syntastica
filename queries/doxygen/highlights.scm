;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-doxygen/ccd998f378c3f9345ea4eeb223f56d7b84d16687/queries/highlights.scm
(
  (tag_name) @keyword
  (#set! "priority" 105)
)

[
  "@code"
  "@endcode"
] @keyword

(identifier) @variable

(
  (tag
    (tag_name) @_param
    (identifier) @parameter
  )
  (#any-of? @_param "@param" "\\param")
)

(function
  (identifier) @function
)

(function_link) @function

(emphasis) @text.emphasis

[
  "\\a"
  "\\c"
] @tag

(code_block_language) @label

[
  "in"
  "out"
  "inout"
] @storageclass

"~" @operator

[
  "<a"
  ">"
  "</a>"
] @tag

[
  "."
  ","
  "::"
  (code_block_start)
  (code_block_end)
] @punctuation.delimiter

[
  "("
  ")"
  "{"
  "}"
  "["
  "]"
] @punctuation.bracket

(code_block_content) @none

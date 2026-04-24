;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ziggy_schema/highlights.scm
;; Licensed under the Apache License 2.0
(struct_field
  key: (_) @keyword
)

(tag_name) @function

[
  "unknown"
  "any"
  "struct"
  "root"
  "enum"
  "null"
] @keyword

(string) @string

(number) @number

[
  "true"
  "false"
] @boolean

(identifier) @type

"?" @type

[
  "bool"
  "bytes"
  "int"
  "float"
] @constant.builtin

(doc_comment) @comment.documentation

[
  ","
  ":"
  "|"
] @punctuation.delimiter

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

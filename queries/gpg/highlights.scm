;; Forked from https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-gpg-config/4024eb268c59204280f8ac71ef146b8ff5e737f6/queries/highlights.scm
(option
  .
  _ @keyword
)

(option
  ("no-"
    @variable.parameter
  )?
  (name) @variable.parameter
)

(string
  (content) @string
)

[
  (value)
  "clear"
] @string.special

(url) @markup.link.url

(key) @constant

[
  (number)
  (expire_time)
  (iso_time)
] @number

(format) @embedded

"sensitive:" @attribute

(filter_name) @variable.parameter

(filter_scope) @module

(filter_property) @property

(filter_value) @string

[
  (filter_op0)
  (filter_op1)
  (filter_lc)
  "="
] @operator

"!" @punctuation.special

[
  "\""
  "'"
  ","
] @punctuation.delimiter

(comment) @comment

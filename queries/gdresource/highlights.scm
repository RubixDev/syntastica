;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/godot_resource/highlights.scm
;; Licensed under the Apache License 2.0
(identifier) @variable

(section
  (identifier) @tag
)

(section
  [
    "["
    "]"
  ] @tag.delimiter
)

(attribute
  (identifier) @tag.attribute
)

(property
  (path) @property
)

(constructor
  (identifier) @constructor
)

(string) @string

(integer) @number

(float) @number.float

[
  (true)
  (false)
] @boolean

(null) @constant.builtin

(array
  [
    "["
    "]"
  ] @punctuation.bracket
)

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

"=" @operator

(comment) @comment @spell

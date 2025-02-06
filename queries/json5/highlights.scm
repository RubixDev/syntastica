;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/json5/highlights.scm
;; Licensed under the Apache License 2.0
[
  (true)
  (false)
] @boolean

(null) @constant

(string) @string

(number) @number

(comment) @comment

(member
  name: (_) @keyword
)

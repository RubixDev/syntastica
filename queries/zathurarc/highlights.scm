;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/zathurarc/highlights.scm
;; Licensed under the Apache License 2.0
; General
(path) @string.special.path

(option) @variable.builtin

(command) @keyword

(include_directive
  (command) @keyword.import
)

(mode_name) @type.builtin

(key) @constant

(function) @function.call

(argument) @variable.parameter

; Comments
(comment) @comment @spell

; Literals
(int) @number

(float) @number.float

(string) @string

(bool) @boolean

[
  "<"
  ">"
  "["
  "]"
] @punctuation.bracket

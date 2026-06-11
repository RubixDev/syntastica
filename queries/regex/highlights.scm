;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/regex/highlights.scm
;; Licensed under the Apache License 2.0
[
  "("
  ")"
  "(?"
  "(?:"
  "(?<"
  "<"
  ">"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(group_name) @property

; These are escaped special characters that lost their special meaning
; -> no special highlighting
(identity_escape) @string.regexp

(class_character) @constant

(decimal_digits) @number

[
  (control_letter_escape)
  (character_class_escape)
  (control_escape)
  (boundary_assertion)
  (non_boundary_assertion)
] @string.escape

[
  "*"
  "+"
  "?"
  "|"
  "="
  "!"
  "-"
  "\\k"
  (lazy)
] @operator

[
  (start_assertion)
  (end_assertion)
  ","
] @punctuation.delimiter

(any_character) @variable.builtin

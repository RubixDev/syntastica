;; Forked from https://github.com/tree-sitter/tree-sitter-regex/blob/master/queries/highlights.scm
;; Licensed under the MIT License
[
  "("
  ")"
  "(?"
  "(?:"
  "(?<"
  ">"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(group_name) @property

[
  (identity_escape)
  (control_letter_escape)
  (character_class_escape)
  (control_escape)
  (boundary_assertion)
  (non_boundary_assertion)
] @string.escape

[
  (any_character)
  (start_assertion)
  (end_assertion)
] @punctuation.special

[
  "*"
  "+"
  "?"
  (lazy)
  "|"
  "="
  "!"
] @operator

(count_quantifier
  [
    (decimal_digits) @number
    "," @punctuation.delimiter
  ]
)

(character_class
  [
    "^" @operator
    (class_range
      "-" @operator
    )
  ]
)

(class_character) @constant.character

(pattern_character) @string.regex

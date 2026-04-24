;; Forked from https://raw.githubusercontent.com/tree-sitter/tree-sitter-regex/b2ac15e27fce703d2f37a79ccd94a5c0cbe9720b/queries/highlights.scm
[
  "("
  ")"
  "(?"
  "(?:"
  "(?<"
  "(?P<"
  "(?P="
  ">"
  "["
  "]"
  "{"
  "}"
  "[:"
  ":]"
] @punctuation.bracket

(group_name) @property

[
  (identity_escape)
  (control_letter_escape)
  (character_class_escape)
  (control_escape)
  (start_assertion)
  (end_assertion)
  (boundary_assertion)
  (non_boundary_assertion)
] @escape

[
  "*"
  "+"
  "?"
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

(inline_flags_group
  "-"? @operator
  ":"? @punctuation.delimiter
)

(flags) @character.special

(character_class
  [
    "^" @operator
    (class_range
      "-" @operator
    )
  ]
)

[
  (class_character)
  (posix_class_name)
] @constant.character

(pattern_character) @string

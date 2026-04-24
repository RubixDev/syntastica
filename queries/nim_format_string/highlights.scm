;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/nim_format_string/highlights.scm
;; Licensed under the Apache License 2.0
(matching_curlies
  opening_curly: (opening_curly) @punctuation.special
  equals: (equals)? @punctuation.special
  closing_curly: (closing_curly) @punctuation.special
)

(format_specifiers
  colon: (colon) @punctuation.delimiter
  fill_align: (fill_align)? @keyword.conditional.ternary
  sign: (sign)? @operator
  hash: (hash)? @punctuation.special
  zero: (zero)? @variable.member
  min_width: (min_width)? @number
  precision: (precision)? @number
  type: (type)? @type
)

(matching_curlies
  nim_expression: (nim_expression
    escaped_curly: (escaped_curly)+ @string.escape
  ) @none
)

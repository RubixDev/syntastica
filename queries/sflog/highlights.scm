;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/sflog/highlights.scm
;; Licensed under the Apache License 2.0
; highlights.scm
[
  "|"
  "|["
  "]"
  "("
  ")"
  "|("
  ")|"
] @punctuation.bracket

[
  ","
  ";"
  ":"
] @punctuation.delimiter

"EXTERNAL" @keyword

"out of" @property

(number) @number

(identifier) @variable

(version) @string.special

(anonymous_block) @string

(limit) @property

(time) @function

(limit
  (identifier) @string
)

(event_detail
  (event_detail_value) @string
)

(log_level_setting
  (component) @type
)

(log_level_setting
  (log_level) @constant
)

(log_entry
  (event_identifier
    (identifier) @type
  )
)

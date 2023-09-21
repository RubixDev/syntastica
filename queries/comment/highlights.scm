;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/comment/highlights.scm
;; Licensed under the Apache License 2.0
(
  (tag
    (name) @text.todo
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of? @text.todo "TODO" "WIP")
)

("text"
  @text.todo
  (#any-of? @text.todo "TODO" "WIP")
)

(
  (tag
    (name) @text.note
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of?
    @text.note
    "NOTE"
    "XXX"
    "INFO"
    "DOCS"
    "PERF"
    "TEST"
  )
)

("text"
  @text.note
  (#any-of?
    @text.note
    "NOTE"
    "XXX"
    "INFO"
    "DOCS"
    "PERF"
    "TEST"
  )
)

(
  (tag
    (name) @text.warning
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of? @text.warning "HACK" "WARNING" "WARN" "FIX")
)

("text"
  @text.warning
  (#any-of? @text.warning "HACK" "WARNING" "WARN" "FIX")
)

(
  (tag
    (name) @text.danger
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of? @text.danger "FIXME" "BUG" "ERROR")
)

("text"
  @text.danger
  (#any-of? @text.danger "FIXME" "BUG" "ERROR")
)

; Issue number (#123)
("text"
  @number
  (#lua-match? @number "^#[0-9]+$")
)

(
  (uri) @text.uri
)

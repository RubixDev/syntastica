;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/comment/highlights.scm
;; Licensed under the Apache License 2.0
(
  (tag
    (name) @comment.todo
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of? @comment.todo "TODO" "WIP")
)

("text"
  @comment.todo
  (#any-of? @comment.todo "TODO" "WIP")
)

(
  (tag
    (name) @comment.note
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of?
    @comment.note
    "NOTE"
    "XXX"
    "INFO"
    "DOCS"
    "PERF"
    "TEST"
  )
)

("text"
  @comment.note
  (#any-of?
    @comment.note
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
    (name) @comment.warning
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of? @comment.warning "HACK" "WARNING" "WARN" "FIX")
)

("text"
  @comment.warning
  (#any-of? @comment.warning "HACK" "WARNING" "WARN" "FIX")
)

(
  (tag
    (name) @comment.error
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )?
    ":" @punctuation.delimiter
  )
  (#any-of? @comment.error "FIXME" "BUG" "ERROR")
)

("text"
  @comment.error
  (#any-of? @comment.error "FIXME" "BUG" "ERROR")
)

; Issue number (#123)
("text"
  @number
  (#lua-match? @number "^#[0-9]+$")
)

(uri) @string.special.url

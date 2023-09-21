(uri) @text.uri

("text"
  @number
  (#match? @number "^#[0-9]+$")
)

("text"
  @text.danger
  (#match? @text.danger "^(FIXME|BUG|ERROR)$")
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
  (#match? @text.danger "^(FIXME|BUG|ERROR)$")
)

("text"
  @text.warning
  (#match? @text.warning "^(HACK|WARNING|WARN|FIX)$")
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
  (#match? @text.warning "^(HACK|WARNING|WARN|FIX)$")
)

("text"
  @text.note
  (#match? @text.note "^(NOTE|XXX|INFO|DOCS|PERF|TEST)$")
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
  (#match? @text.note "^(NOTE|XXX|INFO|DOCS|PERF|TEST)$")
)

("text"
  @text.todo
  (#match? @text.todo "^(TODO|WIP)$")
)

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
  (#match? @text.todo "^(TODO|WIP)$")
)

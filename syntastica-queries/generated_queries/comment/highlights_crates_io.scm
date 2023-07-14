(uri) @text.uri @nospell

("text"
  @number
  (#match? @number "^#[0-9]+$")
)

("text"
  @text.danger @nospell
  (#match? @text.danger "^(FIXME|BUG|ERROR)$")
)

(
  (tag
    (name) @text.danger @nospell
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )
    ?
    ":" @punctuation.delimiter
  )
  (#match? @text.danger "^(FIXME|BUG|ERROR)$")
)

("text"
  @text.warning @nospell
  (#match? @text.warning "^(HACK|WARNING|WARN|FIX)$")
)

(
  (tag
    (name) @text.warning @nospell
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )
    ?
    ":" @punctuation.delimiter
  )
  (#match? @text.warning "^(HACK|WARNING|WARN|FIX)$")
)

("text"
  @text.note @nospell
  (#match? @text.note "^(NOTE|XXX|INFO)$")
)

(
  (tag
    (name) @text.note @nospell
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )
    ?
    ":" @punctuation.delimiter
  )
  (#match? @text.note "^(NOTE|XXX|INFO)$")
)

("text"
  @text.todo @nospell
  (#match? @text.todo "^(TODO|WIP)$")
)

(
  (tag
    (name) @text.todo @nospell
    ("("
      @punctuation.bracket
      (user) @constant
      ")" @punctuation.bracket
    )
    ?
    ":" @punctuation.delimiter
  )
  (#match? @text.todo "^(TODO|WIP)$")
)

(_) @spell

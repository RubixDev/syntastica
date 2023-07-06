(quasiquote
  (
    (quoter) @injection.language
  )
  (
    (quasiquote_body) @injection.content
  )
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(quasiquote
  (quoter) @_name
  (#match? @_name "^(cassius|lucius)$")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "css")
  )
)

(quasiquote
  (quoter) @_name
  (#match? @_name "^(shamlet|xshamlet|hamlet|xhamlet|ihamlet)$")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "html")
  )
)

(quasiquote
  (quoter) @_name
  (#match? @_name "^(js|julius)$")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "javascript")
  )
)

(quasiquote
  (quoter) @_name
  (#match? @_name "^(tsc|tscJSX)$")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "typescript")
  )
)

(quasiquote
  (quoter) @_name
  (#eq? @_name "hsx")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "html")
  )
)

(quasiquote
  (quoter) @_name
  (#eq? @_name "aesonQQ")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "json")
  )
)

(quasiquote
  (quoter) @_name
  (#eq? @_name "sql")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "sql")
  )
)

(quasiquote
  (quoter) @_name
  (#match? @_name "^(persistUpperCase|persistLowerCase|persistWith)$")
  (
    (quasiquote_body) @injection.content
    (#set! injection.language "haskell_persistent")
  )
)

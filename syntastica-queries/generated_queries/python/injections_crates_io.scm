(
  (call
    function: (attribute
      object: (identifier) @_re
    )
    arguments: (argument_list
      (string) @injection.content
      (#set! injection.language "regex")
    )
  )
  (#eq? @_re "re")
  (#match? @injection.content "^r[\\s\\S]*")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

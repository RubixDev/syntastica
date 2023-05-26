(
  (comment) @injection.content
  (#set! injection.language "comment")
)

; crates.io skip
(
  (call
    function: (attribute
      object: (identifier) @_re
    )
    arguments: (argument_list
      (string
        (string_content) @injection.content
      ) @_string
    )
  )
  (#eq? @_re "re")
  (#match? @_string "^r.*")
  (#set! injection.language "regex")
)

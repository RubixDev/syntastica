;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/python/injections.scm
;; Licensed under the Apache License 2.0
(call
  function: (attribute
    object: (identifier) @_re
  )
  arguments: (argument_list
    .
    (string
      (string_content) @injection.content
    )
  )
  (#eq? @_re "re")
  (#set! injection.language "regex")
)

(
  (binary_operator
    left: (string
      (string_content) @injection.content
    )
    operator:
    "%"
  )
  (#set! injection.language "printf")
)

(
  (comment) @injection.content
  (#set! injection.language "comment")
)

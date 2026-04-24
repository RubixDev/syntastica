;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/wing/injections.scm
;; Licensed under the Apache License 2.0
(
  (comment) @injection.content
  (#set! injection.language "comment")
)

(call
  (reference
    (nested_identifier
      object: (reference) @_ref
      property: (member_identifier) @_ident
    )
  )
  (argument_list
    (positional_argument
      (string) @injection.content
    )
  )
  (#eq? @_ref "regex")
  (#eq? @_ident "compile")
  (#offset! @injection.content 0 1 0 -1)
  (#set! injection.language "regex")
)

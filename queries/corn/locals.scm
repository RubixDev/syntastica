;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/corn/locals.scm
;; Licensed under the Apache License 2.0
; scopes
[
  (object)
  (array)
] @local.scope

; definitions
(assign_block
  (assignment
    (input) @local.definition.constant
  )
)

(value
  (input) @local.reference
)

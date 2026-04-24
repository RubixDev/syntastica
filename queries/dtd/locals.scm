;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/dtd/locals.scm
;; Licensed under the Apache License 2.0
(elementdecl
  (Name) @local.definition.type
)

(elementdecl
  (contentspec
    (children
      (Name) @local.reference
    )
  )
)

(AttlistDecl
  .
  (Name) @local.reference
)

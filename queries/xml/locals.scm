;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/xml/locals.scm
;; Licensed under the Apache License 2.0
; tags
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

(STag
  (Name) @local.reference
)

(ETag
  (Name) @local.reference
)

(EmptyElemTag
  (Name) @local.reference
)

; attributes
(AttDef
  (Name) @local.definition.field
)

(Attribute
  (Name) @local.reference
)

; entities
(GEDecl
  (Name) @local.definition.macro
)

(EntityRef
  (Name) @local.reference
)

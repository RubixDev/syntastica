;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/koto/locals.scm
;; Licensed under the Apache License 2.0
; Scopes
(function
  body: (_) @local.scope
)

; Definitions
(arg
  (variable) @local.definition.parameter
)

(assign
  (identifier) @local.definition.var
)

(for_args
  (variable) @local.definition.var
)

(match_patterns
  (variable) @local.definition.var
)

(import_item
  (identifier) @local.definition.import
)

(entry_block
  (identifier) @local.definition.field
)

(entry_inline
  (identifier) @local.definition.field
)

; References
(identifier) @local.reference

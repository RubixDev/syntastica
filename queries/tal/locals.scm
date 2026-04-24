;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/uxntal/locals.scm
;; Licensed under the Apache License 2.0
; Scopes
[
  (program)
  (macro)
  (memory_execution)
  (subroutine)
] @local.scope

; References
(identifier) @local.reference

; Definitions
(label
  "@"
  .
  (identifier) @local.definition.function
)

(macro
  "%"
  .
  (identifier) @local.definition.macro
)

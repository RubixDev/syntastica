;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/sway/locals.scm
;; Licensed under the Apache License 2.0
; Scopes
[
  (function_item)
  (closure_expression)
  (block)
] @local.scope

; Definitions
(parameter
  (identifier) @local.definition
)

(closure_parameters
  (identifier) @local.definition
)

; References
(identifier) @local.reference

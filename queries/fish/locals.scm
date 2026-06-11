;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/fish/locals.scm
;; Licensed under the Apache License 2.0
; Scopes
[
  (command)
  (function_definition)
  (if_statement)
  (for_statement)
  (begin_statement)
  (while_statement)
  (switch_statement)
] @local.scope

; Definitions
(function_definition
  name: (word) @local.definition.function
)

; References
(variable_name) @local.reference

(word) @local.reference

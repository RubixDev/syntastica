;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/bash/locals.scm
;; Licensed under the Apache License 2.0
; Scopes
(function_definition) @scope

; Definitions
(variable_assignment
  name: (variable_name) @definition.var
)

(function_definition
  name: (word) @definition.function
)

; References
(variable_name) @reference

(word) @reference

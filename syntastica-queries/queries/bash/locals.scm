;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/bash/locals.scm
;; Licensed under the Apache License 2.0

; Scopes
(function_definition) @local.scope

; Definitions
(variable_assignment
  name: (variable_name) @local.definition)

(function_definition
  name: (word) @local.definition)

; References
(variable_name) @local.reference
(word) @local.reference

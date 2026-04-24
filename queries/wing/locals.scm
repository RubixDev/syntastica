;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/wing/locals.scm
;; Licensed under the Apache License 2.0
(block) @local.scope

(variable_definition_statement
  name: (identifier) @local.definition
)

; TODO: Missing "@local.reference" usage tuned for each relevant identifier location

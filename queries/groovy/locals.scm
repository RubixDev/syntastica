;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/groovy/locals.scm
;; Licensed under the Apache License 2.0
(function_definition) @local.scope

(parameter
  name: (identifier) @local.definition.parameter
)

(identifier) @local.reference

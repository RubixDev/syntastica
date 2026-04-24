;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/circom/locals.scm
;; Licensed under the Apache License 2.0
(function_definition) @local.scope

(template_definition) @local.scope

(main_component_definition) @local.scope

(block_statement) @local.scope

(parameter
  name: (identifier) @local.definition
) @local.definition

(identifier) @local.reference

;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/rescript/locals.scm
;; Licensed under the Apache License 2.0
(switch_expression) @local.scope

; Definitions
;------------
(type_declaration) @local.definition.type

(let_binding) @local.definition.var

(module_declaration) @local.definition.namespace

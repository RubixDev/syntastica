;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ispc/locals.scm
;; Licensed under the Apache License 2.0
; inherits: c
(reference_declarator
  (identifier) @local.definition.var
)

(type_parameter_declaration
  (type_identifier) @local.definition.type
)

(template_declaration) @local.scope

(template_function
  name: (identifier) @local.definition.function
) @local.scope

[
  (foreach_statement)
  (foreach_instance_statement)
  (unmasked_statement)
] @local.scope

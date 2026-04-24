;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/sourcepawn/locals.scm
;; Licensed under the Apache License 2.0
[
  (function_definition)
  (alias_declaration)
  (enum_struct_method)
  (methodmap_method)
  (methodmap_method_constructor)
  (methodmap_method_destructor)
  (methodmap_property_method)
] @local.scope

; Definitions
(variable_declaration
  name: (identifier) @local.definition
)

(old_variable_declaration
  name: (identifier) @local.definition
)

; References
(identifier) @local.reference

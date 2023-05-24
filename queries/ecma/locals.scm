;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ecma/locals.scm
;; Licensed under the Apache License 2.0
; Scopes
;-------
(statement_block) @local.scope

(function) @local.scope

(arrow_function) @local.scope

(function_declaration) @local.scope

(method_definition) @local.scope

(for_statement) @local.scope

(for_in_statement) @local.scope

(catch_clause) @local.scope

; Definitions
;------------
(variable_declarator
  name: (identifier) @local.definition
)

(import_specifier
  (identifier) @local.definition
)

(namespace_import
  (identifier) @local.definition
)

(function_declaration
  (
    (identifier) @local.definition
  )
  (#set! definition.var.scope parent)
)

(method_definition
  (
    (property_identifier) @local.definition
  )
  (#set! definition.var.scope parent)
)

; References
;------------
(identifier) @local.reference

(shorthand_property_identifier) @local.reference

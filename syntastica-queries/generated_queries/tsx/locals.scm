(
  (statement_block) @local.scope
)

(
  (function) @local.scope
)

(
  (arrow_function) @local.scope
)

(
  (function_declaration) @local.scope
)

(
  (method_definition) @local.scope
)

(
  (for_statement) @local.scope
)

(
  (for_in_statement) @local.scope
)

(
  (catch_clause) @local.scope
)

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

(
  (identifier) @local.reference
)

(
  (shorthand_property_identifier) @local.reference
)

(required_parameter
  (identifier) @local.definition
)

(optional_parameter
  (identifier) @local.definition
)

(arrow_function
  parameter: (identifier) @local.definition
)

(required_parameter
  (object_pattern
    (shorthand_property_identifier_pattern) @local.definition
  )
)

(required_parameter
  (object_pattern
    (pair_pattern
      value: (identifier) @local.definition
    )
  )
)

(required_parameter
  (array_pattern
    (identifier) @local.definition
  )
)

(required_parameter
  (rest_pattern
    (identifier) @local.definition
  )
)

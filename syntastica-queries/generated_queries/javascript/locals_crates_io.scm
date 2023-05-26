(statement_block) @local.scope

(function) @local.scope

(arrow_function) @local.scope

(function_declaration) @local.scope

(method_definition) @local.scope

(for_statement) @local.scope

(for_in_statement) @local.scope

(catch_clause) @local.scope

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

(identifier) @local.reference

(shorthand_property_identifier) @local.reference

(field_definition
  property: [
    (property_identifier)
    (private_property_identifier)
  ] @local.definition
)

(assignment_expression
  left: (member_expression
    object: (this)
    property: (property_identifier) @local.definition
  )
)

(formal_parameters
  (identifier) @local.definition
)

(formal_parameters
  (assignment_pattern
    left: (identifier) @local.definition
  )
)

(arrow_function
  parameter: (identifier) @local.definition
)

(formal_parameters
  (object_pattern
    (shorthand_property_identifier_pattern) @local.definition
  )
)

(formal_parameters
  (object_pattern
    (pair_pattern
      value: (identifier) @local.definition
    )
  )
)

(formal_parameters
  (array_pattern
    (identifier) @local.definition
  )
)

(formal_parameters
  (rest_pattern
    (identifier) @local.definition
  )
)

(method_definition
  (
    [
      (property_identifier)
      (private_property_identifier)
    ] @local.definition
  )
  (#set! definition.var.scope parent)
)

(member_expression
  object: (this)
  property: (property_identifier) @local.reference
)

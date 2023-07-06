(variable_declarator
  .
  (identifier) @local.definition
)

(variable_declarator
  (tuple_pattern
    (identifier) @local.definition
  )
)

(declaration_expression
  name: (identifier) @local.definition
)

(for_each_statement
  left: (identifier) @local.definition
)

(for_each_statement
  left: (tuple_pattern
    (identifier) @local.definition
  )
)

(parameter
  (identifier) @local.definition
)

(method_declaration
  name: (identifier) @local.definition
)

(local_function_statement
  name: (identifier) @local.definition
)

(property_declaration
  name: (identifier) @local.definition
)

(type_parameter
  (identifier) @local.definition
)

(class_declaration
  name: (identifier) @local.definition
)

(identifier) @local.reference

(block) @local.scope

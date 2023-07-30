(program) @local.scope

(class_declaration
  body: (_) @local.scope
)

(record_declaration
  body: (_) @local.scope
)

(enum_declaration
  body: (_) @local.scope
)

(lambda_expression) @local.scope

(enhanced_for_statement) @local.scope

(block) @local.scope

(if_statement) @local.scope

(if_statement
  consequence: (_) @local.scope
)

(if_statement
  alternative: (_) @local.scope
)

(try_statement) @local.scope

(catch_clause) @local.scope

(for_statement) @local.scope

(for_statement
  body: (_) @local.scope
)

(do_statement
  body: (_) @local.scope
)

(while_statement
  body: (_) @local.scope
)

(constructor_declaration) @local.scope

(method_declaration) @local.scope

(package_declaration
  (identifier) @local.definition
)

(class_declaration
  name: (identifier) @local.definition
)

(record_declaration
  name: (identifier) @local.definition
)

(enum_declaration
  name: (identifier) @local.definition
)

(method_declaration
  name: (identifier) @local.definition
)

(local_variable_declaration
  declarator: (variable_declarator
    name: (identifier) @local.definition
  )
)

(enhanced_for_statement
  name: (identifier) @local.definition
)

(formal_parameter
  name: (identifier) @local.definition
)

(catch_formal_parameter
  name: (identifier) @local.definition
)

(inferred_parameters
  (identifier) @local.definition
)

(lambda_expression
  parameters: (identifier) @local.definition
)

(
  (scoped_identifier
    (identifier) @local.definition
  )
  (#has-ancestor? @local.definition import_declaration)
)

(field_declaration
  declarator: (variable_declarator
    name: (identifier) @local.definition
  )
)

(identifier) @local.reference

(type_identifier) @local.reference

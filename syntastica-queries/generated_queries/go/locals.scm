(
  (block) @local.scope
)

(parameter_declaration
  (identifier) @local.definition
)

(variadic_parameter_declaration
  (identifier) @local.definition
)

(short_var_declaration
  left: (expression_list
    (identifier) @local.definition
  )
)

(var_spec
  (identifier) @local.definition
)

(for_statement
  (range_clause
    left: (expression_list
      (identifier) @local.definition
    )
  )
)

(const_declaration
  (const_spec
    name: (identifier) @local.definition
  )
)

(
  (identifier) @local.reference
)

(
  (field_identifier) @local.reference
)

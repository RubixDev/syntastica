(assignment
  (identifier) @local.definition
)

(assignment
  (tuple_expression
    (identifier) @local.definition
  )
)

(let_binding
  (identifier) @local.definition
)

(let_binding
  (tuple_expression
    (identifier) @local.definition
  )
)

(for_binding
  (identifier) @local.definition
)

(for_binding
  (tuple_expression
    (identifier) @local.definition
  )
)

(struct_definition
  name: (identifier) @local.definition
)

(abstract_definition
  name: (identifier) @local.definition
)

(abstract_definition
  name: (identifier) @local.definition
)

(type_parameter_list
  (identifier) @local.definition
)

(import_statement
  (identifier) @local.definition
)

(parameter_list
  (identifier) @local.definition
)

(optional_parameter
  .
  (identifier) @local.definition
)

(slurp_parameter
  (identifier) @local.definition
)

(typed_parameter
  parameter: (identifier) @local.definition
  (_)
)

(function_expression
  .
  (identifier) @local.definition
)

(function_definition
  name: (identifier) @local.definition
) @local.scope

(short_function_definition
  name: (identifier) @local.definition
) @local.scope

(macro_definition
  name: (identifier) @local.definition
) @local.scope

(identifier) @local.reference

[
  (for_statement)
  (while_statement)
  (try_statement)
  (catch_clause)
  (finally_clause)
  (let_statement)
  (quote_statement)
  (do_clause)
] @local.scope

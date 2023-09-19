(function_definition) @local.scope

(variable_assignment
  name: (variable_name) @local.definition
)

(function_definition
  name: (word) @local.definition
)

(variable_name) @local.reference

(word) @local.reference

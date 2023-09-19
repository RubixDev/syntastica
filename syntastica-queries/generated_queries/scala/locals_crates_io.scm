[
  (template_body)
  (lambda_expression)
  (function_definition)
  (block)
] @local.scope

(identifier) @local.reference

(function_declaration
  name: (identifier) @local.definition
)

(function_definition
  name: (identifier) @local.definition
)

(parameter
  name: (identifier) @local.definition
)

(class_parameter
  name: (identifier) @local.definition
)

(binding
  name: (identifier) @local.definition
)

(val_definition
  pattern: (identifier) @local.definition
)

(var_definition
  pattern: (identifier) @local.definition
)

(val_declaration
  name: (identifier) @local.definition
)

(var_declaration
  name: (identifier) @local.definition
)

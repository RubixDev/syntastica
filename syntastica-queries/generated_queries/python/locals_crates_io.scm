[
  (module)
  (function_definition)
  (lambda)
] @local.scope

(parameters
  (identifier) @local.definition
)

(parameters
  (typed_parameter
    (identifier) @local.definition
  )
)

(parameters
  (default_parameter
    name: (identifier) @local.definition
  )
)

(parameters
  (typed_default_parameter
    name: (identifier) @local.definition
  )
)

(parameters
  (list_splat_pattern
    (identifier) @local.definition
  )
)

(parameters
  (dictionary_splat_pattern
    (identifier) @local.definition
  )
)

(lambda_parameters
  (identifier) @local.definition
)

(import_statement
  name: (dotted_name
    (identifier) @local.definition
  )
)

(aliased_import
  alias: (identifier) @local.definition
)

(identifier) @local.reference

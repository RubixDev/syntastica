(method) @local.scope

(class) @local.scope

[
  (block)
  (do_block)
] @local.scope

(identifier) @local.reference

(constant) @local.reference

(instance_variable) @local.reference

(module
  name: (constant) @local.definition
)

(class
  name: (constant) @local.definition
)

(method
  name: [
    (identifier)
    (constant)
  ] @local.definition
)

(singleton_method
  name: [
    (identifier)
    (constant)
  ] @local.definition
)

(method_parameters
  (identifier) @local.definition
)

(lambda_parameters
  (identifier) @local.definition
)

(block_parameters
  (identifier) @local.definition
)

(splat_parameter
  (identifier) @local.definition
)

(hash_splat_parameter
  (identifier) @local.definition
)

(optional_parameter
  name: (identifier) @local.definition
)

(destructured_parameter
  (identifier) @local.definition
)

(block_parameter
  name: (identifier) @local.definition
)

(keyword_parameter
  name: (identifier) @local.definition
)

(assignment
  left: (_) @local.definition
)

(left_assignment_list
  (identifier) @local.definition
)

(rest_assignment
  (identifier) @local.definition
)

(destructured_left_assignment
  (identifier) @local.definition
)

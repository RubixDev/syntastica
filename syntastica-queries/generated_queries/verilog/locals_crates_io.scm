[
  (loop_generate_construct)
  (loop_statement)
  (conditional_statement)
  (case_item)
  (function_declaration)
  (always_construct)
  (module_declaration)
] @local.scope

(data_declaration
  (list_of_variable_decl_assignments
    (variable_decl_assignment
      (simple_identifier) @local.definition
    )
  )
)

(genvar_initialization
  (genvar_identifier
    (simple_identifier) @local.definition
  )
)

(for_initialization
  (for_variable_declaration
    (simple_identifier) @local.definition
  )
)

(net_declaration
  (list_of_net_decl_assignments
    (net_decl_assignment
      (simple_identifier) @local.definition
    )
  )
)

(ansi_port_declaration
  (port_identifier
    (simple_identifier) @local.definition
  )
)

(parameter_declaration
  (list_of_param_assignments
    (param_assignment
      (parameter_identifier
        (simple_identifier) @local.definition
      )
    )
  )
)

(local_parameter_declaration
  (list_of_param_assignments
    (param_assignment
      (parameter_identifier
        (simple_identifier) @local.definition
      )
    )
  )
)

(function_declaration
  (function_body_declaration
    (function_identifier
      (function_identifier
        (simple_identifier) @local.definition
      )
    )
  )
)

(tf_port_item1
  (port_identifier
    (simple_identifier) @local.definition
  )
)

(simple_identifier) @local.reference

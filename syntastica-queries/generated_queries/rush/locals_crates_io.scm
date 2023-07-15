(block) @local.scope

(function_definition
  name: (ident) @local.definition
)

(parameter
  name: (ident) @local.definition
)

(let_stmt
  name: (ident) @local.definition
)

(for_stmt
  name: (ident) @local.definition
)

(ident) @local.reference
